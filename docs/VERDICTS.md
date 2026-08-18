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

PROPOSED:

1. Clear relation oracle and exhaustive property tests.
2. Shielded fixed-size batch with a declared executor and sealed receipts.
3. Threshold shielded batch with malicious input validity and explicit abort.
4. Dark computation candidate with frozen leakage and corruption model.
5. Verifiable dark computation binding exact input root, program, output, and
   settlement.
6. Permissionless availability and liveness study.

No rung inherits the name or guarantees of the next rung.

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

VERIFIED (local repository state): Draft 1 Markdown memoranda, three archived
Draft 2 review PDFs, and four Draft 3 Typst review editions now exist. Draft 3
adds a one-page IAC cover, the early-exit lifecycle milestone, Regulation 40.11
scope, exact deadline instructions, and a narrower Dark research boundary. The
Typst editions compile to letter-size PDFs and remain conspicuously marked as
unfiled review drafts. They are not release artifacts: identity placeholders
remain, the repository has no committed baseline, and no source-build-output
manifest has been frozen.

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
