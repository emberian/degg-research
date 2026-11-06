#!/usr/bin/env python3
"""Counts-only model: cost of one prime-field dot product on tensor-core MMA formats.

For a prime p and a limb scheme (format, base b, representable digit set), compute
  L        = limbs per element
  P        = L*L limb products per field multiply (schoolbook; MMA has no Karatsuba)
  maxprod  = max |digit_a * digit_b| in the format's integer scale
  Nmax(m)  = longest dot product whose every partial sum of one output limb-convolution
             stays < 2^m, i.e. stays an exactly representable integer in an accumulator
             with an m-bit significand (INT32: m=31; FP32: m=24; a truncated FP8-style
             accumulator as reported for H800 FP8 GEMM: m~14).
No wall clock, no vendor TOPS: the throughput ratio r = TOPS(fmt)/TOPS(INT8) is a parameter.
Break-even: format beats INT8 when r > P_fmt / P_int8.
"""
import math

def digits_ok(base, digitset):
    # every residue class 0..base-1 must be reachable as r + base*q with r in digitset, q in {-1,0,1}
    reach = {(r % base) for r in digitset}
    return len(reach) == base

def limbs(p, base, signed):
    # signed (balanced) digits need one extra limb of headroom in the worst case
    # balanced digits cover [-(b^L-1)/2, (b^L-1)/2], so they need b^L > 2p
    return math.ceil(math.log(2 * p + 1, base)) if signed else math.ceil(math.log(p, base))

SCHEMES = [
    # name, base, digit set (in format's integer scale), signed?
    ("INT8 (u8 limbs, TensorZKP)", 256, list(range(256)), False),
    ("FP8 E4M3 (ints 0..15)", 16, list(range(16)), False),
    ("FP4 E2M1 doubled, base 13 (Ozaki, obend)", 13, [0, 1, -1, 2, -2, 3, -3, 4, -4, 6, -6, 8, -8, 12, -12], True),
    ("FP4 E2M1 doubled, base 5 (digits 0..4)", 5, [0, 1, 2, 3, 4], False),
]
PRIMES = [("M31", 2**31 - 1), ("BabyBear", 2**31 - 2**27 + 1), ("KoalaBear", 2**31 - 2**24 + 1),
          ("Goldilocks", 2**64 - 2**32 + 1)]

def main():
    int8P = {}
    rows = []
    for pname, p in PRIMES:
        for sname, base, ds, signed in SCHEMES:
            assert digits_ok(base, ds), sname
            L = limbs(p, base, signed)
            P = L * L
            maxd = max(abs(d) for d in ds)
            # digit after carry-normalisation is bounded by the digit set itself
            maxprod = maxd * maxd
            # TensorZKP layout: each (limb_i, limb_j) pair is its own MMA output entry
            # (Z = V^T U is kL x kL), recombined off-MMA, so one entry sums N products.
            n = {m: (2**m - 1) // maxprod for m in (31, 24, 14)}
            if sname.startswith("INT8"):
                int8P[pname] = P
            rows.append((pname, sname, L, P, maxprod, n[31], n[24], n[14]))
    print(f"{'prime':10} {'scheme':42} {'L':>3} {'P=L^2':>6} {'max|ab|':>8} {'N<2^31':>10} {'N<2^24':>9} {'N<2^14':>7} {'breakeven r':>11}")
    for (pname, sname, L, P, mp, n31, n24, n14) in rows:
        be = P / int8P[pname]
        print(f"{pname:10} {sname:42} {L:3d} {P:6d} {mp:8d} {n31:10d} {n24:9d} {n14:7d} {be:11.2f}")
    print()
    print("Reading: an MMA format beats INT8 for field dot products only on a SKU where")
    print("TOPS(fmt)/TOPS(INT8) exceeds 'breakeven r'. N columns: longest exact block before")
    print("an off-MMA modular reduction, for an accumulator with that many significand bits.")
    print("FP formats accumulate in FP; INT32 column applies to INT8 only, FP32/truncated to FP8/FP4.")

if __name__ == "__main__":
    main()
