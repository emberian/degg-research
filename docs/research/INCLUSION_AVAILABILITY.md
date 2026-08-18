# Inclusion, Availability, Non-Equivocation, and Typed Abort

Status: VERIFIED implementation of a deterministic offline model at exactly the
bounds recorded in section 10; PROPOSED as the mechanism behind the
accepted-input root of `dark-fba/n4-k4-q15/v0`. No privacy, cryptographic
security, network, data-availability, consensus, or economic property is
claimed here, and none is measured. The implementation is
[`experiments/inclusion-availability`](../../experiments/inclusion-availability/).

Model identifier: `degg-inclusion-availability/v0`.

## 1. Why this packet exists

[`CLAUDE_HANDOFF.md`](../../CLAUDE_HANDOFF.md) §2 places one line between the
relation backends and the public receipt:

```text
input inclusion + availability + non-equivocation + typed abort
```

[`../ARCHITECTURE.md`](../ARCHITECTURE.md) §6 states the obligation plainly:
darkness cannot excuse hidden omission, and a correct proof over an
operator-selected subset is not a fair market.

[`DARK_FBA_RELATION.md`](DARK_FBA_RELATION.md) §5 then names four conditions a
valid batch requires — a final admission log, witness slots that open exactly to
the accepted-input root `R`, no conflicting finalized root, and every admitted
payload recoverable by the declared availability threshold — and implements none
of them. Its §3 is explicit: the four boundary statements are booleans supplied
by the named executor, `R` is preserved verbatim into the result, and the toy
"neither hashes the slots nor verifies a cryptographic opening".

REJECTED, restated: an opaque 32-byte field named `R`, a Boolean
`availability_ok`, and a refusal class named `RootEquivocation` are not
inclusion, availability, or non-equivocation. They are placeholders whose
failure modes are invisible.

This packet replaces each placeholder with an object a third party can check
against public data, and states precisely which obligations still have no
object.

## 2. The commitment

VERIFIED. The admission log is committed by a Merkle mountain range implemented
in `src/mmr.rs` from its public description, with no external crate. Leaves are
appended one at a time; after `n` leaves the structure is one perfect binary
tree per set bit of `n`, with strictly decreasing heights, covering disjoint
consecutive leaf ranges. Those roots are the peaks. Appending merges equal-height
peaks and never rewrites an existing node, which is the mechanical content of
"append-only" and is asserted directly: `appending_never_rewrites_an_existing_node`
records every node of a 64-leaf construction and rechecks all of them after
every subsequent append.

Hashing is domain-separated throughout. `tagged(tag, parts)` computes
`SHA-256(t || t || parts)` for `t = SHA-256(tag)`, so a leaf preimage cannot be
reread as an interior node, a bag step, a root, a record, a padding record, a
holder statement, or an equivocation verdict. SHA-256 is implemented from
FIPS 180-4 and checked against the published vectors, including the one-million
character case, before anything is built on it.

The root is

```text
root = tagged(ROOT_TAG, domain_digest || leaf_count_be64 || bag(peaks))
bag  = fold of tagged(BAG_TAG, peak || accumulator), right to left
```

Two bindings inside that formula do real work.

- `leaf_count` is inside the root, so a root commits to the **exact number** of
  admitted records, not only to their hashes. A verifier who is told "this is
  the cutoff root over `n` records" and handed a proof implying some other count
  gets `LeafCountMismatch`, not a plausible-looking acceptance.
- `domain_digest` is inside the root, and covers the relation identifier, batch,
  market, cutoff epoch, capacity, payload length, availability share count, and
  availability threshold. A root produced for one batch cannot be replayed as a
  root for another, and changing any frozen shape field moves the commitment
  rather than reinterpreting it.

The domain digest is *also* inside every leaf. That is not redundant: the root's
binding stops a root being replayed, and the leaf's binding stops a *record*
being lifted into another log. Without the second, two batches that happened to
admit byte-identical records in the same order would honour each other's
receipts. That was found by a test, not by inspection, and the test survives as
`a_receipt_does_not_verify_against_another_batchs_root`.

VERIFIED at exactly these bounds: for every leaf count `0..=256` the peak heights
are the set bits of the count and span it exactly; for every log of `1..=33`
leaves, every leaf proves at its own index and no leaf verifies under another
leaf's proof; for every log of `0..=17` leaves and every prefix of it, the
consistency proof verifies and recovers exactly the root the log had at that
size.

## 3. What is admitted

An admission record is an **envelope**, not an order. The Dark target requires
the log holder to sequence and commit inputs it cannot read, so the record
carries only:

| Field | Width | Meaning |
|---|---:|---|
| `seq` | 4 | canonical position, assigned by the holder |
| `submitter` | 32 | admission-credential commitment, not an identity |
| `payload_commitment` | 32 | commitment to the encrypted order |
| `payload_len` | 4 | must equal the domain's single admissible wire shape |
| `availability_shares` | 1 | must equal the domain's declared share count |
| `arrival_epoch` | 8 | epoch granularity only |
| `nullifier` | 32 | nonzero, batch-scoped, pairwise distinct |

Side, limit, quantity, reservation, and owner are inside the payload and never
appear. This is a modelling decision with teeth: the log cannot price, filter,
or reorder on order content, because it does not have it.

`seq` is the residual-allocation rank of `DARK_FBA_RELATION.md` §5. Committing
it is what makes §11's finding — that swapping equal-remainder orders preserves
public price and volume but changes who receives the residual atom — a property
a later builder cannot manipulate.

The canonical preimage is 113 fixed-width big-endian bytes in a fixed order, so
it is injective with no length prefix, and the leaf is
`tagged(LEAF_TAG, tagged(RECORD_TAG, domain_digest || preimage))`.

### 3.1 A frozen check order, on purpose

`DARK_FBA_RELATION.md` §13.3 recorded a real specification gap: two conforming
implementations of the clearing relation publicly disagreed about which refusal
class a multiply-invalid witness reports, in 11,587 enumerated cases, because
the specification fixed no check order. A published typed refusal class is an
observable, so two conforming implementations could disagree in public.

This model does not inherit that gap. The admission check order is frozen,
documented on the function, and pinned by tests:

1. `LogSealed`
2. `CutoffPassed` — holder clock past the cutoff
3. `LateArrival` — arrival after the cutoff
4. `ArrivalInFuture` — arrival after the holder clock
5. `NonCanonicalPayloadSize`
6. `AvailabilitySharesMismatch`
7. `NullifierZero`
8. `NullifierReservedForPadding`
9. `NullifierRepeated`
10. `CapacityExhausted`

`the_frozen_check_order_holds_under_simultaneous_faults` injects every fault from
each position onward at once and asserts the earliest one is reported.

INFERRED, and load-bearing: rules 3 and 4 must appear in that order. The holder
clock never passes the cutoff without rule 2 firing, so under the opposite order
a late arrival could only ever be reported as a future arrival and `LateArrival`
would be a class no witness can produce. `every_admission_refusal_class_is_reachable`
exhibits a witness for all nine classes, which is the property that catches this
kind of dead branch.

### 3.2 Padding, and the occupancy channel

The disclosure budget in
[`DARK_RELATION_THREAT_MODEL.md`](DARK_RELATION_THREAT_MODEL.md) says public
participation is capacity alone: occupied count is excluded. A root that binds
`leaf_count` therefore leaks exactly the thing the budget forbids.

`seal_padded` closes the leaf-count channel. It appends deterministic padding
records, derived from the domain digest and the position alone, up to the
capacity, so every batch of a relation commits to exactly `capacity` leaves.
Determinism matters twice: two holders of the same admitted set produce the same
padded root, and no holder gets a free degree of freedom inside the commitment.

Padding nullifiers are public, so a submitter can compute them; rule 8 above
exists because without it a submitter could claim a position's padding nullifier
and break batch-scoped uniqueness. That is a modelling bug this packet found in
its own first draft, fixed and pinned.

Two consequences are stated rather than papered over.

- VERIFIED: with a power-of-two capacity and padded sealing, every receipt has
  the same shape — path length 2, no peaks, at all four positions of the frozen
  relation. Receipt size therefore carries no information. With capacity 3 the
  path lengths are `[1, 1, 0]`, so a non-power-of-two capacity reintroduces a
  size channel. This is a constraint on the relation, not an implementation
  detail.
- REJECTED as a privacy claim: padding records are *recognisable*. `is_padding`
  decides occupancy from public data, because this model has no hiding payload
  commitment and no unlinkable nullifier. What padding buys here is that the
  leaf count stops being an occupancy channel; the record bytes still are one.
  `padding_is_recognisable_which_is_the_limit_of_this_model` asserts exactly
  that, so the limit is visible in the suite rather than in a caveat.

## 4. Inclusion receipts

An `InclusionReceipt` is `{ record, proof }`. `verify_receipt(cutoff, receipt)`
takes a `CutoffRoot` — domain, leaf count, root — and the receipt, and nothing
else. The log holder is not consulted and cannot participate.

The proof object carries the node height, the authentication path as
`(side, hash)` steps, and the peaks to the left and right of the containing
peak. The verifier **derives** the position: the local index from the path
sides, the mountain start from the left peak spans, and the total leaf count
from all peak spans. A prover therefore cannot choose the position a proof
attests to, and the receipt is refused if the derived index is not the record's
own `seq`.

VERIFIED refusals, each with its own class:

| Attack | Result |
|---|---|
| Root from another batch, market, or cutoff | `Proof(RootMismatch)` |
| Root over the same records in a different order | `Proof(RootMismatch)` |
| Any byte flipped in the published root | `Proof(RootMismatch)` |
| Leaf count altered | `Proof(LeafCountMismatch)` |
| Record moved to another `seq`, all 12 ordered pairs | `Proof(RootMismatch)` |
| Another position's proof grafted on, all 12 ordered pairs | `Proof(RootMismatch)` |
| Any sibling byte flipped, every path step | `Proof(RootMismatch)` |
| Any sibling side flipped, every path step | `Proof(...)` |
| Submitter, payload commitment, arrival, or nullifier altered | `Proof(RootMismatch)` |
| Payload length, share count, late arrival, zero nullifier, position past capacity | `RecordViolatesDomain` |
| Interior-node proof offered as a receipt | `NotALeaf` |
| Cutoff root claiming more leaves than its capacity | `MalformedCutoffRoot` |
| A record appended after the cutoff | `Proof(LeafCountMismatch)` |
| A receipt taken against an earlier running root | `Proof(LeafCountMismatch)` |

The post-cutoff case is worth stating in full, because it is the one an operator
would actually try. A holder seals at five records, then keeps appending. The
five honest receipts still verify against the cutoff root, because the log is
append-only. The late records verify against the *extended* root and against
nothing else. And the extension is provably an extension: the consistency proof
from the extended root at size five recovers the cutoff root exactly. There is
no arrangement of the published objects in which the late records are inside the
cutoff commitment.

## 5. Equivocation

A cutoff root is only useful if there is exactly one per cutoff. A holder that
shows root `A` to one submitter and root `B` to another can honour every receipt
it ever issues while running two different markets.

The proof object is:

```text
EquivocationProof {
  domain:   LogDomain,
  left:     RootStatement { holder, domain_digest, kind, leaf_count, root, binding },
  right:    RootStatement { ... },
  conflict: Roots
          | Sequence { left: InclusionReceipt, right: InclusionReceipt }
          | Position { left: InclusionReceipt, right: InclusionReceipt }
          | Prefix   { consistency: ConsistencyProof },
}
```

`kind` distinguishes a pre-cutoff acknowledgement from a published cutoff root,
and the distinction is what keeps honest behaviour out of the net: two running
roots at different sizes are ordinary, two sealed roots for one cutoff are not.

`verify_equivocation` returns

```text
EquivocationVerdict { holder, domain_digest, class, left_root, right_root, digest }
```

where `digest = tagged(VERDICT_TAG, holder || domain_digest || class_code ||
left.binding || right.binding)`. It is content-addressed, so the same
equivocation always names the same verdict and different equivocations never
collide by construction. That digest is what a slashing rule would consume; this
model does not contain a slashing rule.

The four classes are genuinely different failures.

- **`conflicting-sealed-roots`** — two sealed roots for one cutoff. The bare case.
- **`conflicting-record-at-sequence`** — two receipts, each valid under its own
  root, placing different records at the same canonical position. This is the
  economically loaded one: the position is the residual-allocation rank.
- **`nullifier-at-two-positions`** — one nullifier admitted at two different
  ranks across the two roots.
- **`acknowledged-prefix-abandoned`** — the holder acknowledged a record, then
  sealed a root whose prefix at the acknowledged size is a different root. This
  is a rollback, and it is detected with a consistency proof rather than with an
  accusation.

VERIFIED, and the point of the packet: honest single-root operation never
produces a verdict. For a four-record honest log the suite enumerates all 32
ordered receipt-pair conflict constructions and all five acknowledged prefixes
against the holder's own cutoff root, and every one is refused —
`RootsIdentical` where the two roots coincide, `PrefixAgrees` where the sealed
log genuinely extends the acknowledgement. Growing a log is refused as
`NotSealed`, not misread as equivocation.

All fifteen rejection classes are exercised, including `MalformedStatement` for
a statement rewritten without rebinding, `DifferentHolders` for two holders
merely disagreeing, `RecordsIdentical` and `SamePosition` for pairs that
contradict nothing, and `Consistency(...)` for a tampered prefix proof.

## 6. Withholding, timeout, and refunds

Inclusion is half the obligation. A holder can commit a record, hand out a
valid receipt, and then withhold the payload, stall, or publish a result bound
to some other root, while the submitter's funds sit reserved.

`BatchMachine` is the verifier-side state machine. It never holds the log: it
observes a published cutoff root, availability reports, results, and proofs,
which is what a relying party can actually see.

```text
Open ──observe_cutoff──▶ Sealed ──begin_compute──▶ Computing ──deliver_result──▶ Settled
 │                        │                          │
 │ tick > cutoff+seal     │ tick > cutoff+avail      │ tick > attempt deadline
 ▼                        ▼                          ▼
CutoffRootWithheld    InputWithheld{seq}      ComputeTimeout ⇄ resume
                                                     └─ retries spent ─▶ ComputeExhausted

any live phase ──present_equivocation──▶ Equivocation
Computing ──result bound to another root──▶ ResultUnbound
```

### 6.1 The abort matrix

VERIFIED, exactly this table, asserted directly and reproduced in the corpus:

| Class | Retryable | Terminal | Consequence |
|---|---|---|---|
| `cutoff-root-withheld` | no | yes | refund every escrowed submission |
| `input-withheld` | no | yes | refund every admitted record |
| `compute-timeout` | **yes** | no | retry against the same cutoff root |
| `compute-exhausted` | no | yes | refund every admitted record |
| `equivocation` | no | yes | refund under either repudiated root, once per nullifier |
| `result-unbound` | no | yes | refund every admitted record |

Every class has a reachability witness, and every terminal phase is immovable:
for each one the suite rebuilds the machine and confirms that `observe_cutoff`,
`report_availability`, `begin_compute`, `deliver_result`, and `resume` all
return `Terminal` or `NotResumable`, and that `tick` a thousand epochs later
returns the same phase.

Three rules in that table are the substance rather than the bookkeeping.

**Silence is unavailability.** A position with no availability report counts as
zero recoverable shares. A holder cannot obtain progress by declining to answer.

**There is no subset transition.** `DARK_RELATION_THREAT_MODEL.md` lists "an
unavailable payload is converted to an empty slot rather than a typed abort" as
a falsifier of the surface contract, and `DARK_FBA_RELATION.md` §5 says the
clearing relation never selects a convenient subset. `compute_with_subset`
therefore exists only to refuse, with `SubsetSelectionForbidden`, and there is
no path anywhere in the machine from a missing payload to a smaller batch.

**A timeout is not an abort class shared with a failure.** `compute-timeout` is
retryable and pays nothing; only `compute-exhausted` releases funds. A result
delivered after its attempt deadline is a timeout, not a settlement.

### 6.2 Refund conservation

Escrow is keyed by nullifier and recorded at submission, independently of any
log, so an equivocating holder cannot inflate or deflate what it owes by
choosing which history to publish. Every nullifier resolves exactly once, to a
refund or to settlement.

Entitlement is typed to the consequence, and mismatches are refused:

- `Escrowed { nullifier }` for `cutoff-root-withheld`, where no committed set
  exists to prove inclusion against;
- `Included(receipt)` for the aborts that have one cutoff root;
- `IncludedUnderRepudiatedRoot { statement, receipt }` for `equivocation`, where
  the statement's root must be one the holder was actually caught publishing —
  `RootNotRepudiated` otherwise.

VERIFIED on every terminal path: total refunded equals total escrowed, total
outstanding is zero, and the ledger's conservation invariant holds. On the
settled path total settled equals total escrowed and total refunded is zero. A
second claim on one nullifier is `AlreadyRefunded`; a claim while the batch is
live or after it settles is `PhaseNotRefundable`; a claim behind a tampered
receipt is `Receipt(...)`. Under equivocation, the claimant sweeps both logs and
the union of nullifiers refunds exactly once, with the duplicate claims
returning `AlreadyRefunded`. A padding record is committed, has a verifying
receipt, and refunds nothing: `NotEscrowed`.

Amounts are exact `u64` integers. There is no floating point anywhere in the
crate.

## 7. What this deliberately does not model

Each of these is a named absence, not an approximation. Nothing in the suite
should be read as evidence about any of them.

- **No network.** "Published" means "handed to the verifier as a value". There
  is no gossip, no broadcast, no partition, no adversarial delivery schedule,
  and therefore no liveness result.
- **No clock.** Every epoch is a caller-supplied integer. There is no
  synchrony assumption, no clock skew, no external time domain, and no argument
  that the deadlines in section 6 are long enough for anything.
- **No signatures, and therefore no attribution.** `RootStatement::binding` is a
  *public function of its own contents*: anyone can compute it. It canonicalises
  a statement; it authenticates nothing. Every equivocation result is of the
  form "given two statements genuinely attributed to one holder, here is a
  checkable contradiction". Producing that attribution — a signature scheme, a
  consensus record, a bonded commitment — is entirely outside this crate. This
  is the single largest gap between the model and a mechanism.
- **No consensus.** Nothing makes one cutoff root canonical. The model detects a
  second root; it does not prevent one, order the two, or decide which is real.
- **No data-availability layer.** An availability report is an integer share
  count someone asserts. There is no erasure coding, no dispersal, no sampling,
  no reconstruction, no custody of shares, and no adversary model for the
  committee reporting them. The threshold arithmetic is bookkeeping over
  declared numbers.
- **No encryption.** `payload_commitment` is 32 opaque bytes the crate never
  opens. Nothing here shows that a payload exists, that it decrypts, that it
  decrypts to a well-formed order, or that its commitment is hiding or binding
  in any cryptographic sense.
- **No economics of slashing.** The verdict object is content-addressed so a
  slashing rule could consume it. There is no bond, no stake, no penalty, no
  payout, no griefing analysis, and no argument that any of it is
  incentive-compatible.
- **No custody.** The reserve ledger is a map from nullifier to integer. It is
  not an account model, not a token, not a chain, and it is not bound to any
  external balance. `DARK_FBA_RELATION.md` §4's requirement that reservations
  refer to distinct non-double-counted custody is untouched here.
- **No settlement.** Section 6's settled path releases reservations to a
  relation that does not exist in this repository.
- **No privacy proof.** The occupancy and rank channels of sections 3.2 and 9
  are measured, not closed. Nothing here is a noninterference, indistinguish-
  ability, or simulator argument.
- **No formal proof.** These are deterministic tests over enumerated finite
  domains. A passing suite is a falsification attempt that failed at stated
  bounds, not a theorem. There is no mechanised refinement, no cryptographic
  reduction, and no independent audit.

## 8. Composition with the batch relation

PROPOSED. The join is one substitution: the relation's accepted-input root `R`
becomes this model's padded cutoff root, and the relation's canonical admitted
slot order becomes the log's `seq`.

Concretely, `DARK_FBA_RELATION.md` §5's four conditions map to objects:

| §5 condition | Object | Status |
|---|---|---|
| The admission log is final | `CutoffRoot` plus the `cutoff-root-withheld` deadline | modelled; no consensus makes it canonical |
| Witness slots open exactly to `R` | `InclusionReceipt` and `verify_receipt` | modelled at the commitment level; the payload itself is never opened |
| No conflicting finalized root exists | `EquivocationProof` and `EquivocationVerdict` | modelled as detection; attribution absent |
| Every admitted payload is recoverable | share reports and `input-withheld` | **weakest**; declared integers, no DA layer |

Two further joins follow from the substitution.

- The residual-allocation rank of §7 is the record's committed `seq`. A builder
  that reorders equal remainders after the fact must move the cutoff root, and a
  builder that reorders them before it while acknowledging otherwise is caught
  by `acknowledged-prefix-abandoned`.
- The relation's admission refusals and this model's are disjoint and stay that
  way: this log refuses on envelope shape, timing, uniqueness, and capacity,
  never on order content, because it cannot read order content.

### 8.1 The Dragon's Clutch parallel

INFERRED from source inspected on 2026-08-18. Dragon's Clutch is a different
repository with a different purpose — greenfield transparent Solana protocol
implementation — and nothing crosses between them. It is worth naming because it
has the same seam open, at a different point of its stack.

Its host relation model, `crates/clutch-batch/src/relation_v1.rs`, carries
`pub order_set_id: u64` with the comment "Order-set identity. Still caller
supplied in the host model", and its design document records the matching gate:
"`order_set_id` is still caller-supplied in the host model; deriving it from
canonical order bytes is an adapter-boundary gate this design inherits but does
not close". Its layout crate, `programs/solana-layout/src/lib.rs`, does have
`canonical_order_set_id(market, epoch, page_count, set_order_count,
page_digests)`, a SHA-256 fold over per-page digests that a page cannot be added
to, dropped from, reordered in, or mutated without changing.

So the two repositories have the same shape of problem and complementary halves
of an answer. Dragon's Clutch has a canonical set commitment in its layout but a
caller-supplied identifier in the relation it actually clears with; this model
has the commitment wired through admission, receipts, equivocation, and refunds,
but no ledger, no accounts, and no adapter. Neither is the other's solution, and
no code, fixture, or constant is shared. What transfers is the observation that
an order-set identifier which the caller supplies is exactly the placeholder
this packet was written to remove, and that a page-digest fold is a set
commitment without an *incremental* one: it has no running root to acknowledge,
so it supports inclusion but not the rollback detection of section 5.

## 9. Falsifier ledger

Against the threat cases listed in `DARK_RELATION_THREAT_MODEL.md`:

| Falsifier | Disposition |
|---|---|
| A public exact submission tick identifies an order | **Partial.** Records commit an arrival *epoch*, never an exact tick. But `seq` is committed and a receipt discloses it; see the open item below. |
| An unpadded number of messages reveals participation | **Partial.** `seal_padded` makes the leaf count a constant, so the root discloses no occupancy. The record bytes still do, because padding is recognisable without a hiding commitment. |
| Variable ciphertext or proof sizes reveal quantity or branch behaviour | **Partial.** Admission enforces one payload length. Receipt shape is constant under a padded power-of-two capacity, and only then. Ciphertext sizes are out of scope: there are no ciphertexts. |
| A root published to some observers but not others permits equivocation | **Addressed as detection.** Four conflict classes, a content-addressed verdict, and no false positive on honest single-root operation. Not addressed as prevention or attribution. |
| An unavailable payload is converted to an empty slot rather than a typed abort | **Addressed.** `SubsetSelectionForbidden` and `input-withheld`; silence counts as unavailability; no subset transition exists. |
| Retries or abort timing reveal whether a batch crossed | **Open.** Abort class, attempt count, and phase timing are public in this model and are not analysed as a channel. |
| Local-fill delivery leaks a user's order through a public notification | Out of scope; this packet has no local outputs. |
| The audit plane silently decrypts the book | Out of scope; there is no audit plane here. |
| Public settlement reconstructs the participant graph | Out of scope; settlement is a separate relation. |

Against `DARK_FBA_RELATION.md` §10's obligations, the ones this packet touches:

- "accepted slots bind to `R` and the frozen admission log" — **met at the
  commitment level**, for envelopes rather than order content.
- "inclusion, equivocation, withholding, timeout, retry, and abort have distinct
  receipts" — **met**: ten admission refusal classes, two seal refusals, three
  domain defects, six receipt defects, eight proof defects, four conflict
  classes, fifteen equivocation defects, six abort classes, four consequences,
  eleven lifecycle errors, and nine refund errors. Every one of them is named by
  some test. Exactly one, `ReceiptDefect::SequenceMismatch`, is unreachable by
  construction rather than by witness: the canonical position lives inside the
  leaf preimage, so a record whose `seq` disagrees with the position its proof
  determines fails the root check first. It is kept as a guard against a future
  encoding that drops `seq` from the preimage, and
  `the_sequence_mismatch_defect_is_defence_in_depth` records the argument
  instead of leaving a class that looks reachable and is not.
- "a builder or computation party cannot learn a partial result and silently
  substitute, censor, or restart the batch on more favourable inputs" —
  **partially met**. Substitution is `conflicting-record-at-sequence`; restart on
  a rewritten history is `acknowledged-prefix-abandoned`; a result computed
  against another root is `result-unbound`. Censorship by *silence* is not.

### 9.1 Open, and named

1. **Attribution.** Without signatures or a consensus record, an equivocation
   proof establishes a contradiction between two statements but not that any
   particular party made them. This is the first thing a successor packet
   should close, and it is a cryptographic dependency, not a modelling one.
2. **Censorship has no positive object.** A holder that simply never answers —
   never acknowledges, never seals, never produces a consistency proof —
   produces no proof of anything. Only the timeout paths respond, and they
   cannot distinguish a censoring holder from a crashed one. `ARCHITECTURE.md`
   §6 asks for a censorship-evidence story; this packet supplies a
   censorship-*timeout* story and says so.
3. **A receipt discloses its own rank.** `seq` is the residual-allocation rank,
   and the budget excludes queue position from the public transcript. A receipt
   is an owner-local object, so this is consistent while it stays local — but
   publishing one, which is exactly what a refund claim or a censorship
   complaint does, discloses that owner's rank in public. The refund path in
   section 6 therefore trades privacy for recourse, and the trade is currently
   unpriced.
4. **Availability is the weakest leg.** Everything about recoverability in this
   model is a declared integer. A real answer needs erasure coding, a dispersal
   protocol, a sampling argument, and a corruption model for the reporting
   committee. Nothing here is evidence about any of those.
5. **Non-power-of-two capacities leak through receipt size.** Stated in section
   3.2 and asserted; a relation choosing capacity 3 would need a different
   padding rule.
6. **Occupancy is still readable from the committed record bytes.** Closing it
   needs a hiding payload commitment and an unlinkable nullifier, which is a
   cryptographic construction this packet does not attempt.

## 10. Reproduction and bounds

From the repository root:

```sh
cargo test --offline --locked \
  --manifest-path experiments/inclusion-availability/Cargo.toml
cargo clippy --offline --locked --all-targets \
  --manifest-path experiments/inclusion-availability/Cargo.toml -- -D warnings
cargo fmt --check \
  --manifest-path experiments/inclusion-availability/Cargo.toml
cargo run --quiet --offline --locked \
  --manifest-path experiments/inclusion-availability/Cargo.toml \
  --bin degg-inclusion-vectors \
  | cmp - experiments/inclusion-availability/vectors/v1.txt
```

VERIFIED on 2026-08-18: 125 tests pass — 5 hash, 29 mountain range, 23
admission, 20 inclusion, 19 equivocation, 27 lifecycle, 2 corpus. Clippy with
`-D warnings` and `cargo fmt --check` are clean. Zero third-party dependencies.

Bounds of the VERIFIED label, stated exactly:

- peak shape is checked for every leaf count in `0..=256`;
- leaf inclusion is exhaustive over every leaf of every log of `1..=33` leaves,
  and cross-index rejection is exhaustive over all ordered pairs at 17 leaves;
- prefix consistency is exhaustive over every prefix of every log of `0..=17`
  leaves;
- append-only node stability is checked over 64 appends;
- admission check priority is checked for every prefix of the frozen order;
- honest non-equivocation is exhaustive over all 32 ordered receipt-pair
  constructions and all five prefixes of a four-record log;
- the abort matrix and terminal immovability are exhaustive over all six abort
  classes plus settlement.

Everything outside those bounds is untested. Larger logs, concurrent holders,
adversarial schedules, real availability, and any cryptographic property are not
covered by any statement in this document.

Corpus byte identity: `experiments/inclusion-availability/vectors/v1.txt`
SHA-256 `725facba9afdb3017c8d2878b6f5a3e11d3e3c08c8bb117fcc6dce37bd71fc0c`.

Validation toolchain: `rustc 1.98.0-nightly (91fe22da8 2026-06-21)`,
`cargo 1.98.0-nightly (a595d0da2 2026-06-20)`.

## 11. Provenance

Original work in this repository, dependency-free. The Merkle mountain range,
the tagged-hash construction, and SHA-256 are public mathematics implemented
here from their public descriptions; no source, fixture, serialization format,
or constant beyond the published SHA-256 tables was transplanted from any
sibling repository. The Dragon's Clutch material in section 8.1 is an
observation about inspected source, cited by path and quoted comment; no code,
identifier scheme, or artifact crossed between the repositories. Because the
author has read related implementations in sibling repositories, this document
does not claim clean-room status.
