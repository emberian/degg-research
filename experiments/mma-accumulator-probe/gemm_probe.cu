// Library-level steps of the GPU protocol (initiative 17, workload 3): copy bandwidth (step 4), the BabyBear^8
// inner product as batched INT8 GEMMs (step 5) and as FP16/FP8 limb GEMMs through cuBLASLt, and an
// accumulator-width probe through the GEMM API (step 3 at library level: no fragment layout needed, because
// the API fixes which products land in which dot product).  The same binary is the T4 dry run (f16 paths)
// and the sm_89+/sm_100 run (e4m3, e2m1 paths).  Exactness is decided on the host (mma_run.py) from the
// bytes this binary dumps; this file only computes on the device.
//
//   copy BYTES REPS                       cudaMemcpy D2D and a read-only streaming kernel; JSON line.
//   i8ip LOGN LAYOUT REPS DUMPDIR         N = 2^LOGN elements x 32 s8 limbs, V^T U per 32,768-element batch
//                                         (32 x 32768 x 32, cublasGemmStridedBatchedEx 8I -> 32I).
//                                         LAYOUT elem: V[n][32] (element-major); plane: per batch V[32][32768].
//                                         Dumps batches {0, mid, last} (A, B, D) and checks every output
//                                         against a SIMT int64 kernel on the device.
//   ltip TYPE LOGN LIMBS REPS FAST DUMPDIR  TYPE f16|e4m3: base-16 digits (0..15) as limbs, LIMBS per element,
//                                         per batch LIMBS x 32768 x LIMBS through cublasLtMatmul (TN, f32 out).
//   peak TYPE N REPS                      dense N^3 GEMM rate: i8, f16 (cublasGemmEx), e4m3, e2m1 (cublasLt).
//   ltprobe TYPE M N K FAST A B DOUT      D = A B (A: M x K row-major, B: N x K row-major, i.e. both
//                                         K-contiguous) for TYPE f16|e4m3|e2m1 (e2m1: two codes per byte,
//                                         element 2i in the low nibble; block scales VEC16_UE4M3 all 1.0),
//                                         once per heuristic algorithm (up to 8), plus each split-K one
//                                         re-run with split-K forced to 1.  DOUT: nalgo x N x M f32.
//
// Build: nvcc -O2 -arch=sm_XX gemm_probe.cu -lcublas -lcublasLt -o gemm_probe
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>
#include <algorithm>
#include <functional>
#include <cuda.h>
#include <cuda_fp16.h>
#include <cublas_v2.h>
#include <cublasLt.h>

#define CK(x)                                                                                        \
  do {                                                                                               \
    cudaError_t e_ = (x);                                                                            \
    if (e_ != cudaSuccess) {                                                                         \
      fprintf(stderr, "CUDA error %s at %s:%d\n", cudaGetErrorString(e_), __FILE__, __LINE__);       \
      exit(2);                                                                                       \
    }                                                                                                \
  } while (0)

static std::vector<char> slurp(const char *path) {
  FILE *f = fopen(path, "rb");
  if (!f) { fprintf(stderr, "cannot open %s\n", path); exit(2); }
  fseek(f, 0, SEEK_END);
  long n = ftell(f);
  fseek(f, 0, SEEK_SET);
  std::vector<char> v(n);
  if (n && fread(v.data(), 1, n, f) != (size_t)n) { fprintf(stderr, "short read %s\n", path); exit(2); }
  fclose(f);
  return v;
}
static void spill(const std::string &path, const void *p, size_t n) {
  FILE *f = fopen(path.c_str(), "wb");
  if (!f || fwrite(p, 1, n, f) != n) { fprintf(stderr, "cannot write %s\n", path.c_str()); exit(2); }
  fclose(f);
}
static void dump_dev(const std::string &path, const void *d, size_t n) {
  std::vector<char> h(n);
  CK(cudaMemcpy(h.data(), d, n, cudaMemcpyDeviceToHost));
  spill(path, h.data(), n);
}

struct Timer {
  cudaEvent_t a, b;
  Timer() { CK(cudaEventCreate(&a)); CK(cudaEventCreate(&b)); }
  void start() { CK(cudaEventRecord(a)); }
  float stop() { CK(cudaEventRecord(b)); CK(cudaEventSynchronize(b)); float ms; CK(cudaEventElapsedTime(&ms, a, b)); return ms; }
};
static double median(std::vector<double> v) { std::sort(v.begin(), v.end()); return v[v.size() / 2]; }

__device__ __forceinline__ uint64_t mix64(uint64_t x) {
  x ^= x >> 30; x *= 0xBF58476D1CE4E5B9ull; x ^= x >> 27; x *= 0x94D049BB133111EBull; x ^= x >> 31;
  return x;
}
// bytes from a counter hash (mode 0), or base-16 digits encoded through a 16-entry code table (modes 1, 2)
__global__ void k_fill_bytes(uint8_t *p, size_t n, uint64_t seed) {
  for (size_t i = blockIdx.x * (size_t)blockDim.x + threadIdx.x; i < n; i += (size_t)gridDim.x * blockDim.x)
    p[i] = (uint8_t)mix64(seed + i);
}
__constant__ uint16_t c_code[16];
__global__ void k_fill_digits(void *p, size_t n, uint64_t seed, int width) {
  for (size_t i = blockIdx.x * (size_t)blockDim.x + threadIdx.x; i < n; i += (size_t)gridDim.x * blockDim.x) {
    uint16_t c = c_code[mix64(seed + i) & 15];
    if (width == 2) ((uint16_t *)p)[i] = c; else ((uint8_t *)p)[i] = (uint8_t)c;
  }
}
__global__ void k_read(const uint4 *p, size_t n, uint32_t *sink) {
  uint32_t x = 0;
  for (size_t i = blockIdx.x * (size_t)blockDim.x + threadIdx.x; i < n; i += (size_t)gridDim.x * blockDim.x) {
    uint4 v = p[i]; x ^= v.x ^ v.y ^ v.z ^ v.w;
  }
  if (x == 0x9E3779B9u) sink[0] = x;
}
// SIMT int64 reference for one 32 x 32768 x 32 batch per block (independent of the tensor cores)
__global__ void k_ref_i8(const int8_t *V, const int8_t *U, int layout, const int32_t *D, unsigned long long *bad) {
  size_t b = blockIdx.x;
  for (int o = threadIdx.x; o < 1024; o += blockDim.x) {
    int i = o & 31, j = o >> 5;
    long long s = 0;
    const int8_t *v = V + b * 32768 * 32, *u = U + b * 32768 * 32;
    if (layout == 0) for (int n = 0; n < 32768; n++) s += (long long)v[n * 32 + i] * u[n * 32 + j];
    else for (int n = 0; n < 32768; n++) s += (long long)v[i * 32768 + n] * u[j * 32768 + n];
    if ((long long)D[b * 1024 + j * 32 + i] != s) atomicAdd(bad, 1ull);
  }
}

static const char *cublas_err(cublasStatus_t s) {
  switch (s) {
    case CUBLAS_STATUS_SUCCESS: return "SUCCESS";
    case CUBLAS_STATUS_NOT_SUPPORTED: return "NOT_SUPPORTED";
    case CUBLAS_STATUS_INVALID_VALUE: return "INVALID_VALUE";
    case CUBLAS_STATUS_ARCH_MISMATCH: return "ARCH_MISMATCH";
    case CUBLAS_STATUS_EXECUTION_FAILED: return "EXECUTION_FAILED";
    default: return "OTHER";
  }
}

// ------------------------------------------------------------------------------------------------ copy
static int cmd_copy(size_t bytes, int reps) {
  void *a, *b; uint32_t *sink;
  CK(cudaMalloc(&a, bytes)); CK(cudaMalloc(&b, bytes)); CK(cudaMalloc(&sink, 4));
  k_fill_bytes<<<1024, 256>>>((uint8_t *)a, bytes, 1);
  CK(cudaDeviceSynchronize());
  Timer t; std::vector<double> cp, rd;
  for (int r = 0; r < reps + 1; r++) {
    t.start(); CK(cudaMemcpy(b, a, bytes, cudaMemcpyDeviceToDevice)); double ms = t.stop();
    if (r) cp.push_back(ms);
  }
  cudaDeviceProp pr; CK(cudaGetDeviceProperties(&pr, 0));
  for (int r = 0; r < reps + 1; r++) {
    t.start(); k_read<<<pr.multiProcessorCount * 16, 256>>>((const uint4 *)a, bytes / 16, sink); double ms = t.stop();
    if (r) rd.push_back(ms);
  }
  CK(cudaGetLastError());
  double mc = median(cp), mr = median(rd);
  printf("{\"cmd\": \"copy\", \"bytes\": %zu, \"reps\": %d, \"copy_ms\": %.4f, \"copy_rw_bytes_per_s\": %.6e, "
         "\"read_ms\": %.4f, \"read_bytes_per_s\": %.6e}\n", bytes, reps, mc, 2.0 * bytes / (mc * 1e-3), mr,
         bytes / (mr * 1e-3));
  return 0;
}

// ------------------------------------------------------------------------------------------------ i8ip
static int cmd_i8ip(int logn, const char *layout, int reps, const std::string &dir) {
  const long long N = 1ll << logn, BK = 32768;
  if (N < BK) { fprintf(stderr, "i8ip: N < 32768\n"); return 1; }
  int batches = (int)(N / BK), lay = !strcmp(layout, "plane");
  size_t vb = (size_t)N * 32;
  int8_t *V, *Uu; int32_t *D; unsigned long long *bad;
  CK(cudaMalloc(&V, vb)); CK(cudaMalloc(&Uu, vb)); CK(cudaMalloc(&D, (size_t)batches * 1024 * 4));
  CK(cudaMalloc(&bad, 8)); CK(cudaMemset(bad, 0, 8)); CK(cudaMemset(D, 0xFF, (size_t)batches * 1024 * 4));
  k_fill_bytes<<<1024, 256>>>((uint8_t *)V, vb, 0x1000 + logn);
  k_fill_bytes<<<1024, 256>>>((uint8_t *)Uu, vb, 0x2000 + logn);
  CK(cudaDeviceSynchronize());
  cublasHandle_t h; cublasCreate(&h);
  int32_t one = 1, zero = 0;
  auto go = [&]() {
    if (!lay)
      return cublasGemmStridedBatchedEx(h, CUBLAS_OP_N, CUBLAS_OP_T, 32, 32, (int)BK, &one, V, CUDA_R_8I, 32, BK * 32,
                                        Uu, CUDA_R_8I, 32, BK * 32, &zero, D, CUDA_R_32I, 32, 1024, batches,
                                        CUBLAS_COMPUTE_32I, CUBLAS_GEMM_DEFAULT);
    return cublasGemmStridedBatchedEx(h, CUBLAS_OP_T, CUBLAS_OP_N, 32, 32, (int)BK, &one, V, CUDA_R_8I, (int)BK, BK * 32,
                                      Uu, CUDA_R_8I, (int)BK, BK * 32, &zero, D, CUDA_R_32I, 32, 1024, batches,
                                      CUBLAS_COMPUTE_32I, CUBLAS_GEMM_DEFAULT);
  };
  cublasStatus_t st = go();
  CK(cudaDeviceSynchronize());
  if (st != CUBLAS_STATUS_SUCCESS) {
    printf("{\"cmd\": \"i8ip\", \"logn\": %d, \"layout\": \"%s\", \"status\": \"%s\"}\n", logn, layout, cublas_err(st));
    return 0;
  }
  Timer t; std::vector<double> ms;
  for (int r = 0; r < reps; r++) { t.start(); go(); ms.push_back(t.stop()); }
  CK(cudaGetLastError());
  k_ref_i8<<<batches, 256>>>(V, Uu, lay, D, bad);
  unsigned long long nbad; CK(cudaMemcpy(&nbad, bad, 8, cudaMemcpyDeviceToHost));
  int pick[3] = {0, batches / 2, batches - 1};
  for (int q = 0; q < 3; q++) {
    size_t b = pick[q];
    std::string s = dir + "/i8ip_" + layout + "_" + std::to_string(logn) + "_b" + std::to_string(b);
    dump_dev(s + "_V.bin", V + b * BK * 32, BK * 32);
    dump_dev(s + "_U.bin", Uu + b * BK * 32, BK * 32);
    dump_dev(s + "_D.bin", D + b * 1024, 4096);
  }
  double m = median(ms);
  printf("{\"cmd\": \"i8ip\", \"logn\": %d, \"layout\": \"%s\", \"status\": \"SUCCESS\", \"batches\": %d, \"reps\": %d, "
         "\"ms\": %.4f, \"ms_min\": %.4f, \"bytes_read\": %zu, \"bytes_per_s\": %.6e, \"macs_per_s\": %.6e, "
         "\"simt_ref_mismatches\": %llu, \"dumped_batches\": [%d, %d, %d]}\n",
         logn, layout, batches, reps, m, *std::min_element(ms.begin(), ms.end()), 2 * vb, 2 * vb / (m * 1e-3),
         (double)N * 1024 / (m * 1e-3), nbad, pick[0], pick[1], pick[2]);
  return 0;
}

// ------------------------------------------------------------------------------------------------ Lt
static cudaDataType lt_type(const char *t) {
  if (!strcmp(t, "f16")) return CUDA_R_16F;
  if (!strcmp(t, "e4m3")) return CUDA_R_8F_E4M3;
#if CUDA_VERSION >= 12080
  if (!strcmp(t, "e2m1")) return CUDA_R_4F_E2M1;
#endif
  fprintf(stderr, "type %s unsupported by this build\n", t); exit(1);
}
static size_t elem_bytes_x2(const char *t) { return !strcmp(t, "f16") ? 4 : !strcmp(t, "e4m3") ? 2 : 1; }  // bytes*2

struct LtGemm {
  cublasLtHandle_t lt; cublasLtMatmulDesc_t op; cublasLtMatrixLayout_t Ad, Bd, Cd; cublasLtMatmulPreference_t pref;
  void *ws = nullptr; size_t wsz = 64ull << 20; void *sa = nullptr, *sb = nullptr;
  cublasLtMatmulHeuristicResult_t res[16]; int nres = 0; cublasStatus_t st = CUBLAS_STATUS_SUCCESS;
  LtGemm(const char *type, int M, int N, int K, int batch, long long sA, long long sB, long long sC, int fast) {
    cudaDataType ty = lt_type(type);
    cublasLtCreate(&lt);
    cublasLtMatmulDescCreate(&op, CUBLAS_COMPUTE_32F, CUDA_R_32F);
    cublasOperation_t T = CUBLAS_OP_T, Nn = CUBLAS_OP_N;
    cublasLtMatmulDescSetAttribute(op, CUBLASLT_MATMUL_DESC_TRANSA, &T, sizeof(T));
    cublasLtMatmulDescSetAttribute(op, CUBLASLT_MATMUL_DESC_TRANSB, &Nn, sizeof(Nn));
    if (ty == CUDA_R_8F_E4M3) {
      int8_t fa = (int8_t)fast;
      cublasLtMatmulDescSetAttribute(op, CUBLASLT_MATMUL_DESC_FAST_ACCUM, &fa, sizeof(fa));
    }
#if CUDA_VERSION >= 12080
    if (ty == CUDA_R_4F_E2M1) {   // block scales of 1.0 (ue4m3 0x38) everywhere: the scale layout is then irrelevant
      size_t ns = (size_t)((std::max(M, N) + 127) / 128 * 128) * ((K / 16 + 3) / 4 * 4) * std::max(batch, 1);
      CK(cudaMalloc(&sa, ns)); CK(cudaMalloc(&sb, ns));
      CK(cudaMemset(sa, 0x38, ns)); CK(cudaMemset(sb, 0x38, ns));
      cublasLtMatmulMatrixScale_t sm = CUBLASLT_MATMUL_MATRIX_SCALE_VEC16_UE4M3;
      cublasLtMatmulDescSetAttribute(op, CUBLASLT_MATMUL_DESC_A_SCALE_MODE, &sm, sizeof(sm));
      cublasLtMatmulDescSetAttribute(op, CUBLASLT_MATMUL_DESC_B_SCALE_MODE, &sm, sizeof(sm));
      cublasLtMatmulDescSetAttribute(op, CUBLASLT_MATMUL_DESC_A_SCALE_POINTER, &sa, sizeof(sa));
      cublasLtMatmulDescSetAttribute(op, CUBLASLT_MATMUL_DESC_B_SCALE_POINTER, &sb, sizeof(sb));
    }
#endif
    cublasLtMatrixLayoutCreate(&Ad, ty, K, M, K);
    cublasLtMatrixLayoutCreate(&Bd, ty, K, N, K);
    cublasLtMatrixLayoutCreate(&Cd, CUDA_R_32F, M, N, M);
    if (batch > 1) {
      for (auto &pr : {std::make_pair(Ad, sA), std::make_pair(Bd, sB), std::make_pair(Cd, sC)}) {
        cublasLtMatrixLayoutSetAttribute(pr.first, CUBLASLT_MATRIX_LAYOUT_BATCH_COUNT, &batch, sizeof(batch));
        long long s = pr.second;
        cublasLtMatrixLayoutSetAttribute(pr.first, CUBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET, &s, sizeof(s));
      }
    }
    cublasLtMatmulPreferenceCreate(&pref);
    cublasLtMatmulPreferenceSetAttribute(pref, CUBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES, &wsz, sizeof(wsz));
    CK(cudaMalloc(&ws, wsz));
    st = cublasLtMatmulAlgoGetHeuristic(lt, op, Ad, Bd, Cd, Cd, pref, 8, res, &nres);
  }
  // append, for every heuristic algorithm that split K, the same algorithm with split-K forced to 1 (when
  // cublasLtMatmulAlgoCheck accepts it): one accumulator chain over the whole K
  void add_no_splitk() {
    int n0 = nres;
    for (int i = 0; i < n0 && nres < 16; i++) {
      int sk = 1; size_t w;
      cublasLtMatmulAlgoConfigGetAttribute(&res[i].algo, CUBLASLT_ALGO_CONFIG_SPLITK_NUM, &sk, sizeof(sk), &w);
      if (sk <= 1) continue;
      cublasLtMatmulHeuristicResult_t r = res[i];
      int one = 1; uint32_t none = CUBLASLT_REDUCTION_SCHEME_NONE;
      cublasLtMatmulAlgoConfigSetAttribute(&r.algo, CUBLASLT_ALGO_CONFIG_SPLITK_NUM, &one, sizeof(one));
      cublasLtMatmulAlgoConfigSetAttribute(&r.algo, CUBLASLT_ALGO_CONFIG_REDUCTION_SCHEME, &none, sizeof(none));
      if (cublasLtMatmulAlgoCheck(lt, op, Ad, Bd, Cd, Cd, &r.algo, &r) == CUBLAS_STATUS_SUCCESS && r.workspaceSize <= wsz)
        res[nres++] = r;
    }
  }
  cublasStatus_t run(int i, const void *A, const void *B, float *D) {
    float one = 1.f, zero = 0.f;
    return cublasLtMatmul(lt, op, &one, A, Ad, B, Bd, &zero, D, Cd, D, Cd, &res[i].algo, ws, wsz, 0);
  }
  std::string algo_json(int i) {
    int id = -1, tile = -1, splitk = -1, red = -1, stages = -1; size_t w;
    cublasLtMatmulAlgoConfigGetAttribute(&res[i].algo, CUBLASLT_ALGO_CONFIG_ID, &id, sizeof(id), &w);
    cublasLtMatmulAlgoConfigGetAttribute(&res[i].algo, CUBLASLT_ALGO_CONFIG_TILE_ID, &tile, sizeof(tile), &w);
    cublasLtMatmulAlgoConfigGetAttribute(&res[i].algo, CUBLASLT_ALGO_CONFIG_SPLITK_NUM, &splitk, sizeof(splitk), &w);
    cublasLtMatmulAlgoConfigGetAttribute(&res[i].algo, CUBLASLT_ALGO_CONFIG_REDUCTION_SCHEME, &red, sizeof(red), &w);
    cublasLtMatmulAlgoConfigGetAttribute(&res[i].algo, CUBLASLT_ALGO_CONFIG_STAGES_ID, &stages, sizeof(stages), &w);
    char b[200];
    snprintf(b, sizeof b, "{\"algo_id\": %d, \"tile\": %d, \"splitk\": %d, \"reduction\": %d, \"stages\": %d}", id, tile,
             splitk, red, stages);
    return b;
  }
};

static int cmd_ltprobe(const char *type, int M, int N, int K, int fast, const char *fa, const char *fb, const char *fd) {
  auto A = slurp(fa), B = slurp(fb);
  size_t ea = elem_bytes_x2(type);
  if (A.size() != (size_t)M * K * ea / 2 || B.size() != (size_t)N * K * ea / 2) { fprintf(stderr, "ltprobe: size\n"); return 2; }
  void *dA, *dB; float *dD;
  CK(cudaMalloc(&dA, A.size())); CK(cudaMalloc(&dB, B.size())); CK(cudaMalloc(&dD, (size_t)M * N * 4));
  CK(cudaMemcpy(dA, A.data(), A.size(), cudaMemcpyHostToDevice));
  CK(cudaMemcpy(dB, B.data(), B.size(), cudaMemcpyHostToDevice));
  LtGemm g(type, M, N, K, 1, 0, 0, 0, fast);
  g.add_no_splitk();
  std::string algos = "[";
  std::vector<float> all;
  int ok = 0;
  for (int i = 0; i < g.nres; i++) {
    CK(cudaMemset(dD, 0xFF, (size_t)M * N * 4));
    cublasStatus_t s = g.run(i, dA, dB, dD);
    CK(cudaDeviceSynchronize());
    if (s != CUBLAS_STATUS_SUCCESS) continue;
    std::vector<float> h((size_t)M * N);
    CK(cudaMemcpy(h.data(), dD, h.size() * 4, cudaMemcpyDeviceToHost));
    all.insert(all.end(), h.begin(), h.end());
    algos += (ok ? ", " : "") + g.algo_json(i);
    ok++;
  }
  algos += "]";
  spill(fd, all.data(), all.size() * 4);
  printf("{\"cmd\": \"ltprobe\", \"type\": \"%s\", \"M\": %d, \"N\": %d, \"K\": %d, \"fast_accum\": %d, "
         "\"heuristic_status\": \"%s\", \"n_heuristic\": %d, \"n_ran\": %d, \"algos\": %s}\n",
         type, M, N, K, fast, cublas_err(g.st), g.nres, ok, algos.c_str());
  return 0;
}

static int cmd_ltip(const char *type, int logn, int limbs, int reps, int fast, const std::string &dir) {
  const long long N = 1ll << logn, BK = 32768;
  int batches = (int)(N / BK);
  cudaDataType ty = lt_type(type);
  int w = ty == CUDA_R_16F ? 2 : 1;
  uint16_t code[16];
  for (int d = 0; d < 16; d++) {
    if (w == 2) { __half hv = __float2half((float)d); memcpy(&code[d], &hv, 2); }
    else {   // e4m3 of the integer d: exact for 0..15
      if (d == 0) code[d] = 0;
      else { int e = 31 - __builtin_clz(d); int man = ((d << 3) >> e) & 7; code[d] = (uint16_t)(((e + 7) << 3) | man); }
    }
  }
  CK(cudaMemcpyToSymbol(c_code, code, sizeof code));
  size_t vb = (size_t)N * limbs * w;
  void *V, *U; float *D;
  CK(cudaMalloc(&V, vb)); CK(cudaMalloc(&U, vb)); CK(cudaMalloc(&D, (size_t)batches * limbs * limbs * 4));
  k_fill_digits<<<1024, 256>>>(V, (size_t)N * limbs, 0x3000 + logn, w);
  k_fill_digits<<<1024, 256>>>(U, (size_t)N * limbs, 0x4000 + logn, w);
  CK(cudaDeviceSynchronize());
  LtGemm g(type, limbs, limbs, (int)BK, batches, BK * limbs, BK * limbs, (long long)limbs * limbs, fast);
  if (g.nres == 0) {
    printf("{\"cmd\": \"ltip\", \"type\": \"%s\", \"logn\": %d, \"status\": \"no_algo %s\"}\n", type, logn, cublas_err(g.st));
    return 0;
  }
  cublasStatus_t st = g.run(0, V, U, D);
  CK(cudaDeviceSynchronize());
  if (st != CUBLAS_STATUS_SUCCESS) {
    printf("{\"cmd\": \"ltip\", \"type\": \"%s\", \"logn\": %d, \"status\": \"%s\"}\n", type, logn, cublas_err(st));
    return 0;
  }
  Timer t; std::vector<double> ms;
  for (int r = 0; r < reps; r++) { t.start(); g.run(0, V, U, D); ms.push_back(t.stop()); }
  CK(cudaGetLastError());
  int pick[3] = {0, batches / 2, batches - 1};
  for (int q = 0; q < 3; q++) {
    size_t b = pick[q];
    std::string s = dir + "/ltip_" + type + "_" + std::to_string(logn) + "_b" + std::to_string(b);
    dump_dev(s + "_V.bin", (char *)V + b * BK * limbs * w, BK * limbs * w);
    dump_dev(s + "_U.bin", (char *)U + b * BK * limbs * w, BK * limbs * w);
    dump_dev(s + "_D.bin", D + b * limbs * limbs, (size_t)limbs * limbs * 4);
  }
  double m = median(ms);
  printf("{\"cmd\": \"ltip\", \"type\": \"%s\", \"logn\": %d, \"limbs\": %d, \"fast_accum\": %d, \"status\": \"SUCCESS\", "
         "\"batches\": %d, \"reps\": %d, \"ms\": %.4f, \"bytes_read\": %zu, \"bytes_per_s\": %.6e, \"macs_per_s\": %.6e, "
         "\"algo\": %s, \"dumped_batches\": [%d, %d, %d]}\n",
         type, logn, limbs, fast, batches, reps, m, 2 * vb, 2 * vb / (m * 1e-3), (double)N * limbs * limbs / (m * 1e-3),
         g.algo_json(0).c_str(), pick[0], pick[1], pick[2]);
  return 0;
}

// ------------------------------------------------------------------------------------------------ peak
// Dense square GEMM rate (the attainable tensor rate the floors divide by): TYPE i8 (cublasGemmEx 8I -> 32I),
// f16 (cublasGemmEx, f32 accumulate and output), e4m3 / e2m1 (cublasLt, f32 output).  Operands are hashed bytes
// (f16: small values via the digit table); only the rate is reported.
static int cmd_peak(const char *type, int n, int reps) {
  size_t ne = (size_t)n * n;
  bool isf16 = !strcmp(type, "f16"), isi8 = !strcmp(type, "i8"), isfp4 = !strcmp(type, "e2m1");
  size_t eb = isf16 ? 2 : 1;
  size_t ab = isfp4 ? ne / 2 : ne * eb;
  void *A, *B, *C;
  CK(cudaMalloc(&A, ab)); CK(cudaMalloc(&B, ab)); CK(cudaMalloc(&C, ne * 4));
  uint16_t code[16];
  for (int d = 0; d < 16; d++) {
    if (isf16) { __half hv = __float2half((float)(d - 8)); memcpy(&code[d], &hv, 2); }
    else if (isi8) code[d] = (uint16_t)(uint8_t)(d - 8);
    else if (isfp4) code[d] = (uint16_t)(d * 0x11);          // two e2m1 codes per byte
    else { int v = d & 7; code[d] = v ? (uint16_t)(((((31 - __builtin_clz(v)) + 7) << 3) | (((v << 3) >> (31 - __builtin_clz(v))) & 7)) | (d & 8 ? 0x80 : 0)) : 0; }
  }
  CK(cudaMemcpyToSymbol(c_code, code, sizeof code));
  k_fill_digits<<<1024, 256>>>(A, ab / eb, 11, (int)eb);
  k_fill_digits<<<1024, 256>>>(B, ab / eb, 12, (int)eb);
  CK(cudaDeviceSynchronize());
  std::function<cublasStatus_t()> go;
  cublasHandle_t h; cublasCreate(&h);
  LtGemm *g = nullptr;
  int32_t ione = 1, izero = 0; float fone = 1.f, fzero = 0.f;
  if (isi8)
    go = [&]() { return cublasGemmEx(h, CUBLAS_OP_T, CUBLAS_OP_N, n, n, n, &ione, A, CUDA_R_8I, n, B, CUDA_R_8I, n, &izero,
                                     C, CUDA_R_32I, n, CUBLAS_COMPUTE_32I, CUBLAS_GEMM_DEFAULT); };
  else if (isf16)
    go = [&]() { return cublasGemmEx(h, CUBLAS_OP_T, CUBLAS_OP_N, n, n, n, &fone, A, CUDA_R_16F, n, B, CUDA_R_16F, n, &fzero,
                                     C, CUDA_R_32F, n, CUBLAS_COMPUTE_32F, CUBLAS_GEMM_DEFAULT); };
  else {
    g = new LtGemm(type, n, n, n, 1, 0, 0, 0, 1);
    if (g->nres == 0) {
      printf("{\"cmd\": \"peak\", \"type\": \"%s\", \"n\": %d, \"status\": \"no_algo %s\"}\n", type, n, cublas_err(g->st));
      return 0;
    }
    go = [&]() { return g->run(0, A, B, (float *)C); };
  }
  cublasStatus_t st = go();
  CK(cudaDeviceSynchronize());
  if (st != CUBLAS_STATUS_SUCCESS) {
    printf("{\"cmd\": \"peak\", \"type\": \"%s\", \"n\": %d, \"status\": \"%s\"}\n", type, n, cublas_err(st));
    return 0;
  }
  Timer t; std::vector<double> ms;
  for (int r = 0; r < reps; r++) { t.start(); go(); ms.push_back(t.stop()); }
  CK(cudaGetLastError());
  double m = median(ms);
  printf("{\"cmd\": \"peak\", \"type\": \"%s\", \"n\": %d, \"status\": \"SUCCESS\", \"reps\": %d, \"ms\": %.4f, "
         "\"macs_per_s\": %.6e, \"algo\": %s}\n", type, n, reps, m, (double)n * n * n / (m * 1e-3),
         g ? g->algo_json(0).c_str() : "null");
  return 0;
}

int main(int argc, char **argv) {
  if (argc < 2) { fprintf(stderr, "usage: see header\n"); return 1; }
  std::string c = argv[1];
  if (c == "peak" && argc == 5) return cmd_peak(argv[2], atoi(argv[3]), atoi(argv[4]));
  if (c == "copy" && argc == 4) return cmd_copy(strtoull(argv[2], 0, 10), atoi(argv[3]));
  if (c == "i8ip" && argc == 6) return cmd_i8ip(atoi(argv[2]), argv[3], atoi(argv[4]), argv[5]);
  if (c == "ltip" && argc == 8) return cmd_ltip(argv[2], atoi(argv[3]), atoi(argv[4]), atoi(argv[5]), atoi(argv[6]), argv[7]);
  if (c == "ltprobe" && argc == 10)
    return cmd_ltprobe(argv[2], atoi(argv[3]), atoi(argv[4]), atoi(argv[5]), atoi(argv[6]), argv[7], argv[8], argv[9]);
  fprintf(stderr, "bad arguments\n");
  return 1;
}
