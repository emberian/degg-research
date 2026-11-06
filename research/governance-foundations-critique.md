# Governance foundations: refute, then repair (2026-11-05)

Status: research note. Claim tags follow `AGENTS.md`: VERIFIED (inspected source or reproduced
measurement), SOURCED (cited, not reproduced), INFERRED, PROPOSED. Companion notes:
`research/GOVERNANCE-CLAIM-LATTICE.md`, `research/governance-codex-synthesis.md`,
`redregg/designs/SILICON-TO-SEMANTICS-ALIGNMENT-2026-11-05.md`.

Question: if we try to carry technical governance of training and inference provision on the
foundations we have built, which foundations hold the weight and which are diagrams? Every item
gives the strongest refutation I could support from the source, then the repair, or the
experiment that settles it.

The headline is not comfortable. **Three of the foundations our pitch leans on hardest are
narrower than the prose about them.** These are the "silicon kernel", the "verified in-path
loop" and the "confidential GPU" verifier. A fourth, the policy-guard weld, says so in its own
header. What does hold is the commitment and non-omission layer (MMR, attested query), the
Lean-decided admission export, the audit-sampling theorem as mathematics, and the
TDX/SNP/Nitro quote verifiers as quote verifiers.

---

## W1. The "silicon Lean kernel" checks a tag computed on the host

**Refutation (VERIFIED).** `obend/lean/Obend/Lower/Proofs/SiliconKernel.lean:270`,
`silicon_kernel_5stage_check_sound`, is a closed statement about five fixed declarations
(`validIdNatDecl`, `invalidMismatchDecl`, `invalidBadAppDecl`, `invalidUniverseDecl`,
`invalidSorryDecl`), discharged by `rfl` and `decide +kernel`. The hardware function it lowers
is `kernelGateBody` (line 221): `boolRec ... (natBin .beq (bvar 0) (lit 1))`, an equality
comparator on its input. At the declaration level, the checking work (`encodeDeclStatusTag`, line 171:
`exprContainsSorry`, `encodeKExprNode`, `stepCamInferNodeWord`, `stepCamDefEqNodeWord`) runs in
host Lean and produces the tag that the gate then compares with 1. `real_silicon_kernel_node_check_sound`
(line 696) is stronger than the gate. It runs lowered four-port RTL modules
(`evalLoweredNodeInferRtl`, `evalLoweredNodeCheckRtl`) that do node-level inference and checking in
the synthesized datapath. It does this on about 15 hand-picked node fixtures, and the node
*encoding* (`encodeKExprNode`) still runs on the host (Codex corrected my first draft here; see
`governance-codex-synthesis.md`). Neither statement quantifies over declarations or nodes.

What the 5-stage chain does establish is real and worth keeping: one `LExpr` is carried, with
proofs, through `SSteps`, `Core4.Evaluates`, `Leaf.Book.Halts`, an `RtlModule` and an automaton
network. The chain is right. The object carried through it is trivial.

**Why it matters for governance.** "A hardware kernel checks the declaration" is the claim that
would let a gate-level device refuse a non-conforming artifact without trusting a host. Today, at the
declaration level, the host decides and the device repeats the answer. At the node level the
device computes, but only on fixtures, and on an encoding the host supplied.

**Repair.** (a) Rename the theorem in prose to what it is: "5-stage lowering soundness, exhibited
on a comparator". (b) The construction-grade statement is universal: for all `decl` in a stated
fragment, `evalModuleOutputs (compileBookToRtlSat ... checkerBook) (encode decl) = [(out, ofBool
(kernelCheckDecl env fuel decl))]`, where `checkerBook` lowers `stepCam*NodeWord` itself and not
its result. (c) Settling experiment: lower `stepCamInferNodeWord` alone (it is already a pure
`Nat` function of four words, line 137) through `compileBookToRtlSat`, then prove `∀` agreement
with `bv_decide` at 16-bit width. If that fails to scale, the silicon-kernel story is a research
direction, not a foundation.

## W2. The Ozaki FP4 tensor-core theorems are about integers, not tensor cores

**Refutation (VERIFIED).** `obend/lean/Obend/Hardware/Accelerators.lean`:
`ozaki_base13_klimb_dot_exact`, `amd_gpu_mfma_dot4_exact` and
`systolic_tpu_eq_dsp58_cascade_eq_kulisch_dot` are ring identities over unbounded `Int`. `vMfmaDot4`
is modelled as `acc + Σ a_k b_k` in `Int` (line ~400). There is no accumulator width, no FP32
significand, no saturation, no alignment or truncation behaviour, no MX block scale, and no
statement that the limbs are FP4-representable at the point of the product. The headline in the
module docstring ("zero intermediate rounding error") is true of `Int`, and true of hardware only
under the unstated hypothesis that every partial sum is exactly representable in the
accumulator.

That hypothesis is the whole question. NVIDIA tensor-core accumulation is not IEEE: Fasi, Higham,
Mikaitis and Pranesh (PeerJ CS 2021, "Numerical behavior of NVIDIA tensor cores") measured
truncation and missing alignment bits on Volta through Ampere (SOURCED). DeepSeek-V3's report
(2024) states that H800 FP8 GEMM accumulation keeps about 14 bits (SOURCED).

**Measurement (VERIFIED, counts only).** `experiments/mma-field-mapping/mma_limbs.py` →
`RESULTS.txt`. The model reproduces TensorZKP's published INT8 exactness bound (N < 33,025 for
UINT8 limbs into INT32) exactly, which validates it. The results for a BabyBear multiply:

- **Products per multiply.** FP4-E2M1 base-13 balanced limbs need 9 limbs, so 81 limb
  products. INT8 limbs need 16.
- **Break-even.** FP4 beats INT8 only on a SKU where TOPS(FP4)/TOPS(INT8) > 5.06.
- **Exact block length.** For an FP4 limb pair this is 262,143 under a 24-bit accumulator and 255
  under a 14-bit one. A 14-bit accumulator still makes the mapping close to useless (corrected 2026-11-06 after the Codex Astra statement audit: the encoder's largest digit is 8, not 12, so max product 64 rather than 144; the limb count 9 and break-even 5.06 are unchanged).

**Repair.** State the hardware theorem with the accumulator as a parameter:
`AccModel m` (an `m`-bit significand, truncating alignment), and
`ozaki_exact_under_acc (h : N * maxprod < 2^m) : hwDot (AccModel m) xs ys = kulischDot xs ys`.
Then *measure* `m` per SKU with a Fasi-style probe. The proof is about `AccModel`, and the
probe's output is the premise. That turns the FP4 work into the one place in the literature
where tensor-core proof arithmetic carries a machine-checked exactness side condition.
TensorZKP has a pen-and-paper bound; FlashProve does not use tensor cores for field arithmetic as
far as its README says.

## W3. The verified Orb loop is a two-branch responder, and it is off by default

**Refutation (VERIFIED).**

What is proved:
- `breadstuffs/orb-compiler/Pancake/ServeExportServes.lean` proves the emitted `serve` PFun
  correct for two branches. The decoder (`decodeModel`) recognises the prefixes `G`, `H`, `O` and
  `PO`, and routes `method < 4` to the 200 response.
- The two branch theorems are exhibited on witness bytes: `G` lands `serialize resp200`
  (164 bytes), and `X` lands `serialize resp405` (227 bytes). My first draft said "first byte `G`,
  else 405", and Codex corrected it.
- The proof is against `Pancake/Sem.lean`, a Lean transcription of CakeML's Pancake semantics. Its
  faithfulness is named assumption A1, transcribed by hand from CakeML `ed31510b3`.
- The x86 bytes then rest on CakeML's own compiler correctness, which is a different proof
  assistant with no mechanised link to our A1.

The good news:
- The proved `.pnk` is byte-identical to the shipped one
  (`orb-compiler/Pancake/serve_slice_export.pnk` vs `orb/crates/dataplane/ffi/cake/serve.pnk`,
  9,857 bytes, VERIFIED with a whitespace-normalised diff today).
- So at the `.pnk` level there is no twin.

The bad news:
- The dataplane routes to it only under `cfg(drorb_cake_serve)`, and at runtime only if
  `DRORB_CAKE_SERVE=1` (`orb/crates/dataplane/src/cake_serve.rs:153`, `blocking.rs:421-437`).
  Every other build and run uses the leanc-compiled Lean serve.
- The leanc path's trust base is the Lean compiler, the Rust runtime and C.
- `serve.S` and `serve.pnk` were last committed together in a sweep commit (`4bcb934a3`,
  2026-07-27). Nothing in the tree regenerates `serve.S` from `serve.pnk` in CI.
- The module doc in `cake_serve.rs` describes "redirect+HSTS". The proved slice serves
  `resp200`/`resp405`. Either the doc or the `.pnk` is stale (INFERRED).

**Why it matters.** Design D2 in the alignment map (a verified in-path receipt device) wants a
loop that *every forwarded frame crosses*, with a theorem about each frame. What we have is a correct
two-branch function and a pipeline pattern (emit → lower → prove → byte-pin) that is the right
pattern.

**Repair.** (a) A CI step that runs `cake --pancake` on the committed `.pnk` and byte-compares
with `serve.S`. (b) The next proved function should be the receipt step itself:
`commitFrame : header bytes → MMR state → MMR state`, with the theorem that the MMR leaf at the
new position is `H(header)`. That is smaller than an HTTP parser and is exactly the function a
receipt device needs. (c) Do not cite "312-module verified proxy" for the served path. Cite
"proved emitted function, byte-pinned, demo-gated".

## W4. "Confidential GPU" verification is CPU-only, and TD 1.5 fields are dropped

**Refutation (VERIFIED).**

GPU evidence is never checked:
- `dregg-chutes-e2ee/src/attest.rs:11` documents the evidence shape `{quote, gpu_evidence:[...],
  instance_id, certificate}`, and no code reads `gpu_evidence`.
- `tee-verify` has no NVIDIA support: no SPDM measurement parsing, no RIM comparison, no device
  certificate chain or OCSP check.
- `tdx.rs:627-640` carries `expected_gpus` and `gpu_count` from the Chutes registry JSON. That is
  the operator's self-description, not evidence.

TD 1.5 fields are dropped:
- `tdx.rs:432-436` takes `as_td10()`, the TD10 base even of a TD15 report (dcap-qvl 0.5.2 does
  parse v5/TD15; `quote.rs:466,536`).
- So `MR_SERVICETD` and `TEE_TCB_SVN_2` are discarded.
- `fold_tdx_measurement` folds MRTD and RTMR0-2 but not RTMR3 (an application measurement
  register, so this is defensible if documented).

Taken together:
- A TD with a bound service TD (for example a migration TD) passes as if it had none (INFERRED
  consequence; I have not built a quote with one).
- What we verify is that a CPU TD with the pinned image produced the ML-KEM key. Whether the
  model ran on a CC-mode GPU, under which firmware, with NVLink encryption on or off, is trusted
  to the image.

What is sound (VERIFIED, consistent with `breadstuffs/docs/TEE-VERIFY-AUDIT-2026-07-25.md` and
later commits):
- Pinned roots and pinned algorithms.
- Fail-closed defaults.
- The SNP TCB floor fix: `SnpPolicy::new(min_tcb, ...)` is now required (`4d98292c2`).
- The Nitro freshness fix (`a9d5df4db`).

**2026 format state (SOURCED, lit notes).**

The NVIDIA GPU report:
- 64 structured measurement records (spec, size, hash) plus an opaque driver/VBIOS block.
- Signed by a DICE-like attestation key derived from a fused device identity key.
- Checked against NVIDIA RIM files and a five-certificate chain with OCSP (Gu et al., MLSys 2026).

TDISP/TEE-IO:
- Not shipping on a documented 2026 B200 + TDX stack (arXiv 2608.26575v2), so bounce buffers are
  the norm.
- Gu et al. report plaintext RPC metadata that allows reordering, replaying or omitting RPCs
  between driver and GPU. That is a non-omission failure *inside* the confidential boundary.

**Repair.** (a) A `NvidiaGpuVerifier` in `tee-verify`: parse the SPDM measurement block, check
the cert chain to a pinned NVIDIA root, compare against a pinned RIM, and bind the GPU report's
nonce to the same nonce as the TD quote. That is the cheapest piece of real governance hardware
evidence we do not have. (b) Read the TD15 body: expose `mr_servicetd` and require it to be zero
or pinned. (c) Settling experiment: fetch a public NVIDIA attestation sample (the
`nvtrust` repository ships sample reports) and parse it. On this Mac that is a parser exercise,
no GPU needed.

## W5. The residual-trust experiment is not about TEEs and does not measure the proof leg

**Refutation (VERIFIED).**

What the experiment is:
- `experiments/shielded-baseline/tests/residual_trust.rs` enumerates 1,125 alternative
  published results of a *named-executor shielded batch auction* against one fixed admitted set.
- It asserts that 377 are accepted by the public check and all four owners' batteries, including
  all 33 well-formed (tick, volume) pairs.
- **377 includes the honest run**, so the count of accepted *lies* is 376. It comes from one
  bounded enumeration, and it is not an attack probability (Codex correction).
- There is no TEE in the model.

What it does not do:
- There is no validity-proof leg, so "0 with a proof" (as phrased in
  `whatember/LUCID-INTERVIEW-BRIEF-2026-11-05.md` §4) is an inference and was not measured.
- What it shows precisely: **local owner checks plus a public well-formedness check constrain the
  executor's fill vector and do not constrain the published tick or volume at all.**

**Why it still matters.** It is the right *shape* of argument for "attestation is not evidence of
correct output". An attested executor is a named executor whose code is pinned, and the
experiment measures what checking by the *recipients* leaves open. Calling it a TEE result is an
overstatement, and an outside reviewer who opens the file will see that.

**Repair.**
- Add the proof leg to the same enumeration: a `public_check` arm that requires a validity
  statement (even a reference re-execution standing in for a proof), and assert `admissible == 1`.
- Add an "attested executor" arm, and state it as *two* assertions, because the TEE leg and the
  proof leg differ in their assumptions, not in their counts:
  - With the executor's module digest pinned and the TEE assumed unbroken, the published result
    is a function of the admitted inputs, so the count is 1.
  - With the TEE broken (key extraction, the 2025 DDR-interposer attacks), the count is 377
    again.
  - The proof arm gives 1 under proof soundness, whatever the TEE's state.
- Neither leg constrains *which admitted set* the executor used, unless admission is itself in
  the log. That third freedom is the one to measure next.
- This is the executable form of the lattice's "same predicate, different `assumes`" (lattice
  §0). As it stands it is a hypothesis.

## W6. The seL4 STARK PD verifies its own proof

**Refutation (VERIFIED, `breadstuffs/sel4/README.md`).**
- The M-STARK PD proves a 4-row AIR on the device, verifies it, and shows two rejection teeth.
- Networking (M3) does not yet deliver packets: there is a virtio-mmio slot misalignment, and the
  smoltcp client is missing.
- So no proof produced elsewhere has ever been verified inside a PD.
- The STARK core is a "verbatim" `std → core` port of the `dregg-circuit` custom STARK, which is a
  second copy (a twin unless byte-pinned).
- The deployed proofs in `breadstuffs/circuit` now go through Plonky3 (`p3-batch-stark`), so the PD
  verifies a different proof system from the one the descriptors are proved with (INFERRED from
  `merkle_air.rs` "AUDITED Plonky3 verifier").

**Repair.**
- A host-tool path that writes a real deployed proof into the PD image as a blob (no networking
  needed), with the PD verifying it.
- The PD verifier should be the same crate as the host verifier compiled `no_std`, not a copy.
- This is a day of work, and it converts "boots" into "verifies something that matters".

## W7. The policy-guard weld is not a deployed security property (its own header says so)

**Refutation (VERIFIED, `metatheory/Dregg2/Circuit/Emit/AttestedAutomatonWeld8.lean`, header).**
- The weld exists on the wire only.
- The IR's `MapOp.holdsAt` forces the opening on lane 0 only, about 31 bits. A generic collision
  then costs about 2^15.5. A second preimage against an honestly fixed root costs about 2^31, so
  2^15.5 applies when the adversary chooses both automata (Codex's distinction).
- Lanes 1-7 are pinned and threaded but content-unforced. `weld_completion_lanes_free` exhibits
  two different completions that satisfy the same descriptor.
- `WELD_NAME` has no routing entry, no by-name artifact, and no consumer.

So "4M tokens / 40 ms welded STARK guard" (PROGRAM §2 G-B) describes a prover throughput, not a
binding.

**Repair.** The IR change the header names: make `MapOp.holdsAt` read all eight lanes. Route the
descriptor. Re-run `weld_completion_lanes_free` and expect it to fail, so the theorem flips to
`weld_completion_lanes_forced`. Until then, every absence or `policy.forbid` claim built on the
automaton is at the ~31-bit tier, and the lattice must say so.

## W8. The Lean-authored AIR law: mostly real for descriptors, not for witnesses or the kernel

**Measurement (VERIFIED, `grep` over 2,839 non-test `.rs` files in breadstuffs, 562 in minidregg).**

| Object | Count | Reading |
|---|---|---|
| `include_str!(...json)` descriptor artifacts | 208 distinct | Lean-emitted IR-v2 descriptors (e.g. `membership_descriptor_4ary.rs:150` parses `MEMBERSHIP_4ARY_DESCRIPTOR_JSON`, "Lean-emitted") |
| `EffectVmDescriptor2 {` hits in Rust src | 111 hits in 51 files (33 in `descriptor_ir2.rs`); the pattern also matches `-> EffectVmDescriptor2 {` signatures | candidate Rust-constructed descriptors. Spot check: `circuit-prove/src/solvency_leaf_adapter.rs:173` `solvency_to_descriptor2` builds `VmConstraint2`s by hand in Rust, a real violation. The `*_leaf_adapter.rs` family (about 12 files) looks like the same pattern |
| hand `impl Air<AB> for` blocks | 23 (12 in src/prove crates) | plonky3 AIRs written in Rust |
| `assert_zero/assert_eq/assert_bool/...` calls in non-test src | 87 in 9 files | the hand-written constraint surface (largest: `descriptor_ir2.rs` 21, the interpreter itself; `dsl_p3_air.rs` 15; `bilateral_aggregation_air.rs` 10; `effect_vm_p3_air.rs` 10) |
| `lean_descriptor_air.rs` | 8,731 lines | self-declared RETIRED IR-v1; `emit_faithful` theorems land on it and nothing runs it |
| files mentioning "Rust twin / mirror of Lean" | 128 | witness generators and folds re-implemented in Rust (e.g. `membership_descriptor_4ary.rs:166` "the Rust twin of Lean's `wideFold4`") |
| minidregg `assert_*` AIR calls | 0 | minidregg's circuits are research (Selvage unreached, W9) |

**Reading.**
- For *constraints*, the law is largely real. The deployed constraint systems are descriptors
  emitted by Lean, interpreted by one Rust interpreter (`descriptor_ir2.rs`), whose 21 assertion
  calls are the generic interpreter. That interpreter is the TCB the law moves the trust into, and
  it is hand-written.
- The violations are concentrated: `bilateral_aggregation_air.rs`, `dsl_p3_air.rs`,
  `effect_vm_p3_air.rs`, and the Rust-built descriptors, chiefly the `circuit-prove/src/*_leaf_adapter.rs`
  family. Reproduce with `experiments/air-census/air_census.sh` (output:
  `experiments/air-census/RESULTS-2026-11-05.txt`).
- For *witness generation*, the law does not apply and was never claimed. That costs completeness,
  not soundness. But the governance layer cares about completeness in one specific place: an honest
  prover that cannot produce a proof for a compliant record turns into fail-open pressure ("the
  prover crashed, accept the record").

**Repair.**
1. A census script in CI that counts the rows above and fails on increase. The script exists
   (`experiments/air-census/air_census.sh`) and the numbers are baselined in its RESULTS file.
   Codex's caution applies: these are syntactic counts. The semantic quantity is the number of
   unchecked boundaries between a proved relation and the code that builds or interprets it.
2. Prove `descriptor_ir2.rs`'s interpreter against `DescriptorIR2.lean` semantics. Then the law
   is real end to end: Lean emits, Lean semantics, one proved interpreter.
3. Delete `lean_descriptor_air.rs` or re-target `emit_faithful` at IR-v2. A faithfulness theorem
   about a path nothing runs is a mirror by definition.

## W9. The audit theorem has no executable consumer

**Refutation (VERIFIED, `minidregg/docs/ASSURANCE-CENSUS-2026-10-27.md`).**
- 46 of about 2,350 assurance theorems sit in the `Deployed` umbrella, in 2 of 144 modules.
- None is in the `Host.Main` closure.
- Codex's objection, which I accept: import counts are not an assurance percentage. A theorem
  need not be imported by the binary to certify a definition the binary executes, because proofs
  erase. The relevant question is whether the *subject* of each theorem is executed.
- Selvage, which holds `AuditSampling.lean` (`detect`, `sequential_bound`, `leakage_bound`,
  `missDecomp_of_failClosed`), reaches the Host in 0 of 153 modules.
- For `AuditSampling` the answer is no. The subject (an auditor's alarm decision) does not exist
  as code anywhere, so the commit-then-audit bound, our strongest governance theorem, has no
  executable consumer.

**Repair.** The construction gate (PROGRAM §1 L2): a Rust auditor whose alarm decision is an
`@[export]`ed Lean predicate, following the `dregg_deleg_admit` pattern (W11 below shows it works),
with `ε_chk` instantiated from a named external checker. Until then, "machine-checked audit
bound" is true and "our auditor satisfies it" is not.

## W10. The agent-receipt ladder is non-omission, not completeness

**Refutation (VERIFIED, `breadstuffs/grain-verify/src/lib.rs:36-40`, `r3.rs:86-97`).**
- The crate says it plainly: "Completeness ('nothing else') is not enforced ... a host can make an
  off-chain call and simply never seal a receipt."
- R3 adds non-truncation of the *receipted* history, reduced to the carried `EngineSound`
  hypothesis.
- This is the same gap Lucid's AIAB admits ("completeness is the logger's job").
- Our MMR theorems (`server_cannot_omit_position`, `server_cannot_omit`) are about a committed
  sequence. They do not say the sequence covers what happened.

**Why it is the most important finding here.** Every governance claim of the form "nothing
happened except X" needs a **choke point**: a component every action physically crosses, that
commits each crossing before it can have an effect. Non-omission is a property of logs.
Completeness is a property of *where the log is written*. That is exactly where silicon meets
semantics: the in-path device (TAP, NIC, seL4 PD, TEE network stack) is what turns a non-omission
theorem into a coverage theorem. See design point D2 in the alignment map.

**Repair.** State coverage as its own Lean object. It must not be inferred from non-omission:
`Covers (port : Trace Frame) (log : List Leaf) := ∀ f ∈ port, ∃ i, log[i] = H f ∧ ...`. Then prove
it for the device's step function (W3 repair (b)). The physical premise, that every frame transits
the port, is irreducibly an installation claim. It belongs in the lattice as an *audited-sampled*
tier (TAP installation assurance is Amodo workstream 12, "not on track").

## W11. What holds (so the critique is not read as "nothing works")

**Admission export (VERIFIED).** `dregg_deleg_admit`
(`metatheory/Dregg2/Apps/DelegAdmit.lean:67`, called from `sdk/src/tool_gateway.rs:212`) is a real
Lean-decided C-ABI decision.
- The Rust mirror was deleted.
- An unreachable oracle refuses.
- Its limits: one grant, one tool, Int-coded. There are no delegation chains or attenuation proofs
  in this export.
- Codex's sharper limit: the five conjuncts do not authenticate the grant, the clock (`now`) or
  the prior counter (`old`), and they do not stop concurrent reuse or check revocation. It is a
  verified *predicate* inside an unverified authorization *system*.
- The construction-grade statement: an admitted external effect consumes exactly one
  authoritative counter transition under an authenticated, unexpired grant, with global usage
  bounded across crashes and concurrency (TrInc-style monotonic counter).

**MMR and attested query (VERIFIED statements).** `server_cannot_omit_position` (MMR.lean:780)
and `server_cannot_omit` (AttestedQuery.lean:358) are the right statements.
- Their hash is `List ℤ → ℤ`, with collision resistance as a hypothesis (`¬ MRootColl`,
  `Poseidon2SpongeCR`). That is the standard idealisation, but over ℤ it is satisfiable only by an
  injective encoding, so the statement transfers to Poseidon2 or Blake3 only as a reduction
  ("omission ⇒ collision"), never as an unconditional fact.
- The prose should say "omission implies a collision", not "cannot omit".

**AuditSampling as mathematics (VERIFIED theorem index, `grep -c sorry` = 0).** The
`detect` / `sequential_bound` / `leakage_bound` / fail-closed structure is in the file.

**Quote verifiers (VERIFIED).** tee-verify's TDX/SNP/Nitro paths are sound as quote verifiers
(W4 says what they do not cover).

## Ranked: the three that matter most for a governance program

1. **W10 (coverage vs non-omission).** Every "nothing else happened" claim, ours or Lucid's or
   Plan A's, fails here first. It is also the one whose repair forces silicon-to-semantics
   alignment, so it is an opportunity as well as a weakness.
2. **W4 (no GPU evidence).** Inference-provision claims about *where* and *on what* are, today,
   claims about a CPU TD. The repair is small and the evidence formats are public.
3. **W1+W2 (the silicon theorems are about the right pipeline and the wrong objects).** The
   alignment program's distinctive asset is "one semantics down to gates". Today the gates carry
   a comparator and the tensor-core theorem carries no accumulator. Both repairs are well-posed
   theorems with cheap first experiments.

W7 and W9 are serious but self-documented. W8 is in better shape than the law's own memory
suggests. W3 is mostly a citation-discipline problem plus one CI step.
