# MMA accumulator probe and the one-hour GPU protocol (initiative 14, F3), 2026-11-06

No GPU has been granted. This directory is what runs when one is. Counts model:
zkml-research `notes/construction-search/mma_counts.py`; Lean: minidregg lane `lane/mma-exact`,
`Theory/MmaAccumulatorExact.lean` (the measured `m` below is the premise `HoldsBelow m rnd`).

## Which box, and why
- **Best single box: B300** (or B200 if B300 is unavailable). It decides the two open premises: the
  `tcgen05 .kind::f8f6f4` accumulator width (no published statement) and whether FP8 limbs reach the HBM
  floor when INT8 is 1/30 of FP8. An **H100** second (or instead) reproduces TensorZKP on its own hardware
  and measures the unpublished `wgmma .b1` rate, which decides Tower256 on Hopper.
- Expected cost at public spot prices: one GPU-hour per SKU [A: ember to quote; nothing is rented
  without ember's yes, and `vast` is not ours to use without it].

## Protocol (60 minutes, in this order; each step tees to `runs/<sku>-<step>.log`)
1. (5 min) `nvidia-smi -q`, `nvcc --version`, driver; `make` this probe (`nvcc -O2 -arch=sm_90 probe.cu`).
2. (5 min) `./probe`: u8 wrap semantics (expect reg == exact mod 2^32 at every length; signed-exact
   fails from 33,026 products) and the e4m3 C-path width `m`.
3. (15 min) Within-instruction group width, Fasi-style: the same probe with one product 2^j and one
   product 1 inside one instruction (needs the fragment layout of PTX 9.x section 9.7.16.5 for
   m16n8k32; or the published Fasi-Higham-Mikaitis-Pranesh probe code [CLAIM: verify repository]),
   for e4m3 (wgmma on sm_90a), and for e2m1 / e4m3 through CUTLASS `tcgen05` example GEMMs on sm_100/103.
   Output: one `m` per (SKU, kind, path). Falsifier for the counts model: `m < 12` on FP8 contradicts
   PTX 9.2's "higher than half precision".
4. (10 min) HBM floor: a device-to-device copy of 2 GiB, attained B/s.
5. (10 min) Inner product as `V^T U`, BabyBear^8 u8 limbs (32 x 32,768 x 32 per batch, `N/32768` batches):
   `cublasGemmStridedBatchedEx` with `CUDA_R_8I` inputs and `CUDA_R_32I` output at N = 2^20..2^25;
   then the FP8 (`cublasLt`, e4m3, base-16 limbs, 64 limbs per element) and, on Blackwell, FP4 variants.
   Report bytes/s against step 4. Prediction (model): inner product at 2^25 is HBM-bound at
   0.64 ms (H100) / 0.27 ms (B200, B300); TensorZKP measured 0.85 ms on H100.
6. (10 min) `cutlass_profiler` 1-bit GEMM (`--A=b1 --B=b1`, and-popc) on H100: the b1 rate. If it is at
   least 8x INT8 (A100's published ratio), Tower256 inner products are HBM-bound on Hopper; if it is near
   INT8, they are bound by the bit-unpack on SIMT (model: 6.2 ms vs 3.2 ms for BabyBear^8 at 2^25).
7. (5 min) Keccak-f rate: `hashcat -b -m 17800` (Keccak-256), H/s, against the model's 4,320 int32
   ops per Keccak-f (3.9e9/s on H100). This prices the Tower256 prover's hash share.

## Acceptance
The model stands if steps 5 and 7 land within 1.5x of its floors (TensorZKP's own primitives sit at
1.17-1.48x). It is refuted on a SKU if a primitive it calls HBM-bound runs more than 2x above the
measured copy bandwidth, or if the measured `m` breaks a block length the model uses.
