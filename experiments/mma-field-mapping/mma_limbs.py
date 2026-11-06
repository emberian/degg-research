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

def representatives(base, digitset):
    # Fix an encoder, not just a complete residue set.  Each quotient stays nonnegative.
    reps = {r: min((d for d in digitset if d % base == r and d <= r), key=abs)
            for r in range(base)}
    assert reps[0] == 0
    return reps

def limbs(p, base, digitset):
    """Certified upper count for this explicit quotient/remainder encoder on 0 <= x < p.
    If 0 <= q <= M and r is the chosen digit then
    0 <= (q-r)//base <= (M-min(reps))//base.  Stop when every possible final quotient
    is itself a digit.  The old balanced-interval formula was false for this gapped set.
    """
    assert p >= 2 and base >= 2
    reps = representatives(base, digitset)
    terminal = 0
    while terminal + 1 in digitset:
        terminal += 1
    bound, count = p - 1, 1
    while bound > terminal:
        nxt = (bound - min(reps.values())) // base
        if nxt >= bound:
            raise ValueError("digit set has no decreasing nonnegative quotient bound")
        bound, count = nxt, count + 1
    return count

def encode(x, base, digitset, count):
    reps = representatives(base, digitset)
    out = []
    for _ in range(count - 1):
        digit = reps[x % base]
        out.append(digit)
        x = (x - digit) // base
    assert x in digitset
    out.append(x)
    return out

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
    assert limbs(67, 13, SCHEMES[2][2]) == 3  # 65 had no two-digit representation
    for x in range(67):
        enc = encode(x, 13, SCHEMES[2][2], 3)
        assert sum(d * 13**i for i, d in enumerate(enc)) == x
    int8P = {}
    rows = []
    for pname, p in PRIMES:
        for sname, base, ds, signed in SCHEMES:
            assert digits_ok(base, ds), sname
            L = limbs(p, base, ds)
            for x in (0, 1, p // 2, p - 1):
                enc = encode(x, base, ds, L)
                assert sum(d * base**i for i, d in enumerate(enc)) == x
            P = L * L
            terminal = 0
            while terminal + 1 in ds:
                terminal += 1
            maxd = max(terminal, max(abs(d) for d in representatives(base, ds).values()))
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
