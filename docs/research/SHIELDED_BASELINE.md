# The Shielded Single-Executor Baseline

Status: VERIFIED implementation of a deterministic offline model at exactly the
bounds recorded in section 10; PROPOSED as rung 2 of the honest prototype
ladder in [`../VERDICTS.md`](../VERDICTS.md) V3. The implementation is
[`experiments/shielded-baseline`](../../experiments/shielded-baseline/).

**The named executor sees everything.** It holds the sealing key material,
opens every admitted order payload, and learns every owner's side, limit,
quantity, reservation, nullifier, and local output. It could publish the entire
book at any moment and nothing in this model would notice or prevent it. That
is exactly what [`../PRIVACY_MODES.md`](../PRIVACY_MODES.md) section 2 means by
Shielded, it is the first sentence of this document because it is the load
bearing one, and **no privacy, confidentiality, noninterference, leakage, or
anonymity property is claimed, measured, or approximated anywhere in this
packet.**

What the packet studies is the other half of the trust. Shielded gives the
executor confidentiality by definition; it does not have to give away
correctness and inclusion as well. Every mechanism below exists to move one
obligation from "the executor asserts it" to "an object a third party or an
affected owner can check", and section 6 states precisely, and with a
measurement, what is left over.

Model identifier: `degg-shielded-baseline/v0`.

## 1. Why this packet exists

[`../../CLAUDE_HANDOFF.md`](../../CLAUDE_HANDOFF.md) section 6 P1.4 asks for a
Shielded baseline with an explicit executor, an input-validity mechanism, a
preprocessing and corruption model, abort and recovery, and local-output
delivery. [`../VERDICTS.md`](../VERDICTS.md) V3 places it as rung 2: "Shielded
fixed-size batch with a declared executor and sealed receipts", above the Clear
oracle and below a threshold committee.

It is also the first packet whose whole job is composition. Three artifacts
landed separately and each names the hole the others fill:

- [`DARK_FBA_RELATION.md`](DARK_FBA_RELATION.md) section 3 says the four
  boundary statements "are booleans supplied by its named Shielded executor.
  They are not proofs", and its `R` is "merely preserved into the result".
- [`INCLUSION_AVAILABILITY.md`](INCLUSION_AVAILABILITY.md) section 8 offers the
  substitution that fixes that — `R` becomes a padded cutoff root and the
  canonical slot index becomes the log's `seq` — and says plainly that it has
  "no ledger, no accounts, and no adapter" to attach it to.
- `experiments/relation-ir` makes the relation a data value with a Clear
  evaluator and refuses to lower to anything else, so there is a well-defined
  thing for an executor to run and a well-defined refusal if one pretends
  otherwise.

This packet performs the substitution and reports what the joined system does
and does not establish.

## 2. The composition map

PROPOSED as a design; VERIFIED as the behaviour of the joined implementation at
the bounds of section 10.

Each row is a statement the relation consumes, and the object that now decides
it. "Derived" means the executor evaluates a predicate over real objects rather
than asserting a boolean; it does not mean the executor proves anything to
anyone.

| Consumer | Was | Now decided by | Supplied by | Status |
|---|---|---|---|---|
| accepted-input root `R` | an opaque preserved field | the padded `CutoffRoot` | inclusion | substituted |
| canonical slot index (residual rank, section 7) | the caller's slot order | the log's committed `seq` | inclusion | substituted |
| `admission-log-final` (rule 2) | boolean | an observed `CutoffRoot` inside `cutoff + seal`, else `cutoff-root-withheld` | inclusion | derived |
| `root-binds-slots` (rule 3) | boolean | one verified `InclusionReceipt` per committed position, plus the payload commitment opening at each occupied one | inclusion + seal | derived |
| `no-conflicting-root` (rule 4) | boolean | absence of a verified `EquivocationVerdict` for this log domain | inclusion | derived, detection only |
| `payloads-available` (rule 5) | boolean | availability reports against the domain threshold, silence counting as zero | inclusion | derived, weakest leg |
| `authorized` (rule 14) | boolean | the credential registry, against the record's committed `submitter` | this packet | derived |
| `eligible` (rule 15) | boolean | enrolment of that credential at all | this packet | derived |
| `included-under-root` (rule 16) | boolean | that slot's own inclusion receipt | inclusion | derived |
| `custody-bound` (rule 17) | boolean | the `ReserveLedger` holding at least the plaintext's reservation under that record's nullifier | inclusion | derived, weakest leg |
| refusal class | the implementation's choice | the frozen section 4.1 order, interpreted from the module value | relation-IR | reused |
| typed abort, retry, refund | absent | `BatchMachine` and `ReserveLedger` | inclusion | reused |
| clearing, allocation, conservation | the Clear evaluator | the same Clear evaluator, run by one named process | relation-IR | reused |

Two rows are new objects this packet adds rather than borrows:

- **the computation receipt** binds module digest, cutoff root, assembled-input
  digest, public-outcome digest, a delivery root, and a typed status;
- **the delivery commitment** is a Merkle mountain range — the inclusion
  lane's, reused — over exactly one entry per committed position, so an owner
  can open its own position and check the local effect committed there.

## 3. The role model

[`../PRIVACY_MODES.md`](../PRIVACY_MODES.md) section 2 requires a Shielded
claim to name seven things. VERIFIED as the behaviour of this model:

| Question | Answer |
|---|---|
| Who may learn | exactly one party, the holder of the executor key material |
| Collusion threshold | one. No committee, no threshold, no second party required for anything |
| Technical, procedural, or policy | procedural. The seal is a modelled SHA-256 counter keystream and the role separation is Rust module visibility inside one process |
| After compromise | everything, immediately |
| Past payloads | readable. No forward secrecy, no rotation schedule, no rotation mechanism |
| Who can censor | the executor, which is also the admission-log holder here, and can refuse admission, refuse to seal, and refuse to compute |
| Rotation and recovery | absent |

The executor being the log holder is a *choice* of this baseline, and the
single strongest reason the promotion path in section 9 starts by splitting
them. It has one visible consequence already: a root equivocation verdict from
the inclusion lane and an outcome equivocation verdict from this one name the
same party.

REJECTED as claims: that the modelled seal is a cipher, that a Rust private
field is an access-control mechanism outside this process, that key
distribution is solved (the seal is symmetric, so every submitter could open
every other submitter's payload), or that the owner check battery is soundness.

**The compile-time half of the role boundary is real and is tested.** A sealed
payload and a sealed local output have private fields and no accessor that
yields the plaintext without a capability, so a public-role function that
reaches for an order field is a compile error rather than a review comment.
Three `compile_fail` doctests pin it. The discipline is
`clutch-accumulator::WindowResult`'s, observed in Dragon's Clutch and freshly
implemented here for different types; nothing crossed between the repositories.

## 4. The lifecycle

```text
submitters --prepare--> sealed payload + admission request + escrow
           --admit-->   AdmissionLog: envelope only, frozen check order
           --seal_padded--> CutoffRoot, leaf count == capacity, constant
           --observe_cutoff--> BatchMachine: Open -> Sealed
           --report_availability--> threshold met, or input-withheld
           --begin_compute--> Computing
executor   --assemble--> BatchInput from the committed positions, in seq order
           --evaluate--> the module's Clear evaluator, section 4.1 order
           --publish--> receipt + delivery commitment + sealed local outputs
           --deliver_result--> Settled, or ResultUnbound
owners     --audit--> findings, or a transferable dispute object
```

Preprocessing is exactly two things and both are named: the executor is
commissioned with an identity and key material before the batch opens, and the
owners are enrolled in a credential registry. There is no trusted setup, no
correlated randomness, no offline phase, and no ceremony, because there is no
cryptography here beyond hashes.

The admission log refuses on envelope shape, timing, uniqueness, and capacity;
the relation refuses on order content. VERIFIED that the two surfaces stay
disjoint in the composed system: an under-reserved order is admitted by the
log, which cannot read a reservation, and refused publicly by the relation as
`reservation-insufficient`.

INFERRED, and recorded because it surprised the author: the envelope's arrival
epoch and the payload's `arrived_at` are two different numbers. The log records
the epoch it observed; the submitter's claim inside the seal is checked only by
the relation, at rule 13. A book that is in time on the envelope and late
inside the seal is admitted and then publicly refused as `late-arrival`. That is
the composition working, but it means "arrival" is two facts and only one of
them is committed before the cutoff.

## 5. Input validity: what the executor cannot do

VERIFIED at the bounds of section 10. Each item has a named test.

1. **It cannot run against a different admitted set.** The receipt binds the
   cutoff root; an owner knows which root its inclusion receipt verifies
   against; `BatchMachine::deliver_result` types a mismatch as
   `result-unbound`, whose consequence is refunding every admitted record.
2. **It cannot compute on a convenient subset.** There is no transition from a
   missing payload to a smaller batch anywhere in the reused machine, and
   silence counts as unavailability.
3. **It cannot drop a committed position from a settled run undetectably.** The
   delivery commitment has one entry per committed position. A no-local-output
   entry at a position whose committed record is provably not the deterministic
   padding record is a contradiction between two published objects, and
   `OmissionProof` reduces it to a content-addressed verdict any third party
   can recheck from the cutoff root, the receipt, the inclusion receipt, and
   the opening. The owner's plaintext is not part of the object.
4. **It cannot inflate an obligation past the escrow.** `custody-bound` is
   derived from the reserve ledger, so a substituted larger reservation is
   publicly refused as `custody-binding-absent`; a substituted smaller one
   fails the relation's own rule 18. Together they bound a substituted
   position's worst-case obligation `quantity * price[limit]` by the amount
   actually escrowed. VERIFIED, and stated exactly because the stronger claim
   is false: this does **not** bound the quantity. Owner 0's buy of 5 at tick 2
   and a buy of 15 at tick 0 cost the same 15 units, so an executor can
   triple a position's quantity inside the same budget by lowering its limit.
   The owner detects that only if the resulting fill exceeds its own quantity;
   otherwise it lands in the hole section 6.2 measures.
5. **It cannot re-attribute an order to another owner.** `authorized` is
   derived from the credential registry against the record's committed
   `submitter`, so the payload's claimed owner and the admitted credential must
   agree or the batch is publicly refused as `unauthorized`.
6. **It cannot give an owner an impossible fill.** The owner checks that the
   fill is at most its own quantity, at most the public volume, zero under a
   no-trade result, zero at a tick its limit does not reach, and inside the
   largest-remainder interval `pro_rata_feasible` derives from the module's
   bounded side totals. It also recomputes the whole slot outcome — deltas and
   released reservations — from its own plaintext, the public price, and the
   committed fill, so the executor's only free parameter in an owner's row is
   the fill itself.
7. **It cannot publish two runs of one cutoff without leaving a verdict.**
   Four conflict classes, with a frozen order among them: public outcome,
   receipt status, delivery commitment, assembled input. The third is the
   economically loaded one — the tick and volume agree and the allocation does
   not, which is exactly the residual-rank manipulation
   `DARK_FBA_RELATION.md` section 11 shows the public result cannot witness.

## 6. What remains executor-trusted: the honest core

VERIFIED as absences of this model, and measured where a measurement exists.

### 6.1 Confidentiality, entirely

Out of model by definition. The executor reads every payload. There is no
threshold, no enclave, no policy, and no audit of what it does with what it
reads. `PRIVACY_MODES.md` section 6's forbidden claims all apply: nothing here
may be described as private, dark, or confidential.

### 6.2 The public clearing tick and the aggregate volume, completely

This is the sharpest finding of the packet, and it is a measurement rather than
an argument.

Over one fixed admitted set — the two-order tie book of the corpus — the suite
enumerates 1,125 alternative published runs: five cleared-tick values, nine
aggregate volumes, and every per-position fill vector bounded by the committed
quantities. It builds each one into a complete published object — delivery
commitment, receipt, per-owner local outputs — and runs the public check and
all four owners' complete batteries against it.

VERIFIED, at exactly those bounds:

- **377 of 1,125** alternative runs are accepted by every check in the model;
- **all 33** well-formed public results are among them, where well-formed means
  a no-trade at volume zero or any tick at any positive volume the book can
  bound.

So the composed verifier constrains the executor's *fill vector* and constrains
the published clearing price and aggregate volume **not at all**. Two tests
exhibit the consequence concretely. Substituting one owner's limit tick moves
the public result from tick 2 volume 5 to tick 0 volume 4 — every participant
trades at a different price — and all four owners' batteries and the public
check accept the run. Substituting that owner's quantity from 5 to 15 while
lowering its limit, which costs the same escrowed 15 units and so passes the
custody and reservation bindings of section 5, changes how much of the book is
willing to trade and is likewise accepted by everyone.

INFERRED, and the reason this matters for the ladder: only a proof of correct
evaluation closes this. A threshold committee does not. Splitting decryption
across `k` of `n` parties divides the *confidentiality* trust and leaves the
correctness trust exactly where section 6.2 finds it, unless the committee also
produces a verifiable statement about the result. Calling a threshold backend
"more trustworthy" without that is the mistake `PRIVACY_MODES.md` section 6
lists as "MPC means decentralized".

### 6.3 The binding from a committed payload to an evaluated witness

The record commits `payload_commitment`; only the owner and the executor can
open it; and an opening published to anyone else would defeat the whole mode.
So there is no object anywhere that says "the witness evaluated at position `k`
is what the commitment at position `k` opens to". The affected owner sees a
substitution only through its effect on its own row, and section 6.2's
measurement is exactly the space of substitutions with no such effect.

This is the single largest gap between the model and a mechanism on the
correctness side, as attribution is on the accountability side.

### 6.4 Refusal classes

A published refusal class is entirely executor-trusted. An owner can ask
whether *its own* order genuinely violates the class — the relation module is
public data, so the owner runs the same Clear lowering on a probe batch holding
only its own order — and that is a real answer about that owner and no answer
at all about the batch. A fabricated per-slot refusal blaming a position the
owner does not hold is indistinguishable from a genuine one, to that owner and
to the public. VERIFIED: on the `under-reserved` corpus scenario the owner at
fault reports `OwnPositionViolates`; the other three report `NotAttributable`.

### 6.5 Non-delivery, censorship, and attribution

Inherited verbatim from `INCLUSION_AVAILABILITY.md` section 9.1 and not
improved here.

- **Nothing signs.** An executor identity is a label and a receipt binding is a
  public function anyone can compute, so every verdict object in this packet
  establishes a contradiction between published objects and never an
  attribution to a party.
- **Non-delivery has no positive object.** An owner that receives no local
  output holds an inclusion receipt and a delivery opening; if the opening
  commits an entry the owner cannot match, the owner has a complaint, not a
  proof, because it cannot prove it was not sent something.
- **Censorship by silence has no positive object.** Only the timeout paths
  respond, and they cannot distinguish a censoring executor from a crashed one.

### 6.6 Availability

Declared integers, exactly as upstream. No erasure coding, no dispersal, no
sampling, no reconstruction, and no corruption model for whoever reports the
share counts.

## 7. Corruption and abort

The corruption profile of this baseline is `static-1-of-1-named`: one named
executor, statically identified before the cutoff, arbitrarily malicious with
respect to correctness and inclusion, and *trusted absolutely* with
confidentiality. Submitters may be arbitrarily malicious and know their own
inputs and outputs. There is no network, so there is no liveness statement.

This is deliberately not `DARK_FBA_RELATION.md` section 9's
`static-active-1-of-4` target, and nothing here is evidence about that target.

VERIFIED abort behaviour, all of it the reused `BatchMachine`:

| Event | Class | Retryable | Consequence |
|---|---|---|---|
| executor never seals | `cutoff-root-withheld` | no | refund every escrowed submission |
| a payload is short of threshold | `input-withheld` | no | refund every admitted record |
| executor crashes mid-compute | `compute-timeout` | **yes** | retry against the same cutoff root |
| retries spent | `compute-exhausted` | no | refund every admitted record |
| result bound to another root | `result-unbound` | no | refund every admitted record |
| verified root equivocation | `equivocation` | no | refund under either repudiated root, once per nullifier |

VERIFIED refund conservation on the crash path of the corpus scenario: 29 units
escrowed across four positions, 29 refunded, zero outstanding, the ledger's
invariant holding, a padding position refunding nothing as `NotEscrowed`, and a
second claim on one nullifier returning `AlreadyRefunded`.

### 7.1 Composition gap C-1

A finding, recorded as a test rather than as a caveat.

The inclusion lane's abort taxonomy has no class for *the relation refused the
admitted batch*. `deliver_result` maps any result delivered inside its deadline
and bound to the right root to `Settled`, regardless of whether the relation
settled or published a typed refusal. So a batch that is publicly refused
reaches a settled phase, `claim_refund` answers `PhaseNotRefundable`, and the
reserved funds have no path back except through a settlement relation that does
not exist.

This packet does not patch the upstream lane. Closing it needs a
`relation-refused { class }` abort class upstream whose consequence is
`RefundEveryAdmittedRecord`, which is a one-row addition to the abort matrix
and a change to that lane's frozen corpus.

## 8. Falsifier ledger

Against the threat cases in
[`DARK_RELATION_THREAT_MODEL.md`](DARK_RELATION_THREAT_MODEL.md):

| Falsifier | Disposition |
|---|---|
| A public exact submission tick identifies an order | **Partial**, unchanged from upstream. Epoch granularity only; the committed `seq` is still a rank a published receipt discloses. |
| An unpadded number of messages reveals participation | **Partial**, unchanged. Padded sealing makes the leaf count constant, and the delivery commitment inherits that leaf count, so it adds no occupancy channel. Padding records remain recognisable. |
| Variable ciphertext or proof sizes reveal quantity | **Addressed for this model**: one fixed 64-byte wire shape, enforced at admission and produced by an injective zero-padded encoding whose padding must be exactly zero. There are no proofs. |
| A root published to some observers but not others | **Detection only**, unchanged upstream; this packet adds the same shape for outcomes, with four conflict classes. Not prevention, not attribution. |
| An unavailable payload becomes an empty slot | **Addressed**, unchanged: no subset transition exists. |
| Retries or abort timing reveal whether a batch crossed | **Open**, unchanged. Abort class, attempt count, and phase timing are public and unanalysed as a channel. |
| Local-fill delivery leaks a user's order through a public notification | **Addressed as far as this model goes.** Local outputs are delivered sealed and never published; the public transcript carries only digests and a delivery root. VERIFIED by a byte scan of the published commitment material. The delivery *vector length* is the constant capacity. |
| The audit plane silently decrypts the book | Out of scope: there is no audit plane. The executor already reads everything, which is worse and is stated. |
| Public settlement reconstructs the participant graph | Out of scope: settlement is a separate relation and does not exist. |

Against `DARK_FBA_RELATION.md` section 10's obligations:

- "accepted slots bind to `R` and the frozen admission log" — **met at the
  commitment level** for envelopes, and now also for the *witness assembly*,
  which walks the committed positions in order; not met for the binding of
  section 6.3.
- "every local output commitment is bound to the same result" — **met**: the
  delivery root is inside the receipt binding, alongside the outcome digest and
  the cutoff root.
- "no invalid output can settle" — **not met**. Section 6.2 measures how far
  from met.
- "inclusion, equivocation, withholding, timeout, retry, and abort have
  distinct receipts" — **met**, by reuse, plus four outcome-conflict classes
  and an omission verdict added here.
- "a computation party cannot learn a partial result and silently substitute,
  censor, or restart the batch on more favourable inputs" — **partially met**.
  Substitution of a reservation or an owner is refused; substitution that moves
  the public price is not detected; restart on a rewritten history is
  `acknowledged-prefix-abandoned` upstream; censorship by silence is not.

## 9. Promotion path

PROPOSED, in dependency order. Each rung is named for what it actually buys.

1. **Split the log holder from the executor.** Cheapest structural change with
   a real effect: the party that sequences cannot read, and the party that
   reads cannot reorder. It does not close section 6.2.
2. **Attribution.** Signatures, a consensus record, or a bonded commitment, so
   that every verdict object in this packet and in the inclusion lane names a
   party instead of a contradiction. This is a cryptographic dependency and is
   the precondition for any economic consequence.
3. **A `relation-refused` abort class upstream.** Closes composition gap C-1.
4. **Threshold decryption to a named committee** — the `ShieldedCommittee`
   lowering target `experiments/relation-ir` currently refuses. This divides
   the confidentiality trust of section 6.1 across `k` of `n` and, on its own,
   changes nothing in section 6.2. Saying so is the point of putting it fourth.
5. **A verifiable statement about the evaluation.** The only thing that closes
   section 6.2, and therefore the only thing that turns a Shielded venue into
   one whose *result* is not trusted. Proof-carrying Clear is the cheapest
   candidate because the relation is already a module value with a canonical
   encoding and a bounded type vocabulary.
6. **Real availability.** Erasure coding, dispersal, sampling, and a corruption
   model for the reporting committee, replacing declared integers.
7. **Settlement, as a separate relation.** Private computation followed by
   public account-and-amount transfers is pre-trade dark at most.

Rungs 4 and 5 are independent, and this packet's contribution to the ladder is
mostly the evidence that they are: a committee is a confidentiality mechanism
and a proof is a correctness mechanism, and a design that ships the first and
calls the result trustworthy has not moved section 6.2 at all.

## 10. Reproduction and bounds

From the repository root:

```sh
cargo test --offline --locked \
  --manifest-path experiments/shielded-baseline/Cargo.toml
cargo clippy --offline --locked --all-targets \
  --manifest-path experiments/shielded-baseline/Cargo.toml -- -D warnings
cargo fmt --check \
  --manifest-path experiments/shielded-baseline/Cargo.toml
cargo run --quiet --offline --locked \
  --manifest-path experiments/shielded-baseline/Cargo.toml \
  --bin degg-shielded-vectors \
  | cmp - experiments/shielded-baseline/vectors/v1.txt
cargo run --quiet --offline --locked --release \
  --manifest-path experiments/shielded-baseline/Cargo.toml \
  --bin degg-shielded-differ
```

VERIFIED on 2026-08-18: 51 tests pass — 5 honest-run, 13 detection, 6
owner-finding, 6 equivocation, 6 abort, 7 visibility, 1 residual-trust, 1
differential, 2 corpus, and 4 doctests of which three are `compile_fail` role
boundaries. Clippy with `-D warnings` and `cargo fmt --check` are clean. Zero
third-party dependencies; the only dependencies are path dependencies on the
two landed experiments, both of which are unmodified and whose own suites (125
and 21 tests) and byte corpora still pass and reproduce.

Differential result, VERIFIED at exactly these bounds:

| Domain | Content | Cases | Divergences |
|---|---|---:|---:|
| A | four slots, each vacant or a (side, tick, quantity `= 1`) triple; `9^4` | 6,561 | 0 |
| B | the same with quantity in `1..=2`; `17^4` | 83,521 | 0 |
| | Total | 90,082 | 0 |

The compared quantities are the assembled slot vector, the public outcome, the
per-slot fills, every owner-local output, and the refusal class.

Bounds of the VERIFIED label, stated exactly:

- both sides of the differential share the Clear evaluator by construction, so
  the differential is evidence about the composed **assembly path** and none at
  all about the relation's clearing or allocation semantics. Those were
  compared over 2,116,916 cases by `experiments/relation-ir` and 300,436,169 by
  the two-oracle run of `DARK_FBA_RELATION.md` section 13.6;
- the residual-trust measurement is over one admitted set, with reservations,
  sides, limits, and the admitted set itself fixed at their honest values. It
  measures the executor's freedom in the *result*, not in the witness;
- the pro-rata feasibility check is proved sound by exhaustion over the
  module's whole bounded domain — every quantity, every volume, every
  admissible side total — and not beyond it;
- every other test is a fixture or a small enumeration, named in the
  experiment's README;
- everything outside those bounds is untested. Larger relations, concurrent
  executors, adversarial schedules, real availability, real encryption, and any
  cryptographic property are covered by no statement in this document.

Corpus byte identity: `experiments/shielded-baseline/vectors/v1.txt` SHA-256
`627e30fcff2a8696d29a649be923bfa84571892433a27f8c5fab34deb8f2b0e9`.

Validation toolchain: `rustc 1.98.0-nightly (91fe22da8 2026-06-21)`,
`cargo 1.98.0-nightly (a595d0da2 2026-06-20)`.

## 11. Provenance

Original work in this repository, with no third-party dependencies. SHA-256,
the tagged hash, the Merkle mountain range, the admission log, the abort
machine, and the reserve ledger are consumed from
[`experiments/inclusion-availability`](../../experiments/inclusion-availability/)
by path dependency rather than reimplemented; the relation module, its
canonical encoding, and its Clear evaluator are consumed from
[`experiments/relation-ir`](../../experiments/relation-ir/) the same way.
Neither upstream experiment was modified.

The role-capability discipline — a value constructible only through one path,
with a `compile_fail` doctest showing that the wrong value cannot be
substituted for it — is an idea observed in Dragon's Clutch's
`crates/clutch-accumulator/src/window.rs`, read on 2026-08-18, and freshly
implemented here for different types in a different repository. No code,
fixture, constant, identifier scheme, or serialization format crossed between
the repositories. Because the author has read related implementations in
sibling repositories, this document does not claim clean-room status.
