// Turing (sm_75) subset of the accumulator probe (initiative 17, workload 3).  probe.cu needs m16n8k32
// (sm_80+) and e4m3 (sm_89+); this file uses only the shapes a T4 has.  Every check is decided on the host
// (mma_run.py) from the stated inputs; this binary only moves data through the tensor cores.
//
//   wrap  u8|s8 PARAMS OUT     m8n8k16 .u8/.s8 -> .s32, layout-free: every A element = a, every B element = b,
//                              C = c0, `iters` chained instructions: D = c0 + 16*iters*a*b (mod 2^32 per PTX).
//                              PARAMS: T x int64[4] = (a, b, c0, iters).  OUT: T x int32[64] (lane-major d0,d1).
//   b1chain PARAMS OUT         m8n8k128 .b1 xor.popc, layout-free: every lane's A word = a, B word = b:
//                              D = c0 + iters*4*popc(a^b) (mod 2^32).  Same file shapes as wrap.
//   b1rand S L A B C D         S warps, each chaining L m8n8k128 xor.popc with per-lane words A[s][it][lane],
//                              B[s][it][lane] (uint32), C[s][lane][2] (int32) -> D[s][lane][2].
//   f16frag T A B C D          T warps, one m16n8k8 f32.f16.f16.f32 each, full matrices through the PTX fragment
//                              layout: A[t] 16x8 f16 (row-major), B[t] 8x8 f16 (B[k][n]), C[t], D[t] 16x8 f32.
//   rate KIND MS               sustained tensor-core rate for KIND in {u8, u4, b1, f16, lop3}; prints one JSON line.
//
// Fragment layouts used (PTX ISA "Matrix fragments for mma.m8n8k16 / m8n8k128 / m16n8k8"):
//   m8n8k16 u8, m8n8k128 b1: A reg: row = g, k = tig*E + i; B reg: k = tig*E + i, col = g; D: row g, col tig*2+j
//   (g = lane>>2, tig = lane&3, E = 4 for u8, 32 for b1).  m16n8k8 f16: a0 = A[g][tig*2 + {0,1}],
//   a1 = A[g+8][tig*2 + {0,1}], b0 = B[tig*2 + {0,1}][g], c/d{0,1} = [g][tig*2+{0,1}], c/d{2,3} = [g+8][...].
//   A wrong layout makes the random-data checks fail; the layout-free checks do not depend on it.
//
// Build: nvcc -O2 -arch=sm_75 probe_sm75.cu -o probe_sm75
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <vector>
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
static void spill(const char *path, const void *p, size_t n) {
  FILE *f = fopen(path, "wb");
  if (!f || fwrite(p, 1, n, f) != n) { fprintf(stderr, "cannot write %s\n", path); exit(2); }
  fclose(f);
}
template <class T> static T *upload(const std::vector<char> &h) {
  T *d; CK(cudaMalloc(&d, h.size() ? h.size() : 4));
  if (h.size()) CK(cudaMemcpy(d, h.data(), h.size(), cudaMemcpyHostToDevice));
  return d;
}

// ------------------------------------------------------------------------------------------------ wrap
template <bool S>
__global__ void k_wrap8(const long long *prm, int32_t *out) {
  const long long *p = prm + 4 * blockIdx.x;
  uint32_t a = (uint32_t)(p[0] & 0xFF) * 0x01010101u, b = (uint32_t)(p[1] & 0xFF) * 0x01010101u;
  int32_t d0 = (int32_t)p[2], d1 = (int32_t)p[2];
  for (long long it = 0; it < p[3]; it++) {
    if (S)
      asm volatile("mma.sync.aligned.m8n8k16.row.col.s32.s8.s8.s32 {%0,%1}, {%2}, {%3}, {%0,%1};\n"
                   : "+r"(d0), "+r"(d1) : "r"(a), "r"(b));
    else
      asm volatile("mma.sync.aligned.m8n8k16.row.col.s32.u8.u8.s32 {%0,%1}, {%2}, {%3}, {%0,%1};\n"
                   : "+r"(d0), "+r"(d1) : "r"(a), "r"(b));
  }
  out[64 * blockIdx.x + 2 * threadIdx.x] = d0;
  out[64 * blockIdx.x + 2 * threadIdx.x + 1] = d1;
}

__global__ void k_b1chain(const long long *prm, int32_t *out) {
  const long long *p = prm + 4 * blockIdx.x;
  uint32_t a = (uint32_t)p[0], b = (uint32_t)p[1];
  int32_t d0 = (int32_t)p[2], d1 = (int32_t)p[2];
  for (long long it = 0; it < p[3]; it++)
    asm volatile("mma.sync.aligned.m8n8k128.row.col.s32.b1.b1.s32.xor.popc {%0,%1}, {%2}, {%3}, {%0,%1};\n"
                 : "+r"(d0), "+r"(d1) : "r"(a), "r"(b));
  out[64 * blockIdx.x + 2 * threadIdx.x] = d0;
  out[64 * blockIdx.x + 2 * threadIdx.x + 1] = d1;
}

__global__ void k_b1rand(int L, const uint32_t *A, const uint32_t *B, const int32_t *C, int32_t *D) {
  int s = blockIdx.x, lane = threadIdx.x;
  int32_t d0 = C[(s * 32 + lane) * 2], d1 = C[(s * 32 + lane) * 2 + 1];
  for (int it = 0; it < L; it++) {
    uint32_t a = A[((size_t)s * L + it) * 32 + lane], b = B[((size_t)s * L + it) * 32 + lane];
    asm volatile("mma.sync.aligned.m8n8k128.row.col.s32.b1.b1.s32.xor.popc {%0,%1}, {%2}, {%3}, {%0,%1};\n"
                 : "+r"(d0), "+r"(d1) : "r"(a), "r"(b));
  }
  D[(s * 32 + lane) * 2] = d0;
  D[(s * 32 + lane) * 2 + 1] = d1;
}

// ------------------------------------------------------------------------------------------------ f16
__device__ __forceinline__ uint32_t pack2(uint16_t lo, uint16_t hi) { return (uint32_t)lo | ((uint32_t)hi << 16); }

__global__ void k_f16frag(const uint16_t *A, const uint16_t *B, const float *C, float *D) {
  int t = blockIdx.x, lane = threadIdx.x, g = lane >> 2, q = lane & 3;
  const uint16_t *a = A + t * 128, *b = B + t * 64;
  const float *c = C + t * 128;
  float *d = D + t * 128;
  uint32_t a0 = pack2(a[g * 8 + q * 2], a[g * 8 + q * 2 + 1]);
  uint32_t a1 = pack2(a[(g + 8) * 8 + q * 2], a[(g + 8) * 8 + q * 2 + 1]);
  uint32_t b0 = pack2(b[(q * 2) * 8 + g], b[(q * 2 + 1) * 8 + g]);
  float d0 = c[g * 8 + q * 2], d1 = c[g * 8 + q * 2 + 1], d2 = c[(g + 8) * 8 + q * 2], d3 = c[(g + 8) * 8 + q * 2 + 1];
  asm volatile("mma.sync.aligned.m16n8k8.row.col.f32.f16.f16.f32 {%0,%1,%2,%3}, {%4,%5}, {%6}, {%0,%1,%2,%3};\n"
               : "+f"(d0), "+f"(d1), "+f"(d2), "+f"(d3) : "r"(a0), "r"(a1), "r"(b0));
  d[g * 8 + q * 2] = d0; d[g * 8 + q * 2 + 1] = d1;
  d[(g + 8) * 8 + q * 2] = d2; d[(g + 8) * 8 + q * 2 + 1] = d3;
}

// ------------------------------------------------------------------------------------------------ rates
// U independent accumulator chains per warp; operands live in registers.  Work per instruction (MACs):
// u8 m8n8k16 1024, u4 m8n8k32 2048, b1 m8n8k128 8192, f16 m16n8k8 1024; lop3: 1 op per thread-instruction.
#define U 8
template <int KIND>
__global__ void k_rate(long long iters, uint32_t seed, int32_t *sink) {
  uint32_t a = 0x3C003C00u ^ (seed & 1), b = 0x3C003C00u ^ (threadIdx.x & 1);
  int32_t acc[U][4];
  float facc[U][4];
#pragma unroll
  for (int u = 0; u < U; u++)
#pragma unroll
    for (int j = 0; j < 4; j++) { acc[u][j] = 0; facc[u][j] = 0.f; }
  uint32_t x[U];
#pragma unroll
  for (int u = 0; u < U; u++) x[u] = seed + u * 0x9E3779B9u + threadIdx.x;
  for (long long it = 0; it < iters; it++) {
#pragma unroll
    for (int u = 0; u < U; u++) {
      if (KIND == 0)
        asm volatile("mma.sync.aligned.m8n8k16.row.col.s32.u8.u8.s32 {%0,%1}, {%2}, {%3}, {%0,%1};\n"
                     : "+r"(acc[u][0]), "+r"(acc[u][1]) : "r"(a), "r"(b));
      else if (KIND == 1)
        asm volatile("mma.sync.aligned.m8n8k32.row.col.s32.u4.u4.s32 {%0,%1}, {%2}, {%3}, {%0,%1};\n"
                     : "+r"(acc[u][0]), "+r"(acc[u][1]) : "r"(a), "r"(b));
      else if (KIND == 2)
        asm volatile("mma.sync.aligned.m8n8k128.row.col.s32.b1.b1.s32.xor.popc {%0,%1}, {%2}, {%3}, {%0,%1};\n"
                     : "+r"(acc[u][0]), "+r"(acc[u][1]) : "r"(a), "r"(b));
      else if (KIND == 3)
        asm volatile("mma.sync.aligned.m16n8k8.row.col.f32.f16.f16.f32 {%0,%1,%2,%3}, {%4,%5}, {%6}, {%0,%1,%2,%3};\n"
                     : "+f"(facc[u][0]), "+f"(facc[u][1]), "+f"(facc[u][2]), "+f"(facc[u][3])
                     : "r"(a), "r"(a), "r"(b));
      else
        asm volatile("lop3.b32 %0, %0, %1, %2, 0x96;\n" : "+r"(x[u]) : "r"(a), "r"(x[(u + 1) % U]));
    }
  }
  int32_t s = 0;
#pragma unroll
  for (int u = 0; u < U; u++) s += acc[u][0] + acc[u][1] + (int32_t)facc[u][0] + (int32_t)x[u];
  if (s == 0x12345678) sink[0] = s;
}

static double run_rate(int kind, int blocks, int threads, long long iters) {
  int32_t *sink; CK(cudaMalloc(&sink, 4));
  cudaEvent_t e0, e1; CK(cudaEventCreate(&e0)); CK(cudaEventCreate(&e1));
  auto go = [&](long long n) {
    switch (kind) {
      case 0: k_rate<0><<<blocks, threads>>>(n, 1, sink); break;
      case 1: k_rate<1><<<blocks, threads>>>(n, 1, sink); break;
      case 2: k_rate<2><<<blocks, threads>>>(n, 1, sink); break;
      case 3: k_rate<3><<<blocks, threads>>>(n, 1, sink); break;
      default: k_rate<4><<<blocks, threads>>>(n, 1, sink); break;
    }
  };
  go(16);
  CK(cudaDeviceSynchronize());
  CK(cudaEventRecord(e0)); go(iters); CK(cudaEventRecord(e1)); CK(cudaEventSynchronize(e1));
  float ms; CK(cudaEventElapsedTime(&ms, e0, e1));
  CK(cudaGetLastError());
  cudaFree(sink);
  return ms;
}

int main(int argc, char **argv) {
  if (argc < 2) { fprintf(stderr, "usage: see header\n"); return 1; }
  const char *cmd = argv[1];
  if (!strcmp(cmd, "wrap") || !strcmp(cmd, "b1chain")) {
    auto prm = slurp(argv[argc - 2]);
    int T = prm.size() / 32;
    long long *dp = upload<long long>(prm);
    int32_t *dout; CK(cudaMalloc(&dout, T * 256));
    if (!strcmp(cmd, "b1chain")) k_b1chain<<<T, 32>>>(dp, dout);
    else if (!strcmp(argv[2], "s8")) k_wrap8<true><<<T, 32>>>(dp, dout);
    else k_wrap8<false><<<T, 32>>>(dp, dout);
    CK(cudaGetLastError()); CK(cudaDeviceSynchronize());
    std::vector<int32_t> h(T * 64);
    CK(cudaMemcpy(h.data(), dout, T * 256, cudaMemcpyDeviceToHost));
    spill(argv[argc - 1], h.data(), T * 256);
    printf("{\"cmd\": \"%s\", \"tests\": %d}\n", cmd, T);
  } else if (!strcmp(cmd, "b1rand")) {
    int S = atoi(argv[2]), L = atoi(argv[3]);
    auto A = slurp(argv[4]), B = slurp(argv[5]), C = slurp(argv[6]);
    if (A.size() != (size_t)S * L * 128 || B.size() != A.size() || C.size() != (size_t)S * 256) {
      fprintf(stderr, "b1rand: size mismatch\n"); return 2;
    }
    int32_t *dD; CK(cudaMalloc(&dD, S * 256));
    k_b1rand<<<S, 32>>>(L, upload<uint32_t>(A), upload<uint32_t>(B), upload<int32_t>(C), dD);
    CK(cudaGetLastError()); CK(cudaDeviceSynchronize());
    std::vector<int32_t> h(S * 64);
    CK(cudaMemcpy(h.data(), dD, S * 256, cudaMemcpyDeviceToHost));
    spill(argv[7], h.data(), S * 256);
    printf("{\"cmd\": \"b1rand\", \"S\": %d, \"L\": %d}\n", S, L);
  } else if (!strcmp(cmd, "f16frag")) {
    int T = atoi(argv[2]);
    auto A = slurp(argv[3]), B = slurp(argv[4]), C = slurp(argv[5]);
    if (A.size() != (size_t)T * 256 || B.size() != (size_t)T * 128 || C.size() != (size_t)T * 512) {
      fprintf(stderr, "f16frag: size mismatch\n"); return 2;
    }
    float *dD; CK(cudaMalloc(&dD, T * 512));
    k_f16frag<<<T, 32>>>(upload<uint16_t>(A), upload<uint16_t>(B), upload<float>(C), dD);
    CK(cudaGetLastError()); CK(cudaDeviceSynchronize());
    std::vector<float> h(T * 128);
    CK(cudaMemcpy(h.data(), dD, T * 512, cudaMemcpyDeviceToHost));
    spill(argv[6], h.data(), T * 512);
    printf("{\"cmd\": \"f16frag\", \"T\": %d}\n", T);
  } else if (!strcmp(cmd, "rate")) {
    const char *kinds[] = {"u8", "u4", "b1", "f16", "lop3"};
    const double macs[] = {1024, 2048, 8192, 1024, 0};
    int kind = -1;
    for (int k = 0; k < 5; k++) if (!strcmp(argv[2], kinds[k])) kind = k;
    if (kind < 0) { fprintf(stderr, "rate: unknown kind\n"); return 1; }
    double target_ms = argc > 3 ? atof(argv[3]) : 200.0;
    cudaDeviceProp pr; CK(cudaGetDeviceProperties(&pr, 0));
    int blocks = pr.multiProcessorCount * 8, threads = 128;
    long long iters = 1024;
    double ms = run_rate(kind, blocks, threads, iters);
    while (ms < target_ms / 4 && iters < (1ll << 34)) { iters *= 4; ms = run_rate(kind, blocks, threads, iters); }
    long long warps = (long long)blocks * threads / 32, inst = warps * iters * U;
    double per_s = (kind == 4) ? (double)inst * 32 / (ms * 1e-3) : (double)inst * macs[kind] / (ms * 1e-3);
    int clk = 0; cudaDeviceGetAttribute(&clk, cudaDevAttrClockRate, 0);
    printf("{\"cmd\": \"rate\", \"kind\": \"%s\", \"blocks\": %d, \"threads\": %d, \"chains\": %d, \"iters\": %lld, "
           "\"ms\": %.4f, \"instructions\": %lld, \"%s\": %.6e, \"sm\": %d, \"clock_khz_attr\": %d}\n",
           kinds[kind], blocks, threads, U, iters, ms, inst, kind == 4 ? "lop3_per_s" : "macs_per_s", per_s,
           pr.multiProcessorCount, clk);
  } else {
    fprintf(stderr, "unknown command %s\n", cmd);
    return 1;
  }
  return 0;
}
