// Ampere/Ada/Hopper/Blackwell companion of probe_sm75.cu (initiative 17, workload 3): the paid-box kernels.
// Compiled for sm_80, sm_89, sm_90a, sm_100a, sm_103a; a kind the build target lacks is compiled out and
// reported as "unsupported" at run time.  Every check is decided on the host (mma_run.py).
//
//   wrap  u8|s8 PARAMS OUT     m16n8k32 .u8/.s8 -> .s32, layout-free (A = a, B = b, C = c0, `iters` chained):
//                              D = c0 + 32*iters*a*b mod 2^32.  PARAMS T x int64[4]; OUT T x int32[128].
//   b1rand and|xor S L A B C D m16n8k256 .b1 popc, per-lane words A[s][it][lane][4], B[s][it][lane][2] (uint32),
//                              C[s][lane][4] int32 -> D[s][lane][4].
//   frag8 u8|s8|e4m3 T A B C D one m16n8k32 per warp through the PTX fragment layout: A[t] 16x32 bytes
//                              (row-major), B[t] 32x8 bytes (B[k][n]), C/D[t] 16x8 (int32 or f32).
//   rate KIND MS               KIND: s8, b1and, b1xor, e4m3, f16 (mma.sync); wg_s8, wg_b1, wg_e4m3, wg_f16
//                              (wgmma m64n128, sm_90a only; smem operands through no-swizzle descriptors).
//   wgcheck KIND ITERS APAT BPAT CINIT OUT
//                              wgmma m64n128 with every A row = the 16-byte pattern APAT repeated along K (B
//                              likewise), accumulators preset to CINIT: every accumulator must equal
//                              CINIT + ITERS * sum_k a_k b_k.  Uniform patterns give the layout-free exactness
//                              check; a big and a small K slot give the native (QGMMA) group/C-path width.
//                              OUT: 128 x 64 words.
//
// Fragment layouts (PTX ISA "Matrix fragments for mma.m16n8k32 / m16n8k256"), g = lane>>2, q = lane&3:
//   8-bit k32: a0 = A[g][q*4+i], a1 = A[g+8][q*4+i], a2 = A[g][16+q*4+i], a3 = A[g+8][16+q*4+i] (byte i);
//              b0 = B[q*4+i][g], b1 = B[16+q*4+i][g]; c/d{0,1} = [g][q*2+{0,1}], c/d{2,3} = [g+8][q*2+{0,1}].
//   b1 k256:   same with 32 bits per register and 128 in place of 16.
//
// Build: nvcc -O2 -arch=sm_90a probe_sm80.cu -o probe_sm80
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <vector>

#define CK(x)                                                                                        \
  do {                                                                                               \
    cudaError_t e_ = (x);                                                                            \
    if (e_ != cudaSuccess) {                                                                         \
      fprintf(stderr, "CUDA error %s at %s:%d\n", cudaGetErrorString(e_), __FILE__, __LINE__);       \
      exit(2);                                                                                       \
    }                                                                                                \
  } while (0)

#if defined(__CUDA_ARCH__) && __CUDA_ARCH__ >= 800
#define HAS_SM80 1
#endif
#if defined(__CUDA_ARCH__) && __CUDA_ARCH__ >= 890
#define HAS_E4M3 1
#endif
#if defined(__CUDA_ARCH_FEAT_SM90_ALL)
#define HAS_WGMMA 1
#endif

__device__ int g_unsupported;

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
static int unsupported_flag() {
  int u = 0; CK(cudaMemcpyFromSymbol(&u, g_unsupported, sizeof u));
  int z = 0; CK(cudaMemcpyToSymbol(g_unsupported, &z, sizeof z));
  return u;
}

// ------------------------------------------------------------------------------------------------ mma.sync
template <int KIND>   // 0 u8, 1 s8, 2 e4m3 (f32 accumulators in the same registers' bits), 3 b1 and, 4 b1 xor, 5 f16
__device__ __forceinline__ void mma_k(uint32_t *d, const uint32_t *a, const uint32_t *b) {
#ifdef HAS_SM80
  if (KIND == 0)
    asm volatile("mma.sync.aligned.m16n8k32.row.col.s32.u8.u8.s32 {%0,%1,%2,%3}, {%4,%5,%6,%7}, {%8,%9}, {%0,%1,%2,%3};\n"
                 : "+r"(d[0]), "+r"(d[1]), "+r"(d[2]), "+r"(d[3]) : "r"(a[0]), "r"(a[1]), "r"(a[2]), "r"(a[3]), "r"(b[0]), "r"(b[1]));
  else if (KIND == 1)
    asm volatile("mma.sync.aligned.m16n8k32.row.col.s32.s8.s8.s32 {%0,%1,%2,%3}, {%4,%5,%6,%7}, {%8,%9}, {%0,%1,%2,%3};\n"
                 : "+r"(d[0]), "+r"(d[1]), "+r"(d[2]), "+r"(d[3]) : "r"(a[0]), "r"(a[1]), "r"(a[2]), "r"(a[3]), "r"(b[0]), "r"(b[1]));
  else if (KIND == 3)
    asm volatile("mma.sync.aligned.m16n8k256.row.col.s32.b1.b1.s32.and.popc {%0,%1,%2,%3}, {%4,%5,%6,%7}, {%8,%9}, {%0,%1,%2,%3};\n"
                 : "+r"(d[0]), "+r"(d[1]), "+r"(d[2]), "+r"(d[3]) : "r"(a[0]), "r"(a[1]), "r"(a[2]), "r"(a[3]), "r"(b[0]), "r"(b[1]));
  else if (KIND == 4)
    asm volatile("mma.sync.aligned.m16n8k256.row.col.s32.b1.b1.s32.xor.popc {%0,%1,%2,%3}, {%4,%5,%6,%7}, {%8,%9}, {%0,%1,%2,%3};\n"
                 : "+r"(d[0]), "+r"(d[1]), "+r"(d[2]), "+r"(d[3]) : "r"(a[0]), "r"(a[1]), "r"(a[2]), "r"(a[3]), "r"(b[0]), "r"(b[1]));
  else if (KIND == 5)
    asm volatile("mma.sync.aligned.m16n8k16.row.col.f32.f16.f16.f32 {%0,%1,%2,%3}, {%4,%5,%6,%7}, {%8,%9}, {%0,%1,%2,%3};\n"
                 : "+r"(d[0]), "+r"(d[1]), "+r"(d[2]), "+r"(d[3]) : "r"(a[0]), "r"(a[1]), "r"(a[2]), "r"(a[3]), "r"(b[0]), "r"(b[1]));
#ifdef HAS_E4M3
  else if (KIND == 2)
    asm volatile("mma.sync.aligned.m16n8k32.row.col.f32.e4m3.e4m3.f32 {%0,%1,%2,%3}, {%4,%5,%6,%7}, {%8,%9}, {%0,%1,%2,%3};\n"
                 : "+r"(d[0]), "+r"(d[1]), "+r"(d[2]), "+r"(d[3]) : "r"(a[0]), "r"(a[1]), "r"(a[2]), "r"(a[3]), "r"(b[0]), "r"(b[1]));
#endif
#endif
}
template <int KIND> __device__ __forceinline__ bool mma_supported() {
#if defined(HAS_E4M3)
  return true;
#elif defined(HAS_SM80)
  return KIND != 2;
#else
  return false;
#endif
}

template <bool S>
__global__ void k_wrap(const long long *prm, int32_t *out) {
  if (!mma_supported<0>()) { g_unsupported = 1; return; }
  const long long *p = prm + 4 * blockIdx.x;
  uint32_t av = (uint32_t)(p[0] & 0xFF) * 0x01010101u, bv = (uint32_t)(p[1] & 0xFF) * 0x01010101u;
  uint32_t a[4] = {av, av, av, av}, b[2] = {bv, bv};
  uint32_t d[4];
  for (int j = 0; j < 4; j++) d[j] = (uint32_t)(int32_t)p[2];
  for (long long it = 0; it < p[3]; it++) mma_k<S ? 1 : 0>(d, a, b);
  for (int j = 0; j < 4; j++) out[128 * blockIdx.x + 4 * threadIdx.x + j] = (int32_t)d[j];
}

template <int KIND>
__global__ void k_b1rand(int L, const uint32_t *A, const uint32_t *B, const int32_t *C, int32_t *D) {
  if (!mma_supported<KIND>()) { g_unsupported = 1; return; }
  int s = blockIdx.x, lane = threadIdx.x;
  uint32_t d[4];
  for (int j = 0; j < 4; j++) d[j] = (uint32_t)C[(s * 32 + lane) * 4 + j];
  for (int it = 0; it < L; it++) {
    const uint32_t *pa = A + (((size_t)s * L + it) * 32 + lane) * 4, *pb = B + (((size_t)s * L + it) * 32 + lane) * 2;
    uint32_t a[4] = {pa[0], pa[1], pa[2], pa[3]}, b[2] = {pb[0], pb[1]};
    mma_k<KIND>(d, a, b);
  }
  for (int j = 0; j < 4; j++) D[(s * 32 + lane) * 4 + j] = (int32_t)d[j];
}

__device__ __forceinline__ uint32_t pack4(const uint8_t *p, int stride) {
  return (uint32_t)p[0] | ((uint32_t)p[stride] << 8) | ((uint32_t)p[2 * stride] << 16) | ((uint32_t)p[3 * stride] << 24);
}
template <int KIND>   // 0 u8, 1 s8, 2 e4m3
__global__ void k_frag8(const uint8_t *A, const uint8_t *B, const uint32_t *C, uint32_t *D) {
  if (!mma_supported<KIND>()) { g_unsupported = 1; return; }
  int t = blockIdx.x, lane = threadIdx.x, g = lane >> 2, q = lane & 3;
  const uint8_t *a = A + t * 512, *b = B + t * 256;
  const uint32_t *c = C + t * 128;
  uint32_t *dd = D + t * 128;
  uint32_t ar[4] = {pack4(a + g * 32 + q * 4, 1), pack4(a + (g + 8) * 32 + q * 4, 1),
                    pack4(a + g * 32 + 16 + q * 4, 1), pack4(a + (g + 8) * 32 + 16 + q * 4, 1)};
  uint32_t br[2] = {pack4(b + (q * 4) * 8 + g, 8), pack4(b + (16 + q * 4) * 8 + g, 8)};
  uint32_t d[4] = {c[g * 8 + q * 2], c[g * 8 + q * 2 + 1], c[(g + 8) * 8 + q * 2], c[(g + 8) * 8 + q * 2 + 1]};
  mma_k<KIND>(d, ar, br);
  dd[g * 8 + q * 2] = d[0]; dd[g * 8 + q * 2 + 1] = d[1];
  dd[(g + 8) * 8 + q * 2] = d[2]; dd[(g + 8) * 8 + q * 2 + 1] = d[3];
}

#define U 8
template <int KIND>
__global__ void k_rate(long long iters, uint32_t seed, int32_t *sink) {
  if (!mma_supported<KIND>()) { g_unsupported = 1; return; }
  // operands: 1.0 in f16 pairs / small bytes; values are irrelevant to the issue rate
  uint32_t v = KIND == 5 ? 0x3C003C00u : KIND == 2 ? 0x38383838u : 0x01010101u;
  uint32_t a[4] = {v, v, v, v ^ (seed & 1)}, b[2] = {v, v};
  uint32_t d[U][4];
#pragma unroll
  for (int u = 0; u < U; u++)
#pragma unroll
    for (int j = 0; j < 4; j++) d[u][j] = 0;
  for (long long it = 0; it < iters; it++) {
#pragma unroll
    for (int u = 0; u < U; u++) mma_k<KIND>(d[u], a, b);
  }
  uint32_t s = 0;
#pragma unroll
  for (int u = 0; u < U; u++) s += d[u][0] ^ d[u][3];
  if (s == 0x12345678u) sink[0] = (int32_t)s;
}

// ------------------------------------------------------------------------------------------------ wgmma
#define WG_D "{%0,%1,%2,%3,%4,%5,%6,%7,%8,%9,%10,%11,%12,%13,%14,%15,%16,%17,%18,%19,%20,%21,%22,%23,%24,%25,%26,%27,%28,%29,%30,%31,%32,%33,%34,%35,%36,%37,%38,%39,%40,%41,%42,%43,%44,%45,%46,%47,%48,%49,%50,%51,%52,%53,%54,%55,%56,%57,%58,%59,%60,%61,%62,%63}"
#define WG_OUT_R(d) "+r"(d[0]), "+r"(d[1]), "+r"(d[2]), "+r"(d[3]), "+r"(d[4]), "+r"(d[5]), "+r"(d[6]), "+r"(d[7]), "+r"(d[8]), "+r"(d[9]), "+r"(d[10]), "+r"(d[11]), "+r"(d[12]), "+r"(d[13]), "+r"(d[14]), "+r"(d[15]), "+r"(d[16]), "+r"(d[17]), "+r"(d[18]), "+r"(d[19]), "+r"(d[20]), "+r"(d[21]), "+r"(d[22]), "+r"(d[23]), "+r"(d[24]), "+r"(d[25]), "+r"(d[26]), "+r"(d[27]), "+r"(d[28]), "+r"(d[29]), "+r"(d[30]), "+r"(d[31]), "+r"(d[32]), "+r"(d[33]), "+r"(d[34]), "+r"(d[35]), "+r"(d[36]), "+r"(d[37]), "+r"(d[38]), "+r"(d[39]), "+r"(d[40]), "+r"(d[41]), "+r"(d[42]), "+r"(d[43]), "+r"(d[44]), "+r"(d[45]), "+r"(d[46]), "+r"(d[47]), "+r"(d[48]), "+r"(d[49]), "+r"(d[50]), "+r"(d[51]), "+r"(d[52]), "+r"(d[53]), "+r"(d[54]), "+r"(d[55]), "+r"(d[56]), "+r"(d[57]), "+r"(d[58]), "+r"(d[59]), "+r"(d[60]), "+r"(d[61]), "+r"(d[62]), "+r"(d[63])
#define WG_OUT_F(d) "+f"(d[0]), "+f"(d[1]), "+f"(d[2]), "+f"(d[3]), "+f"(d[4]), "+f"(d[5]), "+f"(d[6]), "+f"(d[7]), "+f"(d[8]), "+f"(d[9]), "+f"(d[10]), "+f"(d[11]), "+f"(d[12]), "+f"(d[13]), "+f"(d[14]), "+f"(d[15]), "+f"(d[16]), "+f"(d[17]), "+f"(d[18]), "+f"(d[19]), "+f"(d[20]), "+f"(d[21]), "+f"(d[22]), "+f"(d[23]), "+f"(d[24]), "+f"(d[25]), "+f"(d[26]), "+f"(d[27]), "+f"(d[28]), "+f"(d[29]), "+f"(d[30]), "+f"(d[31]), "+f"(d[32]), "+f"(d[33]), "+f"(d[34]), "+f"(d[35]), "+f"(d[36]), "+f"(d[37]), "+f"(d[38]), "+f"(d[39]), "+f"(d[40]), "+f"(d[41]), "+f"(d[42]), "+f"(d[43]), "+f"(d[44]), "+f"(d[45]), "+f"(d[46]), "+f"(d[47]), "+f"(d[48]), "+f"(d[49]), "+f"(d[50]), "+f"(d[51]), "+f"(d[52]), "+f"(d[53]), "+f"(d[54]), "+f"(d[55]), "+f"(d[56]), "+f"(d[57]), "+f"(d[58]), "+f"(d[59]), "+f"(d[60]), "+f"(d[61]), "+f"(d[62]), "+f"(d[63])

__device__ __forceinline__ uint64_t smem_desc(uint32_t addr, uint32_t lbo, uint32_t sbo) {
  // no swizzle, base offset 0: start>>4 in [0,14), LBO>>4 in [16,30), SBO>>4 in [32,46)
  return (uint64_t)((addr & 0x3FFFF) >> 4) | ((uint64_t)((lbo & 0x3FFFF) >> 4) << 16) |
         ((uint64_t)((sbo & 0x3FFFF) >> 4) << 32);
}
template <int KIND>   // 0 s8 (k32), 1 b1 and (k256), 2 e4m3 (k32), 3 f16 (k16)
__device__ __forceinline__ void wg_k(uint32_t *d, uint64_t da, uint64_t db) {
#ifdef HAS_WGMMA
  if (KIND == 0)
    asm volatile("{\n.reg .pred p;\nsetp.ne.b32 p, 1, 0;\n"
                 "wgmma.mma_async.sync.aligned.m64n128k32.s32.s8.s8 " WG_D ", %64, %65, p;\n}\n"
                 : WG_OUT_R(d) : "l"(da), "l"(db));
  else if (KIND == 1)
    asm volatile("{\n.reg .pred p;\nsetp.ne.b32 p, 1, 0;\n"
                 "wgmma.mma_async.sync.aligned.m64n128k256.s32.b1.b1.and.popc " WG_D ", %64, %65, p;\n}\n"
                 : WG_OUT_R(d) : "l"(da), "l"(db));
  else if (KIND == 2)
    asm volatile("{\n.reg .pred p;\nsetp.ne.b32 p, 1, 0;\n"
                 "wgmma.mma_async.sync.aligned.m64n128k32.f32.e4m3.e4m3 " WG_D ", %64, %65, p, 1, 1;\n}\n"
                 : WG_OUT_R(d) : "l"(da), "l"(db));
  else
    asm volatile("{\n.reg .pred p;\nsetp.ne.b32 p, 1, 0;\n"
                 "wgmma.mma_async.sync.aligned.m64n128k16.f32.f16.f16 " WG_D ", %64, %65, p, 1, 1, 0, 0;\n}\n"
                 : WG_OUT_R(d) : "l"(da), "l"(db));
#endif
}
__device__ __forceinline__ void wg_fence() {
#ifdef HAS_WGMMA
  asm volatile("wgmma.fence.sync.aligned;\n" ::: "memory");
#endif
}
__device__ __forceinline__ void wg_commit_wait() {
#ifdef HAS_WGMMA
  asm volatile("wgmma.commit_group.sync.aligned;\nwgmma.wait_group.sync.aligned 0;\n" ::: "memory");
#endif
}

// One warpgroup per block.  smem: A region (16 KiB) then B region (16 KiB), each filled with a 16-byte pattern
// repeated, so every A row reads the same K vector (and likewise B) under any no-swizzle K-major core-matrix
// placement whose rows are 16 contiguous bytes of K: K element k holds pattern element k mod (16 / element
// bytes).  With LBO 128 and SBO 256 every address the descriptors reach stays inside its region.
struct WgFill { uint32_t a[4], b[4], c; };
template <int KIND>
__global__ void __launch_bounds__(128) k_wg(long long iters, WgFill f, uint32_t *out, int write_all) {
#ifdef HAS_WGMMA
  __shared__ __align__(1024) uint32_t sm[8192];
  for (int i = threadIdx.x; i < 8192; i += 128) sm[i] = i < 4096 ? f.a[i & 3] : f.b[i & 3];
  __syncthreads();
  asm volatile("fence.proxy.async.shared::cta;\n" ::: "memory");
  uint32_t base = (uint32_t)__cvta_generic_to_shared(sm);
  uint64_t da = smem_desc(base, 128, 256), db = smem_desc(base + 16384, 128, 256);
  uint32_t d[64];
#pragma unroll
  for (int i = 0; i < 64; i++) d[i] = f.c;
  long long it = 0;
  for (; it + 4 <= iters; it += 4) {
    wg_fence();
    wg_k<KIND>(d, da, db); wg_k<KIND>(d, da, db); wg_k<KIND>(d, da, db); wg_k<KIND>(d, da, db);
    wg_commit_wait();
  }
  for (; it < iters; it++) {
    wg_fence();
    wg_k<KIND>(d, da, db);
    wg_commit_wait();
  }
  if (write_all) {
    for (int i = 0; i < 64; i++) out[(size_t)threadIdx.x * 64 + i] = d[i];
  } else {
    uint32_t s = 0;
#pragma unroll
    for (int i = 0; i < 64; i++) s ^= d[i];
    if (s == 0x12345678u) out[0] = s;
  }
#else
  g_unsupported = 1;
#endif
}

// ------------------------------------------------------------------------------------------------ host
struct Ev {
  cudaEvent_t a, b;
  Ev() { CK(cudaEventCreate(&a)); CK(cudaEventCreate(&b)); }
  float time(void (*f)(long long, int, uint32_t *), long long n, int blocks, uint32_t *sink) {
    CK(cudaEventRecord(a)); f(n, blocks, sink); CK(cudaEventRecord(b)); CK(cudaEventSynchronize(b));
    float ms; CK(cudaEventElapsedTime(&ms, a, b)); return ms;
  }
};
template <int K> static void launch_mma(long long n, int blocks, uint32_t *s) { k_rate<K><<<blocks, 128>>>(n, 1, (int32_t *)s); }
static WgFill uniform_fill(int K) {
  const uint32_t v[4] = {0x01010101u, 0xFFFFFFFFu, 0x38383838u, 0x3C003C00u};
  WgFill f;
  for (int i = 0; i < 4; i++) f.a[i] = f.b[i] = v[K];
  f.c = 0;
  return f;
}
template <int K> static void launch_wg(long long n, int blocks, uint32_t *s) { k_wg<K><<<blocks, 128>>>(n, uniform_fill(K), s, 0); }

int main(int argc, char **argv) {
  if (argc < 2) { fprintf(stderr, "usage: see header\n"); return 1; }
  const char *cmd = argv[1];
  cudaDeviceProp pr; CK(cudaGetDeviceProperties(&pr, 0));
  if (!strcmp(cmd, "wrap")) {
    auto prm = slurp(argv[3]);
    int T = prm.size() / 32;
    int32_t *dout; CK(cudaMalloc(&dout, T * 512));
    if (!strcmp(argv[2], "s8")) k_wrap<true><<<T, 32>>>(upload<long long>(prm), dout);
    else k_wrap<false><<<T, 32>>>(upload<long long>(prm), dout);
    CK(cudaGetLastError()); CK(cudaDeviceSynchronize());
    int u = unsupported_flag();
    std::vector<int32_t> h(T * 128);
    CK(cudaMemcpy(h.data(), dout, T * 512, cudaMemcpyDeviceToHost));
    spill(argv[4], h.data(), T * 512);
    printf("{\"cmd\": \"wrap\", \"type\": \"%s\", \"tests\": %d, \"unsupported\": %d}\n", argv[2], T, u);
  } else if (!strcmp(cmd, "b1rand")) {
    bool x = !strcmp(argv[2], "xor");
    int S = atoi(argv[3]), L = atoi(argv[4]);
    auto A = slurp(argv[5]), B = slurp(argv[6]), C = slurp(argv[7]);
    if (A.size() != (size_t)S * L * 512 || B.size() != (size_t)S * L * 256 || C.size() != (size_t)S * 512) {
      fprintf(stderr, "b1rand: size mismatch\n"); return 2;
    }
    int32_t *dD; CK(cudaMalloc(&dD, S * 512));
    if (x) k_b1rand<4><<<S, 32>>>(L, upload<uint32_t>(A), upload<uint32_t>(B), upload<int32_t>(C), dD);
    else k_b1rand<3><<<S, 32>>>(L, upload<uint32_t>(A), upload<uint32_t>(B), upload<int32_t>(C), dD);
    CK(cudaGetLastError()); CK(cudaDeviceSynchronize());
    int u = unsupported_flag();
    std::vector<int32_t> h(S * 128);
    CK(cudaMemcpy(h.data(), dD, S * 512, cudaMemcpyDeviceToHost));
    spill(argv[8], h.data(), S * 512);
    printf("{\"cmd\": \"b1rand\", \"op\": \"%s\", \"S\": %d, \"L\": %d, \"unsupported\": %d}\n", argv[2], S, L, u);
  } else if (!strcmp(cmd, "frag8")) {
    int T = atoi(argv[3]);
    auto A = slurp(argv[4]), B = slurp(argv[5]), C = slurp(argv[6]);
    if (A.size() != (size_t)T * 512 || B.size() != (size_t)T * 256 || C.size() != (size_t)T * 512) {
      fprintf(stderr, "frag8: size mismatch\n"); return 2;
    }
    uint32_t *dD; CK(cudaMalloc(&dD, T * 512));
    const uint8_t *dA = upload<uint8_t>(A), *dB = upload<uint8_t>(B);
    const uint32_t *dC = upload<uint32_t>(C);
    if (!strcmp(argv[2], "u8")) k_frag8<0><<<T, 32>>>(dA, dB, dC, dD);
    else if (!strcmp(argv[2], "s8")) k_frag8<1><<<T, 32>>>(dA, dB, dC, dD);
    else k_frag8<2><<<T, 32>>>(dA, dB, dC, dD);
    CK(cudaGetLastError()); CK(cudaDeviceSynchronize());
    int u = unsupported_flag();
    std::vector<uint32_t> h(T * 128);
    CK(cudaMemcpy(h.data(), dD, T * 512, cudaMemcpyDeviceToHost));
    spill(argv[7], h.data(), T * 512);
    printf("{\"cmd\": \"frag8\", \"type\": \"%s\", \"T\": %d, \"unsupported\": %d}\n", argv[2], T, u);
  } else if (!strcmp(cmd, "rate")) {
    struct K { const char *name; void (*f)(long long, int, uint32_t *); double macs; int wg; };
    const K ks[] = {{"s8", launch_mma<1>, 4096, 0},        {"b1and", launch_mma<3>, 32768, 0},
                    {"b1xor", launch_mma<4>, 32768, 0},    {"e4m3", launch_mma<2>, 4096, 0},
                    {"f16", launch_mma<5>, 2048, 0},       {"wg_s8", launch_wg<0>, 64.0 * 128 * 32, 1},
                    {"wg_b1", launch_wg<1>, 64.0 * 128 * 256, 1}, {"wg_e4m3", launch_wg<2>, 64.0 * 128 * 32, 1},
                    {"wg_f16", launch_wg<3>, 64.0 * 128 * 16, 1}};
    const K *k = nullptr;
    for (auto &c : ks) if (!strcmp(c.name, argv[2])) k = &c;
    if (!k) { fprintf(stderr, "rate: unknown kind\n"); return 1; }
    double target = argc > 3 ? atof(argv[3]) : 200.0;
    int blocks = pr.multiProcessorCount * (k->wg ? 2 : 8);
    uint32_t *sink; CK(cudaMalloc(&sink, 4));
    Ev ev;
    k->f(16, blocks, sink); CK(cudaDeviceSynchronize());
    if (unsupported_flag()) { printf("{\"cmd\": \"rate\", \"kind\": \"%s\", \"unsupported\": 1}\n", k->name); return 0; }
    long long iters = 1024;
    double ms = ev.time(k->f, iters, blocks, sink);
    while (ms < target / 4 && iters < (1ll << 34)) { iters *= 4; ms = ev.time(k->f, iters, blocks, sink); }
    CK(cudaGetLastError());
    double inst = k->wg ? (double)blocks * iters : (double)blocks * 4 * iters * U;   // per warpgroup / per warp
    printf("{\"cmd\": \"rate\", \"kind\": \"%s\", \"unsupported\": 0, \"blocks\": %d, \"iters\": %lld, \"ms\": %.4f, "
           "\"instructions\": %.0f, \"macs_per_s\": %.6e, \"sm\": %d}\n",
           k->name, blocks, iters, ms, inst, inst * k->macs / (ms * 1e-3), pr.multiProcessorCount);
  } else if (!strcmp(cmd, "wgcheck")) {
    // wgcheck KIND ITERS APAT BPAT CINIT OUT: APAT/BPAT 32 hex digits (16 bytes, byte 0 first), CINIT 8 hex digits
    int kind = !strcmp(argv[2], "s8") ? 0 : !strcmp(argv[2], "b1") ? 1 : !strcmp(argv[2], "e4m3") ? 2 : 3;
    long long iters = atoll(argv[3]);
    WgFill f;
    for (int w = 0; w < 4; w++) {
      f.a[w] = f.b[w] = 0;
      for (int j = 0; j < 4; j++) {
        unsigned x, y;
        sscanf(argv[4] + 8 * w + 2 * j, "%2x", &x); sscanf(argv[5] + 8 * w + 2 * j, "%2x", &y);
        f.a[w] |= x << (8 * j); f.b[w] |= y << (8 * j);
      }
    }
    f.c = (uint32_t)strtoul(argv[6], 0, 16);
    uint32_t *dout; CK(cudaMalloc(&dout, 128 * 64 * 4)); CK(cudaMemset(dout, 0xFF, 128 * 64 * 4));
    if (kind == 0) k_wg<0><<<1, 128>>>(iters, f, dout, 1);
    else if (kind == 1) k_wg<1><<<1, 128>>>(iters, f, dout, 1);
    else if (kind == 2) k_wg<2><<<1, 128>>>(iters, f, dout, 1);
    else k_wg<3><<<1, 128>>>(iters, f, dout, 1);
    CK(cudaGetLastError()); CK(cudaDeviceSynchronize());
    int u = unsupported_flag();
    std::vector<uint32_t> h(128 * 64);
    CK(cudaMemcpy(h.data(), dout, h.size() * 4, cudaMemcpyDeviceToHost));
    spill(argv[7], h.data(), h.size() * 4);
    printf("{\"cmd\": \"wgcheck\", \"kind\": \"%s\", \"iters\": %lld, \"unsupported\": %d}\n", argv[2], iters, u);
  } else {
    fprintf(stderr, "unknown command %s\n", cmd);
    return 1;
  }
  return 0;
}
