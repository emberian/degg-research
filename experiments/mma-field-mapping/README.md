# MMA field mapping (2026-11-05)

Counts-only model of prime-field dot products on tensor-core formats (INT8, FP8-E4M3, FP4-E2M1).
No GPU, no vendor throughput numbers: the format-to-INT8 throughput ratio is a parameter.

Run: `python3 -I mma_limbs.py` (output committed as `RESULTS.txt`).

VERIFIED by the run:
- The model reproduces TensorZKP's published INT8 bound (UINT8 limbs into INT32, N < 33,025).
- BabyBear on FP4-E2M1 with base-13 balanced limbs: 9 limbs, so 81 limb products per multiply,
  against 16 for INT8.
- FP4 beats INT8 only where TOPS(FP4)/TOPS(INT8) > 5.06. FP8-E4M3 needs > 4.00.
- Exact block length for an FP4 limb pair: 116,508 under a 24-bit accumulator, 113 under a 14-bit
  truncated one.

Falsifier: a measured per-SKU accumulator width `m` (a Fasi et al. 2021 style probe on a Blackwell
part). If `m` is near 14 for FP4/FP8 MMA, the FP4 proof-arithmetic design point
(`redregg/designs/SILICON-TO-SEMANTICS-ALIGNMENT-2026-11-05.md` (a)) is dead on that SKU.

Not modelled: modular reduction cost, layout/transpose, memory bandwidth, MX block scales,
Karatsuba-style limb tricks off the MMA. This is a necessary condition, not a performance claim.
