# Dark Egg Research transition handoff

Snapshot date: 2026-08-18. Transition state: **ready for a supervised offline
research handoff; public filings and live systems remain human-gated and
unauthorized**.

Read [`AGENTS.md`](AGENTS.md), [`docs/VERDICTS.md`](docs/VERDICTS.md), and this
file before editing. `docs/VERDICTS.md` is the only operational truth page; this
handoff routes work and must not become a competing verdict ledger. The
repository is unborn and uncommitted, so local byte identities are not a release
or provenance baseline.

## 1. Epistemic and privacy vocabulary

Use the repository's exact claim labels:

- **VERIFIED:** directly inspected source, proof, reproducible measurement, or
  authoritative primary source;
- **SOURCED:** supported by a cited source but not independently reproduced;
- **INFERRED:** a stated deduction from identified premises;
- **PROPOSED:** a design or experiment, not a result;
- **CHAT-REPORTED:** retained for verification, not promoted;
- **REJECTED:** falsified, superseded, or outside the honest claim boundary.

For project-management separation, treat executable local source as
**implementation**, deterministic or formal abstractions as **model**, designs as
**proposal**, and an unmet stop gate as a **blocker**. Never turn one category
into another by changing prose.

Use exactly these privacy modes:

- **Clear:** specified state and computation are public.
- **Shielded:** a named executor, committee, or auditor may learn private inputs.
- **Dark:** no actor learns beyond the frozen leakage function and its authorized
  local output, within an explicit corruption model.

Threshold encryption is not automatically Dark. A regulator/audit master key is
a separately named observable or Shielded modality, not a hidden exception.

## 2. Research thesis and architecture

**PROPOSED:** compile small, economically meaningful private market relations
instead of an arbitrary encrypted computer:

```text
market semantics
      |
typed Dark Relation IR
      |
      +--> Clear reference backend
      +--> proof-carrying verify-not-find backend
      +--> Shielded MPC/threshold backend
      +--> Dark specialized FHE/MPC candidate
      |
input inclusion + availability + non-equivocation + typed abort
      |
local outputs + public receipt + separately specified settlement
```

Every relation must freeze exact integers, capacity, admission, cutoff, leakage,
corruption, abort/retry, availability, conservation, result selection, local
output, and settlement semantics. FHE correctness is not verifiable computation;
an input root is not binding; a proof is not data availability; private clearing
plus public transfers is not end-to-end Dark.

Repository ownership is strict:

- this repository owns verdicts, relation specifications, experiments,
  falsifiers, literature/lineage maps, and regulatory research drafts;
- Dragon's Clutch owns greenfield transparent Solana protocol implementation;
- Minidregg owns formal private-computation/proof-system accretion;
- Leanuweave owns its document-calculus formalization;
- Breadstuffs is prior lineage, not a source tree to copy;
- JOSHI is a potential external policy/execution consumer, never an implicit
  dependency or source of consensus truth.

Cross-repository code, fixtures, constants, generated artifacts, or theorem text
require an explicit provenance manifest. Shared ideas and public mathematics
must be freshly specified with attribution.

## 3. Completed local surface

### 3.1 Research memory and exact relation

- **VERIFIED implementation:** `experiments/dark-fba` is a dependency-free
  offline Rust oracle for `dark-fba/n4-k4-q15/v0`: four padded slots, four owners,
  four price ticks, exact maximum-volume/lowest-tick clearing, exact largest-
  remainder pro-rata allocation, admission refusals, conservation, and byte-
  stable vectors. Nine tests pass. Executable modes are Clear and
  `ShieldedSingleExecutor`; `DarkTarget` deliberately refuses.
- **PROPOSED model:** [`docs/research/DARK_FBA_RELATION.md`](docs/research/DARK_FBA_RELATION.md)
  freezes the intended public/private boundary, local outputs, availability,
  non-equivocation, typed abort, and a `static-active-1-of-4` Dark corruption
  target. No implementation of that target exists.
- **VERIFIED implementation:** `experiments/leakage-lab` projects four synthetic
  traces across public/executor/owner/regulator views for Clear, Shielded, and a
  hypothetical Dark surface. Seven deterministic standard-library tests pass.
- **PROPOSED model:** [`docs/research/DARK_RELATION_THREAT_MODEL.md`](docs/research/DARK_RELATION_THREAT_MODEL.md)
  defines the disclosure budget and enumerates timing, participation, size,
  root, abort, result, local-output, audit, and settlement leakage falsifiers.
- **PROPOSED:** [`docs/DARK_RELATION_IR.md`](docs/DARK_RELATION_IR.md),
  [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md), and
  [`docs/ROADMAP.md`](docs/ROADMAP.md) define the compiler and research ladder.
  No IR compiler, Shielded backend, Dark backend, vFHE system, settlement
  adapter, or permissionless venue exists here.

### 3.2 Regulatory Draft 3 packet

- **VERIFIED local artifact:** four Typst Draft 3 source trees and PDFs exist:
  a one-page IAC cover, the full IAC statement, the joint definitions comment,
  and the joint data-reporting comment. They are conspicuously marked
  `DRAFT 3 FOR REVIEW - NOT FILED`.
- Draft 3 adds the early-exit/compression/unwind lifecycle milestone, current
  Regulation 40.11 versus proposed-rule distinction, DCM/SEF/ECP boundary,
  crypto-native scope limitation, exact filing instructions, and a narrower
  framing of Dark as a research boundary. Regulator-observable Shielded is the
  practical pilot baseline; Dark is not presented as a compliance conclusion.
- **VERIFIED local control:**
  [`docs/regulatory/DRAFT3_CLAIM_AUDIT.md`](docs/regulatory/DRAFT3_CLAIM_AUDIT.md)
  maps 16 material claim families to exact local artifacts and separates
  semantic/formal, admission/control, proof/security, deployment, benchmark, and
  repository-inspection ceilings. Its strongest honest disposition is
  source-grounded formal models plus deterministic offline experiments, with no
  demonstrated end-to-end Dark venue or production reporting system.
- The Markdown drafts remain Draft 1 research memoranda and Draft 2 PDFs remain
  archived. The current source layout and build command are documented in
  [`docs/regulatory/typst/README.md`](docs/regulatory/typst/README.md).
- **BLOCKER:** identity placeholders, legal review, privacy/security review,
  final source traceability, live-docket revalidation, public-disclosure review,
  exact submission route, immutable source/build/output manifest, and explicit
  user approval remain open. Nothing has been filed and no meeting requested.

### 3.3 Sibling formal status: Minidregg

- **VERIFIED local state:** the Minidregg owner selected
  `AGPL-3.0-or-later`. Its current worktree adds root `LICENSE`, `NOTICE`,
  `LICENSING.md`, README notice, and first-party Cargo metadata. Historical
  MIT/Apache grants for two crates are not revoked; Breadstuffs ancestry and
  third-party provenance remain mandatory.
- **VERIFIED source audit, not fresh build:**
  `/Users/ember/dev/minidregg/docs/FORMAL_STATUS_AND_NEXT_PROOFS.md` separates
  semantic/formal (S), admission/control (A), proof/security (P), deployment (D),
  and benchmark (B) ceilings. It records substantial S/A surfaces for guarded
  advice, private turns, escrow, BFV input relations, proof controllers, durable
  state, and other systems work.
- **BLOCKER:** Minidregg contains no FHE executor, vFHE system, encrypted clearing,
  accelerator backend, Solana adapter, or permissionless no-viewer venue. No
  umbrella build was run for the latest dirty worktree. Acceptance of a modeled
  BFV relation is not FHE correctness, confidentiality, native refinement, or a
  Dark system.

Minidregg is a sibling, not a subdirectory owned by this handoff. Continue
private-computation formal work there; summarize only reviewed results in
`docs/VERDICTS.md` and provenance maps here.

### 3.4 JOSHI seam

- **PROPOSED:** JOSHI may later consume Clear or authorized private market
  outputs as one field/flow signal and may submit ordinary bounded intents as an
  external trader. It must not receive a privileged oracle, matching, resolution,
  or settlement role.
- **PROPOSED:** receipt-compiled OCO/bracket/guarded policies are an interesting
  relation family only when the authority, cutoff, inputs, future evidence,
  leakage, cancellation, and effect are typed. A JOSHI precommitment or reference
  is not independently executable authority.
- **BLOCKER:** no completed JOSHI-to-Dark-IR schema, privacy proof, settlement
  join, or production execution integration exists. Keep JOSHI's trading data,
  wallets, configuration, and live services out of this repository.
- **VERIFIED sibling implementation, bounded ceiling:** JOSHI Lane 26 adds
  strict session-close, knowledge-closure, outcome-at-horizon, and interview-
  disposition leaf contracts with eight passing tests. It deliberately has no
  store writer/receipt, core route, production export, real prospective episode,
  or execution authority.
- **VERIFIED sibling audit, blocked outcome:** JOSHI Lane 27 validates an exact
  source-to-census-to-market-to-projection-to-publication prefix with seven
  passing tests, but correctly refuses an end-to-end circulation witness because
  census membership rows are not semantically inspectable, the projection does
  not reference the exact market-state artifact, and the V1 publication receipt
  does not bind exact publication bytes. Those are useful schema requests, not
  evidence to infer the missing joins.
- **VERIFIED sibling implementation, bounded ceiling:** JOSHI Lane 25 exports a
  V8 operational-store cutoff into 14 checked Snapshot V2 Parquet relations,
  validates a restricted derived artifact in Rust and Python, and commits/imports/
  reopens exact bytes while preserving occurrence/content identity. Coverage is
  lossless for its admitted shapes. Protocol/session/choice/outcome/interview
  relations correctly refuse because Snapshot V2 cannot preserve them, and the
  public V1 receipt still omits three occurrence identities.
- **VERIFIED sibling integration status:** the latest Wave 4 readiness run
  closes immutable export/import mappings, restricted-artifact restart readback,
  and exact protocol-registration admission. Its honest root verdict remains
  `useful_partial`: launch-bound pairing, store-derived launch/choice closure,
  prospective choice writers, semantic publication-byte receipts, and outcome/
  interview/knowledge writers remain fail-closed. The current uncommitted witness
  reports component-readiness digest
  `a426a227cd9d9839f7d1b17a69e3770ff79e15271841d385f628a0e9d7b909bf`
  and V8 catalog-migration digest
  `386c4ec473e0bf33408ac91c77aa46b8d3012cd0ec2f14d7c9acf0263d14d1c9`;
  these are snapshot aids, not release provenance.

## 4. Reproduction

From this repository root:

```sh
./scripts/check.sh

cargo fmt --manifest-path experiments/dark-fba/Cargo.toml --check
cargo test --manifest-path experiments/dark-fba/Cargo.toml --offline --locked
cargo run --quiet --manifest-path experiments/dark-fba/Cargo.toml \
  --offline --locked --bin dark-fba-vectors \
  | cmp - experiments/dark-fba/vectors/v1.txt

python3 -m unittest discover -s experiments/leakage-lab \
  -p 'test_*.py' -v

./scripts/build-regulatory-pdfs.sh
```

The repository checker, nine Dark FBA tests, and seven leakage-lab tests passed
on 2026-08-18. The PDF build is deterministic at the source level only after a
clean immutable source revision and exact Typst/tool/font manifest are frozen;
rendering is not filing authorization.

Current stable experiment identity:

- `experiments/dark-fba/vectors/v1.txt` SHA-256:
  `9a00d7393d00b5cca1e1b980a468a48cb7c21053fac8ae9e15abe2ba7fc9a767`.

Latest observed Draft 3 review-byte identities under Typst 0.15.0 after the
visual-QA build are:

- IAC cover, 1 page:
  `937cc67643a641ef6c7f563fea1e7fa065cdb7b2f069674fa61e1726955d437a`;
- IAC full statement, 7 pages:
  `ca43de19c5743c8fff27dfa78fca44fba114c99a472831e2edc8f1285d1e2177`;
- joint data-reporting comment, 7 pages:
  `e41a4295b8d9e3b0250f6b4910e20799ef78cbb361761605aa895ab959d0e737`;
- joint definitions comment, 6 pages:
  `4c3a65e23c33a9d160b6b4397fc61d9d9ce3b9f35cefb324d7d88e920c2d17a9`.

These hashes identify unfiled review bytes only. The source repository is still
uncommitted, Typst reports an unknown tool commit, and rendered PDFs embed build
timestamps, so an unchanged-source rebuild can produce different bytes. None is
a deterministic source attestation or filing/release manifest.

Regenerate and record filing-edition PDF hashes, page counts, exact Typst/tool/
font identity, source revision, and source-tree digest only after text and human
identity are frozen. Do not treat the unborn worktree as that manifest.

## 5. Regulatory calendar and human-only gates

Primary-source research retrieved 2026-08-17 records:

- **2026-08-20, 1:00 p.m. ET:** public CFTC Innovation Advisory Committee
  meeting. Attendance/monitoring is a human decision; panel remarks are not a
  Commission rule or project approval.
- **2026-08-24:** joint CFTC/SEC definitions comment, CFTC RIN `3038-AF71`, SEC
  File `S7-2026-21`, FR Doc. `2026-12743`, must be received.
- **2026-08-24:** joint CFTC/SEC data-reporting comment, CFTC RIN `3038-AF70`, SEC
  File `S7-2026-22`, FR Doc. `2026-12742`, must be received.
- **2026-08-27:** IAC written statement for Docket `CFTC-2026-1717`, Document
  `CFTC-2026-1717-0001`, must be received.

Before any filing, a human must choose identity/affiliation, obtain appropriate
legal review, revalidate the live docket and deadlines, decide the single filing
route for each record, remove placeholders and unnecessary personal/confidential
material, approve the exact final bytes and channel, submit, and retain receipts.
This repository and handoff authorize none of those acts. Public submissions may
be posted without removal of personal or confidential information.

## 6. Prioritized work and file ownership

### P0: transition and filing integrity

1. **Regulatory traceability owner - `docs/regulatory/typst/**`:** map every
   material factual/legal/formal claim in Draft 3 to a primary source or exact
   local source path and commit. Mark unresolved claims; do not silently rewrite
   them as fact.
2. **Filing-release owner - `docs/regulatory/**`, `output/pdf/**`:** after human
   edits and review, freeze source, Typst/tool/font versions, output hashes, page
   counts, and a disclosure checklist. This lane may build artifacts but may not
   file or contact anyone.
3. **Truth owner - `docs/VERDICTS.md`:** reconcile only landed, independently
   checked results. Other documents must link here rather than invent a second
   status page.
4. **Provenance owner - `LICENSE`, `LICENSING.md`, `NOTICE`, lineage docs:** audit
   repository artifacts and sibling references before publication. Minidregg's
   AGPL choice does not erase historical grants, third-party licenses, patents,
   or source lineage.
5. **Human gate:** identity, counsel, regulatory attendance/contact/filing,
   publication, and any public statement remain outside agent authority.

### P1: next research wave

1. **Independent oracle owner - new nonoverlapping experiment path:** implement
   the same `dark-fba/n4-k4-q15/v0` semantics independently and differentially
   compare all bounded books. Do not copy the first oracle.
2. **Relation-IR owner - `docs/DARK_RELATION_IR.md` plus a new isolated compiler
   experiment:** freeze types, visibility, refusal, receipts, canonical bytes,
   and a Clear lowering for only the landed FBA relation.
3. **Inclusion/availability owner - new isolated experiment:** model append-only
   admission, cutoff root, inclusion receipts, equivocation, withholding,
   timeout, and refunds. An opaque root or Boolean flag cannot satisfy this.
4. **Shielded-baseline owner - new isolated experiment:** choose an explicit
   executor/committee, input-validity mechanism, preprocessing and corruption
   model, abort/recovery, and local-output delivery. Call it Shielded.
5. **Backend-bakeoff owner - `docs/FHE_FRONTIER.md` and isolated measurements:**
   compare proof-carrying clear, BFV/hybrid, discrete CKKS, TFHE, MPC, and
   relation-specific candidates using identical vectors and complete setup,
   communication, proof, leakage, failure, patent, and provenance ledgers.
6. **Formal owner - Minidregg, not this repo:** close one exact private-
   computation proof suite and deployed-verifier boundary before any privacy
   promotion. Import only a manifest-bound result summary.
7. **Settlement owner - specification first:** define authorized custody,
   nullifiers, retries, conservation, and public/private transfer leakage as a
   separate relation. Private computation alone does not make settlement Dark.

## 7. Recommended first Claude session

Start with a truth-and-release rehearsal, not a new cryptographic backend:

1. read `AGENTS.md`, `docs/VERDICTS.md`, this file,
   `docs/research/DARK_FBA_RELATION.md`,
   `docs/research/DARK_RELATION_THREAT_MODEL.md`, and
   `docs/regulatory/SUBMISSION_WEEK_PLAN.md`;
2. run the offline checks and confirm the Dark target still refuses;
3. review the latest Draft 3 traceability findings and every human-only filing
   gate; do not contact, file, or publish;
4. reconcile any late agent outputs into `docs/VERDICTS.md` only at their honest
   evidence level;
5. produce a one-page proceed/redesign/refuse memo choosing either independent
   FBA differential closure or relation-IR Clear lowering as the next bounded
   research packet;
6. keep Minidregg, Dragon's Clutch, and JOSHI changes in their owning
   repositories with explicit provenance.

## 8. Authority boundary

Default work is deterministic, offline, and synthetic. Never access keys,
wallets, customer/private trading data, browser sessions, or paid services.
Never contact a regulator, submit a comment, deploy, construct or sign a
transaction, create a market, solicit an order, move funds, use a public RPC,
push/commit/publish, install an unreviewed tool, or mutate a remote host without
explicit current authorization naming the act. Research may challenge a
regulatory model; it does not authorize a live venue or conceal unfinished
privacy, correctness, availability, or legal work.
