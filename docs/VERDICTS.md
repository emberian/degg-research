# Current Verdicts

Last reviewed: 2026-08-18.

This is the single current-status document. Other documents may explain a
verdict, but they must not silently upgrade it.

## V0 — Research identity

VERIFIED: Dark Egg Research is a research and experiment-design repository. It
does not contain production protocol code, live credentials, a deployed venue,
or a regulator submission.

## V1 — Specialized relations

INFERRED: A specialized market relation is a better near-term research target
than a generic FHE computer. Market clearing has exploitable structure:

- discretized prices;
- additive demand and supply aggregation;
- monotonicity;
- a canonical crossing rule;
- bounded exact allocation;
- conservation;
- a small public output;
- a natural batch boundary.

This structure should be encoded explicitly rather than rediscovered as a long
sequence of generic encrypted instructions.

PROPOSED as a separate first-class target, not an auction example: energy
providers coordinate one physically admissible efficient operating plan while
keeping plant-level cost curves, capacity and minimum output, ramps, outages,
inventory, hedge positions, and other operational state within a frozen
leakage contract. This target needs its own dispatch relation, optimality story,
local-output delivery, physical-source boundary, and settlement join. A shaped
financial payoff or generic market clear does not substitute for it.

## V2 — Existing local capability

VERIFIED by read-only repository audit:

- Breadstuffs contains a true FHE uniform-price clearing implementation with
  historical measurements, BFV aggregation, an MPC comparison boundary,
  private-book proof experiments, a broad cleartext solver/certificate family,
  and extensive Lean market theory.
- Its strongest dark path is not operationally composed. Current boundaries
  include trusted or semi-honest preprocessing, a proof producer that may view a
  complete book, incomplete distributed nonlinear execution, no generic
  Solana/EVM settlement, and no permissionless liveness.
- Minidregg contains proved private-computation semantics, sealed-receipt and
  public-settlement separation, guarded advice, public escrow semantics, and a
  narrow fixed BFV input relation.
- Minidregg does not contain an FHE implementation, a vFHE system, encrypted
  clearing, an accelerator backend, or a Solana adapter.
- No local Google TPU/Jaxite implementation was found. The remembered TPU work
  is external literature, not a hidden local asset.

## V3 — Honest prototype ladder

PROPOSED as a promotion path. Each rung now carries its own status. No rung
inherits the name or guarantees of the next rung, and no rung reached so far
claims, measures, or approximates any privacy property.

1. Clear relation oracle and exhaustive property tests. **VERIFIED at exactly
   the stated bounds**: two implementations of `dark-fba/n4-k4-q15/v0` agree on
   300,436,169 enumerated batches — domains of 214,358,881, 85,525,504, and
   551,784 — with zero divergences of any kind, compared on the complete output
   rather than a summary. Record: `experiments/dark-fba-independent`, its
   `INDEPENDENCE.md` per-file digests at the pre-read boundary,
   `docs/research/DARK_FBA_RELATION.md` §4.1 and §13.6, and C024 in
   `paper/CLAIM_LEDGER.md`. The two halves of that agreement are not equally
   strong, and the ledger's wording governs: allocation/clearing agreement is
   independent, refusal-class agreement is conformance to the frozen §4.1 order.
   Both oracles are Clear-mode semantics: one process sees every order.
2. Shielded fixed-size batch with a declared executor and sealed receipts.
   **VERIFIED as a deterministic offline model at exactly the bounds its tests
   state**: 52 tests, and a 90,082-case differential — 6,561 and 83,521 —
   between the Shielded run and the Clear lowering with zero divergences. Both
   sides share the evaluator by construction, so that differential is evidence
   about the composed assembly path and none at all about the relation's
   semantics; the packet states that boundary itself rather than leaving it to
   be inferred.
   The named executor holds the sealing material and sees every order.
   Record: `experiments/shielded-baseline` and
   `docs/research/SHIELDED_BASELINE.md`.
3. Threshold shielded batch with malicious input validity and explicit abort.
   PROPOSED.
4. Dark computation candidate with frozen leakage and corruption model.
   PROPOSED.
5. Verifiable dark computation binding exact input root, program, output, and
   settlement. PROPOSED.
6. Permissionless availability and liveness study. PROPOSED.

Rungs 3 and 4 stay PROPOSED behind a measurement rather than behind an opinion.
Over one fixed admitted set, rung 2 enumerates 1,125 alternative published runs
and finds 377 of them accepted by every check the model gives the public and all
four owners, including all 33 well-formed public results: the composed verifier
constrains the executor's fill vector and constrains the published clearing tick
and aggregate volume not at all. Dividing decryption across a committee divides
confidentiality trust and leaves that correctness trust exactly where the
measurement finds it, so rungs 3 and 4 need the kind of object rung 5 names — a
verifiable statement about the evaluation — and do not substitute for it.
Record and bounds: `docs/research/SHIELDED_BASELINE.md` §6.2, and
`experiments/shielded-baseline/tests/residual_trust.rs`.

## V4 — Formal methods

PROPOSED: Use complementary proof tracks.

- Executable kernel verification: Verus is the pragmatic first candidate for a
  restricted safe-Rust relation kernel, subject to a dual-toolchain/SBF spike.
- Economic and state-machine theorems: hand-written Lean or Rocq specifications
  should state collateral, settlement, determinism, and refinement properties.
- Rust translation research: rocq-of-rust remains a shadow/refinement study,
  not a release blocker, until its Solana and axiom story matures.
- Minidregg remains the primary accretion point for private-computation and
  proof-system formal initiatives.

## V5 — Privacy and anti-oppression

PROPOSED and normative: The research should minimize unnecessary knowledge and
power. A Dark system must not quietly become a universal surveillance system or
an operator-held master-decryption service.

This does not imply that every design can or should be launched. It means every
mode must say who can learn what, under which process, with what transcript,
what collusion threshold, what user notice, and what failure behavior.

## V6 — Guarded holes and event-contract classification

VERIFIED technically: The weak guarded-hole design commits the transition shape
eagerly and permits only a later value/proof to fill the slot. The fill is
guarded, one-shot, receipt-bound, and fail-closed. Minidregg further models
candidate-independent guarded advice with a fixed code, authority demand,
footprint, guard commitment, effect commitment, deadline, continuation, and
nullifier domain.

INFERRED legally, not a legal conclusion: If such a hole represents a funded
economic claim whose later fill selects, determines, or authorizes a payoff
based on a future contingency, its software name does not prevent it from being
an event-contingent instrument. Classification likely turns on economic
function, rights, obligations, payout, tradability, consideration, and venue
operation. This must be presented to the CFTC as a precise question.

## V7 — CFTC engagement

VERIFIED from current primary sources:

- The CFTC Innovation effort lists crypto assets, AI/autonomous systems, and
  prediction markets/event contracts among its focus areas.
- The Innovation Advisory Committee is scheduled to meet publicly and virtually
  on August 20, 2026, 1:00–4:00 p.m. Eastern.
- Written comments associated with the meeting are due August 27, 2026 in
  docket CFTC-2026-1717. Submitted material may be published publicly.
- Two separate joint CFTC/SEC requests close August 24, 2026:
  - RIN 3038-AF71 / SEC File S7-2026-21 asks for principled definitional
    criteria for swaps, security-based swaps, mixed swaps, event contracts, and
    innovative onchain automated product structures.
  - RIN 3038-AF70 / SEC File S7-2026-22 asks about blockchain transaction
    reporting, public identity and strategy leakage, and machine-readable
    reporting logic.
- The committee membership includes representatives from prediction markets,
  crypto, traditional finance, academia, and public-interest organizations.
- Innovation Task Force meetings are publicly logged.

CHAT-REPORTED, not yet verified in primary materials: granular session times and
titles quoted in chat, and the claim that Walt Lukken chairs the committee.
Official material currently lists Walt Lukken as a member and does not expose
that detailed agenda.

PROPOSED: Prepare a nonduplicative filing set:

1. A joint-definitions comment centered on guarded commitments and candidate
   results.
2. A joint-data comment centered on blockchain provenance, public leakage, and
   machine-readable rules.
3. A broader IAC statement offering technical observations and questions.
4. A private-meeting request and technical appendix describing exact factual
   matrices for Clear Eggs, Guarded Holes, and Dark Eggs research.

VERIFIED (local repository state): Draft 1 Markdown memoranda and archived
earlier review PDFs remain available. The current Typst packet consists of a
one-page Draft 8 IAC cover, an eight-page Draft 8 IAC statement, a nine-page
Draft 8 joint data-reporting comment, and an eight-page Draft 8 joint-definitions
comment. Draft 8 distinguishes gross instrument creation from net contingent
exposure; narrows no-leverage, no-adjudicator, manipulation-cost, and formal-
verification claims; preserves the early-exit lifecycle milestone and Regulation
40.11 scope; and states the transparent-prototype versus proposed-Shielded
boundary. A separate six-page Draft 2 perpetuals research comment is not in the
filing-ready set because its energy-market and real-venue evidence remains
insufficient. All five Typst editions compile to tagged, embedded-font,
letter-size PDFs and remain conspicuously marked as unfiled review drafts. They
are not submission artifacts: identity placeholders, live-docket revalidation,
privacy review, legal review, and a frozen source-build-output manifest remain
open gates.

Nothing has been submitted and no meeting has been requested by this repository.

## V8 — Licensing and provenance

VERIFIED:

- Dragon's Clutch is intended as AGPL-3.0-or-later greenfield work.
- Breadstuffs first-party code is AGPL-3.0-or-later, but has mixed-origin and
  vendor boundaries requiring narrow review.
- The Minidregg owner has explicitly selected AGPL-3.0-or-later. A canonical
  LICENSE, NOTICE, LICENSING.md, README notice, and first-party Rust package
  metadata now record that choice. Breadstuffs ancestry and third-party
  provenance remain mandatory and are not erased by the root license.
- Zama-derived TFHE/HPU work carries patent and transitive-component concerns
  beyond a simple open-source license label.

PROPOSED: Rewrite concepts from public mathematics and specifications. Even
between AGPL-compatible first-party repositories, move code or generated
artifacts only through a signed provenance manifest. No legacy implementation is
imported here.

## V9 — Landed relation and composition packets

VERIFIED at exactly the bounds each packet's own record states, as deterministic
offline measurements of the named crates and of nothing else. Neither packet
claims, measures, or approximates any privacy property.

- **The relation as data.** `experiments/relation-ir` makes the typed relation
  IR real for `dark-fba/n4-k4-q15/v0`: a module value freezing ports and their
  visibility annotations, the numeric parameters, the admission predicates and
  their check priority, the refusal-class vocabulary, and the receipt shapes,
  each with canonical bytes and a digest, plus a Clear lowering that interprets
  the module. Its differential against both existing oracles is zero divergences
  over 2,116,916 enumerated cases; those domains are deliberately smaller than
  the 300M-case run and test the lowering's fidelity to two already
  cross-validated implementations, not the relation's semantics from scratch.
  Visibility annotations are types, not mechanisms: the Shielded and Dark
  lowering targets refuse with typed errors, and receipts here are plain data
  that nothing signs and nothing verifies. Record:
  `experiments/relation-ir/README.md`, its `goldens/v1.txt`, and
  `docs/DARK_RELATION_IR.md`.
- **Inclusion, availability, non-equivocation, and typed abort.**
  `experiments/inclusion-availability` replaces the batch relation's four
  boolean boundary statements with objects a third party can check: an
  append-only Merkle mountain range and admission log with a frozen check order,
  receipts verifiable from a root alone, four equivocation conflict classes with
  content-addressed verdicts, and a typed abort machine with exact integer
  refund conservation on every terminal path. 131 tests, with the exhaustive
  bounds — peak shape to 256 leaves, leaf inclusion to 33, prefix consistency to
  17, honest non-equivocation over all 32 ordered receipt-pair constructions,
  the abort matrix and terminal immovability over all seven abort classes plus
  settlement — stated in the record rather than implied. The seventh abort
  class, `relation-refused`, was added on 2026-08-18 to close composition gap
  C-1, which the rung-2 packet had recorded as a passing test: a publicly
  refused batch previously reached a settled phase and its reserved funds had no
  path back. Availability in this model remains a declared integer and no
  cryptographic, network, or economic property is claimed. Record:
  `experiments/inclusion-availability/README.md` and
  `docs/research/INCLUSION_AVAILABILITY.md`.

## V10 — Regulatory evidence corpora

VERIFIED at exactly the bounds each record states, as offline deterministic
measurements of synthetic corpora. Neither corpus contains market, venue, or
customer data; neither describes any real instrument, venue, or rule text; and
no category label in either is a legal conclusion. Each exists because a
research memo named the computation its own claim needed before that claim
could stand.

- **Bundling invariance.** `experiments/bundling-invariance` enumerates 16,320
  payoff objects over partitions of 2 to 5 cells with per-cell payouts 0 to 3,
  and checks 56,936 vector-level decompositions — 683,232 counting fact profiles
  — against eight candidate classification criteria under both a strict reading
  and each criterion's own declared aggregation story. Four criteria are
  recorded as exhibiting a zero-cost classification arbitrage, one of them only
  under the strict reading, and four are invariant on this corpus; the smallest
  witness in the corpus is the memo's own example, found rather than chosen. A
  separate census settles, by enumeration, how many two-label criteria reading
  only which outcomes an object pays in are invariant at 2, 3, and 4 cells, and
  says nothing about 5. Deliberately absent and named: no price, probability,
  rate, or fee, so the corpus shows an arbitrage costs nothing and never how
  much it earns; and no time, so it does not distinguish decomposition before
  resolution from after. Record: `experiments/bundling-invariance/README.md` and
  `docs/regulatory/research-memos/definitions-q8-event-contracts-and-options-on-securities.md`.
- **Manipulation cost.** `experiments/manipulation-cost` computes, in exact
  integers over synthetic constant-product pools, what it costs to move a
  bucket-sampled time-weighted print across a decision boundary: 1,080 rows over
  four depths, three fees, six window-and-bucket settings, three hold counts,
  and five boundary distances, with two independent computations cross-checking
  each other. The reported figures are a lower bound under an explicitly stated
  recovery model that omits competing flow, arbitrageurs, inventory limits, gas,
  latency, second venues, and detection — every omission raising a real
  attacker's cost. Three results came out against expectation and are recorded
  as such rather than smoothed over, one of them an explicit refutation of the
  hypothesis the experiment was commissioned with: window length does not enter
  the cost at all, requiring a longer hold makes the attack cheaper rather than
  dearer, and a per-bucket-independent estimate overstates rather than
  understates. Record:
  `experiments/manipulation-cost/README.md`, its `PROVENANCE.md`, and
  `docs/regulatory/research-memos/definitions-q15-reference-integrity.md`.

## V11 — Confidential energy dispatch relation

VERIFIED at exactly the frozen bounds, as deterministic offline Clear semantics
and no privacy result: commit `08a0fc357aa32dabf64e2c55f47c33211c148d67`
adds `confidential-energy-dispatch/p3-t3-b2-q4/v0`. It models exactly three
canonically padded provider slots, three periods, two buses, and integer output
from zero through four. Private relation fields are two-segment costs,
capacity/minimum output, ramp bounds, pre-horizon output, forced availability,
provider bus, and local-output recipient binding. Public fields are zonal
demand, system reserve, line limits, instance, epoch, an accepted-input
commitment, and externally supplied finality and payload-availability booleans.
Those booleans are model inputs, not proofs or a data-availability mechanism.

The Clear oracle enforces exact nodal balance, lossless line capacity, the
frozen reserve rule, checked cost arithmetic, global minimum modeled production
cost, deterministic equal-cost priority, padded provider-local dispatch and
credits, and a load debit exactly equal to provider credits and objective cost.
The representative fixture visits 8,025 first-two-provider trajectory pairs,
finds 468 feasible complete schedules, and selects objective 56. A feasible,
conserving, fully recommitted alternative costing 60 is rejected as
non-optimal. Minimum-output and interperiod-ramp counterexamples also show why
independent per-period merit order is not the relation.

VERIFIED within the same implementation boundary: 25 tests pass in debug and
release; a separate full `5^6` Cartesian search agrees with the optimized
derive-the-third-provider search on its two-provider fixture; strict Clippy,
formatting, rustdoc, locked corpus reproduction, and repository checks pass.
The canonical plaintext witness is 156 bytes and every public result frame is
176 bytes. The frame exposes only the relation/domain/input/plan/delivery
commitments, coarse status, and four invariant bits beyond the already-public
domain. Equal frame length proves no runtime, traffic, endpoint, or
cryptographic privacy property.

REJECTED as current claims: Dark execution, FHE or vFHE evaluation, MPC,
succinct proof, encrypted input validity, hiding commitments, constant-time
execution, selective private delivery, data availability, liveness, custody,
physical-market adequacy, and deployment. The verifier establishes optimality
only by repeating the bounded exhaustive search; its counters are a transcript,
not a cheaper certificate. Storage and inventory are first-class future energy
requirements but are deliberately absent until charge/discharge, efficiency,
terminal inventory, reserve, and settlement semantics are frozen together.
Record: `research/confidential-energy-dispatch/README.md` and
`docs/research/CONFIDENTIAL_ENERGY_DISPATCH_RELATION.md`.
