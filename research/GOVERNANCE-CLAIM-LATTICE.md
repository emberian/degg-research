# The governance claim lattice for training and inference provision (2026-11-05)

Status: PROPOSED research note, with VERIFIED/SOURCED/INFERRED tags on every material fact.
Companions: `research/governance-foundations-critique.md` (W1-W11, the weaknesses this lattice
must not paper over), `research/governance-codex-synthesis.md` (the cross-model attack and what
changed), `redregg/designs/SILICON-TO-SEMANTICS-ALIGNMENT-2026-11-05.md` (the constructions that
would move claims up tiers), `experiments/mma-field-mapping/` and `experiments/air-census/` (the measurements made today), `research/governance-literature-notes-2026-11-05.md` (primary-source notes).

Purpose: list the claims a provider of AI training or inference could be asked to certify, to a
regulator, a counterparty or a treaty body. For each claim, give the evidence tiers that are
possible, what we have at each tier today, what is missing, and the attack that separates one
tier from the next. Then compare with what Lucid, Plan A/Amodo, Ansari, Baker et al. and
Peigné et al. can state, in both directions.

This is not Lucid's four questions with more rows. The structural claim of this note is that
**governance claims form a lattice indexed by their assumption sets, not a list indexed by
topic.** The interesting design points are the meets: claims that need two kinds of evidence at
once and that no single-mechanism programme can state.

---

## 0. The object: a semantic lattice plus evidence judgments

The first draft defined one order on (predicate, assumptions, eps, tier) tuples and called it a
lattice. Codex's round-2 attack showed it was not one (`governance-codex-synthesis.md` §2):
- antisymmetry fails on equivalent predicate ASTs and on tiers the order ignores;
- conjunction with summed error is neither the meet nor an upper bound;
- composing a receipt with itself doubles its error, so idempotence fails.

The repaired object keeps two things apart.

**(i) The semantic lattice.** Fix a subject, a window and a world space Ω (executions together
with physical facts). Claims are predicates `P : Ω → Prop`, quotiented by logical equivalence and
ordered by implication. Meet is `∧` and join is `∨`. That is a Boolean lattice, and it is the
right home for "which statement is stronger". Claims about different subjects compose only
through an explicit relation or projection into a common subject (a device epoch, a session, a
model commitment). Identical statement bytes are neither necessary nor sufficient.

**(ii) Evidence judgments.** Evidence is a judgment, not a lattice element:

```
A ⊢ V : P [ε]      -- "for every adversary in the experiment permitted by assumptions A,
                    --  Pr[V accepts ∧ ¬P] ≤ ε"
```

`V` is a verifier together with the evidence it consumes. Tier records *how* `V` was produced
(signature, hardware quote, proof, sample, inspection). It is provenance, not order. The
composition rule:

```
A₁ ⊢ V₁ : P₁ [ε₁]    A₂ ⊢ V₂ : P₂ [ε₂]    both remain valid in one common experiment under A₁ ∪ A₂
B ⊢ Bind : V₁ and V₂ concern the same projected subject [ε_b]
──────────────────────────────────────────────────────────────────────────────────────────────
A₁ ∪ A₂ ∪ B ⊢ (V₁ ∧ V₂ ∧ Bind) : P₁ ∧ P₂ [min(1, ε₁ + ε₂ + ε_b)]
```

Independence of the two legs is unnecessary. What is necessary:

- **The common-experiment premise.** Each bound must survive in the composed experiment. A TEE
  bound proved against a host adversary does not automatically survive an adversary who also
  controls the prover.
- **The binding leg.** Two valid leaves in one log (a fresh GPU quote and an unrelated valid
  proof) establish no conjunction (Codex round 1, attack D3). `Bind` checks that both open one
  typed statement binding device/session, program, model, input, output, epoch and policy, and
  its own failure probability is charged.
- **Quantifiers do not upgrade.** A universal safety judgment (T2), a statistical detection
  judgment (T3) and an enforcement action are different kinds of thing. Composition keeps each
  conjunct's quantifier.
- **Assumptions model capabilities and behaviour separately.** "The operator controls the host"
  is a capability. "The operator follows the prover protocol" is a behaviour. They do not
  contradict each other. Consistency of an assumption set need not be decidable in general, so the
  receipt language must restrict `A` to a fragment where it is decidable, or carry a
  satisfiability certificate.
- **`A` is sufficient, never minimal.** Serialising the assumptions the proof used cannot show
  that no unstated physical premise exists.

A receipt (design point (i) of the alignment map) is a serialised judgment `(subject, P, window,
tier, ε, A, evidence refs)`. As a Lean sketch:

```
structure Judgment where
  subject   : Subject
  predicate : Pred subject      -- an AST, never an opaque Bool
  window    : Window
  tier      : Tier              -- provenance of V, not an order
  eps       : ℝ≥0
  assumes   : Finset Assumption  -- in a decidable assumption fragment
  evidence  : List EvidenceRef
```

Two consequences carry most of the design weight below:

1. **A composed claim is only as assumption-light as the union.** Gluing a TEE-attested model
   hash to a STARK-proved inference adds the TEE's assumptions to the proof's. Ask which
   assumptions each leg *removes from the other's residue*. The residual-trust measurement
   (`experiments/shielded-baseline`, critique W5) is exactly a count of the predicate-space
   admitted under an assumption set.
2. **Coverage starts as an assumption and can be discharged into a predicate** (critique W10).
   "Every action crossed port P" sits in `assumes` for every completeness claim. It can itself be
   proved as a predicate under narrower architectural premises (the device step function, design
   point (b)), which removes it from downstream `assumes` (Codex round 2). What remains is physical
   (installation audit, tamper evidence) or attested (the TEE's only network path is a measured
   device).

### 0.1 Tiers

| Tier | Name | What produces it | eps is | Typical `assumes` |
|---|---|---|---|---|
| T0 | assertion | a party signs a statement | undefined (1) | the signer is honest |
| T1 | attested | a hardware root signs a measurement of state (TEE quote, SPDM GPU report, TPM PCRs, DICE) | key-extraction/forgery probability, unquantified | vendor root, microcode, no physical attack, measured code does what its source says |
| T2 | proved | a proof system verifies a computational statement about committed inputs | stated (e.g. FlashProve `~q/2^124 + 2^-101`, SOURCED) | hash in ROM/CR, proof-system soundness, *verifier implementation correct*, statement means what the regulator thinks |
| T2* | proved-and-verified-verifier | as T2, and the deployed verifier is the checked one (Lean-decided export, or a proof of the verifier binary) | as T2 | as T2 minus "verifier implementation correct" |
| T3 | audited-sampled | commit, then audit a beacon-chosen sample by recomputation, proof or inspection | `1 - q` per corrupted item with `q = p(1-ε_chk) - ε_bind - ε_beacon` (`minidregg/Selvage/AuditSampling.lean`, VERIFIED statement) | commitment binds before the beacon, the beacon is unpredictable, audits are fail-closed |
| T4 | physically-audited | inspection, TAP installation audit, tamper-evident seals, side-channel wardens | none, an adversarial physical test | inspectors honest and competent |

For T3, the auditor might sample k items uniformly without replacement from N committed items,
b of them bad. The non-adaptive miss probability is then C(N−b,k)/C(N,k).
`AuditSampling.lean` generalises this to an adaptive, Bernoulli-per-item stopping-time form. The
formula is void if the auditor chooses easy samples, if the evidence can be reconstructed after the
challenge, or if evidence is unavailable and treated as a pass.

T3 is not "weaker than T2". It is orthogonal. It reaches statements that T2 cannot afford
(frontier training steps, every inference at 409.6 GB/s tap rate — `zkml-research/notes/hpu-seam-study.md`,
"100%-proving is physically impossible at the tap") at an eps the auditor chooses through `p`.

**The attacks that separate tiers.** Each one is a reason a regulator should care which tier they
were given.

- **T0 → T1: the party lies.** Attestation removes the signer's freedom over the measured state.
  It leaves the signer's freedom over inputs, scheduling and everything unmeasured.
- **T1 → T2: the hardware root is broken or the host steers inputs.** Physical DDR-bus
  interposition attacks on TDX and SEV-SNP in 2025 (TEE.Fail, Battering RAM, WireTap) extracted
  attestation keys or broke memory integrity. TEE.Fail reportedly used the extracted keys to
  forge NVIDIA GPU-CC attestation, because that chain bottoms out in the CPU TEE. SOURCED from
  memory; check the papers before citing.
- **T1 → T2, a second failure: the boundary leaks.** Gu et al. (MLSys 2026, SOURCED via lit notes)
  found plaintext driver-to-GPU RPC metadata inside the CC boundary that allows reorder, replay or
  omit.
- **T2 → T2\*: the deployed verifier is not the proved one.** FlashProve's Lean theorem covers the
  Lean verifier, while the fast C++ `cp_verify` is "tested, not proved" (SOURCED). Our own
  `dregg_deleg_admit` is the T2\* pattern (critique W11).
- **T2/T3 → wrong claim: the statement is not the regulator's predicate.** FlashProve has no
  theorem that the committed integer program is the shipped FP model (SOURCED). A proved claim
  about the wrong object is a T0 claim about the right one.
- **T3 → failure: fail-open.** If an audit timeout is "no alarm", the adversary aborts whenever
  audited and `q → 0` (`failOpen_no_legs`/`beaconless` in `AuditSampling.lean`, VERIFIED to
  exist). Also *grinding*: the beacon is fixed after the commitment is visible to the operator.
- **any → T4: physical relay.** RTT location bounds prove "near an anchor", never "far from one".
  A colluding anchor or a relayed key breaks it (sovcert v0.1.0 leaves `minimum_anchors`
  undefined, SOURCED).

### 0.2 Exactness is a separate axis

A claim checked with a tolerance (float recompute with an AUC threshold, LSH match, power-to-compute
within 10%) admits a steering channel whose capacity grows with the tolerance (Zamir's theorem as
cited in `zkml-research/docs/the-position.md`, SOURCED). So every claim also carries
`exact : Bool`. Exact needs integer or bit-exact semantics. MXFP4 block dots are exact ≤13-bit
integers (`zkml-research/phase0`, SOURCED from the brief). For training, Peigné et al. demand
bit-exact BF16/FP32 with RNE and pinned kernels, and find tensor-parallel allreduce is the blocker
(SOURCED). **Corrected by the Astra statement audit (G07).** My first sentence here said
tolerance claims are never stronger than T3 with an ε that includes the channel, and that was
wrong. An exact proof of `|y − f(x)| ≤ δ` can have zero proof error. Its *predicate* is weaker
than equality, but it is not a randomly wrong proof of equality. What the steering result
actually says is that the tolerance relation admits outputs an adversary can choose, so the
receipt must carry the tolerance relation itself (δ, units, quantifiers) as its predicate. It
must never convert δ into ε unless a game over perturbations is separately specified.

---

## 1. The claims

Notation per row: **E** = evidence achievable per tier; **Ours** = which construction supplies
which tier today, with its real rung (critique numbers in brackets); **Missing**; **Attack** = what
distinguishes the tiers for this claim specifically.

### 1.1 WHERE (location, jurisdiction, enrolled hardware)

| ID | Claim | E | Ours today | Missing | Attack |
|---|---|---|---|---|---|
| W-1 | Job J ran on enrolled device D (serial/identity key in a registry) | T1 (SPDM GPU report, DICE identity), T4 | none for GPUs [W4]; CPU TD identity via tee-verify TDX/SNP (VERIFIED) | NVIDIA SPDM/RIM verifier; device registry as an MMR so enrolment is non-omittable | key transplant: an extracted identity key signs on unenrolled silicon |
| W-2 | D is within radius r of anchor set A at time t | T1+T4 (RTT challenge signed by D's TEE key, anchors sign receipts) | none | everything; our contribution would be a proved geometric verifier (multilateration consistency as a decidable predicate) and anchor receipts in the same MMR | relay plus colluding anchor; RTT shows near, never far |
| W-3 | D did not leave jurisdiction during window W | T1 (heartbeat chain) + T3 (sampled RTT) | MMR heartbeat chain is buildable (dregg-query, VERIFIED exists) | the heartbeat as a dense-index log with gap = alarm (fail-closed); freshness theorem | suspend-and-move between heartbeats; heartbeat period sets the gap |
| W-4 | The *data* of job J never left jurisdiction (residency) | T1 (TEE egress policy), T2 (egress log coverage + Count=0 of out-of-zone destinations) | MMR + Count=0 aggregation (dregg-query, audit `BudgetProof`, VERIFIED exist) | coverage of egress [W10]; destination-zone mapping is itself a T0 table | a covert channel the egress device does not see (timing, power) |

### 1.2 HOW (training vs inference, budget, manifest, fabric, wipes)

| ID | Claim | E | Ours today | Missing | Attack |
|---|---|---|---|---|---|
| H-1 | Job J is inference, not training (class) | T3 physical (power side channel 97%, 2609.00309, SOURCED), T1 (TEE runs an inference-only image), T2 (proof of the forward pass only, no optimizer state) | none physical; FlashProve-style inference proofs exist externally | a *proved* class claim: the job's committed program has no write path to weights. That is a static property of a committed artifact (a slot-confinement theorem shape we have: `SlotConfinement.lean`), not of power | diluted LoRA: training detection falls to 30-74% (48-88% hardened) at 34-69% evader cost (SOURCED) |
| H-2 | Cumulative FLOP of J ≤ B | T3 (meter + sampling), T1 (on-chip counter in attested report), T2 (proof of the committed workload's op count) | counts-over-clocks discipline only; `AuditSampling` `leakage_bound` (`E[Λ] ≤ b/q`) gives the sampled-overspend bound shape (VERIFIED statement) | an attested counter (Ansari M5 "Requires R&D"; Peigné OP-22 "compute accounting as a distinct certified object", SOURCED); a theorem that a proved program's op count is a function of its shape | counter reset; work done on unmetered units; the operator labels training as inference with the same FLOP |
| H-3 | J matches its Verifiable Job Manifest (declared model, class, shape, duration) | T0 (VJM signed), T1 (TEE measures image = VJM.image), T3 (reconcile against power and utilisation), T2 (VJM is the public input of every proof J emits) | none named VJM; `vk_v2` binds program + AIR shape + verifier + proving system into a VK (`circuit/src/air_descriptor.rs` doc, VERIFIED) | the VJM as a commitment that *every* receipt from J opens: one manifest root, every proof's public input | a second undeclared job sharing the hardware; a manifest that is true but incomplete |
| H-4 | No undeclared east-west fabric use (no all-reduce of gradients between pods) | T4/T1 (TAP or attested NIC hashes flow headers into an append-only commitment), T3 (bandwidth caps make covert training uneconomic) | MMR non-omission statements (VERIFIED); a proved emitted function pattern (Orb, critique W3) | the in-path device and its coverage theorem (alignment D2); NVLink inside a node is invisible to inter-node TAPs (Peigné OP-10, SOURCED) | gradients smuggled inside inference-shaped traffic; compressed or low-rank updates under the cap |
| H-5 | Memory/weights were wiped at job end | T1 (TEE teardown attested), T4 | none | everything; "no persistence is not proof of erasure" (Lucid AIAB, SOURCED); Amodo workstream 15 "Uncertain" | cold-boot or HBM remanence; a host snapshot taken before teardown |
| H-6 | The training run followed its committed schedule: data order, optimizer, steps | T3 (beacon-selected step proofs after a terminal freeze, Peigné et al.), T2 per step | `Rank1GradientCheck.lean`, `ZkmlLowRankUpdate.lean`, DARK-TRAINING (L1 per the PROGRAM table, SOURCED from it) | bit-exact training stack; the freeze-then-beacon ordering as a proved protocol property (our AuditSampling's `ε_bind` leg is this) | determinism failures masquerading as tolerance; the seed opened before the freeze |

### 1.3 WHAT (model identity and provenance)

| ID | Claim | E | Ours today | Missing | Attack |
|---|---|---|---|---|---|
| M-1 | The weights served are commitment C (openable, not a flat hash) | T1 (hash at load in TEE), T2 (proof of inference against C: FlashProve's BLAKE2b commitment ID, SOURCED) | `zkml-research/registry` v1 flat SHA-256 (L3), v2 per-expert Poseidon2 roots designed (L1) (SOURCED from PROGRAM table) | a commitment that both a TEE loader and a proof system open, so that T1 and T2 evidence name the *same* object | hollow-LLM: serve a different model behind a correct-hash façade; a flat hash is unopenable by a proof |
| M-2 | Output y was produced by C on prompt x (per request) | T2 (FlashProve: one H100, Maverick, 18.78 MB/request, SOURCED), T3 (recompute a beacon sample; Lucid, Plan A) | AuditSampling theorem for the T3 leg (L1, unreached [W9]); MXFP4 exactness for exact recompute (L1/L2) | the auditor that routes its alarm through the Lean predicate; recompute as exact, not tolerance | tolerance steering (§0.2); seed grinding; fail-open on timeout |
| M-3 | C's provenance: C = quantize(merge(base B, adapters A₁..A_k)), with tools T, system prompt S, RAG corpus R | T0 today everywhere; T2 per edge (a proof per transformation) | none as a graph; the parts exist: per-expert roots, Freivalds/low-rank update checks | the provenance graph as a Merkle DAG whose edges carry proofs or attestations, each at its own tier (meet semantics §0) | a "provenance graph" whose edges are all T0 is a signed diagram |
| M-4 | C is the model that was evaluated (eval binding) | T0 (lab signs "eval E ran on hash h"), T2 (the eval run's proofs open the same C as deployment's proofs) | none | **eval and deployment proofs sharing one commitment opening.** The commitment ID is the join point | evaluated-model swap: eval on C, deploy C′ with the same marketing name |
| M-5 | The quantised model is within δ of the evaluated FP model *on the evaluation distribution* | T3 (re-run eval on C), T2 (proof of the eval) | none | an explicit δ claim; FlashProve proves a 4-bit *conversion*, with no theorem that it is the shipped model (SOURCED) | quantisation as a place to hide a behaviour change |

### 1.4 WHO (identity, mandate, delegation, agent accountability)

| ID | Claim | E | Ours today | Missing | Attack |
|---|---|---|---|---|---|
| A-1 | Request r came from principal P under KYC record K | T0/T1 | credential stacks (macaroons, PQ creds, SOURCED dossier 03) | binding the principal to the per-request receipt in the same log as M-2 | shared accounts; resale |
| A-2 | Agent action a was admitted under mandate g (scope, deadline, rate) | T2\* (`dregg_deleg_admit`, Lean-decided, VERIFIED) | real, single grant [W11] | delegation chains (attenuation proof per hop); mandate revocation as a non-omittable log | the action ran off-gateway (coverage again) |
| A-3 | Every action of agent session S is receipted, ordered, non-truncated (R0-R3) | T0 (host-signed chain), T1 (renter-anchored), T2 (R3 whole-history STARK, reduced to `EngineSound`) | grain-verify R0-R2 landed, R3 reduced (VERIFIED) | coverage [W10]: R3 is non-truncation of what was receipted | the un-receipted side call |
| A-4 | Monitoring policy P was applied to every transcript in window W | T1 (AIAB-style CVM runs plan, SOURCED), T2 (per-transcript policy proof) | complement DFA + CFG certificates (L2), weld at ~31-bit tier [W7] | eight-lane forced weld; the policy as an AST in the log, not a classifier score | splitting a forbidden token across structural boundaries (what CFG-aware complement defeats); classifier evasion at T1 |

### 1.5 WHAT-NOT (absence)

| ID | Claim | E | Ours today | Missing | Attack |
|---|---|---|---|---|---|
| N-1 | No record in corpus D matches forbidden pattern P (canary, benchmark item, PII regex) | T2 (per-record complement-DFA proof + dense-index Count=0), T3 (sampled scans) | the protocol in `whatember/sources/01-...md` §5 composes `tableDfa_faithful`, `compact_sound`, `server_cannot_omit_position`, Count=0 (statements VERIFIED to exist); weld not forced [W7] | forced weld; the corpus commitment taken *before* training; a binding from "corpus D" to "what the training job read" (H-6) | prove absence over D while training on D′; near-duplicate or paraphrase evasion of a regular P (absence is only as good as P) |
| N-2 | No transcript in window W contains a forbidden action pattern | T2 as N-1 over transcripts | as N-1 | coverage of transcripts [W10] | the omitted transcript |
| N-3 | No job outside the declared set ran in window W | T2 over a covered job log: Count(undeclared) = 0; T3 via power reconciliation | MMR + Count=0 (VERIFIED exists) | coverage of job launches: a choke point where jobs are admitted (the scheduler as a measured, in-path component) | jobs launched outside the scheduler |
| N-4 | No weights left the facility (exfiltration) | T3/T4 (egress caps, Rinberg et al. 2511.02620 as cited in the brief), T2 (Count=0 over covered egress) | none | everything at T2 | steganographic exfiltration inside permitted outputs |

### 1.6 HOW-WELL (eval provenance)

| ID | Claim | E | Ours today | Missing | Attack |
|---|---|---|---|---|---|
| Q-1 | Eval E with item set I scored s on C | T2 (proved eval run), T3 (sampled re-scoring) | none | eval item set as a commitment with an absence certificate against the training corpus (N-1 joined with Q-1) | contamination; grader-model drift |
| Q-2 | The eval items were not in the training corpus | T2 (N-1 with P = the item set) | as N-1 | this *is* N-1 with P derived from I's commitment, so the eval and the corpus share a log | paraphrased items |
| Q-3 | The red-team or dangerous-capability eval was run with the declared elicitation budget | T0, T3 | none | the elicitation transcript as an A-3 session | sandbagging under eval |

### 1.6a OCCURRENCE, QUANTITY, LINEAGE, LIFECYCLE, SERVICE (added after the Codex attack)

The first draft missed these. Each is a distinct predicate that the tables above do not imply.

| ID | Claim | E | Ours today | Missing | Attack |
|---|---|---|---|---|---|
| O-1 | **Occurrence:** the provider *performed* computation K in window W on device D. Validity of K is a separate claim | T1 (device-signed, epoch-bound), T4 | none | binding a proof's statement to a device epoch: a proof of a valid trajectory is not evidence it happened then, there, on that hardware | replay a proof computed elsewhere or earlier; outsource; reconstruct after the fact |
| O-2 | **Uniqueness / once-only:** receipt R is the only one issued for request r; no double billing; retries are counted | T2 over a covered log with a monotonic counter (TrInc-style) | MMR (positions are unique by construction, VERIFIED statements) | an authoritative counter bound to the device | cloned VM snapshots racing the same lease; cached answers; idle padding |
| O-3 | **Useful vs executed operations:** the declared FLOP were spent on the declared job | T2 (op count a function of the proved shape), T3 (power) | none | as H-2 | padding with useless work to hit, or hide, a budget |
| L-1 | **Training lineage:** init, dataset order, optimizer moments, RNG, adapters, merges, imported checkpoints and parents are all declared | T2 per transition (recursive commitment of the whole optimizer state), T3 | DARK-TRAINING lemmas (L1) | the lineage capsule: commit (weights, Adam moments, RNG, data cursor, parent ids) every step (Codex construction 4) | a clean short run that starts from smuggled pre-trained weights (Proof-of-Learning spoofing, 2103.05633 and its attacks) |
| I-1 | **Inference identity beyond weights:** tokenizer, system prompt, retrieval state, adapters, quantization, decoding randomness, KV-cache reuse, speculative decoding | T2 if all are in the committed statement; T0 otherwise | none | each component in the M-3 provenance DAG and in the per-request statement | swap the system prompt or the draft model while the weights hash still matches |
| C-1 | **Confidentiality lifecycle:** allowed observers, retention, migration, rollback, output leakage | T1 (TEE policy), T2 (log of key releases) | ML-KEM envelope (dregg-chutes-e2ee, VERIFIED exists), TD15 `MR_SERVICETD` dropped [W4] | migration TD pinning; rollback protection | migrate the TD to a hostile host through a bound service TD |
| S-1 | **Service behaviour:** delivery, deadlines, cancellation, selective abort, audit-data availability | T2 over a covered log; T3 | none | availability as a claim distinct from fail-closed. Fail-closed is safety, not availability | selective abort on exactly the audited requests (this is the fail-open attack seen from the other side) |

### 1.7 META (claims about the evidence system)

| ID | Claim | E | Ours today | Missing | Attack |
|---|---|---|---|---|---|
| X-1 | Receipt chaining: every receipt commits its predecessor | T2 | MMR, R0-R2 chain (VERIFIED) | none at the statement level | key compromise rewrites the suffix (needs X-3) |
| X-2 | Non-omission: a window query cannot drop a position | T2 (omission ⇒ collision) | `server_cannot_omit_position`, `server_cannot_omit` (VERIFIED; hash over ℤ, CR as hypothesis [W11]) | a Poseidon2/Blake3 instantiation as a reduction | collision (≈2^15.5 where only one 31-bit lane is forced [W7]) |
| X-3 | Equivocation detection: two relying parties see one history | T2 given gossip; T3 | `degg-research/experiments/inclusion-availability` `equivocation.rs` (SOURCED from brief) | a consistency-proof theorem (MMR prefix proofs) and a witness/gossip protocol between treaty parties | split view to parties that never compare |
| X-4 | Fail-closed: absence of evidence is an alarm (timeout, missing heartbeat, unreachable verifier) | T2 as a property of the verifier code | `missDecomp_of_failClosed` (VERIFIED statement); `dregg_deleg_admit` refuses on unreachable oracle (VERIFIED) | fail-closed as a census-checked property of every verifier entry point | operators lobbying to make timeouts warnings |
| X-5 | Transparency: every receipt is in a public, append-only, witnessed log | T2 + T4 (independent witnesses) | MMR (VERIFIED) | witness cosigning | log operator collusion |
| X-6 | Coverage: the log is written at a choke point every action crosses | T4 + T1 (the device is measured), T2 for the device's step function | none [W10] | the coverage theorem for a device step function (alignment D2) | a second path around the device |
| X-7 | Prover-independence: a receipt names its predicate and commitment, not its proof system | T2 | PROGRAM F6 (PROPOSED) | the receipt schema | migration to a weaker proof system under the same receipt name |
| X-8 | Assumption disclosure: every receipt carries its `assumes` set and `eps` | T2 (the schema), T0 (honesty of the set) | none | the `Judgment` structure of §0 as the receipt format | hiding an assumption in prose |

---

## 2. Comparison: what others can state, and what they cannot

### 2.1 Against Lucid's four questions (SOURCED: `whatember/LUCID-INTERVIEW-BRIEF-2026-11-05.md`, lucidcomputing.ai/validation-cluster)

Lucid's Where/How/What/Who map to W-, H-, M-, A- above, and their Assurance Node is a T1
collector that emits chained T0 receipts (signed by the node) over T1 evidence. Their own
tiering, from the essay "From Assertions to Evidence", is assertion / cryptographic /
probabilistic, which matches our T0 / T1+T2 / T3. It does not separate T1 from T2, and it has no
T2\*.

**What the lattice can express that their schema cannot.** These are narrowed after Codex
round 2, which named ZkAudit and CryptoVerif as partial occupants of items 2 and 4.

1. **Absence with a count.** N-1..N-4 at T2: "Count of matching records in the covered window is
   exactly 0, with omission ⇒ collision." Lucid's How is a reconciliation (power vs manifest,
   ≤10% error target, SOURCED). That is a tolerance claim, and by §0.2 it can never say *zero*.
   Their `policy.forbid(...)` evaluator returns permit/forbid per evidence item. It has no
   non-omission over a window, so it cannot certify that the forbidden thing did not occur
   anywhere.
2. **Meets that share a commitment.** M-1 ∧ M-2 ∧ M-4 ∧ Q-2: the TEE loader's hash, the
   per-request proof, the eval run and the corpus absence certificate all open *one* commitment.
   Lucid's model identity is "not an immutable weight digest" by their own admission (SOURCED), so
   none of these meets can be formed.
3. **Meta-claims X-2, X-3, X-6.** Lucid has hash-chained receipts (X-1) and heartbeats. It does
   not state non-omission, equivocation or coverage. The AIAB code says "completeness is the
   logger's job" (SOURCED).
4. **Assumption-indexed composition (§0).** Their receipts do not carry `assumes`, so a relying
   party cannot compute what a composed receipt still trusts.

**What they can express and we cannot:** W-2 location (sovcert RTT, a real spec); H-1 and H-2 by
physical measurement (shunt power, DC-SCM/BMC); H-5 at T4; treaty-tier SPDM freshness; the
operating facts of an 18-month adversarial testbed with RISE. We have no physics.

### 2.2 Against Plan A / Amodo's 17 workstreams (SOURCED: amododesign.com SitRep via lit notes)

| Amodo workstream (status) | Lattice claims it serves | What the lattice adds |
|---|---|---|
| 1 Passive optical TAPs (Active) | H-4, X-6 | coverage is X-6, an *assumption*; the TAP's own step function needs a T2 theorem (D2) |
| 2 Recompute servers capture (Active) | M-2 T3 | the capture log needs X-2 or a dropped request is undetectable |
| 3 New TAP types / bandwidth limits (Not started) | H-4 | a bandwidth cap is a T3 claim with `leakage_bound` shape `E[Λ] ≤ b/q` |
| 4 Storage bank to inference path (Not on track) | M-1 | the path is a coverage claim on weights movement |
| 5-7 Reproducibility (5 Active; 6, 7 Not started) | §0.2 exactness | exactness by format (MXFP4) instead of by workaround |
| 8-10 Recomputation algorithms, frontier, red-team (8 Active; 9, 10 Not started) | M-2 T3 | the composed bound with ε_bind, ε_chk, ε_beacon and the fail-open refutation (AuditSampling); "tolerance is a channel" |
| 11 Recompute server security (Not on track) | T2\* | the recompute server *is a verifier*; the T2→T2\* attack applies |
| 12 TAP installation and links (Not on track) | X-6 | the irreducible T4 premise of every completeness claim |
| 13 Verification reporting (Not on track) | X-8, X-7 | the `Judgment` structure as the report format |
| 14 Physical security (Not on track) | T4 | no addition |
| 15 Memory wipes (Uncertain) | H-5 | no addition at T2 |
| 16-17 Side channels: mitigation, wardens (Not on track) | T4 | none from us; note that 2609.00309 turns the side channel into evidence for H-1 |

Plan A's programme is organised by *mechanism*. Its claims are implicit. The lattice makes
explicit two things the SitRep cannot represent. **Completeness is an assumption shared by
workstreams 1, 2, 4 and 12**, so one failure there voids N-, H-4 and A-3 together. And **the
recompute server is a verifier**, so it inherits the T2→T2\* gap.

### 2.3 Against Ansari's 20 mechanisms (SOURCED, 2604.04712)

Ansari rates *mechanisms* by feasibility. The lattice rates *claims* by evidence tier. The
mapping is many-to-many:

- M5 (on-chip compute metering, Requires R&D) is one route to H-2 at T1. We offer a T2 route for
  committed workloads, so the claim is not blocked on M5 for proved jobs.
- V2 (proof-of-training) is H-6 at T2/T3.
- V3 (FlexHEG) is H-4 and X-6 with an attested in-path device.
- V5 (multiparty control of training runs, Speculative) is the X-3 and X-4 meta-claims with a
  threshold verifier.
- E2 (chip-to-chip networking restrictions) is H-4 enforced rather than observed.

What Ansari lacks: no meta-claims, no absence claims, no exactness axis. What Ansari has and we
lack: the enforcement tier (E1-E7). The lattice is about evidence, not enforcement. An off-switch
is an action, not a claim.

### 2.4 Against Baker et al.'s six layers and Peigné et al.

- **Baker et al.** Layers 1-3 (on-chip, network taps, analog sensors) are T1, T4+X-6 and T3
  physical. Layers 4-6 (whistleblowers, interviews, intelligence) are outside any cryptographic
  tier, and they are what catches the X-6 failure (the second path around the device). A
  realistic treaty claim is a meet with a human-intelligence leg. The lattice should hold a T5
  "institutional" tier rather than pretend.
- **Peigné et al.** state H-6 precisely, with a freeze-then-beacon order, a TAP tier and an
  attested-SmartNIC tier, which is our T4 and T1 for X-6. Their open problems OP-10 (intra-node
  witness) and OP-22 (compute accounting) are H-4 and H-2 holes the lattice also has. What they do
  not state: absence (N-1) on the training corpus, eval binding (M-4) and agent claims.

---

## 3. The three most valuable claims nobody else can currently express

Each needs a composition of evidence kinds that only a stack spanning commitments, proofs,
attestation and a verified decision procedure can form. Prior-art check: Kagi and arXiv sweeps
through the lit helper, plus Codex round 2, which attacked this section directly. "Nobody" means
"none of Lucid, Plan A/Amodo, Ansari, Baker et al., Peigné et al., FlashProve or SASH as read
today". It is not a literature-wide first. Round 2 also narrowed all three claims, and the
narrowed forms are given here.

1. **N-3 ∧ X-6 ∧ X-2: "Zero undeclared *admissions* on enrolled hardware in window W, starting
   from an attested inventory of active contexts."** A dense, non-omittable job-admission log
   written at a measured choke point, with Count(undeclared) = 0 and omission ⇒ collision.
   Physical reconciliation (Lucid H, Plan A taps) estimates. This counts.
   - Codex's attack, accepted: admission coverage is not computation coverage. An approved
     persistent job can train internally without a second admission, and jobs already running at
     the start of W are invisible to an admission log.
   - So the honest claim is about admissions, joined with H-1 (class) per admitted job, plus an
     initial inventory.
   - APEX (USENIX Sec'20) links verified hardware mechanisms to authenticated execution evidence on
     one device. It does not make the facility-wide negative claim.
2. **M-1 ∧ M-2 ∧ M-4 ∧ Q-2 ∧ H-6/L-1 over one commitment: "The model that answered this
   request is the model that was evaluated, it was trained on corpus D, and the eval items are
   absent from D."** One commitment is the join point of a TEE load measurement (which supplies
   *occurrence*), a per-request proof or sampled recompute, an eval-run proof, a training-lineage
   proof, and a corpus absence certificate.
   - **Partly occupied, so this is narrowed.** ZkAudit (arXiv 2404.04500) already binds committed
     weights and data through training proofs, and proves functions of the committed objects,
     including audits. Codex also notes that a flat hash *is* openable in a proof, by witnessing
     its preimage. Merkle and per-expert commitments buy efficiency, not expressibility.
   - What remains distinct: joining that with the *served* request at deployment scale (FlashProve
     for M-2), the TEE occurrence leg, and a structure-aware absence certificate (complement DFA +
     CFG) rather than a hash-set check.
   - Without H-6/L-1 the claim is broken: prove absence over a clean D while the model was trained
     on D′. The first draft omitted that leg.
3. **X-8 as the receipt: "This receipt holds with error ε under assumptions A (sufficient, not
   minimal), and its composition with that one is computed by the §0 rule."**
   - **Partly occupied, so this is narrowed.** CryptoVerif already outputs concrete bounds in terms
     of primitive assumptions and session counts. What is new is not the expressiveness of
     ε-under-A. It is carrying the judgment *as the operational receipt* that treaty parties
     exchange and compose, with a Lean-decided composition and a binding check.
   - That changes the negotiation. Treaty parties can argue about which assumption to buy out,
     using the residual-trust measurement as the price of an assumption. They no longer argue about
     whether "verified" means anything.
   - The word "exactly" in the first draft overclaimed and is withdrawn.

---

## 4. What would move the most claims up a tier (ranked by claims unlocked)

1. **A coverage theorem for an in-path step function** (alignment D2, critique W10). It unlocks
   X-6 at T2-for-the-device, and with it H-4, N-2, N-3, N-4, A-3 and W-4.
2. **NVIDIA GPU evidence in tee-verify** (critique W4). It gives W-1 at T1, M-1 at T1 for GPU
   loaders, and the T1 leg of every inference meet.
3. **The eight-lane forced weld** (critique W7). It moves N-1, N-2, A-4 and Q-2 from a ~31-bit
   tier to the 8-felt tier.
4. **The AuditSampling auditor as an `@[export]`ed Lean predicate** (critique W9). M-2, H-2 and
   H-6 move from T3-on-paper to T3 with a T2\* verifier.
5. **The `Judgment` structure in Lean, with the §0 composition rule as a theorem.** It gives X-8
   and makes this document checkable.

## 5. Falsifiers for this lattice

- If a regulator's actual question cannot be written as a `Pred` over a committed subject and a
  window (for example "is this model dangerous"), the lattice says nothing about it. That is
  correct, but it bounds the programme's reach: capability judgements stay T0 or T3-with-tolerance.
- If coverage (X-6) cannot be made checkable at any tier above T4 for any realistic facility,
  claims 1 and N-* collapse to "Count = 0 in the part of the world we saw". The settling experiment
  is the D2 step-function theorem plus one installation audit protocol.
- If the union-bound composition proves too loose in practice (eps adds up past usefulness over
  millions of requests), the X-8 receipt needs a sequential or martingale composition instead.
  `AuditSampling.sequential_bound` is the shape to reuse. A related warning: composing a receipt
  with itself doubles ε. My first repair, deduplicating by evidence identity, is *unsound* (Astra
  audit G09): identical bytes can support different predicates whose false-accept events are
  disjoint. Deduplicate only exact repeated judgments (same experiment, resources, predicate,
  subject, window and assumptions), or uses proved to imply one shared primitive failure event,
  recorded in a derivation DAG.
