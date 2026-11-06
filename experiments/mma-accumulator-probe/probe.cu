// Accumulator probe for tensor-core MMA kinds (initiative 14, F3).  UNCOMPILED on the Mac (no nvcc):
// first action on a GPU box is `make` and fixing what nvcc reports.  Layout-free by construction: every
// element of A and of B holds the same value, so D[i][j] = C[i][j] + K*a*b for every fragment layout.
//
//   probe_wrap_u8   : u8 x u8 -> s32 (sm_80+). Chains D -> C past 2^31 and 2^32; checks the register
//                     equals the exact sum mod 2^32 (PTX: wraps without .satfinite).  Theory:
//                     MmaAccumulatorExact.hwWrap_eq / s32_parity_exact.
//   probe_cacc_e4m3 : e4m3 x e4m3 -> f32 (sm_89+). C = 2^j, every product 2^-5 (a = 0.25, b = 0.125),
//                     K = 32, so D should be 2^j + 1.  The largest j with D exact gives the retained
//                     width of the C + sum path: m = j + 1.  This is HoldsBelow m for the accumulate-into-C
//                     step only; the within-instruction group adder needs the fragment-layout probe (README).
//
// Build: nvcc -O2 -arch=sm_90 probe.cu -o probe   (sm_89 for Ada; the e4m3 probe needs sm_89+)
#include <cstdio>
#include <cstdint>
#include <cuda_fp8.h>

__global__ void probe_wrap_u8(int iters, int32_t *out) {
  uint32_t a[4], b[2];
  for (int i = 0; i < 4; i++) a[i] = 0xFFFFFFFFu;   // sixteen u8 limbs = 255
  for (int i = 0; i < 2; i++) b[i] = 0xFFFFFFFFu;
  int32_t d0 = 0, d1 = 0, d2 = 0, d3 = 0;
  for (int it = 0; it < iters; it++) {
    asm volatile(
        "mma.sync.aligned.m16n8k32.row.col.s32.u8.u8.s32 {%0,%1,%2,%3}, {%4,%5,%6,%7}, {%8,%9}, {%0,%1,%2,%3};\n"
        : "+r"(d0), "+r"(d1), "+r"(d2), "+r"(d3)
        : "r"(a[0]), "r"(a[1]), "r"(a[2]), "r"(a[3]), "r"(b[0]), "r"(b[1]));
  }
  if (threadIdx.x == 0) { out[0] = d0; out[1] = d1; out[2] = d2; out[3] = d3; }
}

__global__ void probe_cacc_e4m3(int j, float *out) {
#if __CUDA_ARCH__ >= 890
  __nv_fp8x4_e4m3 qa(make_float4(0.25f, 0.25f, 0.25f, 0.25f));
  __nv_fp8x4_e4m3 qb(make_float4(0.125f, 0.125f, 0.125f, 0.125f));
  uint32_t a = *reinterpret_cast<uint32_t *>(&qa), b = *reinterpret_cast<uint32_t *>(&qb);
  float c = ldexpf(1.0f, j);
  float d0 = c, d1 = c, d2 = c, d3 = c;
  asm volatile(
      "mma.sync.aligned.m16n8k32.row.col.f32.e4m3.e4m3.f32 {%0,%1,%2,%3}, {%4,%4,%4,%4}, {%5,%5}, {%0,%1,%2,%3};\n"
      : "+f"(d0), "+f"(d1), "+f"(d2), "+f"(d3)
      : "r"(a), "r"(b));
  if (threadIdx.x == 0) out[0] = d0;
#endif
}

int main() {
  int32_t *dw; float *df; cudaMalloc(&dw, 16); cudaMalloc(&df, 4);
  // wrap: 32 products of 65025 per instruction; 1032 iterations = 33,024 products (< 2^31), then past
  const int its[] = {1032, 1033, 2064, 2065, 4000};
  for (int t = 0; t < 5; t++) {
    probe_wrap_u8<<<1, 32>>>(its[t], dw);
    int32_t h[4]; cudaMemcpy(h, dw, 16, cudaMemcpyDeviceToHost);
    int64_t exact = (int64_t)its[t] * 32 * 65025;
    uint32_t want = (uint32_t)(exact & 0xFFFFFFFFll);
    printf("u8 wrap   products=%-7lld exact=%-12lld reg=%-12d reg==exact mod 2^32: %s  signed-exact: %s\n",
           (long long)its[t] * 32, (long long)exact, h[0], (uint32_t)h[0] == want ? "yes" : "NO",
           (int64_t)h[0] == exact ? "yes" : "no");
  }
  for (int j = 8; j <= 26; j++) {
    probe_cacc_e4m3<<<1, 32>>>(j, df);
    float h; cudaMemcpy(&h, df, 4, cudaMemcpyDeviceToHost);
    double want = ldexp(1.0, j) + 1.0;
    printf("e4m3 C-acc j=%2d  D=%.1f  exact=%s\n", j, (double)h, (double)h == want ? "yes" : "NO");
  }
  printf("m(e4m3, C path) = 1 + largest j with exact=yes.  Record SKU, driver, nvcc, sm.\n");
  return 0;
}
