# Cross-model synthesis: governance claim lattice and silicon-to-semantics map (2026-11-05)

Two rounds with `codex exec -c model="gpt-6-astra" --sandbox read-only`, run from
`/Users/ember/dev/degg-research`. Codex read the cited files itself. Below: what was asked, what it
said in compressed form, and my adjudication. "Accepted" means the target document was changed.
The full Codex transcripts are in the session scratchpad and not committed; the substance is here.

Targets: `research/GOVERNANCE-CLAIM-LATTICE.md`, `research/governance-foundations-critique.md`,
`redregg/designs/SILICON-TO-SEMANTICS-ALIGNMENT-2026-11-05.md`.

---

## Round 1: attack F1-F11 and D1-D7, propose six constructions

**Prompt (summary).** A brief listing my eleven code findings (F1-F11), seven draft design points
(D1-D7) and the MMA counts experiment. Codex was asked to attack each, name occupants with years,
give the theorem that would make each a construction, attack the claim lattice, and propose six
constructions we had not listed.

### Code findings: corrections Codex made, all accepted

| Finding | Codex | Adjudication |
|---|---|---|
| F1 silicon kernel is a comparator | Overstated. `real_silicon_kernel_node_check_sound` (line 696) runs lowered four-port RTL that does node inference and checking, on fixtures, with host encoding | **Accepted**, critique W1 rewritten: the declaration gate is a comparator, node checking is in RTL on fixtures, neither is universal. Occupant: Kami (2017) |
| F2 Ozaki over Int | Correct, but the file has interval machinery. What is missing is its connection to these operations. Prior art for FP-exact finite-field arithmetic proofs: cs/0703026 (2007) | **Accepted**; cited in alignment (a) |
| F3 Orb responder | **My decoder summary was wrong**: prefixes G, H, O, PO are recognised, and 200/405 are statuses with lengths 164/227. Byte equality does not link Lean Pancake semantics to HOL4 or to the binary | **Accepted**, W3 corrected. This was a real error in my reading |
| F4 GPU evidence ignored | Shows the *client* skips GPU verification. It does not show the service lacks CC execution, since pinned CPU code could verify the GPU internally | **Accepted as a refinement.** The claim is now "we have no evidence", not "it is not confidential" |
| F5 residual trust | **377 includes the honest run.** It is a bounded corpus, not a probability | **Accepted**: 376 accepted lies; W5 rewritten with three arms |
| F6 seL4 PD | "No external proof is ever verified" exceeds the README; seL4 isolation does not prove the verifier correct | Partly accepted. I keep "no evidence of an external proof verified in a PD" and add the verifier-correctness point |
| F7 weld | 2^15.5 is generic collision; second preimage against an honest root is ~2^31 | **Accepted**, W7 |
| F8 assurance census | Import counts are not assurance percentages; proofs erase | **Accepted**; W9 retitled. The surviving point: AuditSampling's *subject* (an auditor) does not exist as code |
| F9 AIR census | Witness generators are not a soundness hole; count unchecked semantic boundaries, not syntax | Accepted as a caution. I had already said witnesses cost completeness only. The census stays as a cheap tripwire, labelled syntactic |
| F10 completeness | Correct. Occupant: PeerReview (2007) for accountable execution. EngineSound is a legitimate reduction | Accepted |
| F11 admission | Verified predicate, weak authorization system: no authenticated grant, clock or counter, no concurrency or revocation. Occupant: macaroons (2014) | **Accepted**, W11 extended. This became construction (j)-2 (consumable leases) |

### Design points: verdicts

- **D1 (one MMA datapath) is occupied by TensorZKP.** 81/16 is a schoolbook count, and
  accumulator lengths are model bounds, not measurements. Accepted: (a) is now SKU-conditional,
  with the accumulator as a premise in `assumes`.
- **D2 (receipt device): dense positions do not prove coverage.** You can drop a packet before
  numbering it, headers do not bind payload, power loss breaks the statement, and "every arriving
  frame" is impossible. Accepted in full: (b) is now `commit_before_forward` over forwarded frames,
  with payload binding, overflow refusal and reboot continuity. Occupants: TrInc (2009), VRASED
  (2019).
- **D3 (attestation and proof in one MMR): co-location makes no conjunction.** Accepted in full.
  This is the most important attack of the round. (c) now joins through one typed `Stmt`, and
  lattice §0 requires it for any meet.
- **D4 (one Lean function, three outputs): three identically wrong implementations.** Use
  refinement maps, or translation validation (Necula, 2000). Accepted: (d) is refinement to `f`,
  with translation validation as the default for vendor artifacts.
- **D5 (ML-KEM sessions): KEM is not AKE, not transcript completeness, not PQ attestation.**
  Occupant: RA-TLS (2018). Accepted.
- **D6 (joint modulus): neither necessary nor sufficient.** Occupants: F1 (2021), Trinity (2024).
  Accepted, and it agrees with our own HPU seam study. (f) is demoted to "sampled tap prices q".
- **D7 (lattice): "a taxonomy, not a lattice".** Meets need definitions, conflicting assumptions
  cannot be merged silently, and RATS (RFC 9334) already separates evidence from appraisal.
  Partly accepted. The order was already defined. I added the consistency condition, the
  shared-`Stmt` binding, and the no-quantifier-upgrade rule.

### Missing claims Codex named, all added to lattice §1.6a

- **Occurrence vs validity** (a valid trajectory proof is not evidence it happened).
- Quantity and uniqueness (once-only billing, retries, padding).
- Training lineage including optimizer moments and smuggled initial weights.
- Inference identity beyond weights (tokenizer, system prompt, RAG, KV cache, speculative
  decoding).
- Confidentiality lifecycle.
- Service behaviour: availability is not fail-closed.
- The hypergeometric miss formula for T3 and its failure modes.

The occurrence/validity split was the most valuable single addition. It is what makes the "why
both TEE and proof" theorem in (c) non-trivial: the TEE leg supplies occurrence, which no proof
can.

### Six constructions Codex proposed, adjudicated in alignment §(j)

1. Proof-checked accelerator cutpoints (nearest: Slalom, 2019). Adopted.
2. Device-bound consumable execution leases (nearest: TrInc and macaroons). Adopted.
3. Per-artifact translation certificates (nearest: Necula, 2000). Adopted.
4. Optimizer-state lineage capsules (nearest: Nova and Peigné et al.). Adopted.
5. Proof-gated FHE decryption with a persistent DP budget (nearest: PINQ, 2009). Adopted as a
   separate track.
6. Leakage-preserving accelerator sharing (nearest: SecVerilog, 2015). Adopted as the strongest
   argument against (a) for multi-tenant use.

None is a novelty claim. Codex said so itself.

### What Codex did not catch, or got wrong

- On F8 it argued that a theorem need not be imported to certify executed code. True, but it did
  not check whether AuditSampling's subject is executed. It is not. The weakness survives in a
  narrower form.
- It accepted my W4 TD15 claim (`MR_SERVICETD` dropped) without re-checking. I verified it myself
  against dcap-qvl 0.5.2 `quote.rs:466-584` and `tdx.rs:432`.
- It marked the F9 census CHAT-REPORTED. It is now a committed script with output
  (`experiments/air-census/`).

---

## Round 2: attack the proposed theorems, the three "nobody else" claims, and the order

**Prompt (summary).** Codex read the revised alignment map and lattice, which already included
the round-1 corrections. It was asked four things:
1. classify each of the twelve proposed theorems as vacuous, false, trivial or missing a
   hypothesis, and correct it;
2. name anyone who already expresses lattice §3's three claims;
3. check whether §0 is actually a lattice;
4. propose four more alignments that reduce the number of trusted parties.

### Theorems: eleven of twelve needed repair

| Theorem | Verdict | Fix (now in the alignment doc) |
|---|---|---|
| `ozaki_exact_under_acc` | missing hypotheses: rounding, alignment, scale, exponent range, init, schedule; `zip` truncates | full accumulator semantics; every intermediate exact |
| `field_dot_via_mma` | missing: canonical reps, reconstruction, recombination width, proved reduction | listed; primality irrelevant to the identity |
| `commit_before_forward` | **false**: final VJM root, shared witnesses for repeated frames, arbitrary initial state | per-occurrence distinct earlier durable commit with admission-time root, from a valid initial state |
| `leaf_binds` | trivial (a definition) | injective length-delimited domain-separated encoding with job id, collision reduction |
| `overflow_refuses` | σ′ unbound; satisfied vacuously by always-refuse | refusal semantics plus a liveness companion |
| `reboot_continuity` | trivial or false: an old snapshot reproduces its own root | non-rollbackable epoch anchor; older snapshots refuse |
| residue intersection | definitional over one world space, false for projections | the content is the per-leg interpretation theorems |
| `air_denotes` | **false** for full traces (wrong intermediates) | ∃ auxiliary trace ↔ correct output, plus per-trace soundness, over supported syntax |
| `rtl_refines` | compile may fail; combinational ≠ clocked | `= some m` premise; sequential refinement under explicit reset/handshake |
| `transcript_agree` | **false** as a service claim: records ≠ execution ≠ delivery | split into record agreement, endpoint execution and delivery, and a terminal obligation |
| `q_of_tap` | **wrong direction**: a ceiling cannot give a floor | feasibility `p·B_tap ≤ B_engine`, plus an assurance floor under a service guarantee |
| `meet_sound` | acceptance cannot imply Holds when ε > 0 | probabilistic composition with a binding leg; dedupe evidence |

Adjudication: all accepted. Four of my twelve sketches were false and two were trivial. That is
the strongest argument in this whole exercise for the PROGRAM's L1 gate: an adversarial statement
audit before anything is called a construction. My own drafts would have failed it.

### The lattice was not a lattice

Codex's argument:
- Antisymmetry fails on equivalent ASTs and on the ignored tier.
- Conjunction with summed ε is neither the meet nor an upper bound.
- Self-composition doubles ε.
- For the product order I actually wrote, the formal meet is `(P₁∨P₂, A₁∪A₂, max ε)`, which is not
  what I meant.

Its proposed repair: a Boolean lattice of predicates (quotiented by equivalence, ordered by
implication), plus evidence judgments `A ⊢ V : P [ε]` with a composition rule. Tier becomes
provenance, not order.

**Accepted and implemented** as the new lattice §0. Two further corrections, also accepted:
- "Operator controls host" vs "operator follows protocol" is capability vs behaviour, not a
  contradiction.
- Consistency of an assumption set need not be decidable, so the receipt language must use a
  restricted fragment.

### The three "nobody else" claims, narrowed

1. "Zero undeclared jobs" is really "zero undeclared admissions". Admission coverage is not
   computation coverage, and pre-existing contexts are invisible. Nearest: APEX (USENIX Sec'20).
   **Accepted**: the claim is renamed, and an initial inventory and H-1 per admitted job are
   added.
2. "Evaluated = served, eval items not in corpus" lacked the training-lineage leg. ZkAudit
   (2404.04500) already binds weights and data through training proofs. A flat hash is openable by
   preimage witness, so Merkle structure buys efficiency, not expressibility. **Accepted**: the
   claim is narrowed to the join with served requests, TEE occurrence and structure-aware absence.
   Alignment (l) becomes its data-read leg.
3. "ε under exactly A". CryptoVerif already outputs concrete bounds under named assumptions, and
   "exactly" overclaims minimality. **Accepted**: "exactly" is withdrawn. What remains new is the
   judgment as the operational, composable receipt.

My own judgment after narrowing: claim 1 (counted admissions at an attested choke point) is the
strongest survivor. Claim 2 survives only as an engineering join of occupied parts. Claim 3 is a
format proposal, not an expressive novelty. I am keeping all three in the lattice with these
labels rather than deleting them, because the narrowed forms are still the ones Lucid's and Plan
A's schemas cannot state.

### Four new alignments (alignment doc (k)-(n))

- **Retirement counters (RVFI):** accountable compute units.
- **Authenticated memory reads (AEGIS):** training-data provenance at the operand boundary.
- **Memory-controller key epochs (NIST 800-88r2 cryptographic erase):** bounded retention.
- **PHY distance bounding (UWB, WiSec'15):** proximity of an enrolled key.

All four are adopted as directions, each with a laptop-scale model-checking experiment. (l) is the
most valuable, because it repairs lattice claim 2.

---

## What Codex contributed, net

1. **The binding requirement** (co-location is not conjunction). It is now part of the definition
   of evidence composition.
2. **The occurrence/validity split.** It is what makes "why both TEE and proof" a theorem.
3. **The lattice repair**: semantic lattice plus judgments.
4. **Eleven theorem repairs**, four of them of false statements.
5. **Two factual corrections to my code reading**: the Orb decoder prefixes, and 377 including the
   honest run.
6. **Ten constructions** with nearest prior art: six in round 1, four in round 2.

What it did not supply, and could not: measurements. Every number in these notes is either from
our code (grep/count, the counts experiment) or SOURCED from the lit helper's reading of primary
sources.
