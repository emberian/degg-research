# Local Lineage and Reuse Boundary

Status: VERIFIED by read-only audits on 2026-08-17, except where marked.

## Dragon's Clutch

Role: greenfield transparent Solana protocol and executable implementation
planning.

Current state:

- AGPL-3.0-or-later intent and strong existing design/legal corpus.
- Architecture, transparent batch relation, path accumulator, evidence matrix,
  benchmark plan, V1 backlog, research agenda, provenance plan, and ADR scaffold
  now exist.
- No program code, deployment, RPC, keys, or regulator contact was introduced by
  the scaffold work.
- Repository is unborn and all files are currently untracked. There is no commit
  anchor yet.

Relationship to this repository:

- Dragon's Clutch consumes stable relation specifications and research verdicts.
- It does not import experimental dark code merely because a design appears
  promising here.
- Its transparent relation is the reference environment for semantics, costs,
  and public settlement.

## Breadstuffs

Role: prior Dregg/DrEX implementation and theory corpus.

Verified capabilities:

- true uniform-price FHE clearing;
- BFV additive aggregation and MPC nonlinear boundary;
- threshold committee lifecycle components;
- private-book proof experiments;
- cleartext market, LP, QP, routing, and certificate solvers;
- Lean market and Dregg metatheory;
- local DrEX and Oracle Pit surfaces.

Verified limits:

- no composed permissionless no-viewer venue;
- nonlinear threshold crossing not closed as a robust independent-party system;
- semi-honest/trusted preprocessing remains in important paths;
- private proof production includes a complete-book viewer;
- no generic Solana or EVM dark-token settlement;
- captured timings are historical repository evidence, not a fresh audit run.

Reuse:

- Public mathematics, mechanism concepts, interface lessons, and tests may be
  freshly rewritten and attributed.
- Narrow first-party AGPL code reuse might be legally compatible, but the mixed
  vendor/provenance graph requires a separate gate.
- No source is copied into Dark Egg Research.

## Minidregg

Role: formal accretion point for proof systems, private-computation semantics,
receipt admission, and guarded transitions.

Verified capabilities:

- mode-indexed private-computation semantics;
- sealed computation and separately authorized disclosure;
- public escrow/order/nullifier/fee/retry semantics;
- guarded advice with eager shape and late value;
- fixed four-owner/four-price BFV input-validity relation;
- zero-valued proof-suite pins that deliberately make missing deployment
  inexpressible.

Verified limits:

- no BFV runtime, FHE evaluation, threshold ceremony, vFHE proof, encrypted
  clearing, accelerator implementation, or chain adapter;
- BFV relation proves input encoding, not clearing;
- current inhabitable private consumer example is declared MPC and Shielded.

License:

- On 2026-08-17 the owner explicitly selected AGPL-3.0-or-later.
- The repository now contains the canonical AGPL text, NOTICE, LICENSING.md,
  README notice, and matching first-party Rust package metadata.
- This resolves the missing first-party license blocker. It does not erase
  Breadstuffs ancestry, third-party rights, or artifact provenance requirements.

## Leanuweave

Role: formal document calculus, gluing, partial computation, and guarded-hole
theory.

Relevant verified work:

- guarded holes as constrained fill positions over merge states;
- admissibility requires both fitting fill and post-merge guard;
- gluing and I-confluence theorems;
- explicit distinction between weak eager-shape/late-witness holes and unsafe
  strong lazy-shape/economic-delta holes;
- partial computations and observer-indexed views.

Relationship:

- It supplies concepts and theorem targets.
- It is not a deployment dependency for Dragon's Clutch.
- Regulatory descriptions must translate the mathematics into economic facts:
  funding, rights, obligations, contingency, payout, tradability, and operation.

## External cryptographic lineage

Zama TFHE/HPU:

- useful research baseline;
- BSD-3-Clause-Clear source does not imply patent clearance;
- current Zama materials state commercial-use patent licensing conditions;
- Breadstuffs contains some Zama-derived semantic ports.

No Zama-derived implementation crosses into this repository.

## Provenance manifest required for any future artifact

Every received artifact records:

- artifact type and semantic role;
- producer repository and immutable commit;
- exact source paths;
- author/copyright statement;
- license identifier and included license texts;
- third-party derivation;
- generator command and environment;
- canonical bytes and SHA-256;
- receiver parser/checker version;
- independently recomputed semantic invariants;
- whether the artifact is public, private, synthetic, or restricted.

Missing provenance is a refusal, not a warning.
