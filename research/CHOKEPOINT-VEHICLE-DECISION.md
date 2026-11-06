# Choke-point vehicle decision (2026-11-05)

Question (ember, through the coordinator): before grinding on the Pancake orb, is it the right
vehicle for a 2026 in-path receipt device, or legacy code we should rethink?

What we want is not a particular vehicle. It is the **choke-point step function and its
`commit_before_forward` theorem** (lattice X-6; alignment map (b)+(g); critique W10): every
forwarded frame has an earlier durable commit binding its id, its payload digest and the manifest
root at admission. That theorem has to live somewhere the device actually runs, and has to cost
little to keep true as the device changes.

Tags: VERIFIED (inspected or run today), SOURCED, INFERRED, PROPOSED.

## Decision

**Option (2): the step function is defined and proved in Lean, and the deployed object is that
same Lean definition, compiled by Lean's compiler and hosted in a seL4 Microkit protection
domain.** It is also `@[export]`able into the host dataplane on the `dregg_deleg_admit` pattern,
for the non-seL4 deployment.

The Pancake orb is retired as the vehicle for this, and I suspended the lowering I had started.
Keep the orb's *method* as the lesson (emit, lower, prove, byte-pin). Drop the vehicle.

The first step of the winner is done and checks: `experiments/chokepoint-step/CommitStep.lean`
(below).

## The five options, priced

| | (1) Pancake orb | (2) Lean step on Microkit PD | (3) Rust + Verus/Aeneas | (4) eBPF/XDP or P4/DPU | (5) FPGA via obend |
|---|---|---|---|---|---|
| **Theorem carried** | about the *lowered program* under a hand transcription of CakeML Pancake semantics (assumption A1, `orb-compiler/Pancake/Sem.lean`); no link to CakeML's HOL4 compiler theorem | about the Lean definition itself: no transcription, no second model | Verus: an SMT-checked spec in Verus, a third logic beside Lean and HOL4. Aeneas: a Lean translation of the Rust, provable next to the MMR theorems | today none: the kernel verifier proves memory safety and termination, not function; P4 has Coq semantics (Petr4), and there is no Lean semantics of eBPF here | today combinational only (critique W1); needs sequential semantics first |
| **What else must be trusted** | the transcription A1; the FFI drivers. `health_ffi.c` already writes into the *conf* buffer (`memcpy(c, …)`), which the model says cannot happen [VERIFIED]. Sem also omits CakeML's FFI length check. Blake3 and the MMR cannot be written in Pancake reasonably, so they sit in C FFI anyway | the Lean compiler and runtime (unverified, but already the TCB of every `@[export]`ed deployed decision), musl and clang, the Microkit tool and monitor (unverified), the seL4 kernel (verified) | rustc; Charon/Aeneas translation (or Verus's own encoding) | the kernel, the loader, NIC firmware or DPU vendor firmware | synthesis/P&R tools, bitstream integrity, the vendor's FPGA silicon |
| **Who can bypass the choke point** | anyone with root on the Linux host: it is one userspace path among many, gated behind `cfg(drorb_cake_serve)` and `DRORB_CAKE_SERVE=1` [VERIFIED] | **only what the system description gives a device capability.** In Microkit the NIC's MMIO/IRQ capabilities go to exactly the PDs listed in the `.system` file, so "only the commit PD holds the NIC" is a static, checkable fact about the image. That discharges most of the coverage premise into configuration [INFERRED from the Microkit model; not yet stated as a theorem]. Residual: other devices (GPU DMA, management NIC), physical paths | root on the host, as for (1) | root can detach an XDP program; a DPU is a separate machine whose firmware the operator or vendor controls. This is Lucid's FlexNIC setting, and it is only as strong as DPU attestation | physical access and bitstream swap; otherwise the strongest |
| **Reprove cost per change** | Lean per-file rebuild is fast (Sem+Lower 1-3 s on the Mac [MEASURED]), but every change also needs `cake` on hbox (x86-64 only) plus FFI-driver re-audit, and the proof is per-lowered-program | **1.2 s to re-check all theorems and falsifiers in `CommitStep.lean` on the Mac** [MEASURED]; `lean -c` emits C (3,153 lines, 0 errors) [MEASURED] | Verus: seconds to minutes per function. Aeneas: retranslate, then Lean proofs break where the Rust changes shape | per program, with no theorem target yet | months per change until a sequential elaborator exists |
| **Runs on** | x86-64 Linux, as one userspace path | aarch64/riscv64 boards or VMs (the executor PD boots under qemu-system-aarch64, `sel4/dregg-pd/executor-microkit-pd/WALL.md`, 2026-06-15 [VERIFIED by reading the evidence log reference]); the same object `@[export]`ed into any host | anywhere Rust runs, at line-rate-class speed | NICs and DPUs at line rate | FPGA boards, at line rate |
| **Who else can maintain it** | us only. HOL4/CakeML expertise, plus a bespoke Lean transcription; parked since July (whatember strand 15, C7: "the orb never got cut; Bread then lost to Mini") | anyone who writes Lean, which is the whole programme | Rust people, plus a Verus or Aeneas specialist | network-dataplane engineers | hardware engineers plus obend's owner |
| **Weeks to a working device** | 1-2 for the step, and it would still be off-path and x86-only | step and theorems: done today. `@[export]` + PD hosting: 1-2 (reuse `executor-microkit-pd`). Networking (M3, the wall `sel4/README.md` names): 2-4 | 2-4 | 4-8, and no theorem target | price only: quarters |

Tools present on this Mac [VERIFIED]: Verus (`~/tools/verus/verus-arm64-macos`), Microkit SDK
2.2.0 (`~/sel4-sdk`), qemu-system-aarch64, Lean 4.30-4.33 toolchains. Not present: Aeneas,
Charon, Kani. The `cake` binary is on hbox at `/home/hbox/r05/cake-x64-64/cake`. It compiled a
10-line probe `commit_frame` with two FFI calls to an 11,595-byte `.S` (rc=0) [MEASURED, probe
files since removed from hbox `/tmp`].

## Why (2), and the trade it makes

- **It removes the transcription.** The Pancake proof is about a Lean copy of a HOL4 semantics,
  and the verified CakeML compiler's guarantee is about the HOL4 original. Nothing mechanised
  joins them, so (1)'s "verified compilation" does not reach our theorem. In (2) the theorem's
  subject *is* the deployed definition. The unverified link is the Lean compiler, which is
  already the TCB of every Lean-decided deployed path (`dregg_deleg_admit`, the executor PD), so
  this adds no *new* trusted component.
- **It is the only option whose choke point is a static fact.** Under seL4/Microkit, device
  capabilities are distributed at image build. "The commit PD is the only holder of the NIC
  capability" can be checked against the `.system` file and, with seL4's proofs and capDL, holds
  at runtime. Options (1), (3) and (4) put the choke point in a host the operator roots.
- **One language for the device theorem and the log theorems.** The MMR non-omission statements
  are Lean (`breadstuffs/metatheory/Dregg2/Lightclient/MMR.lean`). The step function's durable log
  is a `List Leaf`, which is exactly the list they quantify over.
- **Reprove cost is seconds.** It also matches the minidregg law: the derived path is the only
  path; nothing lands beside.

The trade, stated plainly:
- **No verified compilation.** We give it up. The sharp mitigations are a later
  translation-validation step (alignment (d)/(j)-3) and the fact that the step function is small
  and loop-free.
- **No line rate.** A Lean-compiled step in a PD is a control-plane device: job admissions,
  manifest changes, per-request receipts, agent tool calls, at roughly thousands to hundreds of
  thousands of events per second [INFERRED, not measured]. It is not a 400 Gb/s packet tap.
- **The line-rate path is (4) or (5) later, with this Lean definition as the spec** that a DPU
  program or RTL is translation-validated against. So (2) is not a dead end for line rate. It
  is the specification every later vehicle refines.

## First step of the winner (done): `experiments/chokepoint-step/CommitStep.lean`

The file is Lean 4 core only, with no imports; it checks with a bare `lean` in 1.2 s.

**Definitions.**
- `step : (H : List UInt8 → D) → DevState D → Event → DevState D × List (Output D)`.
- Events: `arrive job payload`, `setVjm root`, `restore len ctr`.
- Outputs: `persist leaf`, `forward occ payload`, `refuse occ`.
- `Leaf = {occ, epoch, vjm, job, digest}`.
- The state carries a non-rollbackable hardware counter `hw` (TrInc-style), the durable `log`,
  a capacity and a `halted` flag.

**Theorems.** All hold for every hash `H`, every initial state and every event sequence. The
axioms are at most `propext` and `Quot.sound`; `concrete_run` and `rollback_halts` use none.
- `commit_before_forward`: every `forward occ p` in the output stream is *immediately preceded*
  by `persist l` with `l.occ = occ` and `l.digest = H p`. This is the per-occurrence form Codex
  round 2 asked for.
- `persisted_in_log`: every persisted leaf is in the final durable log.
- `forwarded_ids_nodup`: forwarded ids are pairwise distinct and lie in `[hw₀, hw_final)`,
  across restores.
- `forwards_when_capacity`: the liveness companion. A live device with room forwards, and the
  leaf carries the epoch and manifest root *at admission*.
- `overflow_refuses`: a full device refuses, persists nothing, forwards nothing, and still burns
  an id.
- `rollback_halts` and `halted_never_forwards`: a snapshot whose counter or length differs from
  the hardware-anchored state halts the device for good.

**Vacuity falsifiers.**
- `concrete_run` (by `decide`): a concrete run forwards, refuses at capacity, and halts on a
  stale restore.
- `mutant_violates`: a forward-then-persist mutant's concrete output violates
  `CommitBeforeForward`.
- Together they show the safety property is neither trivially true nor satisfied only by a
  device that always refuses.

**Named residuals.** These are not hidden.
- **Leaf encoding.** `Leaf` is a structure, so binding is structural. A byte encoding `enc : Leaf
  → bytes`, with an injectivity theorem and a collision reduction for `H`, is next.
- **Snapshot content integrity.** It is a MAC assumption. The counter catches rollback; it does
  not catch a forged log of the right length.
- **Delivery.** It is not claimed: this is forwarding coverage only.
- **Bypass paths** belong in the receipt's `assumes`.

**Next steps, in order.**
1. Move the file into its canonical home. I propose a statement-first keystone in minidregg (the
   kernel that won), with its owner's agreement. degg-research holds it as an experiment until
   then.
2. Add a concrete `D := ByteArray` (Blake3 via the existing extern pattern), plus
   `@[export dregg_commit_step]`.
3. Host it in a Microkit PD next to the existing persist PD (`sel4/dregg.system` already has a
   persist seat).
4. Write the `.system` capability statement: only this PD holds the NIC.

## What this changes in the other notes

- Alignment map (b): the vehicle is now (2). The Pancake `commit_before_forward` sketch is
  superseded by the theorem in `CommitStep.lean`.
- Critique W3: the orb stays a correct, demo-gated two-branch function. It is no longer on the
  governance path.
- Unchanged, and not legacy: NVIDIA GPU evidence and the TD 1.5 fields in tee-verify, the two
  residual_trust arms, the silicon-theorem repair proposals.
