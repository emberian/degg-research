// Rotation probes (initiative 14 ROTATION.md section 5, run by initiative 17): the exact-integer datapaths that
// replace a narrow integer unit.  mma.sync m16n8k8 f32.f16.f16.f32 only, so every kernel runs on sm_75 (T4) and
// on every later arch (HMMA).  Checks are decided on the host (mma_run.py).
//
//   f16parity PARAMS A B OUT   0/1 operands (f16 0x0000 / 0x3C00), accumulator preset to C (raw f32 bits).  Data:
//                              P instructions of per-lane words A[P][32][2], B[P][32] (uint32 = two f16).  Test t
//                              (PARAMS int64[t][4] = (P_used, R, T, cbits)) chains R passes over the first P_used
//                              instructions then T more: count = R * c(P_used) + c(T).  OUT: raw D words [t][32][4].
//                              The bias read: C = 2^(m-1) keeps D in [2^(m-1), 2^m) while count < 2^(m-1), so raw
//                              f32 bit (24 - m) of D is the parity of the count (ROTATION section 2(b)).
//   f16ip LOGN REPS DUMPDIR    BabyBear^8 inner product V^T U with byte limbs as f16 (values 0..255), limb planes
//                              V[32][N], U[32][N]: f32 fragments accumulate 256 products (< 258, the block at m = 24:
//                              256 * 65025 < 2^24), then are promoted (cvt.rni.u32.f32, add to u64).  Every output
//                              against a SIMT int64 kernel; N <= 2^22 also dumped for the host.
//
// Fragment layout (PTX "Matrix fragments for mma.m16n8k8", f16): g = lane>>2, q = lane&3; a0 = A[g][2q..2q+1],
// a1 = A[g+8][2q..2q+1], b0 = B[2q..2q+1][g], d0,d1 = D[g][2q..2q+1], d2,d3 = D[g+8][2q..2q+1].
//
// Build: nvcc -O2 -arch=sm_75 rotation_probe.cu -o rotation_probe
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>
#include <algorithm>
#include <cuda_fp16.h>

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
template <class T> static T *upload(const std::vector<char> &h) {
  T *d; CK(cudaMalloc(&d, h.size() ? h.size() : 4));
  if (h.size()) CK(cudaMemcpy(d, h.data(), h.size(), cudaMemcpyHostToDevice));
  return d;
}

__device__ __forceinline__ void hmma(float *d, uint32_t a0, uint32_t a1, uint32_t b0) {
  asm volatile("mma.sync.aligned.m16n8k8.row.col.f32.f16.f16.f32 {%0,%1,%2,%3}, {%4,%5}, {%6}, {%0,%1,%2,%3};\n"
               : "+f"(d[0]), "+f"(d[1]), "+f"(d[2]), "+f"(d[3]) : "r"(a0), "r"(a1), "r"(b0));
}

// ------------------------------------------------------------------------------------------------ f16parity
__global__ void k_f16parity(const long long *prm, const uint32_t *A, const uint32_t *B, uint32_t *out) {
  const long long *p = prm + 4 * blockIdx.x;
  int lane = threadIdx.x;
  long long P = p[0], R = p[1], T = p[2];
  float c = __uint_as_float((uint32_t)p[3]);
  float d[4] = {c, c, c, c};
  for (long long r = 0; r < R; r++)
    for (long long i = 0; i < P; i++)
      hmma(d, A[(i * 32 + lane) * 2], A[(i * 32 + lane) * 2 + 1], B[i * 32 + lane]);
  for (long long i = 0; i < T; i++)
    hmma(d, A[(i * 32 + lane) * 2], A[(i * 32 + lane) * 2 + 1], B[i * 32 + lane]);
  for (int j = 0; j < 4; j++) out[(blockIdx.x * 32 + lane) * 4 + j] = __float_as_uint(d[j]);
}

// ------------------------------------------------------------------------------------------------ f16ip
__device__ __forceinline__ uint64_t mix64(uint64_t x) {
  x ^= x >> 30; x *= 0xBF58476D1CE4E5B9ull; x ^= x >> 27; x *= 0x94D049BB133111EBull; x ^= x >> 31;
  return x;
}
__global__ void k_fill_bytes_f16(__half *p, size_t n, uint64_t seed) {
  for (size_t i = blockIdx.x * (size_t)blockDim.x + threadIdx.x; i < n; i += (size_t)gridDim.x * blockDim.x)
    p[i] = __ushort2half_rn((unsigned short)(mix64(seed + i) & 0xFF));
}

// one warp: a 32 x 32 output over chunks of 256 elements; chunks strided over all warps; u64 atomics at the end
__global__ void __launch_bounds__(128) k_f16ip(const __half *V, const __half *U, long long N, unsigned long long *out) {
  int lane = threadIdx.x & 31, g = lane >> 2, q = lane & 3;
  long long warp = (blockIdx.x * (long long)blockDim.x + threadIdx.x) >> 5, nwarps = (gridDim.x * (long long)blockDim.x) >> 5;
  const uint32_t *v32 = reinterpret_cast<const uint32_t *>(V), *u32 = reinterpret_cast<const uint32_t *>(U);
  long long N2 = N / 2;   // plane length in uint32 words
  unsigned long long acc[2][4][4];
#pragma unroll
  for (int a = 0; a < 2; a++)
#pragma unroll
    for (int b = 0; b < 4; b++)
#pragma unroll
      for (int j = 0; j < 4; j++) acc[a][b][j] = 0;
  for (long long ch = warp; ch < N / 256; ch += nwarps) {
    float f[2][4][4];
#pragma unroll
    for (int a = 0; a < 2; a++)
#pragma unroll
      for (int b = 0; b < 4; b++)
#pragma unroll
        for (int j = 0; j < 4; j++) f[a][b][j] = 0.f;
    long long w0 = ch * 128;   // first uint32 word of the chunk in each plane
#pragma unroll 4
    for (int ks = 0; ks < 32; ks++) {
      long long w = w0 + ks * 4 + q;
      uint32_t a[2][2], b[4];
#pragma unroll
      for (int mt = 0; mt < 2; mt++) {
        a[mt][0] = __ldg(v32 + (long long)(mt * 16 + g) * N2 + w);
        a[mt][1] = __ldg(v32 + (long long)(mt * 16 + g + 8) * N2 + w);
      }
#pragma unroll
      for (int nt = 0; nt < 4; nt++) b[nt] = __ldg(u32 + (long long)(nt * 8 + g) * N2 + w);
#pragma unroll
      for (int mt = 0; mt < 2; mt++)
#pragma unroll
        for (int nt = 0; nt < 4; nt++) hmma(f[mt][nt], a[mt][0], a[mt][1], b[nt]);
    }
#pragma unroll
    for (int a = 0; a < 2; a++)
#pragma unroll
      for (int b = 0; b < 4; b++)
#pragma unroll
        for (int j = 0; j < 4; j++) acc[a][b][j] += (unsigned long long)__float2uint_rn(f[a][b][j]);
  }
#pragma unroll
  for (int mt = 0; mt < 2; mt++)
#pragma unroll
    for (int nt = 0; nt < 4; nt++)
#pragma unroll
      for (int j = 0; j < 4; j++) {
        int row = mt * 16 + g + (j >= 2 ? 8 : 0), col = nt * 8 + q * 2 + (j & 1);
        atomicAdd(out + row * 32 + col, acc[mt][nt][j]);
      }
}
// SIMT reference: block (row, col), threads stride over N
__global__ void k_f16ip_ref(const __half *V, const __half *U, long long N, unsigned long long *out) {
  int row = blockIdx.x >> 5, col = blockIdx.x & 31;
  unsigned long long s = 0;
  for (long long n = blockIdx.y * (long long)blockDim.x + threadIdx.x; n < N; n += (long long)gridDim.y * blockDim.x)
    s += (unsigned long long)__half2uint_rn(V[row * N + n]) * __half2uint_rn(U[col * N + n]);
  atomicAdd(out + row * 32 + col, s);
}

int main(int argc, char **argv) {
  if (argc < 2) { fprintf(stderr, "usage: see header\n"); return 1; }
  std::string cmd = argv[1];
  if (cmd == "f16parity" && argc == 6) {
    auto prm = slurp(argv[2]), A = slurp(argv[3]), B = slurp(argv[4]);
    int T = prm.size() / 32;
    uint32_t *dout; CK(cudaMalloc(&dout, (size_t)T * 512));
    cudaEvent_t e0, e1; CK(cudaEventCreate(&e0)); CK(cudaEventCreate(&e1));
    CK(cudaEventRecord(e0));
    k_f16parity<<<T, 32>>>(upload<long long>(prm), upload<uint32_t>(A), upload<uint32_t>(B), dout);
    CK(cudaEventRecord(e1)); CK(cudaGetLastError()); CK(cudaEventSynchronize(e1));
    float ms; CK(cudaEventElapsedTime(&ms, e0, e1));
    std::vector<uint32_t> h((size_t)T * 128);
    CK(cudaMemcpy(h.data(), dout, h.size() * 4, cudaMemcpyDeviceToHost));
    spill(argv[5], h.data(), h.size() * 4);
    printf("{\"cmd\": \"f16parity\", \"tests\": %d, \"ms\": %.3f}\n", T, ms);
    return 0;
  }
  if (cmd == "f16ip" && argc == 5) {
    int logn = atoi(argv[2]), reps = atoi(argv[3]);
    std::string dir = argv[4];
    long long N = 1ll << logn;
    if (N < 256) { fprintf(stderr, "f16ip: N < 256\n"); return 1; }
    size_t pb = (size_t)N * 32 * 2;
    __half *V, *U; unsigned long long *D, *R;
    CK(cudaMalloc(&V, pb)); CK(cudaMalloc(&U, pb)); CK(cudaMalloc(&D, 8192)); CK(cudaMalloc(&R, 8192));
    k_fill_bytes_f16<<<1024, 256>>>(V, (size_t)N * 32, 0x5000 + logn);
    k_fill_bytes_f16<<<1024, 256>>>(U, (size_t)N * 32, 0x6000 + logn);
    cudaDeviceProp pr; CK(cudaGetDeviceProperties(&pr, 0));
    int blocks = std::min<long long>(pr.multiProcessorCount * 8, std::max<long long>(1, N / 256 / 4));
    cudaEvent_t e0, e1; CK(cudaEventCreate(&e0)); CK(cudaEventCreate(&e1));
    std::vector<double> ms;
    for (int r = 0; r < reps + 1; r++) {
      CK(cudaMemset(D, 0, 8192));
      CK(cudaEventRecord(e0)); k_f16ip<<<blocks, 128>>>(V, U, N, D); CK(cudaEventRecord(e1));
      CK(cudaEventSynchronize(e1)); float t; CK(cudaEventElapsedTime(&t, e0, e1));
      if (r) ms.push_back(t);
    }
    CK(cudaGetLastError());
    CK(cudaMemset(R, 0, 8192));
    k_f16ip_ref<<<dim3(1024, 64), 256>>>(V, U, N, R);
    CK(cudaGetLastError()); CK(cudaDeviceSynchronize());
    std::vector<unsigned long long> hd(1024), hr(1024);
    CK(cudaMemcpy(hd.data(), D, 8192, cudaMemcpyDeviceToHost));
    CK(cudaMemcpy(hr.data(), R, 8192, cudaMemcpyDeviceToHost));
    int bad = 0;
    for (int i = 0; i < 1024; i++) bad += hd[i] != hr[i];
    std::string s = dir + "/f16ip_" + std::to_string(logn);
    spill(s + "_D.bin", hd.data(), 8192);
    int dumped = 0;
    if (logn <= 22) {
      std::vector<char> h(pb);
      CK(cudaMemcpy(h.data(), V, pb, cudaMemcpyDeviceToHost)); spill(s + "_V.bin", h.data(), pb);
      CK(cudaMemcpy(h.data(), U, pb, cudaMemcpyDeviceToHost)); spill(s + "_U.bin", h.data(), pb);
      dumped = 1;
    }
    std::sort(ms.begin(), ms.end());
    double m = ms[ms.size() / 2];
    printf("{\"cmd\": \"f16ip\", \"logn\": %d, \"chunk\": 256, \"blocks\": %d, \"reps\": %d, \"ms\": %.4f, \"ms_min\": %.4f, "
           "\"bytes_read\": %zu, \"bytes_per_s\": %.6e, \"macs_per_s\": %.6e, \"simt_ref_mismatches\": %d, \"dumped\": %d}\n",
           logn, blocks, reps, m, ms[0], 2 * pb, 2 * pb / (m * 1e-3), (double)N * 1024 / (m * 1e-3), bad, dumped);
    return 0;
  }
  fprintf(stderr, "bad arguments\n");
  return 1;
}
