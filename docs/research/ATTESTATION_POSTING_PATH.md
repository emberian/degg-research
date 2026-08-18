# Attestation posting path

An onchain record for a zkoracle attestation: the account shape, the admission
relation, and the consumption seam.

Status: PROPOSED specification, written 2026-08-18. This document closes a
*specification* gap, not an artifact gap. It designs the route by which an
attestation produced by the zkoracle stack could reach a ledger and be consumed
there. It is not an implementation, not a program, not a deployment, and it does
not move any wording ceiling: after this document, exactly as before it, the
survey sentence "no onchain posting path" remains true of the artifacts, because
nothing here is built. What changes is that the missing piece is now a *named,
byte-exact, testable* step rather than a gesture, which is the property
[OPERATORLESS_AGENTS.md](../regulatory/research-memos/OPERATORLESS_AGENTS.md)
§"The gap structure is the contribution" claims for every gap in the set.

Claim labels follow [AGENTS.md](../../AGENTS.md) and are carried at section
level. Sections 1, 2, 3, 5, and 6 are PROPOSED. Section 4's quoted ceilings are
SOURCED (the artifact survey of 2026-08-19, as reproduced verbatim in
OPERATORLESS_AGENTS §2). Appendix A records the source inspection behind every
statement about what the existing artifacts contain; those are VERIFIED by
reading the files at the commits named there.

Genre model: `docs/implementation/RESOLUTION_EVIDENCE_PLAN.md` in the Dragon's
Clutch repository, which specifies a typed evidence plane at the rigor this
document aims for — exact byte layouts, named refusal classes, an explicit
"what this does not establish", and promotion gates before any refusal is
relaxed. Its domain-binding discipline is transposed here in §3.

---

## 0. What is missing, exactly

*Label: SOURCED (the survey ceilings) plus VERIFIED (source inspection,
Appendix A).*

The zkoracle stack produces an attestation object that already welds three legs
to one response: a TLSNotary-shaped presentation of a session, a re-executed
JSON well-formedness certificate, and an injection/guard verdict carried
optionally by a Lean-emitted descriptor STARK — all bound by a shared Poseidon2
content commitment over the authenticated response body, with a tested refusal
policy for spliced evidence. That object is a value in a process. Two things
it is not:

1. It is not addressed. There is no name under which a third party can ask "is
   there an attestation for this content, under this policy?" and get an answer
   that does not depend on who is asked.
2. It is not admitted. There is no relation that says which attestation objects
   are well-formed enough to be recorded at all, what a recording establishes,
   and what it refuses.

There is one nearby landed thing, and conflating it with a posting path would
be a mistake worth naming up front. The Dregg side already witnesses a 32-byte
attestation fingerprint on a committed kernel turn (`grain_turn::ATTESTATION_SLOT`,
written through `bind_attestation`, with the canonical fingerprint computed by
`deos_hermes::attestation_commitment` under a versioned domain separator). That
is a *binding*: it says "this turn ran under attestation X" and lets a light
client holding X recompute the fingerprint and compare. It carries no policy, no
admission relation, no uniqueness domain, and no public ledger — the tool loop
runs against an in-process node. A posting path is exactly those four things.
This document supplies them and, in §1.4, carries the existing fingerprint as a
field so the two ledgers can name the same object rather than two.

---

## 1. The record shape

*Label: PROPOSED. Every byte offset, domain string, and constant below is
authored here. No constant, fixture, or code was copied from any sibling
repository; the conventions the shapes follow are cited by identifier and path
in Appendix A, per the repository-boundary rule in AGENTS.md.*

### 1.1 Two accounts, and why two

A posting path needs a *policy* and a *record*, and they must be separate
immutable objects.

The policy is what a consumer freezes in advance: which host, which notary,
which Lean-emitted descriptor, which grammar, which provenance class is
sufficient, which proof backend. If those lived in the record, the poster would
choose them at posting time — the record would be a place where a submitter
announces the standard its own evidence is judged by, which is discretion
wearing a byte layout. Splitting them makes the standard eager and the evidence
lazy: exactly the "determination is eager, witness is lazy" rule the guarded-hole
calculus states ([GUARDED_EVENT_FOUNDATIONS.md](GUARDED_EVENT_FOUNDATIONS.md)
§2.1), applied to an admission rule instead of a state transition.

The record is one posting: one content commitment, under one policy, with the
fingerprints of the evidence that is claimed to exist for it.

Both accounts follow the conventions of `programs/solana-layout` in the
Dragon's Clutch repository: a one-byte tag and a one-byte per-account schema
version, an exact fixed byte length (shorter is `Truncated`, longer is
`TrailingBytes`), 32-byte domain-separated identities with the all-zero value
reserved as an absent sentinel, little-endian scalars, a stored PDA bump the
layout does not interpret, a reserved `flags` byte that must be zero, and a
decoder that requires byte-for-byte re-encoding. `ResolutionAccount` in that
crate is the nearest landed analogue and the shape below is deliberately its
sibling: an immutable fact account that names the evidence that selected it and
carries a `binds_*` check against the frozen terms it answers to.

Tag numbers 17 and 18 are proposed relative to that crate's registry, whose
highest assigned tag is 16 (`RESOLUTION_TAG`). If these accounts land in a
different program — which is likely, since a Dregg posting program is not a
Dragon's Clutch program — the tag namespace is that program's, and the numbers
must be re-checked rather than inherited. The `(tag, version)` pair must never
name two shapes; both accounts start at version 1 because both tags are fresh
and have never named a prior shape.

### 1.2 Identities and domain separation

`H` denotes the profile's selected 32-byte hash. **The primitive is not decided
here.** The Dragon's Clutch layout crate carries a dependency-free SHA-256 for
canonical IDs and correctly states that no deployment has selected that
primitive until a profile says so; the Dregg attestation fingerprint uses
BLAKE3. A posting profile must select one and say so; the derivations below are
prefix-free under either, because every variable-length input is
length-prefixed and every fixed-length input has a fixed length.

```text
PolicyId           := H( "dark-egg/attestation-policy/v1"       || policy_body[184] )
AttestationRecordId:= H( "dark-egg/attestation-record/v1"       || policy[32] || content_commit[32] )
NotaryId           := H( "dark-egg/attestation-notary/v1"       || alg_id(u16 LE) || key_len(u16 LE) || key_bytes )
DescriptorCommit   := H( "dark-egg/attestation-descriptor/v1"   || len(u32 LE) || descriptor_bytes )
GrammarCommit      := H( "dark-egg/attestation-grammar/v1"      || len(u32 LE) || grammar_bytes )
ServerId           := H( "dark-egg/attestation-server/v1"       || len(u16 LE) || server_name )
PresentationDigest := H( "dark-egg/attestation-presentation/v1" || provenance(u8)
                                                                || len(u16 LE) || server_name
                                                                || connection_time(u64 LE)
                                                                || len(u32 LE) || sent
                                                                || len(u32 LE) || recv
                                                                || len(u16 LE) || notary_sig
                                                                || len(u32 LE) || live_presentation )
ProofCommit        := H( "dark-egg/attestation-proof/v1"        || proof_kind(u8)
                                                                || descriptor_commit[32]
                                                                || public_inputs[16]
                                                                || len(u32 LE) || proof_bytes )
```

Three of these derivations carry a design decision worth stating separately.

**`PresentationDigest` folds the provenance byte in, and gives the live
presentation its own length-prefixed slot even when it is absent.** A
fixture-class presentation (length zero) and a live-class presentation can then
never produce the same digest, and two different live sessions over the same
body cannot either. This is the same failure the producing stack's own
fingerprint had to be versioned to fix — under its v1 domain a live attestation
committed only to the fixture carrier, so the real presentation the accept
rested on was not fingerprinted at all. The correction is inherited here as a
construction rule rather than as a version bump waiting to happen.

**`ProofCommit` folds in the descriptor identity and the public inputs, not
only the proof bytes.** A proof is evidence *for a statement*. Committing to the
bytes alone would let the same bytes be presented as evidence for a different
statement if any two descriptors ever admitted a common proof encoding. Folding
the statement into the commitment makes that inexpressible rather than
unlikely.

**`AttestationRecordId` is derived from exactly `(policy, content_commit)`,**
which is what makes uniqueness structural (§2.5). It is deliberately *not* a
fingerprint of the whole attestation: a record's address must be computable by
anyone who knows only which policy they care about and which content they are
asking about, including a consumer who has never seen the attestation object.

### 1.3 `AttestationPolicyAccount` — exactly 220 bytes

Immutable once created. Permissionlessly creatable: publishing a policy is
publishing a standard, and a standard nobody is obliged to consume needs no
gatekeeper.

```text
account_len::ATTESTATION_POLICY
  = 2 + (6 * 32) + 4 + 4 + 4 + 4 + 4 + 1 + 1 + 1 + 1 + 1 + 1
  = 220
```

| Offset | Bytes | Field | Meaning |
| ---: | ---: | --- | --- |
| 0 | 1 | tag | `18` |
| 1 | 1 | version | `1` |
| 2 | 32 | `policy` | self-identity; `PolicyId` over the body below |
| 34 | 32 | `server` | `ServerId` of the pinned TLS host |
| 66 | 32 | `notary` | `NotaryId` of the pinned notary anchor |
| 98 | 32 | `descriptor_commit` | the Lean-emitted guard/parse descriptor the STARK leg must be under |
| 130 | 32 | `grammar_commit` | the grammar the well-formedness certificate must replay against |
| 162 | 32 | `template_commit` | the frozen template identity; all-zero iff `pins` bit 0 is clear |
| 194 | 4 | `table_commit` | pinned DFA table commitment, one field lane, LE `u32` |
| 198 | 4 | `initial_state` | required `public_inputs[0]`, LE `u32` |
| 202 | 4 | `accepting_mask` | bit `i` set iff state `i` is an accepting final state, LE `u32` |
| 206 | 4 | `max_body_len` | upper bound on a record's `body_len`, LE `u32` |
| 210 | 4 | `max_proof_len` | upper bound on a record's `proof_len`, LE `u32` |
| 214 | 1 | `pins` | bit 0: template pinned. Other bits reserved zero |
| 215 | 1 | `required_provenance` | `0` admits a fixture-class record; `1` requires MPC-TLS class |
| 216 | 1 | `proof_kind` | registered proof-backend identifier |
| 217 | 1 | `commit_scheme` | registered identifier of the attestation-fingerprint scheme and version |
| 218 | 1 | `stored_bump` | PDA bump, opaque |
| 219 | 1 | `flags` | reserved, must be zero |

The digest preimage is `policy_body := bytes[34..218]`, exactly 184 bytes:
everything after the header and the self-identity, and before the stored bump
and the reserved flags. That exclusion rule is inherited from the layout crate's
`TERMS_BODY_BYTES`, and it forces one placement decision: **`pins` is a body
field, not a `flags` bit.** A pin bit outside the digest would let a policy be
re-encoded with the template pin flipped while keeping its identity, so
"template pinned to `T`" and "template unpinned" would be the same policy. The
layout crate solves the adjacent problem — unfrozen versus frozen-to-zero — with
a flag bit plus a decoder that refuses every other combination
(`PROFILE_FLAG_POLICY_FROZEN`); here the same discipline applies, but the bit
must live inside the digested body. The decoder refuses `pins` bit 0 clear with
a nonzero `template_commit`, and refuses `pins` bit 0 set with an all-zero one.

`required_provenance` deserves its default said out loud: **any policy meant to
carry weight sets it to `1`.** A policy at `0` admits a self-signed fixture,
under which the poster constructed the transcript and signed it with a key it
holds itself; an admitted record under such a policy asserts that a record
exists, and nothing whatsoever about where any bytes came from.

There is deliberately no freshness field. Whether a session is recent enough is
a *consumption* question and is bound by the consumer (§3.2), because admission
has no trustworthy relation between a chain clock and a session time: the
session time is the notary's claim, and a policy that bounded it at admission
would be silently promoting the notary to the timekeeper for everyone
downstream.

### 1.4 `AttestationRecord` — exactly 374 bytes

Immutable once created. One per `(policy, content commitment)`.

```text
account_len::ATTESTATION_RECORD
  = 2 + (10 * 32) + 16 + 8 + 8 + 4 + 4 + 4 + 4 + 1 + 1 + 1 + 1
  = 374
```

| Offset | Bytes | Field | Meaning |
| ---: | ---: | --- | --- |
| 0 | 1 | tag | `17` |
| 1 | 1 | version | `1` |
| 2 | 32 | `record` | self-identity; `AttestationRecordId(policy, content_commit)` |
| 34 | 32 | `policy` | the `PolicyId` this record is admitted under |
| 66 | 32 | `descriptor_commit` | the descriptor the claimed proof is for |
| 98 | 32 | `notary` | the `NotaryId` the claimed presentation is under |
| 130 | 32 | `content_commit` | the cross-leg content commitment, 8 lanes, LE `u32` each |
| 162 | 32 | `presentation_digest` | `PresentationDigest` over the claimed presentation |
| 194 | 32 | `proof_commit` | `ProofCommit` over the claimed proof and its statement |
| 226 | 32 | `proof_locator` | opaque content address where the proof bytes may be fetched |
| 258 | 32 | `attestation_commit` | the producing stack's whole-object fingerprint, under `policy.commit_scheme` |
| 290 | 32 | `submitter` | the posting identity, for attribution only |
| 322 | 16 | `public_inputs` | the claimed STARK public inputs, 4 lanes, LE `u32` each |
| 338 | 8 | `connection_time` | the presentation's session time, LE `u64` |
| 346 | 8 | `posted_slot` | the slot this record was admitted in, LE `u64` |
| 354 | 4 | `body_len` | length of the authenticated response body, LE `u32` |
| 358 | 4 | `field_span_offset` | start of the guard-checked span within the body, LE `u32` |
| 362 | 4 | `field_span_len` | length of that span, LE `u32` |
| 366 | 4 | `proof_len` | length of the committed proof bytes, LE `u32` |
| 370 | 1 | `provenance` | `0` self-signed fixture class; `1` MPC-TLS class |
| 371 | 1 | `proof_kind` | registered proof-backend identifier |
| 372 | 1 | `stored_bump` | PDA bump, opaque |
| 373 | 1 | `flags` | reserved, must be zero |

Four fields carry decisions rather than data.

**`public_inputs` is carried in the clear, not digested.** Four field lanes are
sixteen bytes; a digest of them would be thirty-two and would tell a chain
nothing. In the clear, the admission rule can check the *statement* — that the
run started at the policy's initial state, ran the policy's pinned transition
table, and ended in an accepting state — even though it cannot check the proof.
This is where the commit/verify line actually falls, and putting the public
inputs onchain is what makes the line fall somewhere useful instead of nowhere.

**`proof_commit` and `proof_locator` are separate.** The commitment is what
binds; the locator is where the bytes may be found. The chain never dereferences
a locator, never learns whether it resolves, and attaches no meaning to its
value beyond "nonzero". A locator that resolves to nothing produces a record
whose off-chain verification is impossible — which is a fact about that record,
correctly represented as an unverifiable claim rather than as an admission
failure the chain pretends it could have detected.

**There is no `cfg_cert` field, on purpose.** The well-formedness leg is a
compact certificate checked by deterministic re-execution, not carried by a
STARK, and that distinction is a binding ceiling. A re-executable certificate is
a *witness anyone holding the body can recompute*, so committing to it would pin
a witness rather than record a fact; what must not be a runtime choice is the
grammar, and the grammar is pinned in the policy. Keeping the certificate out is
the same judgment `RESOLUTION_EVIDENCE_PLAN` §1.5 makes when the accumulator
publishes a preimage and computes no identity: publish what recomputation needs,
and do not store what recomputation produces.

**There is no `verified` bit, and admission installs none.** No onchain fact
would justify one. A field that a reader could mistake for "the chain checked
this" is the single most dangerous byte such a record could carry, and its
absence is a specification requirement, not an omission.

### 1.5 Canonicality rules the codec must enforce

Beyond the layout crate's standard discipline (exact length, known tag and
version, zero reserved bytes, byte-for-byte re-encode), two rules are specific
to this record and are load-bearing rather than cosmetic.

**Field lanes must be reduced.** Every four-byte lane inside `content_commit`
(eight lanes), `public_inputs` (four lanes), and `policy.table_commit` (one
lane) encodes an element of the prover's field, whose modulus is
`p = 2^31 - 2^27 + 1 = 2013265921`. Any lane whose LE `u32` value is `>= p` is a
byte string no honest encoder produces, and the decoder must refuse it. This is
not tidiness: `AttestationRecordId` is derived over `content_commit` *bytes*, so
if two byte encodings of one commitment were both admissible, the same
attestation could be posted at two addresses and the uniqueness argument in §2.5
would be false.

**All-zero is reserved.** Every 32-byte identity in both accounts must be
nonzero, with exactly one exception: `template_commit` is all-zero precisely
when `pins` bit 0 is clear, and the decoder refuses both other combinations.

### 1.6 Commit onchain, verify offchain — stated as a decision

*The honest default: the record commits; verification happens off the chain.*

A full descriptor STARK does not fit in an account, and this document does not
pretend to know how far from fitting it is: the proof is carried as serialized
bytes in the producing stack, and no size measurement is recorded here or, as
far as this lane has read, anywhere that this lane inspected. So the choice is
not between two implementable options; it is between recording a claim and
recording nothing.

What the chosen default buys is precise and worth stating positively. A posted
record makes an off-chain verification **scoped and non-repudiable**: the record
fixes exactly one policy, exactly one content commitment, exactly one
presentation, exactly one proof-for-a-statement, and exactly one provenance
class, at exactly one slot, at an address anyone can derive. An independent
verifier who fetches the proof and the body can then either confirm or refute a
posting that its submitter cannot subsequently reinterpret. That is materially
more than an unaddressed value in a process, and materially less than
verification.

**Onchain proof verification is a named future gate, and its costs are
unknown.** Naming what is unknown, rather than estimating it:

- the compute cost of a plonky3-family FRI verifier inside a chain runtime is
  unmeasured here;
- the proof size, and therefore whether the bytes can reach the chain at all
  (single transaction, chunked accounts, or neither), is unmeasured here;
- the verifier's own code size and whether it fits a program is unmeasured here;
- and, decisively, **even a landed onchain verifier would not upgrade soundness.**
  The standing provenance gate applies: no soundness claim may rest on the
  proving backend, whose FRI dependency is locally patched with the fix not yet
  merged upstream. An onchain verifier would move *who checks* the proof, not
  *what the proof is worth*.

There is a cheaper gate that could land first and should be named separately,
because conflating the two would overstate the distance: **onchain verification
of the notary signature.** That is one signature check against a pinned key,
not a STARK verifier, and it would let admission establish that a presentation
with the claimed digest was signed by the pinned notary — a real strengthening
of §2.3 that costs a signature verification and nothing else. It is not
specified here (it requires the presentation bytes, or a signature over the
digest, to reach the chain, which is a separate transport question), but it is
the correct next rung to price, ahead of the STARK.

---

## 2. The admission relation

*Label: PROPOSED. Nothing below is implemented; §5 names the cheapest artifacts
that would make it exercisable.*

### 2.1 Signature and totality

```text
admit :
    (AttestationPolicyAccount, candidate_bytes[374], current_slot)
      -> Result<AttestationRecord, AdmissionRefusal>
```

Total, allocation-free, clock-free apart from the supplied slot, and a pure
function of its three arguments. It reads no signer, consults no authority, and
performs no cryptographic verification. Every path out of it that is not
`Ok(AttestationRecord)` is one of the named classes in §2.6, and — mirroring the
guarded-hole model's "refusal changes nothing" — a refusal leaves durable state
exactly as it was: no partial record, no reservation, no penalty branch.

### 2.2 Who may post

**Permissionless.** Any signer may submit a candidate. The signer's key enters
the record only as `submitter`, and `submitter` is consulted by no rule: not by
admission, not by any binding check in §3, not by any consumer. It exists for
attribution and to name a rent payer.

This is the acceptance-versus-execution split the architecture already argues:
authority lives in the verification relation, not in the executor, and the
acceptance rule can see nothing of a submitter's but the certificates. A posting
path that admitted records from an allowlist would relocate exactly the operator
this design is trying not to have.

### 2.3 What verifies at admission

Ten checks, in order. All are byte-level or arithmetic; none is cryptographic
beyond recomputing `H` for two identities.

| # | Check |
| ---: | --- |
| A1 | **Codec.** Exact 374 bytes; tag `17`; version `1`; `flags == 0`; every field lane reduced mod `p`; every required identity nonzero; the decoded value re-encodes byte-for-byte. |
| A2 | **Self-identity.** `record == AttestationRecordId(policy, content_commit)`, recomputed. |
| A3 | **Policy decode.** The presented policy account passes its own codec, and its self-identity recomputes from its 184-byte body. |
| A4 | **Policy binding.** `record.policy == policy.policy`, `record.descriptor_commit == policy.descriptor_commit`, `record.notary == policy.notary`, `record.proof_kind == policy.proof_kind`. |
| A5 | **Provenance gate.** `record.provenance` is `0` or `1`, and if `policy.required_provenance == 1` then `record.provenance == 1`. |
| A6 | **Statement shape.** `public_inputs[0] == policy.initial_state`; `public_inputs[2] == policy.table_commit`; `public_inputs[1] < 32` and bit `public_inputs[1]` of `policy.accepting_mask` is set. |
| A7 | **Span bounds.** `field_span_len > 0`; `field_span_offset + field_span_len` does not overflow and is `<= body_len`; `0 < body_len <= policy.max_body_len`. |
| A8 | **Proof envelope.** `0 < proof_len <= policy.max_proof_len`; `proof_commit` and `proof_locator` nonzero. |
| A9 | **Uniqueness.** The record is created at the canonical address derived from `(policy, content_commit)`; an existing account there refuses. |
| A10 | **Slot.** `record.posted_slot == current_slot`. |

Check A6 is the only one that inspects the substance of a proof claim, and its
exact strength must be stated: it checks that the claimed *statement* is the
policy's statement and that the claimed *verdict* is accepting. It does not
check that a proof of that statement exists, verifies, or is about this body. A
submitter who invents `public_inputs` matching the policy and a `proof_commit`
over nothing passes A6 and produces a record that fails off-chain verification
at the first fetch. **Admission checks the statement's shape, never its weld.**

### 2.4 What is committed and not verified

Everything cryptographic. The record commits to it; the chain checks none of it.

| Committed as | The claim it stands for | Checked at admission |
| --- | --- | --- |
| `proof_commit`, `proof_len`, `proof_locator` | a descriptor STARK for the claimed statement exists at that locator | no |
| `presentation_digest` | a presentation with that transcript, session time, and notary signature exists | no |
| `notary`, `provenance` | that presentation is under the pinned notary, in the claimed class | no (only class equality against the policy) |
| `content_commit` | the sponge over the authenticated response body | no (the chain never sees a body) |
| `field_span_offset`, `field_span_len`, `body_len` | the guard ran over that span of that body | no (only arithmetic containment) |
| `attestation_commit` | the whole-object fingerprint under the pinned scheme | no |

### 2.5 Replay and uniqueness

**One record per content commitment per policy, enforced by address derivation.**
The record's address is the canonical derivation from `(policy, content_commit)`,
so a second posting for the same pair is not a duplicate the program must detect
by scanning — it is a request to create an account that already exists.
Uniqueness is structural, and §1.5's lane-reduction rule is what makes the
argument sound.

Two consequences must be stated rather than discovered.

**First-poster-wins, within a policy.** If the same body were attested twice —
two sessions returning identical bytes, or one body re-attested with a stronger
proof — only the first posting exists. Under a policy at
`required_provenance == 0`, a fixture-class record could therefore occupy the
address that a later MPC-TLS-class record for the same body would need. The
resolution is a design rule, not a runtime branch: **a policy that admits
fixture provenance and a policy that requires MPC-TLS are different policies
with different identities, and therefore disjoint address spaces.** A fixture
record cannot squat the live record's address, because they are not the same
address. Consumers that care bind the strict policy and are unaffected by
anything posted under the permissive one.

**The alternative is named and refused for v1.** Deriving the address from
`(policy, attestation_commit)` instead would give one record per attestation
object rather than per body, admitting re-attestation at the cost of unbounded
records per body and a consumer that can no longer ask a single well-formed
question ("is there a record for this content under this policy?"). Making the
derivation a policy-selected mode is worse than either: two derivations are two
address spaces, and a consumer that computed the wrong one would look up a
nonexistent account and see absence rather than a refusal. Per the repository's
preference for refusal over ambiguity, v1 has exactly one derivation; a
per-session mode is a v2 schema decision with its own version byte.

### 2.6 Refusal taxonomy

Distinguishable on purpose: "the policy disagreed" and "the guard's verdict was
rejecting" have different operational responses.

| Id | Class | Raised when |
| ---: | --- | --- |
| A-01 | `RecordMalformed` | wrong length, tag, or version; nonzero reserved bytes; re-encode mismatch |
| A-02 | `NonCanonicalLane` | a lane of `content_commit`, `public_inputs`, or `policy.table_commit` is `>= p` |
| A-03 | `ZeroIdentity` | a 32-byte identity required nonzero is all-zero |
| A-04 | `PinDisagreement` | `pins` bit 0 clear with nonzero `template_commit`, or set with an all-zero one |
| A-05 | `NonCanonicalRecordId` | `record` is not the canonical derivation from `(policy, content_commit)` |
| A-06 | `PolicyMalformed` | the presented policy fails its own codec or self-identity recomputation |
| A-07 | `PolicyMismatch` | `record.policy` is not the presented policy's identity |
| A-08 | `DescriptorMismatch` | `record.descriptor_commit` is not the policy's pinned descriptor |
| A-09 | `NotaryMismatch` | `record.notary` is not the policy's pinned notary |
| A-10 | `ProvenanceRefused` | a fixture-class record under a policy requiring MPC-TLS class |
| A-11 | `WrongInitialState` | `public_inputs[0]` is not the policy's initial state |
| A-12 | `WrongTable` | `public_inputs[2]` is not the policy's pinned table commitment |
| A-13 | `NonAcceptingFinalState` | `public_inputs[1]` is outside the policy's accepting mask — the claimed verdict is itself a rejection |
| A-14 | `SpanOutOfRange` | empty span, overflowing span, span past `body_len`, or `body_len` past the policy bound |
| A-15 | `ProofEnvelopeRefused` | wrong `proof_kind`, zero or over-bound `proof_len`, zero commitment or locator |
| A-16 | `DuplicateRecord` | an account already exists at the canonical address |
| A-17 | `SlotMismatch` | `posted_slot` is not the slot the record is created in |
| A-18 | `UnregisteredId` | `proof_kind` or `commit_scheme` is outside its registry |

A-10 is the chain-side twin of the producing stack's own provenance gate, which
refuses a self-signed fixture before any leg runs when the caller demanded a
real MPC-TLS presentation. It is the single most important refusal in the table,
because it is the only one that separates "a record exists" from "a record
exists that claims transport provenance".

A-13 deserves its own note: a record whose claimed final state is rejecting is
refused rather than admitted-and-marked. The chain does not store guard
failures. A guard verdict of "this field injects" is not a fact worth an address
under this policy; it is a reason the posting does not happen.

### 2.7 The named non-checks

These are what an admitted record does **not** assert. They are numbered so that
a downstream document, filing sentence, or code comment that overstates one can
be pointed at a specific line.

| Id | Not established by admission |
| ---: | --- |
| N-01 | that the committed proof exists, decodes, or verifies |
| N-02 | that the presentation's notary signature verifies |
| N-03 | that the endpoint's declared secret request header was actually redacted |
| N-04 | that the body is well-formed under the pinned grammar |
| N-05 | that the body was served by the pinned host, or by any host |
| N-06 | that the committed span contains what the guard ran over — the chain holds no body and cannot check the weld |
| N-07 | that the proof locator resolves to anything |
| N-08 | that a model produced, or was involved in, any of it |

---

## 3. The consumption seam

*Label: PROPOSED for the binding rules; VERIFIED (Appendix A) for the statements
about what the cited existing artifacts do.*

### 3.1 The binding discipline, five rules

Transposed from `RESOLUTION_EVIDENCE_PLAN`'s domain-binding discipline, whose
central move is that a settlement-facing function names a *domain-bound* type and
re-checks the domain rather than trusting the evidence to describe itself.

1. **Every identity a consumer relies on is frozen in the consumer's own terms,
   never read off the record.** The record supplies values to compare, never
   policies to adopt. This is exactly `ResolutionAccount::binds_terms`: the
   resolution names a terms digest, and the check is *equality with the terms the
   consumer already froze* — not "use whatever terms the resolution names."
2. **Decoding is not checking.** A well-formed record is not evidence of the
   right policy, in the same way a well-formed parent profile is not evidence of
   the right collateral subfield: only recomputation and comparison bind. A
   consumer recomputes `AttestationRecordId` from the policy *it* froze and the
   content commitment *it* expects, and compares.
3. **Domain separation everywhere, prefix-freeness argued not assumed.** A record
   identity is never reused as a policy identity, a proof commitment, or an
   attestation fingerprint. Each has its own domain string; every variable-length
   payload is length-prefixed; every fixed-length payload has one fixed length.
   A future variable-length payload under any of these domains must re-derive
   prefix-freeness rather than inherit this sentence.
4. **One-shot on the consumer's own key.** A record is a public fact and may be
   consumed by many different events; each event fills at most once, and its
   replay key derives from *its own eager shape*, never from the record. Otherwise
   two distinct commitments referencing one record would share a replay domain.
5. **The record is never an authority.** No field of it is a signer, and
   `submitter` grants nothing. A consumer that branched on `submitter` would have
   reintroduced a discretionary party through a data field.

### 3.2 Consumer A — a guarded fill

The guarded-hole calculus fixes a transition's complete shape at creation and
lets exactly one value arrive late; the late value is untrusted advice, typed by
the specification, so advice for a different shape does not type-check.

A posted attestation is a natural late value. The hole's eager specification
pins, at creation:

```text
policy              : the PolicyId whose standard this fill accepts
content_commit      : the exact content commitment this fill is about
descriptor_commit   : the descriptor the guard leg must have been under
notary              : the pinned NotaryId
min_provenance      : 0 or 1
slot_window         : [start, end) on posted_slot
```

and the advice is exactly one value: the record. The guard is then a pure
predicate over eager fields and the arriving record, with no discretion
available:

```text
admits(spec, record, policy) iff
      record decodes and re-encodes byte-identically            (rule 2)
  and policy decodes and self-identifies                        (rule 2)
  and policy.policy      == spec.policy
  and record.policy      == spec.policy
  and record.record      == AttestationRecordId(spec.policy, spec.content_commit)
  and record.content_commit    == spec.content_commit
  and record.descriptor_commit == spec.descriptor_commit
  and record.notary            == spec.notary
  and record.provenance        >= spec.min_provenance
  and spec.slot_window.start <= record.posted_slot < spec.slot_window.end
```

Everything not in that list is unavailable by construction. "Recent enough" is
`slot_window`, frozen at creation, over `posted_slot` — a *chain* clock. A
consumer that instead bound `connection_time` would be trusting the notary for
time as well as for transport, and if a design wants that, it must say so at the
hole, not discover it at the fill.

A fill that admits produces exactly the pre-committed transition; a fill that
does not admit produces no state change at all. The failure modes worth naming
are the ones a reader will otherwise assume away: the record's *absence* is not
a refusal reason distinguishable from its *rejection* unless the consumer says
so, because a missing account and a refused predicate are different events with
different operational responses. Specify both: `RecordAbsent` and
`RecordRefused(field)`, with the refusing field named, mirroring the field-level
naming in `WindowResult::check_domain`.

### 3.3 Consumer B — an evidence-gated resolution

The Dragon's Clutch pattern, transposed. There, `ResolutionTerms` freezes an
exact expected `WindowDomain` and `derive_payout(ResolutionTerms, WindowResult)`
re-checks the domain field by field before any statistic is read; a
`WindowResult` is the only settlement-facing type and there is no public
constructor from the underlying `Summary`, so the substitution of unbound
evidence is a compile error rather than a review item.

The analogue is `AttestationTerms` freezing an *attestation domain* — policy,
descriptor, notary, minimum provenance, and the expected content commitment or
the rule that determines it — with

```text
derive_from_attestation :
    (AttestationTerms, AdmittedRecord) -> Result<PayoutIndex, ResolutionRefusal>
```

where **`AdmittedRecord` has no public constructor.** The only way to obtain one
is through the admission relation of §2. A raw decoded `AttestationRecord` must
not be passable to any settlement-facing function, and the discipline should be
enforced the way the accumulator crate enforces its own: a `compile_fail`
doctest exhibiting the substitution, so the rule is checked rather than
documented.

### 3.4 The pinned notary, restated at each consumption point

The transcript leg's trust is trustless-to-that-notary, not operatorless: a
specific, named third party sits inside the evidence chain performing a function
someone must currently be trusted for. That sentence must appear at every point
where a posted record is consumed, because each consumption is a fresh place
where a reader could forget it:

- **At admission**, the notary identity is compared, never verified (N-02). An
  admitted record inherits the notary trust role in full and adds nothing.
- **At a guarded fill**, the hole's `notary` field is the consumer *naming the
  party it is trusting*. Freezing it eagerly is what makes the trust inspectable;
  it does not make the trust smaller.
- **At an evidence-gated resolution**, the same, with a sharper consequence: a
  payout derived through this path is a payout that a named notary could have
  changed. A market whose terms name a notary has named an operator for that
  function, and any description of it as operatorless would be false.
- **At light-client re-verification**, the notary is what the signature check
  resolves to. Recomputing every commitment in the record and confirming the
  proof establishes everything *except* that the transport happened, which is
  precisely the notary's contribution.

And below all four: a record at `provenance == 0` carries no transport
provenance at all — not weaker provenance, none. Its accept is a plumbing fact.

### 3.5 The value gap

One thing a consumer cannot do with a posted record today, stated plainly
because a reader will otherwise assume it.

A `WindowResult` yields a statistic, and a frozen partition maps that statistic
to a payout index; the whole path from evidence to settlement is arithmetic over
values the evidence carries. **A posted attestation record commits to a body,
not to a value.** Its `public_inputs` are a DFA run — an initial state, a final
state, a table commitment, and a route commitment — which answer "did the
guarded field satisfy the policy automaton", not "what number did the endpoint
report". The chain holds no body, so it cannot extract a value from one either.

Therefore: **no path from a posted record to a payout index exists, and none can
be specified on top of this record shape alone.** Closing it requires a
value-carrying leg whose public inputs include the extracted numeral, bound to
the same content commitment — a different circuit than the one that exists, over
a different statement, which nothing in this document designs and nothing in the
surveyed artifacts implements. Consumer B above is therefore specified as a
*shape* the evidence would slot into, exactly as `RESOLUTION_EVIDENCE_PLAN` §2 is
specified against an evidence decoder that is itself unwritten. Saying otherwise
would be the composition hole that plan was written to close, reintroduced.

---

## 4. What this does not establish

*Label: SOURCED for every quoted sentence — the artifact survey of 2026-08-19 as
revised by its corrected second pass, reproduced verbatim in
[OPERATORLESS_AGENTS.md](../regulatory/research-memos/OPERATORLESS_AGENTS.md)
§2, where those sentences are binding wording ceilings. Nothing in this document
upgrades any of them.*

**This document does not retire the gap it addresses.** The defensible summary
ceiling stands unchanged and in full: "local research artifacts implement a
Lean-authored parse/guard STARK and a genuine TLSNotary 2PC integration, joined
by a shared content commitment and tested for refusal, with no live-provider
session, no onchain posting path, and no verifiable-inference backend." A
specification of a posting path is not a posting path. The clause "no onchain
posting path" remains true after this document and is retired only by an
artifact, of which §5 names the three cheapest.

**No live model-provider session exists.** (A live exchange-API MPC-TLS session against real `api.coinbase.com` is recorded at breadstuffs `183d82817`, 2026-07-11 — gate R; see the dated correction in OPERATORLESS_AGENTS.md.) "A genuine TLSNotary MPC-TLS 2PC session is
integrated at a pinned upstream revision, with server/notary pinning, API-key
redaction, and a tested refusal of self-signed fixtures on the live path; the
authenticated endpoint exercised is a local test server, and no live provider
session has been performed." May not claim: "that any provider actually attested
anything." Consequently every record this document could produce today is a
record about a local test session, and the honest provenance byte for it is the
one the attestation object actually carries — read off, never assumed.

**The notary is an operator for its function.** "A specific, named third party
sits inside the evidence chain performing a function someone must currently be
trusted for. This is not a footnote." A posting path does not shrink that role;
§3.4 restates it at each consumption point precisely because posting spreads the
role to more readers.

**Posting does not authenticate the model interaction beyond what the
presentation proves.** The parse and guard leg is "research circuits over
bounded grammars, not a proof of model-input generation", and may not claim
"that an input was proved to have been generated from an approved template" or
"any zero-knowledge or hiding property (the proven field is disclosed)". The
inference leg is empty: "no proving backend, zero inference proofs". What a
presentation proves is a *transport* fact — that a pinned host served these
bytes over a session — not a *computation* fact. N-08 in §2.7 is the record-level
statement of the same boundary.

**The executor host is trusted.** The whole-history proving rung is "an explicit
machine-readable gap, not an implementation. The executor host is trusted at the
current rung." A posting path is downstream of that: the host that assembles a
record chooses which attestation to post and what to put in the fields, and the
admission relation checks the fields' shape, not the host's honesty.

**No soundness claim rests on the proving backend.** Its FRI dependency is
locally patched with the fix not yet merged upstream. A-06's statement check and
any future onchain verifier inherit that floor unchanged.

**The reproduced test record's caveats travel.** The 86 tests (61 + 6 + 19), zero
failures, are three filtered measurements, not workspace health; the Lean emit
step was not re-run, and the committed emitted descriptor — whose Lean pinning is
inherited from the commit — is what the tamper canaries exercised; and the tests
establish nothing about all inputs
([ATTESTATION_SUITE_RERUN_2026-08-19.md](ATTESTATION_SUITE_RERUN_2026-08-19.md)).

**And this document's own ceilings.** No program, no deployment, no chain, no
transaction, no measurement. The hash primitive is undecided, exactly as
`WindowId`'s is in the plan this document takes as its genre model. The record's
byte layout has never been encoded or decoded by anything. Every refusal class in
§2.6 is a name without a test. The tag numbers are proposed against a registry in
a repository that will probably not host these accounts.

---

## 5. The bounded-experiment ladder

*Label: PROPOSED. Costs are stated as effort classes, not as measurements.*

Three artifacts, each cheap, each falsifiable, each establishing strictly less
than a reader might hope — with the "establishes nothing about" line written
first, because that is the line that keeps the ladder honest.

### E1 — the codec prototype

Encode and decode both shapes as a dependency-free, allocator-free codec in
`experiments/`, applying the full discipline: exact length, tag, version, lane
reduction mod `p`, the `pins`/`template_commit` combination rule, zero reserved
bytes, byte-for-byte re-encode, and the two identity derivations.

Deliverable: three positive golden vectors (a fixture-class record, an
MPC-TLS-class record, and a policy account), at least twelve decode refusals
covering A-01 through A-06, and at least four domain-separation confusions — the
record domain over policy bytes, the policy domain over record bytes, undomained
bytes, and a spurious separator byte — each asserted distinct from both real
digests and from each other, in the manner of the cross-language identity vectors
in `RESOLUTION_EVIDENCE_PLAN` §3.3.

Establishes: the shapes are expressible, their lengths are as stated, and their
refusals are exercisable. Establishes nothing about a chain, a proof, a session,
or an attestation.

### E2 — the host-side admission checker

A pure function implementing A1 through A10 with the refusal taxonomy A-01
through A-18, plus one adversarial fixture per refusal class. Include the typed
discipline of §3.3: an `AdmittedRecord` with no public constructor, and a
`compile_fail` witness that a raw decoded record cannot reach a settlement-facing
function.

Establishes: the admission relation is total, its refusals discriminate (each
fixture hits its own class and no other), and the unbound-evidence substitution
is a compile error rather than a review item. Establishes nothing about a chain,
verifies no proof, and checks no signature — the checker's own N-01 through N-08
should be asserted as a comment block that the tests cannot satisfy.

### E3 — one end-to-end fixture from an existing attestation

Take one attestation object the Breadstuffs tree already produces on its
loopback path. Compute, under this document's domain strings, its content
commitment lanes, its presentation digest, its proof commitment, and its
whole-object fingerprint; emit the 374-byte record and the 220-byte policy it is
admitted under; run E2's checker.

The record's `provenance` byte must be **read off the actual attestation object**
— whether it carries a real MPC-TLS presentation or only the modeled carrier —
never assumed by the fixture builder. This is the whole point of the exercise
and the one place it could quietly go wrong.

Deliver the *pair*: the accept under a policy at `required_provenance == 0`, and
the refusal (A-10) of the same record under a policy at
`required_provenance == 1`. If the source attestation is fixture-class, then the
accept is the uninteresting half and **the refusal is the result** — an
end-to-end demonstration that the posting path's provenance gate has teeth
before any live session exists.

Establishes: the path from a real existing attestation object to a posted record
shape is arithmetic, not design — the fields all have sources. Establishes
nothing about a live provider session, a notary, a chain, or a proof; a
fixture-class accept must never be described as an attested posting.

### Not on the ladder, and why

Onchain proof verification (§1.6 — unmeasured costs, and it would not upgrade
soundness); onchain notary-signature verification (a genuine next rung, but it
needs a transport decision this document does not make); a live provider session
(a deploy step owned by another lane, and the single most claim-changing step
available, which is exactly why it should not be bundled into a posting-path
experiment); the R3 whole-history proving rung and any inference proof (research
programs); notary decentralization (design work, handled the only honest way
available — by naming the pinned notary as an operator).

---

## 6. Promotion gates before any refusal is relaxed

*Label: PROPOSED.*

In dependency order, what must exist before a posted record may be described as
anything more than a scoped, non-repudiable claim:

1. **A codec with vectors** (E1). Until bytes round-trip, the shape is prose.
2. **An admission checker with adversarial fixtures** (E2). Until every refusal
   class has a test that hits it and only it, the taxonomy is a table.
3. **An off-chain verifier that consumes a record end to end** — fetches the
   locator, checks `proof_commit`, verifies the proof against
   `descriptor_commit` and `public_inputs`, re-derives `content_commit` from the
   body, replays the certificate against the pinned grammar, re-extracts the
   span, and checks `presentation_digest` and the notary signature. Only this
   makes "verify offchain" a thing that happens rather than a thing that is
   possible in principle. It is the natural E4 and was left off the ladder only
   because it is a day's work larger than E3.
4. **A named transport decision for the presentation**, without which onchain
   signature verification cannot be specified.
5. **A value-carrying leg** (§3.5), without which Consumer B remains a shape.
6. **A hash-primitive selection in a profile**, without which the identities in
   §1.2 are underdetermined.

Until items 1, 2, and 3 have checked artifacts, the honest description of the
posting path is "specified, unbuilt", and the survey's "no onchain posting path"
sentence stays exactly as it is.

---

## Appendix A — inspection record and claim sources

Labels follow [AGENTS.md](../../AGENTS.md). Every VERIFIED row below is a
source-inspection claim: the named item was read at the commit in the table; no
build, test, or proof was executed for this document, and no measurement is
reported anywhere in it.

| Repository | Commit at inspection | Working tree |
|---|---|---|
| /Users/ember/dev/breadstuffs | `436c2a865a0a0e6b8222050ef27464750a0471d7` | dirty (three tracked modifications and six untracked files, none of them a file cited here) |
| /Users/ember/dev/dragons-clutch | `d7ee404cf0614a5c574c58abbb8cafbddc09d4fa` | dirty (one untracked directory, `toolchain/probes/token2022/`); all cited files unmodified |
| /Users/ember/dev/degg-research | `bea2d385726b8645226af9574ac8d42ebed15a25` | dirty (a concurrent regulatory-typst lane); no file cited here modified |

| ID | Claim (as used in the body) | Label | Exact artifact |
|---|---|---|---|
| P-01 | The attestation object bundles a presentation, a re-executed JSON certificate, a committed field span, and an optional descriptor STARK, all welded by a shared eight-lane content commitment over the authenticated body. | VERIFIED (source inspection) | `/Users/ember/dev/breadstuffs/zkoracle-prove/src/attestation.rs:110` (`ZkOracleAttestation`), `:84` (`content_commitment`), `:48` (`CONTENT_COMMIT_W`), `:92` (`FieldSpan`) |
| P-02 | The content commitment was widened from one field element to eight because the value leaves the verifier and lands in a committed, chain-linked receipt compared by value equality; the recorded collision bound moves from about 2^15.45 to about 2^123.63. | VERIFIED (source inspection) | `/Users/ember/dev/breadstuffs/zkoracle-prove/src/attestation.rs:52-84` (the widening note on `content_commitment`) |
| P-03 | Provenance is a first-class distinction: a self-signed fixture carrier versus a real MPC-TLS presentation, with a policy that refuses the former on a live path before any leg runs, and a fail-closed default backend that authenticates nothing. | VERIFIED (source inspection) | `/Users/ember/dev/breadstuffs/zkoracle-prove/src/attestation.rs:185` (`AuthenticProvenance`), `:206` (`authentic_provenance`), `:216` (`AuthenticPolicy`), `:244` (`ZkOracleError`, incl. `FixtureOnLivePath`, `LiveBackendUnavailable`), `:367` (`MpcTlsLeg`), `:385` (`NoMpcTlsBackend`) |
| P-04 | The presentation object carries the pinned server identity, the session time, both directions' delivered bytes, and a notary signature over canonical signing bytes; the adapter enforces server pinning, notary pinning, the signature, and the declared secret-header redaction, with a named refusal for each. | VERIFIED (source inspection) | `/Users/ember/dev/breadstuffs/zkoracle-prove/src/authentic.rs:155` (`EndpointPresentation`), `:176` (`canonical_signing_bytes`), `:233` (`AuthenticError`), `:294` (`verify_endpoint_presentation`), `:82` (`EndpointSpec`), `:140` (`TlsnVerifyingKey`) |
| P-05 | The STARK leg carries serialized proof bytes plus four public inputs `[initial, final, table_commit, route_commit]`, under a byte-pinned Lean-emitted descriptor; the accepting states are the non-dead states of a three-state automaton; the run is over a padded projection, so the proof transfers within a padding block but never across the accept/reject boundary; binding the field bytes to the authenticated body is the attestation's span weld, not this leg. | VERIFIED (source inspection) | `/Users/ember/dev/breadstuffs/zkoracle-prove/src/zk_leg.rs:1-42` (module header, incl. the honest-boundary paragraph), `:94` (the pinned descriptor literal), `:106` (`injection_dfa_table`), `:157` (`expected_public_inputs`), `:261` (`ZkInjectionProof`), `:272` (`ZkLegError`), `:309` (`verify_injection_leg`) |
| P-06 | A whole-object 32-byte attestation fingerprint exists, versioned to v2 precisely because v1 left the real MPC-TLS presentation and the STARK leg unbound, so two different live sessions over one body committed identically. | VERIFIED (source inspection) | `/Users/ember/dev/breadstuffs/deos-hermes/src/attest.rs:66-92` (the domain note and `attestation_commitment`) |
| P-07 | That fingerprint is already witnessed on a committed kernel turn slot in the Dregg in-process ledger, with a light client recomputing and comparing it; the slot carries a caller-supplied hash and no policy, admission rule, or uniqueness domain. | VERIFIED (source inspection) | `/Users/ember/dev/breadstuffs/grain-turn/src/lib.rs:274` (`ATTESTATION_SLOT` and its doc), `:410` (`bind_attestation`) |
| P-08 | A template commitment exists as the sole binding of "which template produced this" for a reader who does not hold the template. | VERIFIED (source inspection) | `/Users/ember/dev/breadstuffs/zkoracle-prove/src/render.rs:168-191` (`template_commitment` and its attribution-forgery note) |
| P-09 | The prover's field modulus is `p = 2^31 - 2^27 + 1 = 2013265921`. | VERIFIED (source inspection) | `/Users/ember/dev/breadstuffs/circuit/src/field.rs:3`, `:11` |
| P-10 | The layout crate's account conventions: one-byte tag plus one-byte per-account schema version, exact fixed length with `Truncated`/`TrailingBytes` refusals, zero-reserved identities, little-endian scalars, stored bump, reserved zero flags, and a `(tag, version)` pair that never names two shapes. | VERIFIED (source inspection) | `/Users/ember/dev/dragons-clutch/programs/solana-layout/src/lib.rs:144` (`CodecError`), `:379` (`account_version`), `:413` (`account_len`), `:692` (`check_header`), `:772-828` (`Reader`), `:830` (`put_header`), `:101-110` (`Hash32`, `ZERO`) |
| P-11 | `ResolutionAccount` is the nearest landed analogue: an immutable fact account naming the evidence that selected it, with a `binds_terms` check against the frozen terms, an explicit unresolved/resolved field discipline, and a doc note that it is bytes only and not evidence that a window was sealed. | VERIFIED (source inspection) | `/Users/ember/dev/dragons-clutch/programs/solana-layout/src/lib.rs:2826-2857` (doc and struct), `:2866` (`validate`), `:2892` (`binds_terms`) |
| P-12 | The terms digest is taken over the body bytes after the account's own identity field and before the trailing bump and flags. | VERIFIED (source inspection) | `/Users/ember/dev/dragons-clutch/programs/solana-layout/src/lib.rs:305` (`canonical_terms_digest`), `:1527` (`TERMS_BODY_BYTES`) |
| P-13 | The unfrozen-versus-frozen-to-zero confusion is closed by a flag bit whose every other combination the decoder refuses. | VERIFIED (source inspection) | `/Users/ember/dev/dragons-clutch/programs/solana-layout/src/lib.rs:508` (`PROFILE_FLAG_POLICY_FROZEN`); `/Users/ember/dev/dragons-clutch/docs/implementation/RESOLUTION_EVIDENCE_PLAN.md` §3.4 |
| P-14 | The genre model's disciplines transposed in §3: a settlement-facing function names a domain-bound type with no public constructor from the unbound one, checked by a `compile_fail` doctest; the hash primitive is explicitly undecided; decoding is not checking, and only recomputation binds; refusals are named per field. | VERIFIED (source inspection) | `/Users/ember/dev/dragons-clutch/docs/implementation/RESOLUTION_EVIDENCE_PLAN.md` §1.1, §1.4, §1.5, §1.6, §2.5, §3.3, §4 |
| P-15 | The guarded-hole calculus fixes a transition's shape eagerly and admits exactly one late value as untrusted, spec-typed advice; refusal changes nothing; fills are one-shot on a replay key derived from the eager shape. | SOURCED (this repository's own exposition of the Lean models, not re-inspected here) | [GUARDED_EVENT_FOUNDATIONS.md](GUARDED_EVENT_FOUNDATIONS.md) §§2.1, 2.2 (claims C-01 through C-09) |
| P-16 | Every quoted ceiling in §4. | SOURCED (the artifact survey of 2026-08-19, second pass; reproduced verbatim in the memo, not independently re-inspected by this lane) | [OPERATORLESS_AGENTS.md](../regulatory/research-memos/OPERATORLESS_AGENTS.md) §2 |
| P-17 | The reproduced suite counts and the three caveats that travel with them. | SOURCED (a reproduction record in this repository; not re-run for this document) | [ATTESTATION_SUITE_RERUN_2026-08-19.md](ATTESTATION_SUITE_RERUN_2026-08-19.md) |
| P-18 | Every byte offset, length, domain string, tag number, refusal class, and binding rule in §§1 through 3, and the ladder in §5. | PROPOSED (authored here) | This document |

**A stale comment observed in passing, recorded for the artifact lane, not acted
on.** The pinned descriptor literal in `zkoracle-prove/src/zk_leg.rs` carries a
prominent warning that it is the pre-flag-day shape lacking a mandatory
`"challenges"` key, and that the consequence is a panic taking the STARK
injection leg with it. The literal as it stands at the inspected commit *does*
carry `"challenges":0`, so the warning appears to have outlived the re-emit that
fixed it. This document neither relies on nor resolves that; it is noted because
E3 would run into the comment and might believe it.

**Provenance note.** This document was written fresh in this repository. No
source code, fixture, constant, domain literal, or generated artifact was copied
from the Breadstuffs or Dragon's Clutch repositories; identifiers and file paths
are cited, and all descriptions are restatements with attribution, per the
repository-boundary rule in [AGENTS.md](../../AGENTS.md). The domain strings,
byte layouts, tag numbers, and refusal classes proposed in §§1 and 2 are
authored here and are not present in any sibling repository.
