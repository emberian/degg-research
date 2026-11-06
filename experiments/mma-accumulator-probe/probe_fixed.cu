// Accumulator probe for tensor-core MMA kinds (initiative 14, F3).  probe_fixed.cu: probe.cu with the two
// interpretation fixes of initiative 17 (patch: zkml-research notes/accelerator-readiness/patches/
// probe-cu-e4m3-path.patch).  probe.cu itself compiles unchanged for sm_80..sm_103a (nvcc 13.0.88).
//   1. ptxas lowers mma.sync e4m3 to F2FP unpack + HMMA (FP16) on sm_90a and sm_100a/103a (SASS census,
//      Kaggle accel-mma-t4-dry-01): only sm_89 runs it on QMMA.  The e4m3 rows below are labelled with
//      the path; on Hopper use wgmma (probe_sm80.cu wgcheck), on Blackwell cuBLASLt (gemm_probe.cu).
//   2. With fractional products (2^-5 each) the C path drops each product separately once it falls below
//      the alignment window, so "m = j + 1" under-reads the window by log2(1/product) = 5 bits (T4 HMMA,
//      fp16 analogue: 22 fractional vs 25 integer).  The integer-addend probe below (C = 2^j, every
//      product 1, D = 2^j + 32) reads the window that Lean's HoldsBelow uses for integer limb sums.  Layout-free by construction: every
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

__global__ void probe_cacc_e4m3(int j, int integer_addends, float *out) {
#if __CUDA_ARCH__ >= 890
  float va = integer_addends ? 1.0f : 0.25f, vb = integer_addends ? 1.0f : 0.125f;
  __nv_fp8x4_e4m3 qa(make_float4(va, va, va, va));
  __nv_fp8x4_e4m3 qb(make_float4(vb, vb, vb, vb));
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
  int dev = 0, maj = 0, min = 0;
  cudaGetDevice(&dev);
  cudaDeviceGetAttribute(&maj, cudaDevAttrComputeCapabilityMajor, dev);
  cudaDeviceGetAttribute(&min, cudaDevAttrComputeCapabilityMinor, dev);
  const char *path = (maj == 8 && min == 9) ? "QMMA (native fp8)" : "F2FP+HMMA (fp8 widened to fp16; not the fp8 datapath)";
  printf("e4m3 mma.sync path on sm_%d%d: %s\n", maj, min, path);
  for (int ia = 0; ia < 2; ia++) {
    for (int j = 8; j <= 30; j++) {
      probe_cacc_e4m3<<<1, 32>>>(j, ia, df);
      float h; cudaMemcpy(&h, df, 4, cudaMemcpyDeviceToHost);
      double want = ldexp(1.0, j) + (ia ? 32.0 : 1.0);
      printf("e4m3 C-acc %s j=%2d  D=%.1f  exact=%s\n", ia ? "integer   " : "fractional", j, (double)h,
             (double)h == want ? "yes" : "NO");
    }
  }
  printf("integer rows: window m = 1 + largest j with exact=yes (2^j + 32 is an f32 up to j = 28).\n"
         "fractional rows: 1 + largest j reads m - 5 under per-product alignment.  Record SKU, driver, nvcc, sm.\n");
  return 0;
}
