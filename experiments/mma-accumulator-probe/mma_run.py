#!/usr/bin/env python3
"""mma_run.py -- driver for the accumulator probe and the GPU protocol (initiative 17, workload 3).

Builds probe_sm75.cu, probe_sm80.cu, gemm_probe.cu (and the hardware lane's probe.cu) for the device it runs
on, compile-checks every .cu for every target arch (with a SASS opcode census when cuobjdump is present),
runs every probe the device supports, and decides every check HERE, on the host, from the inputs it wrote:
expected values are exact Python integers / Fractions, never the device's own output.

    python3 mma_run.py --out DIR [--shrink] [--sku NAME] [--arches sm_80,sm_89,...] [--skip SECTION ...]

--shrink is the parameter set that ran on the Kaggle T4 dry run; without it the full paid-box sizes run.
Writes DIR/results.json after every section (a watchdog kill keeps what finished) and prints progress lines.
Needs numpy and nvcc (NVCC env, PATH, or /usr/local/cuda/bin/nvcc).
"""
import argparse
import json
import os
import pathlib
import re
import shutil
import struct
import subprocess
import sys
import time
from fractions import Fraction

import numpy as np

HERE = pathlib.Path(__file__).resolve().parent
T0 = time.time()
R = {"started_unix": T0, "checks": {}, "errors": {}}
OUT = None
WORK = None


def log(*a):
    print(f"[{time.time() - T0:7.1f}s]", *a, flush=True)


def save():
    R["elapsed_s"] = round(time.time() - T0, 1)
    R["all_checks_pass"] = bool(R["checks"]) and all(v["pass"] for v in R["checks"].values())
    R["n_checks"] = len(R["checks"])
    R["n_failed"] = sum(1 for v in R["checks"].values() if not v["pass"])
    (OUT / "results.json").write_text(json.dumps(R, indent=1, default=str))
    p = os.environ.get("ACCEL_RESULTS")
    if p:
        pathlib.Path(p).write_text(json.dumps(R, indent=1, default=str))


def check(name, ok, **detail):
    R["checks"][name] = {"pass": bool(ok), **detail}
    log(("PASS " if ok else "FAIL ") + name, json.dumps(detail, default=str)[:300])


def sh(cmd, timeout=600, cwd=None):
    t = time.time()
    try:
        r = subprocess.run(cmd, shell=isinstance(cmd, str), capture_output=True, text=True, timeout=timeout, cwd=cwd)
        return r.returncode, r.stdout, r.stderr, time.time() - t
    except subprocess.TimeoutExpired as e:
        return -9, e.stdout or "", "TIMEOUT", time.time() - t


def run_json(args, timeout=600):
    """run a probe binary; return the parsed JSON lines it printed"""
    rc, so, se, dt = sh([str(a) for a in args], timeout=timeout)
    if rc != 0:
        raise RuntimeError(f"{args[0]} {args[1]} rc={rc}: {se.strip()[-800:]}")
    out = [json.loads(l) for l in so.splitlines() if l.startswith("{")]
    for o in out:
        o["wall_s"] = round(dt, 3)
    return out


# ---------------------------------------------------------------------------------------------- toolchain
# extra nvcc flags from the environment (the paid-box script passes -ccbin for a conda toolkit)
NVCC_PRE = os.environ.get("NVCC_FLAGS", "").split()


def find_nvcc():
    for c in (os.environ.get("NVCC"), shutil.which("nvcc"), "/usr/local/cuda/bin/nvcc"):
        if c and pathlib.Path(c).exists():
            return c
    sys.exit("no nvcc")


def arch_of(cc):
    major, minor = (int(x) for x in cc.split("."))
    v = major * 10 + minor
    return {75: "sm_75", 80: "sm_80", 86: "sm_86", 89: "sm_89", 90: "sm_90a", 100: "sm_100a", 103: "sm_103a",
            120: "sm_120a"}.get(v, f"sm_{v}"), v


def link_flags(nvcc):
    """cuBLAS from the toolkit, else from a pip nvidia-cublas wheel (lib dir also put on the rpath)"""
    root = pathlib.Path(nvcc).resolve().parent.parent
    for d in (root / "lib64", root / "lib", root / "targets/x86_64-linux/lib"):
        if list(d.glob("libcublasLt.so*")):
            return ["-L" + str(d), "-lcublas", "-lcublasLt", "-Xlinker", "-rpath=" + str(d)]
    import site
    for sp in site.getsitepackages() + [site.getusersitepackages()]:
        for d in pathlib.Path(sp).glob("nvidia/*/lib"):
            a, b = list(d.glob("libcublas.so*")), list(d.glob("libcublasLt.so*"))
            if a and b:
                return ["-L" + str(d), "-l:" + a[0].name, "-l:" + b[0].name, "-Xlinker", "-rpath=" + str(d)]
    return ["-lcublas", "-lcublasLt"]


def sass_census(cubin, cuobjdump):
    rc, so, se, _ = sh([cuobjdump, "-sass", str(cubin)], timeout=300)
    if rc != 0:
        return {"error": se.strip()[-200:]}
    c = {}
    for m in re.finditer(r"\b([A-Z]*MMA[.A-Z0-9x]*|[A-Z]GMMA[.A-Z0-9x]*|UTC[A-Z0-9]+[.A-Z0-9]*|CALL\.REL\.NOINC|F2FP\.F16\.E4M3\.UNPACK)", so):
        k = m.group(1)
        c[k] = c.get(k, 0) + 1
    return c


def compile_check(nvcc, arches):
    inc = []
    cuobjdump = str(pathlib.Path(nvcc).parent / "cuobjdump")
    have_cuobj = pathlib.Path(cuobjdump).exists() and pathlib.Path(nvcc).parent.joinpath("nvdisasm").exists()
    res = {}
    for src in ("probe.cu", "probe_fixed.cu", "probe_sm75.cu", "probe_sm80.cu", "gemm_probe.cu", "rotation_probe.cu"):
        if not (HERE / src).exists():
            continue
        for a in arches:
            cub = WORK / f"cc_{src[:-3]}_{a}.cubin"
            rc, so, se, dt = sh([nvcc] + NVCC_PRE + ["-O2", f"-arch={a}", "-cubin", "-o", str(cub), str(HERE / src)] + inc, timeout=900)
            e = dict(rc=rc, s=round(dt, 1), log=(so + se).strip()[-600:])
            if rc == 0 and have_cuobj:
                e["sass"] = sass_census(cub, cuobjdump)
            res[f"{src}@{a}"] = e
            log("compile", src, a, "rc", rc, f"{dt:.1f}s", json.dumps(e.get("sass", {}))[:200])
    return res


# ---------------------------------------------------------------------------------------------- numerics
def s32(x):
    x &= 0xFFFFFFFF
    return x - (1 << 32) if x >= 1 << 31 else x


def f16_bits(v):
    """exact fp16 encoding of a Fraction/int; raises if not representable"""
    b = struct.unpack("<H", struct.pack("<e", float(v)))[0]
    assert f16_val(b) == Fraction(v), v
    return b


def f16_val(b):
    return Fraction(struct.unpack("<e", struct.pack("<H", b))[0])


def e4m3_val(c):
    s = -1 if c & 0x80 else 1
    e, m = (c >> 3) & 15, c & 7
    if e == 15 and m == 7:
        raise ValueError("nan")
    if e == 0:
        return s * Fraction(m, 8) * Fraction(1, 64)
    return s * (1 + Fraction(m, 8)) * Fraction(2) ** (e - 7)


E4M3 = {}
for _c in range(256):
    try:
        E4M3.setdefault(e4m3_val(_c), _c)
    except ValueError:
        pass


def e4m3_bits(v):
    return E4M3[Fraction(v)]


E2M1 = {Fraction(0): 0, Fraction(1, 2): 1, Fraction(1): 2, Fraction(3, 2): 3, Fraction(2): 4, Fraction(3): 5,
        Fraction(4): 6, Fraction(6): 7}


def e2m1_bits(v):
    v = Fraction(v)
    return E2M1[v] if v >= 0 else 8 | E2M1[-v]


def f32_val(u32):
    return Fraction(struct.unpack("<f", struct.pack("<I", u32 & 0xFFFFFFFF))[0])


def f32_bits(v):
    b = struct.unpack("<I", struct.pack("<f", float(v)))[0]
    assert f32_val(b) == Fraction(v)
    return b


def width_from_gaps(rows):
    """rows: list of (gap, exact?) for one (position, sign).  m = 1 + the largest g such that every gap <= g is
    exact; also report non-monotone gaps (exact again after a failure)."""
    rows = sorted(rows)
    good = None
    for g, ok in rows:
        if ok:
            good = g
        else:
            break
    first_fail = next((g for g, ok in rows if not ok), None)
    later_ok = [g for g, ok in rows if ok and first_fail is not None and g > first_fail]
    return dict(m=None if good is None else good + 1, first_fail_gap=first_fail, max_gap_tested=rows[-1][0],
                exact_after_fail=later_ok[:5])


def popc32(a):
    return np.bitwise_count(a.astype(np.uint32)).astype(np.int64)


# ---------------------------------------------------------------------------------------------- turing (sm_75 shapes)
def turing(binp, shrink):
    T = {}
    # (a) u8 / s8 wrap, layout-free.  Product counts P are reached as iters = P // 16 instructions plus a C preload
    # of (P % 16) products, so the README's 33,025 / 33,026 boundary is hit exactly (pure-instruction counts are
    # multiples of 16: 33,024 and 33,040 are also run with no preload).
    cases = []
    for P in (16, 33024, 33025, 33026, 33040, 66048, 66051, 66052, 66064, 1_000_000, 16 << 20):
        cases.append(("u8", 255, 255, P))
    for P in (131056, 131071, 131072, 131088, 262144):
        cases.append(("s8", -128, -128, P))
    for P in (132096, 132104, 132105, 132112):
        cases.append(("s8", -128, 127, P))
    for ty in ("u8", "s8"):
        cs = [c for c in cases if c[0] == ty]
        prm = np.array([[a & 0xFF, b & 0xFF, s32((P % 16) * a * b), P // 16] for _, a, b, P in cs], dtype=np.int64)
        pf, of = WORK / f"wrap75_{ty}.bin", WORK / f"wrap75_{ty}_out.bin"
        prm.tofile(pf)
        info = run_json([binp, "wrap", ty, pf, of])
        out = np.fromfile(of, dtype=np.int32).reshape(len(cs), 64)
        rows = []
        for (t, a, b, P), o in zip(cs, out):
            exact = P * a * b
            reg = int(o[0])
            uniform = bool((o == o[0]).all())
            mod_ok = (reg & 0xFFFFFFFF) == (exact % (1 << 32))
            signed_ok = reg == exact
            unsigned_ok = (reg & 0xFFFFFFFF) == exact
            want_signed = -(1 << 31) <= exact < (1 << 31)
            rows.append(dict(type=t, a=a, b=b, products=P, exact=exact, reg=reg, all_64_equal=uniform,
                             reg_eq_exact_mod_2_32=mod_ok, signed_exact=signed_ok, unsigned_exact=unsigned_ok))
            check(f"t4.wrap.{t}.a{a}b{b}.P{P}", uniform and mod_ok and signed_ok == want_signed,
                  exact=exact, reg=reg, signed_exact=signed_ok, predicted_signed_exact=want_signed)
        T[f"wrap_{ty}"] = dict(info=info, rows=rows)
    # the Lean bounds, recomputed here from the same integers
    check("lean.tensorzkp_bound", 33025 * 65025 < 2 ** 31 <= 33026 * 65025)
    check("lean.u8_unsigned_bound", 66051 * 65025 < 2 ** 32 <= 66052 * 65025)

    # (b) b1 xor.popc.  Chain (layout-free): every A word a, B word b: +4*popc(a^b) per instruction per element.
    chains = [(0xFFFFFFFF, 0, 0, 1), (0xFFFFFFFF, 0, 0, (1 << 24) - 1), (0xFFFFFFFF, 0, 0, 1 << 24),
              (0xFFFFFFFF, 0, 0, (1 << 25) + 5), (0xFFFFFFFF, 0, -7, 1), (0xAAAAAAAA, 0x0F0F0F0F, 1, 1000003),
              (0xFFFFFFFF, 0, -(1 << 31) + 3, 1 << 24)]
    if shrink:
        chains = chains  # the full set runs on the T4 in ~1 s
    prm = np.array([[a, b, c, n] for a, b, c, n in chains], dtype=np.int64)
    pf, of = WORK / "b1chain.bin", WORK / "b1chain_out.bin"
    prm.tofile(pf)
    info = run_json([binp, "b1chain", pf, of], timeout=900)
    out = np.fromfile(of, dtype=np.int32).reshape(len(chains), 64)
    rows = []
    for (a, b, c, n), o in zip(chains, out):
        exact = c + n * 4 * bin(a ^ b).count("1")
        reg = int(o[0])
        ok = bool((o == o[0]).all()) and (reg & 0xFFFFFFFF) == exact % (1 << 32)
        par = (reg & 1) == (exact & 1)
        rows.append(dict(a=hex(a), b=hex(b), c0=c, iters=n, exact=exact, reg=reg, wraps=exact >= 1 << 32 or exact < 0,
                         parity_equal=par))
        check(f"t4.b1chain.a{a:08x}.b{b:08x}.c{c}.n{n}", ok and par, exact=exact, reg=reg)
    T["b1chain"] = dict(info=info, rows=rows)

    # b1 random words through the fragment layout, half the sequences preloaded just below 2^32 so they wrap.
    rng = np.random.default_rng(75)
    S, L = (64, 1024)
    A = rng.integers(0, 1 << 32, size=(S, L, 32), dtype=np.uint64).astype(np.uint32)
    B = rng.integers(0, 1 << 32, size=(S, L, 32), dtype=np.uint64).astype(np.uint32)
    C = rng.integers(-(1 << 31), 1 << 31, size=(S, 32, 2), dtype=np.int64)
    C[: S // 2] = (1 << 32) - rng.integers(1, L * 128, size=(S // 2, 32, 2)) - (1 << 32)  # = -small: wraps up through 0
    files = [WORK / f"b1r_{x}.bin" for x in "ABCD"]
    A.tofile(files[0]); B.tofile(files[1]); C.astype(np.int32).tofile(files[2])
    info = run_json([binp, "b1rand", S, L] + files, timeout=900)
    D = np.fromfile(files[3], dtype=np.int32).reshape(S, 32, 2).astype(np.int64)
    # layout: lane = g*4 + q; A row g bits k = q*32 + i from lane g*4+q; B col n bits from lanes n*4+q.
    Ar, Br = A.reshape(S, L, 8, 4), B.reshape(S, L, 8, 4)
    X = np.zeros((S, 8, 8), dtype=np.int64)
    AND = np.zeros((S, 8, 8), dtype=np.int64)
    for r in range(8):
        X[:, r, :] = popc32(Ar[:, :, r, None, :] ^ Br[:, :, :, :]).sum(axis=(1, 3))
        AND[:, r, :] = popc32(Ar[:, :, r, None, :] & Br[:, :, :, :]).sum(axis=(1, 3))
    pa = popc32(Ar).sum(axis=(1, 3))          # (S, 8) per row
    pb = popc32(Br).sum(axis=(1, 3))          # (S, 8) per col
    want = np.zeros((S, 32, 2), dtype=np.int64)
    for lane in range(32):
        g, q = lane >> 2, lane & 3
        for j in range(2):
            want[:, lane, j] = C[:, lane, j] + X[:, g, q * 2 + j]
    ok_exact = ((D - want) % (1 << 32) == 0)
    # GF(2) inner product from an xor-only MMA: AND = (pa + pb - XOR) / 2, so its parity needs XOR mod 4,
    # which survives the mod-2^32 wrap.  Derive it from the device register and compare with the direct AND.
    par_dev = np.zeros((S, 32, 2), dtype=np.int64)
    par_host = np.zeros((S, 32, 2), dtype=np.int64)
    for lane in range(32):
        g, q = lane >> 2, lane & 3
        for j in range(2):
            n = q * 2 + j
            xs = (D[:, lane, j] - C[:, lane, j]) % (1 << 32)
            par_dev[:, lane, j] = ((pa[:, g] + pb[:, n] - xs) % 4) // 2
            par_host[:, lane, j] = AND[:, g, n] & 1
    crossed = int(((C < 0) & (want >= 0)).sum())
    check("t4.b1rand.exact_mod_2_32", ok_exact.all(), elements=int(ok_exact.size), mismatches=int((~ok_exact).sum()),
          sums_crossing_2_32=crossed, S=S, L=L)
    check("t4.b1rand.gf2_parity_from_xor_through_wrap", (par_dev == par_host).all(), elements=int(par_dev.size),
          mismatches=int((par_dev != par_host).sum()), sums_crossing_2_32=crossed)
    T["b1rand"] = dict(info=info, S=S, L=L, crossing=crossed)

    # (c) fp16 m16n8k8 through the fragment layout
    tests = []   # (name, A dict{(r,k):v}, B dict{(k,n):v}, C dict{(r,n):v}, meta)
    rng = np.random.default_rng(1688)
    for t in range(16):   # layout sanity: random small integers everywhere, all sums far below 2^11
        Aa = {(r, k): int(rng.integers(-4, 5)) for r in range(16) for k in range(8)}
        Bb = {(k, n): int(rng.integers(-4, 5)) for k in range(8) for n in range(8)}
        Cc = {(r, n): int(rng.integers(-64, 65)) for r in range(16) for n in range(8)}
        tests.append(("sanity", Aa, Bb, Cc, {}))
    for j in range(0, 31):   # C path, layout-free: every product 1/8, K = 8: D = 2^j + 1
        tests.append(("cpath", {(r, k): Fraction(1, 2) for r in range(16) for k in range(8)},
                      {(k, n): Fraction(1, 4) for k in range(8) for n in range(8)},
                      {(r, n): 2 ** j for r in range(16) for n in range(8)}, dict(j=j)))
    for j in range(0, 31):   # C path with integer addends (the Lean leaves): every product 1, D = 2^j + 8
        tests.append(("cint", {(r, k): 1 for r in range(16) for k in range(8)},
                      {(k, n): 1 for k in range(8) for n in range(8)},
                      {(r, n): 2 ** j for r in range(16) for n in range(8)}, dict(j=j)))
    for kb in (0, 4):        # within-instruction: big product 2^16 at k = kb, small s*2^(16-g) at k = ks, C = 0
        for ks in range(8):
            if ks == kb:
                continue
            for s in (1, -1):
                for g in range(1, 41):
                    e = 16 - g
                    tests.append(("group", {(0, kb): 256, (0, ks): 1}, {(kb, 0): 256, (ks, 0): s * Fraction(2) ** e}, {},
                                  dict(kb=kb, ks=ks, s=s, g=g)))
    for s in (1, -1):        # C holds the big term, one small product (C path with a single product), and reverse
        for g in range(1, 41):
            e = 16 - g
            tests.append(("cbig", {(0, 3): 1}, {(3, 0): s * Fraction(2) ** e}, {(0, 0): 2 ** 16}, dict(s=s, g=g)))
            if e >= -24 + 0:
                tests.append(("csmall", {(0, 5): 256}, {(5, 0): 256}, {(0, 0): s * Fraction(2) ** e}, dict(s=s, g=g)))
    nT = len(tests)
    Ah = np.zeros((nT, 16, 8), dtype=np.uint16)
    Bh = np.zeros((nT, 8, 8), dtype=np.uint16)
    Ch = np.zeros((nT, 16, 8), dtype=np.uint32)
    for t, (_, Aa, Bb, Cc, _) in enumerate(tests):
        for (r, k), v in Aa.items():
            Ah[t, r, k] = f16_bits(v)
        for (k, n), v in Bb.items():
            Bh[t, k, n] = f16_bits(v)
        for (r, n), v in Cc.items():
            Ch[t, r, n] = f32_bits(v)
    files = [WORK / f"f16f_{x}.bin" for x in "ABCD"]
    Ah.tofile(files[0]); Bh.tofile(files[1]); Ch.tofile(files[2])
    info = run_json([binp, "f16frag", nT] + files)
    Dh = np.fromfile(files[3], dtype=np.uint32).reshape(nT, 16, 8)
    sanity_bad, groups, cpath, cbig, csmall, cint = 0, {}, [], {}, {}, []
    for t, (kind, Aa, Bb, Cc, meta) in enumerate(tests):
        def exact(r, n):
            return Fraction(Cc.get((r, n), 0)) + sum(Fraction(Aa.get((r, k), 0)) * Fraction(Bb.get((k, n), 0)) for k in range(8))
        if kind == "sanity":
            sanity_bad += sum(f32_val(int(Dh[t, r, n])) != exact(r, n) for r in range(16) for n in range(8))
            continue
        got, want = f32_val(int(Dh[t, 0, 0])), exact(0, 0)
        ok = got == want
        if kind == "cpath":
            other_ok = all(f32_val(int(Dh[t, r, n])) == exact(r, n) for r in range(16) for n in range(8))
            cpath.append((meta["j"], ok and other_ok, float(got)))
        elif kind == "cint":
            cint.append((meta["j"], ok, float(got)))
        elif kind == "group":
            groups.setdefault((meta["kb"], meta["ks"], meta["s"]), []).append((meta["g"], ok, float(got - want)))
        elif kind == "cbig":
            cbig.setdefault(meta["s"], []).append((meta["g"], ok, float(got - want)))
        else:
            csmall.setdefault(meta["s"], []).append((meta["g"], ok, float(got - want)))
    check("t4.f16frag.layout_sanity", sanity_bad == 0, mismatches=int(sanity_bad), tests=16, elements=16 * 128)
    m_c = width_from_gaps([(j, ok) for j, ok, _ in cpath])
    gw = {f"kb{kb}_ks{ks}_{'+' if s > 0 else '-'}": dict(width_from_gaps([(g, ok) for g, ok, _ in v]),
                                                           err_at_first_fail=next((e for g, ok, e in sorted(v) if not ok), None))
          for (kb, ks, s), v in sorted(groups.items())}
    T["f16"] = dict(info=info, tests=nT,
                    cpath_m=dict(m_c, note="D = 2^j + 1 exact for j < m; f32 output caps the measurable m at 24"),
                    cpath_rows=cpath,
                    cint_jmax=dict(width_from_gaps([(j, ok) for j, ok, _ in cint]),
                                   note="C = 2^j, eight products of 1: exact for j <= jmax; 2^j + 8 is representable in "
                                        "f32 up to j = 26, so a smaller jmax is the alignment window for integer addends"),
                    cint_rows=cint,
                    group_m=gw,
                    cbig_m={str(s): dict(width_from_gaps([(g, ok) for g, ok, _ in v]),
                                         err_at_first_fail=next((e for g, ok, e in sorted(v) if not ok), None))
                            for s, v in cbig.items()},
                    csmall_m={str(s): dict(width_from_gaps([(g, ok) for g, ok, _ in v]),
                                           err_at_first_fail=next((e for g, ok, e in sorted(v) if not ok), None))
                              for s, v in csmall.items()})
    ms = sorted({v["m"] for v in gw.values()})
    log("f16 C-path m", m_c["m"], "group m by position:", ms)
    return T


def rates(binp, kinds, ms):
    out = {}
    for k in kinds:
        try:
            out[k] = run_json([binp, "rate", k, ms], timeout=300)[0]
            log("rate", k, out[k].get("macs_per_s", out[k].get("lop3_per_s")), "unsupported" if out[k].get("unsupported") else "")
        except Exception as e:
            out[k] = {"error": str(e)[-400:]}
            log("rate", k, "ERROR", str(e)[-200:])
    return out


# ---------------------------------------------------------------------------------------------- ampere+ shapes
def ampere(binp, cc):
    T = {}
    cases = [("u8", 255, 255, P) for P in (32, 33024, 33025, 33026, 33056, 66051, 66052, 1 << 22)]
    cases += [("s8", -128, -128, P) for P in (131071, 131072)] + [("s8", -128, 127, P) for P in (132104, 132105)]
    for ty in ("u8", "s8"):
        cs = [c for c in cases if c[0] == ty]
        prm = np.array([[a & 0xFF, b & 0xFF, s32((P % 32) * a * b), P // 32] for _, a, b, P in cs], dtype=np.int64)
        pf, of = WORK / f"wrap80_{ty}.bin", WORK / f"wrap80_{ty}_out.bin"
        prm.tofile(pf)
        info = run_json([binp, "wrap", ty, pf, of])
        if info[0].get("unsupported"):
            T[f"wrap_{ty}"] = "unsupported"
            continue
        out = np.fromfile(of, dtype=np.int32).reshape(len(cs), 128)
        for (t, a, b, P), o in zip(cs, out):
            exact = P * a * b
            reg = int(o[0])
            check(f"sm80.wrap.{t}.a{a}b{b}.P{P}", bool((o == o[0]).all()) and (reg & 0xFFFFFFFF) == exact % (1 << 32)
                  and (reg == exact) == (-(1 << 31) <= exact < (1 << 31)), exact=exact, reg=reg)
    # b1 m16n8k256 and/xor through the fragment layout
    rng = np.random.default_rng(80)
    S, L = 32, 512
    for op in ("and", "xor"):
        A = rng.integers(0, 1 << 32, size=(S, L, 32, 4), dtype=np.uint64).astype(np.uint32)
        B = rng.integers(0, 1 << 32, size=(S, L, 32, 2), dtype=np.uint64).astype(np.uint32)
        C = rng.integers(-(1 << 31), 1 << 31, size=(S, 32, 4), dtype=np.int64)
        C[: S // 2] = -rng.integers(1, L * 1024, size=(S // 2, 32, 4))
        files = [WORK / f"b1r80_{op}_{x}.bin" for x in "ABCD"]
        A.tofile(files[0]); B.tofile(files[1]); C.astype(np.int32).tofile(files[2])
        info = run_json([binp, "b1rand", op, S, L] + files, timeout=900)
        if info[0].get("unsupported"):
            T[f"b1rand_{op}"] = "unsupported"
            continue
        D = np.fromfile(files[3], dtype=np.int32).reshape(S, 32, 4).astype(np.int64)
        f = (lambda x, y: x & y) if op == "and" else (lambda x, y: x ^ y)
        # A row r (r < 8: lanes r*4+q regs 0 (k<128) and 2 (k>=128); r >= 8: regs 1, 3); B col n: lanes n*4+q regs 0, 1
        Arow = np.zeros((S, L, 16, 4, 2), dtype=np.uint32)
        for r in range(16):
            g, hi = r % 8, r >= 8
            Arow[:, :, r, :, 0] = A[:, :, g * 4:g * 4 + 4, 1 if hi else 0]
            Arow[:, :, r, :, 1] = A[:, :, g * 4:g * 4 + 4, 3 if hi else 2]
        Bcol = B.reshape(S, L, 8, 4, 2)
        Y = np.zeros((S, 16, 8), dtype=np.int64)
        for r in range(16):
            Y[:, r, :] = popc32(f(Arow[:, :, r, None, :, :], Bcol)).sum(axis=(1, 3, 4))
        want = np.zeros((S, 32, 4), dtype=np.int64)
        for lane in range(32):
            g, q = lane >> 2, lane & 3
            for j in range(4):
                want[:, lane, j] = C[:, lane, j] + Y[:, g + (8 if j >= 2 else 0), q * 2 + (j & 1)]
        ok = (D - want) % (1 << 32) == 0
        check(f"sm80.b1rand.{op}.exact_mod_2_32", ok.all(), elements=int(ok.size), mismatches=int((~ok).sum()),
              sums_crossing_2_32=int(((C < 0) & (want >= 0)).sum()))
    # 8-bit fragments: sanity (u8, s8, e4m3) and the e4m3 within-instruction gap probe
    for ty in ("u8", "s8") + (("e4m3",) if cc == 89 else ()):
        tests = []
        for t in range(16):
            if ty == "e4m3":
                Aa = {(r, k): Fraction(int(rng.integers(-3, 4))) for r in range(16) for k in range(32)}
                Bb = {(k, n): Fraction(int(rng.integers(-3, 4))) for k in range(32) for n in range(8)}
                Cc = {(r, n): Fraction(int(rng.integers(-64, 65))) for r in range(16) for n in range(8)}
            else:
                lo, hi = (0, 256) if ty == "u8" else (-128, 128)
                Aa = {(r, k): int(rng.integers(lo, hi)) for r in range(16) for k in range(32)}
                Bb = {(k, n): int(rng.integers(lo, hi)) for k in range(32) for n in range(8)}
                Cc = {(r, n): int(rng.integers(-(1 << 31), 1 << 31)) for r in range(16) for n in range(8)}
            tests.append(("sanity", Aa, Bb, Cc, {}))
        if ty == "e4m3":
            for kb in (0, 16):
                for ks in (1, 3, 4, 7, 8, 15, 16, 17, 24, 31):
                    if ks == kb:
                        continue
                    for s in (1, -1):
                        for asm in (8, -6):
                            for e in range(-6, 9):
                                g = 16 - (asm + e)
                                if g < 1:
                                    continue
                                tests.append(("group", {(0, kb): 256, (0, ks): Fraction(2) ** asm},
                                              {(kb, 0): 256, (ks, 0): s * Fraction(2) ** e}, {},
                                              dict(kb=kb, ks=ks, s=s, g=g)))
            for j in range(0, 31):
                tests.append(("cint", {(r, k): 1 for r in range(16) for k in range(32)},
                              {(k, n): 1 for k in range(32) for n in range(8)},
                              {(r, n): 2 ** j for r in range(16) for n in range(8)}, dict(j=j)))
            for j in range(0, 27):
                tests.append(("cpath", {(r, k): Fraction(1, 4) for r in range(16) for k in range(32)},
                              {(k, n): Fraction(1, 8) for k in range(32) for n in range(8)},
                              {(r, n): 2 ** j for r in range(16) for n in range(8)}, dict(j=j)))
        nT = len(tests)
        Ah = np.zeros((nT, 16, 32), dtype=np.uint8)
        Bh = np.zeros((nT, 32, 8), dtype=np.uint8)
        Ch = np.zeros((nT, 16, 8), dtype=np.uint32)
        enc = e4m3_bits if ty == "e4m3" else (lambda v: int(v) & 0xFF)
        cenc = f32_bits if ty == "e4m3" else (lambda v: int(v) & 0xFFFFFFFF)
        for t, (_, Aa, Bb, Cc, _) in enumerate(tests):
            for (r, k), v in Aa.items():
                Ah[t, r, k] = enc(v)
            for (k, n), v in Bb.items():
                Bh[t, k, n] = enc(v)
            for (r, n), v in Cc.items():
                Ch[t, r, n] = cenc(v)
        files = [WORK / f"frag8_{ty}_{x}.bin" for x in "ABCD"]
        Ah.tofile(files[0]); Bh.tofile(files[1]); Ch.tofile(files[2])
        info = run_json([binp, "frag8", ty, nT] + files)
        if info[0].get("unsupported"):
            T[f"frag8_{ty}"] = "unsupported"
            continue
        Dh = np.fromfile(files[3], dtype=np.uint32).reshape(nT, 16, 8)
        bad, groups, cp, ci = 0, {}, [], []
        for t, (kind, Aa, Bb, Cc, meta) in enumerate(tests):
            def ex(r, n):
                return Fraction(Cc.get((r, n), 0)) + sum(Fraction(Aa.get((r, k), 0)) * Fraction(Bb.get((k, n), 0)) for k in range(32))
            def got(r, n):
                return f32_val(int(Dh[t, r, n])) if ty == "e4m3" else s32(int(Dh[t, r, n]))
            def want(r, n):
                return ex(r, n) if ty == "e4m3" else s32(int(ex(r, n)))
            if kind == "sanity":
                bad += sum(got(r, n) != want(r, n) for r in range(16) for n in range(8))
            elif kind == "group":
                groups.setdefault((meta["kb"], meta["ks"], meta["s"]), []).append((meta["g"], got(0, 0) == want(0, 0)))
            elif kind == "cint":
                ci.append((meta["j"], got(0, 0) == want(0, 0)))
            else:
                cp.append((meta["j"], got(0, 0) == want(0, 0)))
        check(f"sm80.frag8.{ty}.layout_sanity", bad == 0, mismatches=int(bad))
        if ty == "e4m3":
            T["e4m3_mma_sync"] = dict(
                note="on sm_90a/sm_100a ptxas lowers mma.sync e4m3 to F2FP unpack + FP16 math (see SASS census): "
                     "only sm_89 runs it on QMMA",
                cpath=width_from_gaps(cp),
                cint_jmax=width_from_gaps(ci),
                group={f"kb{kb}_ks{ks}_{'+' if s > 0 else '-'}": width_from_gaps(v) for (kb, ks, s), v in sorted(groups.items())})
    return T


def wgmma(binp):
    """sm_90a native GMMA: layout-free exactness, C-path and in-instruction width through repeated K patterns"""
    T = {}
    def run(kind, iters, pa, pb, c):
        of = WORK / f"wg_{kind}.bin"
        info = run_json([binp, "wgcheck", kind, iters, bytes(pa).hex(), bytes(pb).hex(), f"{c & 0xFFFFFFFF:08x}", of],
                        timeout=600)[0]
        if info.get("unsupported"):
            return None
        return np.fromfile(of, dtype=np.uint32)
    ksz = {"s8": 32, "b1": 256, "e4m3": 32, "f16": 16}
    # integer exactness and wrap, uniform patterns
    for kind, byte, iters, per in (("s8", 0x7F, 1, 32 * 127 * 127), ("s8", 0x7F, 4161, 32 * 127 * 127),
                                   ("b1", 0xFF, 1, 256), ("b1", 0xFF, (1 << 24) + 3, 256)):
        d = run(kind, iters, [byte] * 16, [byte] * 16, 0)
        if d is None:
            T["unsupported"] = True
            return T
        exact = iters * per
        check(f"wgmma.{kind}.uniform.n{iters}", bool((d == d[0]).all()) and int(d[0]) == exact % (1 << 32),
              exact=exact, reg=int(d[0]), wraps=exact >= 1 << 31)
    # count path (C path across instructions): every product 1, D = K * iters; find where it stops being exact
    for kind, one in (("e4m3", e4m3_bits(1)), ("f16", None)):
        pat = [one] * 16 if kind == "e4m3" else list(struct.pack("<8H", *[f16_bits(1)] * 8))
        rows = []
        for lg in range(4, 21):
            it = 1 << lg
            d = run(kind, it, pat, pat, 0)
            v = f32_val(int(d[0]))
            rows.append((int(np.log2(ksz[kind] * it)), bool((d == d[0]).all()) and v == ksz[kind] * it))
            it2 = it + 1
            d = run(kind, it2, pat, pat, 0)
            v = f32_val(int(d[0]))
            rows.append((int(np.log2(ksz[kind] * it)) + 0.5, bool((d == d[0]).all()) and v == ksz[kind] * it2))
        T[f"{kind}_count"] = dict(rows=rows, note="exponent of the running sum K*iters (x.5 = one more instruction "
                                                  "past the power of two); exact while the accumulator holds it")
    # in-instruction gap probe: pattern slot 0 big, slot p small; K repeats the 16-byte pattern
    for kind in ("e4m3", "f16"):
        per = 16 if kind == "e4m3" else 8
        res = {}
        for p in (1, 3, per - 1):
            for s in (1, -1):
                rows = []
                for g in range(1, 30):
                    if kind == "e4m3":
                        a_s = 8 if g <= 14 else -6
                        e = 16 - g - a_s
                        if not -6 <= e <= 8:
                            continue
                        pa = [0] * 16; pb = [0] * 16
                        pa[0], pb[0] = e4m3_bits(256), e4m3_bits(256)
                        pa[p], pb[p] = e4m3_bits(Fraction(2) ** a_s), e4m3_bits(s * Fraction(2) ** e)
                        vals_a = [e4m3_val(x) for x in pa]; vals_b = [e4m3_val(x) for x in pb]
                    else:
                        e = 16 - g
                        ha = [0] * 8; hb = [0] * 8
                        ha[0], hb[0] = f16_bits(256), f16_bits(256)
                        ha[p], hb[p] = f16_bits(1), f16_bits(s * Fraction(2) ** e)
                        pa, pb = list(struct.pack("<8H", *ha)), list(struct.pack("<8H", *hb))
                        vals_a = [f16_val(x) for x in ha]; vals_b = [f16_val(x) for x in hb]
                    exact = sum(vals_a[k % per] * vals_b[k % per] for k in range(ksz[kind]))
                    d = run(kind, 1, pa, pb, 0)
                    rows.append((g, bool((d == d[0]).all()) and f32_val(int(d[0])) == exact))
                res[f"p{p}_{'+' if s > 0 else '-'}"] = width_from_gaps(rows)
        T[f"{kind}_group"] = res
        log("wgmma", kind, "group widths", {k: v["m"] for k, v in res.items()})
    # C path with integer addends: C = 2^j, every product 1 (K per instruction): D = 2^j + K
    for kind in ("e4m3", "f16"):
        one = [e4m3_bits(1)] * 16 if kind == "e4m3" else list(struct.pack("<8H", *[f16_bits(1)] * 8))
        rows = []
        for j in range(0, 31):
            d = run(kind, 1, one, one, f32_bits(2 ** j))
            rows.append((j, bool((d == d[0]).all()) and f32_val(int(d[0])) == 2 ** j + ksz[kind]))
        T[f"{kind}_cint_jmax"] = width_from_gaps(rows)
    # C path with one instruction: C = 2^j, products summing to 1
    for kind in ("e4m3", "f16"):
        rows = []
        for j in range(0, 27):
            if kind == "e4m3":
                pa, pb = [e4m3_bits(Fraction(1, 4))] * 16, [e4m3_bits(Fraction(1, 8))] * 16
            else:
                pa = list(struct.pack("<8H", *[f16_bits(Fraction(1, 4))] * 8))
                pb = list(struct.pack("<8H", *[f16_bits(Fraction(1, 4))] * 8))
            d = run(kind, 1, pa, pb, f32_bits(2 ** j))
            rows.append((j, bool((d == d[0]).all()) and f32_val(int(d[0])) == 2 ** j + 1))
        T[f"{kind}_cpath"] = width_from_gaps(rows)
    return T


# ---------------------------------------------------------------------------------------------- library steps
def i8ip_check(dump, layout, logn, batches):
    bad = 0
    for b in batches:
        s = dump / f"i8ip_{layout}_{logn}_b{b}"
        V = np.fromfile(f"{s}_V.bin", dtype=np.int8).astype(np.int64)
        U = np.fromfile(f"{s}_U.bin", dtype=np.int8).astype(np.int64)
        D = np.fromfile(f"{s}_D.bin", dtype=np.int32).astype(np.int64).reshape(32, 32).T   # D[i][j], col-major
        if layout == "elem":
            want = V.reshape(32768, 32).T @ U.reshape(32768, 32)
        else:
            want = V.reshape(32, 32768) @ U.reshape(32, 32768).T
        bad += int((want != D).sum())
    return bad


def ltip_check(dump, ty, logn, limbs, batches):
    bad, maxsum = 0, 0
    for b in batches:
        s = dump / f"ltip_{ty}_{logn}_b{b}"
        raw_v, raw_u = np.fromfile(f"{s}_V.bin", dtype=np.uint8), np.fromfile(f"{s}_U.bin", dtype=np.uint8)
        if ty == "f16":
            V = raw_v.view(np.float16).astype(np.int64); U = raw_u.view(np.float16).astype(np.int64)
        else:
            lut = np.array([int(e4m3_val(c)) if c not in (0x7F, 0xFF) and e4m3_val(c).denominator == 1 else -999
                            for c in range(256)], dtype=np.int64)
            V, U = lut[raw_v], lut[raw_u]
        assert V.min() >= 0 and V.max() <= 15 and U.min() >= 0 and U.max() <= 15
        want = V.reshape(limbs, 32768) @ U.reshape(limbs, 32768).T
        D = np.fromfile(f"{s}_D.bin", dtype=np.float32).reshape(limbs, limbs).T
        bad += int((D.astype(np.float64) != want.astype(np.float64)).sum())
        maxsum = max(maxsum, int(want.max()))
    return bad, maxsum


def ltprobe(binp, ty, fast):
    """three GEMM-API probes for one type: sparse +-1 sanity, gap (big at k=0, small at k=ks), count (sum of ones)"""
    enc = {"f16": lambda v: f16_bits(v), "e4m3": e4m3_bits, "e2m1": e2m1_bits}[ty]
    np_t = np.uint16 if ty == "f16" else np.uint8
    T = {}

    def pack_codes(codes):   # rows x K uint16 codes -> bytes, K-contiguous (e2m1: element 2i in the low nibble)
        if ty == "e2m1":
            return (codes[:, 0::2] | (codes[:, 1::2] << 4)).astype(np.uint8)
        return codes.astype(np_t)

    def pack(mat):
        return pack_codes(np.array([[enc(v) for v in row] for row in mat], dtype=np.uint16))

    def gemm(name, Am, Bm, K):
        Ac = Am if isinstance(Am, np.ndarray) else None
        Bc = Bm if isinstance(Bm, np.ndarray) else None
        M, N = len(Am), len(Bm)
        fa, fb, fd = (WORK / f"lt_{ty}_{fast}_{name}_{x}.bin" for x in "ABD")
        (pack_codes(Ac) if Ac is not None else pack(Am)).tofile(fa)
        (pack_codes(Bc) if Bc is not None else pack(Bm)).tofile(fb)
        info = run_json([binp, "ltprobe", ty, M, N, K, fast, fa, fb, fd], timeout=600)[0]
        D = np.fromfile(fd, dtype=np.float32).reshape(-1, N, M) if info["n_ran"] else None
        return info, D

    # sanity: sparse +-1, 32 nonzeros per row/col, K = 2048
    rng = np.random.default_rng(7)
    M, N, K = 32, 32, 2048
    Am = [[0] * K for _ in range(M)]
    Bm = [[0] * K for _ in range(N)]
    for r in range(M):
        for k in rng.choice(K, 32, replace=False):
            Am[r][k] = int(rng.choice([-1, 1]))
    for c in range(N):
        for k in rng.choice(K, 32, replace=False):
            Bm[c][k] = int(rng.choice([-1, 1]))
    info, D = gemm("sanity", Am, Bm, K)
    want = np.array(Am, dtype=np.int64) @ np.array(Bm, dtype=np.int64).T
    if D is None:
        check(f"lt.{ty}.fast{fast}.sanity", False, status=info["heuristic_status"], n_heuristic=info["n_heuristic"])
        T["sanity"] = info
        return T
    bad = [int((D[a].T != want).sum()) for a in range(D.shape[0])]
    check(f"lt.{ty}.fast{fast}.sanity", all(b == 0 for b in bad), mismatches_per_algo=bad, algos=info["algos"])
    T["sanity"] = dict(info=info, mismatches_per_algo=bad)

    # count probe: A row = ones, B column j = ones on its first n_j entries: D = n_j.  Exact while the
    # accumulation path (in-instruction adds, C path across instructions, split-K reduction) holds n_j.
    lgK = 20
    K = 1 << lgK
    ns = sorted({1 << e for e in range(4, lgK + 1)} | {(1 << e) + 1 for e in range(4, lgK)} | {(1 << e) + 16 for e in range(4, lgK)})
    while len(ns) % 16:
        ns.append(K)
    c1 = enc(1)
    Am = np.full((16, K), c1, dtype=np.uint16)
    Bm = np.zeros((len(ns), K), dtype=np.uint16)
    for j, n in enumerate(ns):
        Bm[j, :n] = c1
    info, D = gemm("count", Am, Bm, K)
    if D is not None:
        per_algo = []
        for a in range(D.shape[0]):
            rows = [(n, bool((D[a, j] == n).all()), float(D[a, j, 0])) for j, n in enumerate(ns)]
            first_bad = next((n for n, ok, _ in rows if not ok), None)
            per_algo.append(dict(algo=info["algos"][a], first_inexact_n=first_bad,
                                 got_at_first_inexact=next((v for n, ok, v in rows if not ok), None),
                                 all_exact_to=max([n for n, ok, _ in rows if ok and (first_bad is None or n < first_bad)], default=0)))
        T["count"] = dict(K=K, per_algo=per_algo)
        log("lt", ty, "fast", fast, "count probe first inexact n per algo:", [p["first_inexact_n"] for p in per_algo])
    if ty == "e2m1":
        return T

    # gap probe: row i = (ks_i, a_small_exp_i): A[i][0] = 2^8, A[i][ks] = 2^a_s; column j = (s_j, e_j):
    # B[0][j] = 2^8, B[k][j] = s_j 2^e_j for k >= 1.  D[i][j] = 2^16 + s_j 2^(a_s + e_j): gap g = 16 - a_s - e_j.
    K = 2048
    poss = [1, 2, 3, 4, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128, 255, 256, 511, 1024, 2047]
    if ty == "f16":
        rows_spec = [(p, 0) for p in poss]
        cols_spec = [(s, e) for s in (1, -1) for e in range(15, -25, -1)]
    else:
        rows_spec = [(p, a) for p in poss for a in (8, -6)]
        cols_spec = [(s, e) for s in (1, -1) for e in range(8, -7, -1)]
    while len(rows_spec) % 16:
        rows_spec.append((None, 0))
    while len(cols_spec) % 16:
        cols_spec.append((0, 0))
    Am, Bm = [], []
    for ks, a_s in rows_spec:
        r = [0] * K
        r[0] = 256
        if ks is not None:
            r[ks] = Fraction(2) ** a_s
        Am.append(r)
    for s, e in cols_spec:
        c = [s * Fraction(2) ** e if s else 0] * K
        c[0] = 256
        Bm.append(c)
    info, D = gemm("gap", Am, Bm, K)
    if D is not None:
        per_algo = []
        for a in range(D.shape[0]):
            by = {}
            for i, (ks, a_s) in enumerate(rows_spec):
                if ks is None:
                    continue
                for j, (s, e) in enumerate(cols_spec):
                    if not s:
                        continue
                    want = Fraction(2) ** 16 + s * Fraction(2) ** (a_s + e)
                    g = 16 - a_s - e
                    if g < 1:
                        continue
                    by.setdefault((ks, s), []).append((g, Fraction(float(D[a, j, i])) == want))
            per_algo.append(dict(algo=info["algos"][a],
                                 m={f"ks{ks}_{'+' if s > 0 else '-'}": width_from_gaps(v)["m"] for (ks, s), v in sorted(by.items())}))
        T["gap"] = dict(K=K, per_algo=per_algo)
        log("lt", ty, "fast", fast, "gap m (algo 0):", per_algo[0]["m"])
    return T


# ---------------------------------------------------------------------------------------------- rotation (ROTATION.md section 5)
def f16_parity(binp):
    """FP16 0/1 parity path with the bias read on HMMA m16n8k8: C = 2^(m'-1), count n of 0/1 products; while
    n < 2^(m'-1) the register must equal 2^(m'-1) + n exactly and raw f32 bit (24 - m') must be n mod 2."""
    rng = np.random.default_rng(2401)
    P = 1024
    Am = rng.integers(0, 2, size=(P, 16, 8))       # A[i][row][k]
    Bm = rng.integers(0, 2, size=(P, 8, 8))        # B[i][k][col]
    one = f16_bits(1)
    Aw = np.zeros((P, 32, 2), dtype=np.uint32)
    Bw = np.zeros((P, 32), dtype=np.uint32)
    for lane in range(32):
        g, q = lane >> 2, lane & 3
        Aw[:, lane, 0] = Am[:, g, 2 * q] * one + (Am[:, g, 2 * q + 1] * one << 16)
        Aw[:, lane, 1] = Am[:, g + 8, 2 * q] * one + (Am[:, g + 8, 2 * q + 1] * one << 16)
        Bw[:, lane] = Bm[:, 2 * q, g] * one + (Bm[:, 2 * q + 1, g] * one << 16)
    per = np.einsum("irk,ikc->irc", Am, Bm)                         # counts per instruction, (P, 16, 8)
    S = np.concatenate([np.zeros((1, 16, 8), dtype=np.int64), np.cumsum(per, axis=0)])   # prefix sums
    full = int(S[P].mean())
    tests = []
    for mb in (24, 20, 16, 13):
        bound = 1 << (mb - 1)
        for f in (0.0, 0.25, 0.97, 1.03, 1.5):
            R_ = int(f * bound / full)
            T_ = int(rng.integers(0, P))
            tests.append((mb, P, R_, T_))
        tests.append((mb, 7, 0, 7))
    prm = np.array([[Pu, R_, T_, f32_bits(Fraction(2) ** (mb - 1))] for mb, Pu, R_, T_ in tests], dtype=np.int64)
    files = [WORK / f"f16par_{x}.bin" for x in ("prm", "A", "B", "D")]
    prm.tofile(files[0]); Aw.tofile(files[1]); Bw.tofile(files[2])
    info = run_json([binp, "f16parity"] + files, timeout=900)[0]
    D = np.fromfile(files[3], dtype=np.uint32).reshape(len(tests), 32, 4)
    out = []
    for t, (mb, Pu, R_, T_) in enumerate(tests):
        cnt = R_ * S[Pu] + S[T_]                                    # (16, 8)
        bit = 24 - mb
        inside = ok_inside = past = past_read_ok = 0
        for lane in range(32):
            g, q = lane >> 2, lane & 3
            for j in range(4):
                r, c = g + (8 if j >= 2 else 0), 2 * q + (j & 1)
                n = int(cnt[r, c])
                raw = int(D[t, lane, j])
                read = (raw >> bit) & 1
                if n < (1 << (mb - 1)):
                    inside += 1
                    ok_inside += (f32_val(raw) == (1 << (mb - 1)) + n) and read == (n & 1)
                else:
                    past += 1
                    past_read_ok += read == (n & 1)
        row = dict(m=mb, P=Pu, R=R_, T=T_, count_min=int(cnt.min()), count_max=int(cnt.max()), inside=inside,
                   inside_ok=ok_inside, past=past, past_read_matches=past_read_ok)
        out.append(row)
        check(f"f16parity.m{mb}.R{R_}.T{T_}", ok_inside == inside, **row)
    fals = [r for r in out if r["past"] and r["past_read_matches"] < r["past"]]
    check("f16parity.falsifier_past_bound", bool(fals), note="past 2^(m-1) the bit no longer reads the parity",
          tests_with_wrong_reads=len(fals))
    return dict(info=info, tests=out)


def f16ip(binp, shrink, free, copy_bw):
    out = []
    for lg in ((20, 22, 24, 25) if shrink else (20, 22, 24, 25)):
        if 2 * 64 << lg > free * 0.8:
            continue
        r = run_json([binp, "f16ip", lg, 10 if shrink else 20, WORK], timeout=900)[0]
        host_bad = None
        if r.get("dumped"):
            V = np.fromfile(WORK / f"f16ip_{lg}_V.bin", dtype=np.float16).astype(np.float64).reshape(32, -1)
            U = np.fromfile(WORK / f"f16ip_{lg}_U.bin", dtype=np.float16).astype(np.float64).reshape(32, -1)
            want = (V @ U.T).astype(np.uint64)
            got = np.fromfile(WORK / f"f16ip_{lg}_D.bin", dtype=np.uint64).reshape(32, 32)
            host_bad = int((want != got).sum())
        r["host_mismatches"] = host_bad
        if copy_bw:
            r["ratio_to_copy_bw"] = r["bytes_per_s"] / copy_bw
        check(f"f16ip.2^{lg}", r["simt_ref_mismatches"] == 0 and host_bad in (None, 0),
              simt_ref_mismatches=r["simt_ref_mismatches"], host_mismatches=host_bad)
        log("f16ip", lg, r["ms"], "ms", f"{r['bytes_per_s']:.3e} B/s", f"x copy {r.get('ratio_to_copy_bw', 0):.2f}")
        out.append(r)
    return out


def ltparity(binp, ty, fast, shrink):
    """0/1 count and parity sweep through cuBLASLt, C = 2^23 (fp32 epilogue: the API-level bias read at m = 24)"""
    rng = np.random.default_rng(1 + fast)
    M = N = 64
    Kmax = 1 << 20
    A = rng.integers(0, 2, size=(M, Kmax), dtype=np.uint8)
    B = rng.integers(0, 2, size=(N, Kmax), dtype=np.uint8)
    A[0, :] = 1
    B[0, :] = 1
    c1 = {"f16": f16_bits(1), "e4m3": e4m3_bits(1)}.get(ty, 2)
    def pack(X):
        if ty in ("e2m1", "mxf4"):
            return (X[:, 0::2] * c1 | (X[:, 1::2] * c1) << 4).astype(np.uint8)
        return (X.astype(np.uint16) * c1).astype(np.uint16 if ty == "f16" else np.uint8)
    fa, fb = WORK / f"ltpar_{ty}_A.bin", WORK / f"ltpar_{ty}_B.bin"
    pack(A).tofile(fa); pack(B).tofile(fb)
    ks = [1 << e for e in (10, 12, 14, 16, 18, 20)]
    cb = f32_bits(2 ** 23)
    info = run_json([binp, "ltparity", ty, M, N, Kmax, fast, fa, fb, f"{cb:08x}", WORK / f"ltpar_{ty}_{fast}_D",
                     ",".join(map(str, ks))], timeout=1200)[0]
    Af, Bf = A.astype(np.float32), B.astype(np.float32)
    res = []
    first_inexact = None
    for run in info["runs"]:
        K = run["K"]
        if not run["n_ran"]:
            res.append(dict(K=K, status=run["heuristic_status"], n_ran=0))
            continue
        cnt = (Af[:, :K] @ Bf[:, :K].T).astype(np.int64)            # exact: sums < 2^24 in float32
        D = np.fromfile(WORK / f"ltpar_{ty}_{fast}_D_{K}.bin", dtype=np.float32).reshape(-1, N, M)
        per = []
        for a in range(D.shape[0]):
            Dm = D[a].T.astype(np.float64)                             # [i][j]
            exact = Dm - 2 ** 23 == cnt
            raw = D[a].T.copy().view(np.uint32)
            read = (raw & 1) == (cnt & 1)
            per.append(dict(algo=run["algos"][a], count_exact=bool(exact.all()), parity_read=bool(read.all()),
                            inexact=int((~exact).sum()), max_count=int(cnt.max())))
        res.append(dict(K=K, ms_first=run["ms_first"], per_algo=per))
        if first_inexact is None and not all(p["count_exact"] for p in per):
            first_inexact = K
    T = dict(info={k: v for k, v in info.items() if k != "runs"}, rows=res, first_inexact_K=first_inexact,
             note="row 0 / column 0 all ones: the (0,0) count is K; others about K/4.  C enters in cuBLAS's fp32 "
                  "epilogue, so this is the bias read at m = 24 at API level; the in-accumulator read is wgmma/HMMA")
    log("ltparity", ty, "fast", fast, "first inexact K", first_inexact,
        [(r["K"], [p["count_exact"] for p in r.get("per_algo", [])]) for r in res])
    return T


def towermul(binp, ty, shrink):
    vec = HERE / "t256_vectors.jsonl"
    rows = [json.loads(l) for l in vec.read_text().splitlines()]
    T = {}
    for ri, d in enumerate(rows):
        cols = [int.from_bytes(bytes.fromhex(c), "little") for c in d["cols"]]
        mr = np.zeros((256, 256), dtype=np.uint8)                       # row c, column j: bit c of r * e_j
        for j in range(256):
            for c in range(256):
                mr[c, j] = cols[j] >> c & 1
        xs = b"".join(bytes.fromhex(x) for x, _ in d["tests"])
        fm, fx, fo = WORK / f"t256_mr{ri}.bin", WORK / f"t256_x{ri}.bin", WORK / f"t256_out_{ty}_{ri}.bin"
        mr.tofile(fm); fx.write_bytes(xs)
        lg = (18 if shrink else 20) if ri == 0 else 8
        info = run_json([binp, "towermul", ty, lg, fm, fx, fo, 10 if ri == 0 else 1], timeout=900)[0]
        if info.get("status") != "SUCCESS":
            T[f"r{ri}"] = info
            continue
        out = fo.read_bytes()
        xin = pathlib.Path(str(fo) + ".x").read_bytes()
        bad_rust = sum(out[32 * i:32 * i + 32] != bytes.fromhex(y) for i, (_, y) in enumerate(d["tests"]))
        bad_lin = 0
        for i in range(len(d["tests"]), len(out) // 32):
            xi = int.from_bytes(xin[32 * i:32 * i + 32], "little")
            acc = 0
            for j in range(256):
                if xi >> j & 1:
                    acc ^= cols[j]
            bad_lin += out[32 * i:32 * i + 32] != acc.to_bytes(32, "little")
        info.update(rust_tests=len(d["tests"]), rust_mismatches=bad_rust, random_rows=len(out) // 32 - len(d["tests"]),
                    xor_of_rust_columns_mismatches=bad_lin)
        check(f"towermul.{ty}.r{ri}", bad_rust == 0 and bad_lin == 0, rust_mismatches=bad_rust,
              xor_mismatches=bad_lin, rows=len(out) // 32)
        T[f"r{ri}"] = info
        if ri == 0:
            log("towermul", ty, f"{info['ns_per_element']:.3f} ns/element (gemm {info['gemm_ns_per_element']:.3f})")
    return T


def wgmma_bias(binp, m_measured):
    """in-accumulator bias read on QGMMA (e4m3) and HGMMA (f16): C = 2^(m-1) + c0, 0/1 patterns, count n"""
    T = {}
    rng = np.random.default_rng(90)
    for kind, per, one_enc, val in (("e4m3", 16, lambda b: e4m3_bits(1) if b else 0, e4m3_val),
                                    ("f16", 8, lambda b: f16_bits(1) if b else 0, f16_val)):
        ms = sorted({m for m in (m_measured.get(kind), 12, 13, 14, 24) if m and 2 <= m <= 24})
        rows = []
        for m in ms:
            for c0 in (0, 1):
                pa = rng.integers(0, 2, size=per); pb = rng.integers(0, 2, size=per)
                pa[0] = pb[0] = 1
                kk = 32 if kind == "e4m3" else 16
                n_inst = sum(int(pa[k % per] * pb[k % per]) for k in range(kk))
                for f in (0.5, 0.95, 1.2):
                    iters = max(1, int(f * (1 << (m - 1)) / n_inst))
                    if kind == "e4m3":
                        A = [one_enc(int(b)) for b in pa]; B = [one_enc(int(b)) for b in pb]
                    else:
                        A = list(struct.pack("<8H", *[one_enc(int(b)) for b in pa]))
                        B = list(struct.pack("<8H", *[one_enc(int(b)) for b in pb]))
                    of = WORK / f"wgbias_{kind}.bin"
                    run_json([binp, "wgcheck", kind, iters, bytes(A).hex(), bytes(B).hex(),
                                     f"{f32_bits(Fraction(2) ** (m - 1) + c0):08x}", of], timeout=600)[0]
                    d = np.fromfile(of, dtype=np.uint32)
                    n = iters * n_inst + c0
                    raw = int(d[0])
                    inside = n < (1 << (m - 1))
                    exact = bool((d == d[0]).all()) and f32_val(raw) == (1 << (m - 1)) + n
                    read = ((raw >> (24 - m)) & 1) == (n & 1)
                    rows.append(dict(m=m, c0=c0, iters=iters, count=n, inside=inside, exact=exact, read=read))
                    if inside:
                        check(f"wgmma.bias.{kind}.m{m}.c{c0}.n{n}", exact and read, raw=hex(raw))
        T[kind] = rows
    return T


# ---------------------------------------------------------------------------------------------- main
def main():
    global OUT, WORK
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True)
    ap.add_argument("--shrink", action="store_true", help="the Kaggle T4 dry-run sizes")
    ap.add_argument("--sku", default="")
    ap.add_argument("--arches", default="sm_75,sm_80,sm_89,sm_90a,sm_100a,sm_103a")
    ap.add_argument("--skip", nargs="*", default=[])
    ap.add_argument("--sections", default="all", help="comma list of sections to run (default all)")
    ap.add_argument("--gpu", type=int, default=0)
    a = ap.parse_args()
    OUT = pathlib.Path(a.out).resolve()
    OUT.mkdir(parents=True, exist_ok=True)
    WORK = OUT / "work"
    WORK.mkdir(exist_ok=True)
    os.environ.setdefault("CUDA_VISIBLE_DEVICES", str(a.gpu))
    nvcc = find_nvcc()
    smi = sh("nvidia-smi --query-gpu=name,compute_cap,memory.total,clocks.max.sm,clocks.max.mem,driver_version,power.limit "
             "--format=csv,noheader")[1].strip().splitlines()
    name, cc = [x.strip() for x in smi[a.gpu].split(",")][:2]
    arch, ccv = arch_of(cc)
    R.update(sku_requested=a.sku, shrink=a.shrink, gpu=smi, device=name, cc=cc, arch=arch, nvcc=nvcc, nvcc_flags=NVCC_PRE,
             nvcc_version=sh([nvcc, "--version"])[1].strip().splitlines()[-2:], numpy=np.__version__,
             python=sys.version.split()[0])
    log("device", name, cc, arch, "nvcc", nvcc)
    save()
    shrink = a.shrink

    want = None if a.sections == "all" else set(a.sections.split(","))
    R["sections_requested"] = a.sections

    def section(nm, f):
        if nm in a.skip or (want is not None and nm not in want):
            return
        log("== section", nm)
        t0 = time.time()
        try:
            R[nm] = f()
        except Exception as e:
            import traceback
            R["errors"][nm] = traceback.format_exc()[-2000:]
            log("SECTION ERROR", nm, str(e)[-400:])
        R.setdefault("section_s", {})[nm] = round(time.time() - t0, 1)
        save()

    section("compile", lambda: compile_check(nvcc, a.arches.split(",")))
    for k, e in (R.get("compile") or {}).items():
        src_k, arch_k = k.split("@") if "@" in k else (k, "")
        # sm_80+ sources cannot build for sm_75 (m16n8k32 u8, e4m3): expected, as in every T4 run
        expected = src_k in ("probe.cu", "probe_fixed.cu", "probe_sm80.cu") and arch_k in ("sm_75",)
        if isinstance(e, dict) and e.get("rc") not in (0, None) and not expected:
            R["errors"][f"compile {k}"] = e.get("log", "")[-800:]

    # builds for this device
    bins = {}
    lf = link_flags(nvcc)
    for src, extra in (("probe_sm75.cu", []), ("probe_sm80.cu", []), ("gemm_probe.cu", lf), ("probe.cu", []),
                       ("probe_fixed.cu", []), ("rotation_probe.cu", [])):
        if src in ("probe_sm80.cu", "probe.cu", "probe_fixed.cu") and ccv < 80 or not (HERE / src).exists():
            continue
        # dropped on sm_90+: probe.cu's e4m3 mma.sync C path runs on HMMA there (ROTATION section 5)
        if src in ("probe.cu", "probe_fixed.cu") and ccv >= 90:
            continue
        exe = WORK / src[:-3]
        rc, so, se, dt = sh([nvcc] + NVCC_PRE + ["-O2", f"-arch={arch}", "-o", str(exe), str(HERE / src)] + extra, timeout=900)
        R.setdefault("build", {})[src] = dict(rc=rc, s=round(dt, 1), log=(so + se).strip()[-800:], flags=extra)
        log("build", src, arch, "rc", rc, f"{dt:.1f}s")
        if rc == 0:
            bins[src] = exe
        else:
            # fail closed: a source that applies to this device and does not build is an error, not a skipped
            # section (accel-failclosed-t4-01: a truncated probe_sm75.cu ran 45 checks, 0 failed, rc 0)
            R["errors"][f"build {src}"] = (so + se).strip()[-1500:]
    save()

    if "probe_sm75.cu" in bins:
        b = bins["probe_sm75.cu"]
        section("turing", lambda: turing(b, shrink))
        section("rates_sm75", lambda: rates(b, ["u8", "u4", "b1", "f16", "lop3"], 200 if shrink else 300))
    if "probe_sm80.cu" in bins:
        b = bins["probe_sm80.cu"]
        section("ampere", lambda: ampere(b, ccv))
        section("rates_sm80", lambda: rates(b, ["s8", "f16"] + (["b1and", "b1xor"] if ccv < 100 else []) +
                                            (["e4m3"] if ccv == 89 else []) +
                                            (["wg_s8", "wg_b1", "wg_e4m3", "wg_f16"] if arch == "sm_90a" else []), 300))
        if arch == "sm_90a":
            section("wgmma", lambda: wgmma(b))

            def wgb():
                wg = R.get("wgmma", {})
                mm = {k: min(24, wg[f"{k}_cint_jmax"]["m"]) for k in ("e4m3", "f16")
                      if wg.get(f"{k}_cint_jmax", {}).get("m")}
                return wgmma_bias(b, mm)
            section("wgmma_bias", wgb)
    if "probe.cu" in bins:
        def orig():
            rc, so, se, dt = sh([str(bins["probe.cu"])], timeout=300)
            return dict(rc=rc, stdout=so[-4000:], stderr=se[-1000:], s=round(dt, 2),
                        note="the hardware lane's probe as written; on sm_90a/sm_100a its e4m3 kernel runs on FP16 HMMA")
        section("probe_cu_as_written", orig)
    if "probe_fixed.cu" in bins:
        def fixed():
            rc, so, se, dt = sh([str(bins["probe_fixed.cu"])], timeout=300)
            rows = re.findall(r"e4m3 C-acc (integer|fractional)\s+j=\s*(\d+)\s+D=(\S+)\s+exact=(\S+)", so)
            out = dict(rc=rc, stdout=so[-6000:], stderr=se[-1000:], s=round(dt, 2))
            for kind in ("integer", "fractional"):
                r = [(int(j), ok == "yes") for k, j, _, ok in rows if k == kind]
                if r:
                    out[kind] = width_from_gaps(r)
            return out
        section("probe_fixed", fixed)
    if "gemm_probe.cu" in bins:
        g = bins["gemm_probe.cu"]
        dump = WORK / "dump"
        dump.mkdir(exist_ok=True)
        free = int(sh("nvidia-smi --query-gpu=memory.free --format=csv,noheader,nounits")[1].split()[a.gpu]) << 20

        def copy():
            nb = min(2 << 30, free // 3) if shrink else min(8 << 30, free // 3)
            return run_json([g, "copy", nb, 10 if shrink else 20], timeout=600)[0]
        section("copy", copy)

        def peak():
            out = {}
            for ty in ["i8", "f16"] + (["e4m3"] if ccv >= 89 else []) + (["e2m1", "mxf4"] if ccv >= 100 else []):
                try:
                    out[ty] = run_json([g, "peak", ty, 8192, 10 if shrink else 20], timeout=600)[0]
                    log("peak", ty, out[ty].get("status"), f"{out[ty].get('macs_per_s', 0):.3e} MAC/s")
                except Exception as e:
                    out[ty] = {"error": str(e)[-400:]}
            return out
        section("peak", peak)

        def i8ip():
            out = []
            logs = (20, 22, 24, 25) if shrink else (20, 21, 22, 23, 24, 25)
            for layout in ("elem", "plane"):
                for lg in logs:
                    if 2 * 32 << lg > free * 0.8:
                        continue
                    r = run_json([g, "i8ip", lg, layout, 10, dump], timeout=900)[0]
                    if r.get("status") == "SUCCESS":
                        bad = i8ip_check(dump, layout, lg, r["dumped_batches"])
                        r["host_sample_mismatches"] = bad
                        check(f"i8ip.{layout}.2^{lg}", bad == 0 and r["simt_ref_mismatches"] == 0,
                              host_sample_mismatches=bad, simt_ref_mismatches=r["simt_ref_mismatches"])
                        bw = R.get("copy", {}).get("copy_rw_bytes_per_s")
                        if bw:
                            r["ratio_to_copy_bw"] = r["bytes_per_s"] / bw
                    out.append(r)
                    log("i8ip", layout, lg, r.get("status"), r.get("ms"), "ms", f"{r.get('bytes_per_s', 0):.3e} B/s")
            return out
        section("i8ip", i8ip)

        def ltip():
            out = []
            types = ["f16"] + (["e4m3"] if ccv >= 89 else [])
            for ty in types:
                logs = (20, 22, 24) if shrink else ((20, 22, 24) if ty == "f16" else (20, 22, 24, 25))
                for fast in ((0, 1) if ty == "e4m3" else (0,)):
                    for lg in logs:
                        w = 2 if ty == "f16" else 1
                        if 2 * 64 * w << lg > free * 0.8:
                            continue
                        r = run_json([g, "ltip", ty, lg, 64, 10, fast, dump], timeout=900)[0]
                        if r.get("status") == "SUCCESS":
                            bad, mx = ltip_check(dump, ty, lg, 64, r["dumped_batches"])
                            r["host_sample_mismatches"], r["max_sum"] = bad, mx
                            check(f"ltip.{ty}.fast{fast}.2^{lg}", bad == 0, host_sample_mismatches=bad, max_sum=mx,
                                  note="sums of 32,768 products of base-16 digits stay below 2^23")
                        out.append(r)
                        log("ltip", ty, fast, lg, r.get("status"), r.get("ms"), "ms")
            return out
        section("ltip", ltip)

        def ltp():
            out = {}
            specs = [("f16", 0)] + ([("e4m3", 0), ("e4m3", 1)] if ccv >= 89 else []) + ([("e2m1", 0)] if ccv >= 100 else [])
            for ty, fast in specs:
                try:
                    out[f"{ty}_fast{fast}"] = ltprobe(g, ty, fast)
                except Exception as e:
                    out[f"{ty}_fast{fast}"] = {"error": str(e)[-600:]}
                    log("ltprobe", ty, fast, "ERROR", str(e)[-300:])
            return out
        section("ltprobe", ltp)

        def ltpar():
            out = {}
            specs = [("f16", 0)] + ([("e4m3", 0), ("e4m3", 1)] if ccv >= 89 else []) + \
                    ([("mxf4", 0), ("e2m1", 0)] if ccv >= 100 else [])
            for ty, fast in specs:
                try:
                    out[f"{ty}_fast{fast}"] = ltparity(g, ty, fast, shrink)
                except Exception as e:
                    out[f"{ty}_fast{fast}"] = {"error": str(e)[-600:]}
                    log("ltparity", ty, fast, "ERROR", str(e)[-300:])
            return out
        section("ltparity", ltpar)

        def tmul():
            out = {}
            for ty in ["f16", "i8"] + (["e4m3"] if ccv >= 89 else []) + (["mxf4", "e2m1"] if ccv >= 100 else []):
                try:
                    out[ty] = towermul(g, ty, shrink)
                except Exception as e:
                    out[ty] = {"error": str(e)[-600:]}
                    log("towermul", ty, "ERROR", str(e)[-300:])
            return out
        section("towermul", tmul)
    if "rotation_probe.cu" in bins:
        rp = bins["rotation_probe.cu"]
        section("f16parity", lambda: f16_parity(rp))
        freem = int(sh("nvidia-smi --query-gpu=memory.free --format=csv,noheader,nounits")[1].split()[a.gpu]) << 20
        section("f16ip", lambda: f16ip(rp, shrink, freem, R.get("copy", {}).get("copy_rw_bytes_per_s")))

    # summary ratios
    s = {}
    r75 = R.get("rates_sm75", {})
    if "u8" in r75 and "macs_per_s" in r75.get("u8", {}):
        base = r75["u8"]["macs_per_s"]
        s["sm75_rate_ratio_to_u8"] = {k: v["macs_per_s"] / base for k, v in r75.items() if "macs_per_s" in v}
    r80 = R.get("rates_sm80", {})
    if "s8" in r80 and "macs_per_s" in r80.get("s8", {}):
        base = r80["s8"]["macs_per_s"]
        s["sm80_rate_ratio_to_s8"] = {k: v["macs_per_s"] / base for k, v in r80.items() if "macs_per_s" in v}
    R["summary"] = s
    save()
    log("DONE checks", R["n_checks"], "failed", R["n_failed"], "errors", list(R["errors"]))
    return 0 if R["all_checks_pass"] and not R["errors"] else 1


if __name__ == "__main__":
    sys.exit(main())
