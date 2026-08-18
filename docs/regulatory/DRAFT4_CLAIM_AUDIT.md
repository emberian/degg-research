# Draft 4 filing-preflight claim ledger — Definitions comment

> **Superseded for ID purposes by [`DRAFT5_CLAIM_LEDGER.md`](DRAFT5_CLAIM_LEDGER.md)**
> (2026-08-18). This file remains the authoritative full row text for
> V-17 through V-21; the consolidated ledger owns the shared ID space and the
> Draft 5 rows. Record correction: the post-audit addendum below records fix
> F-6 as applied, but the committed Draft 4 text and rebuilt Draft 4 PDF still
> read "commodity price"; F-6 actually landed in Draft 5 (consolidated row
> V-35).

Status: filing-preflight audit of the Draft 4 DEFINITIONS rewrite, prepared
2026-08-18. This is a local research control, not legal advice, a filing
authorization, a source audit for the sibling repositories, or a conclusion
about any product or jurisdiction.

Scope: this ledger covers only the Definitions filing rewritten in Draft 4
(`docs/regulatory/typst/definitions/{main,metadata,body,sources}.typ` and the
review PDF `output/pdf/joint-definitions-comment-draft-4.pdf`). The other three
filings are owned by other lanes and remain covered by
`DRAFT3_CLAIM_AUDIT.md` until their own Draft 4 ledgers exist.

## What changed in Draft 4 and what the audit must check

Draft 4 removes the inline claim-label prefixes (`SOURCED:`, `VERIFIED (local
research object):`, `INFERRED:`, `PROPOSED:`) from the body prose. The claim
discipline did not go away; it moved into two places:

1. natural hedging language in the prose ("in my Lean models…", "I have
   built an offline research prototype…", "the Commissions ask…",
   "by construction of the terms"); and
2. a traceability appendix ("Appendix: basis of material technical claims",
   `body.typ:431-453`) mapping each material technical claim to its basis.

The audit question for every row below is therefore: **does the Draft 4
sentence, without its old label, still sit at or below the Draft 3 support
ceiling?** Each row records the answer and the operative Draft 4 wording.

Gate letters are the same as Draft 3:

| Gate | Meaning | Does not mean |
|---|---|---|
| S | Lean definition/theorem about a model, with stated premises | deployed cryptography, physical execution, or legal effect |
| A | Lean-owned admission/controller decision over a modeled request | signature security, custody, consensus, or physical CAS |
| B | Reproducible matched measurement or deterministic test corpus | security, anonymity, liveness, or production readiness |
| R | Read-only repository/provenance inspection | independent audit, clean-room status, or public reproducibility |

## Carried-forward rows (Draft 3 ledger, re-checked against Draft 4 wording)

| ID | Ceiling (from Draft 3 ledger) | Draft 4 wording and check |
|---|---|---|
| V-01 | Bounded local-state negatives only; never a universal negative. | Body says "It is not a deployed system, a product, or an offer" (`body.typ:24-26`) and the appendix row limits itself to "a statement about the submitter's own artifacts, not about any third party" (`body.typ:452`). CHECKED: at ceiling; the third-party limitation is now explicit in the filing itself. |
| V-02 | Review artifacts only; a PDF is not a filing. | `metadata.typ` now reads "JOINT PUBLIC COMMENT - DRAFT 4" / "DRAFT 4 FOR REVIEW - NOT FILED"; the template's public-review warning block is unchanged; the Draft 3 and Draft 2 definitions PDFs are untouched on disk and Draft 3 sources are preserved in git history. CHECKED: at ceiling. |
| V-03 | Guarded-hole prototype: keep "weak"/"prototype"/"modeled"; fail-closed wording only; not a legal category or production control. Artifact: `/Users/ember/dev/breadstuffs/metatheory/Dregg2/Exec/GuardedHole.lean:48-70`. Gate S. | Body: "In my Lean models of this pattern, an accepted fill is exactly the committed transition and a guard violation fails closed; a deliberately excluded stronger variant would leave an unbounded value or authority decision to the future filler. These are theorems about a modeled state machine, not deployed controls." (`body.typ:194-199`). CHECKED: at ceiling — "modeled", fail-closed, excluded-stronger-variant, and the not-deployed-controls hedge all survive the label removal. |
| V-04 | Minidregg eager advice/guarded reactions: source-inspection evidence at commit `bf45a611`; never "deployed", "cryptographically bound", "physically atomic". Gate S/A. | Draft 4 does not restate the Minidregg feature list; it relies only on the generic guarded-commitment description covered by V-03's wording plus the sources note "research artifacts reviewed by the submitter" (`sources.typ` local-materials paragraph). CHECKED: below ceiling (claim narrowed, not upgraded). |
| V-06 | Leanuweave candidate-result model: state the set/evaluation-commutes-with-union theorem only; determinacy requires a separately supplied stability/coordination premise; no oracle validity, legal finality, or enforceable selection. Artifact: `/Users/ember/dev/leanuweave/Uwueave/Holes.lean:79-117,555-621,899-904` at commit `f1450667`. Gate S. | Body: "a partial result is the set of answers it could still be, and evaluating a deterministic function over such a set commutes with merging sets by union… Nothing in the model makes one candidate authoritative. Selecting the real answer requires a premise supplied from outside the computation… The model does not implement or validate an oracle, a legal finality process, or an enforceable selection." (`body.typ:210-225`). Appendix row repeats the premise limitation (`body.typ:450`). CHECKED: at ceiling. |
| V-07 | Gluing results are `Spanning`-conditional; never "always glue/converge". | Draft 4 does not mention gluing or confluence at all. CHECKED: below ceiling (claim removed). |
| V-08 | "Does not presently compose", never "cannot compose"; bounded local statement. | Draft 4's negatives are all present-tense and artifact-bounded: "the artifacts behind it are an offline prototype and formal models, not production market infrastructure" (`body.typ:398-405`). No modal "cannot" claims added. CHECKED: at ceiling. |
| V-14 | No "clean room", "no third-party provenance", or "copyright cleared" claims. | Draft 4 makes no provenance claims; the sources section still defers public reproducibility to frozen public commits "if those repositories are public by filing time". CHECKED: at ceiling. |
| V-16 | Preserve the express disclaimers; moving a claim to a stronger register is a material upgrade requiring audit. | Draft 4 preserves, in plain first person: the Question 1 focus and express non-position on Questions 12-15 (`body.typ:8-9`); no jurisdictional-exemption argument and no classification of, or permission request for, any deployed product ("Limits of this comment", `body.typ:396-406`); no request that this proceeding decide any facility's registration status ("Separate the instrument from the system"); crypto-native objective-events scope with the Regulation 40.11 boundary and "This scope choice is not a claim that any example falls outside the CEA or any other law" (`body.typ:109-117`). CHECKED: all Draft 3 limits survive; the scope/40.11 disclaimer is NEW to the definitions filing (it previously lived only in the IAC documents) and is an added restriction, not an upgrade. |

Rows V-05, V-09 through V-13, and V-15 from the Draft 3 ledger concern the
data-reporting and IAC filings or Dark/FHE claims that the Draft 4 definitions
filing does not make. Not applicable here; nothing in Draft 4 touches them.

## New rows for Draft 4

| ID | Material claim family and allowed wording | Exact local artifact/path | Strongest actual gate | Filing boundary |
|---|---|---|---|---|
| V-17 | The worked example's core accounting exists as an offline pure-Rust research prototype whose deterministic tests pass: a `no_std`, `forbid(unsafe_code)` transition kernel (split/deposit, merge/recombination, materialize, dematerialize, resolve, redeem, with `check_invariants` enforcing payout-shape validity and `collateral >= required_collateral`), an observation-accumulator (associativity, determinism, exact-integer TWAP boundaries, refusal cases), and a batch-clearing relation (canonical allocation, forged-candidate rejection, conservation). Allowed wording: "offline research prototype", "pure-Rust", "integer-exact", "passing deterministic tests", "tested, not formally verified", "not a deployed system, product, or offer". | `/Users/ember/dev/dragons-clutch/PROJECT.md`; `/Users/ember/dev/dragons-clutch/README.md` (Status: "offline prototype implementation… tested and linted but not formally verified"); `/Users/ember/dev/dragons-clutch/crates/clutch-kernel/src/lib.rs` (7 tests; `check_invariants` at lines 253-264); `/Users/ember/dev/dragons-clutch/crates/clutch-accumulator/src/lib.rs` (10 tests); `/Users/ember/dev/dragons-clutch/crates/clutch-batch/src/lib.rs` (9 tests). Commit `fa4efb4e5a5a3ef14c6b8b33a949525928ae5a70`, clean tree, tests run 2026-08-18 (26 passed, 0 failed). | B for offline semantics; R for repo status. No S (no formal verification of the Rust), no deployment gate of any kind. | Draft 4 wording (`body.typ:22-26,99-108`, appendix row at `body.typ:451`): "offline research prototype… It is not a deployed system, a product, or an offer, and I do not ask either Commission to approve it"; "Its deterministic tests pass. It is tested, not formally verified. It has no deployed program, no keys, no customers, and no funds". CHECKED: at ceiling. The word "transfer" was deliberately removed from the accounting list because the kernel exposes no transfer transition; the list now matches the actual public API (split, merge, materialize, dematerialize, resolve, redeem). Do NOT add: "verified kernel", "Verus/Rocq verified" (neither toolchain is installed per the README), "solvency proven", "deployed", "operating", or any DREGG/fee/treasury economics (SUBMISSION_WEEK_PLAN keep-out). |
| V-18 | The worked example's market terms (five-band exhaustive disjoint partition; segregated market-local pool; complete-set issuance against full deposit; recombination right before resolution; frozen deterministic observation program; batch close freezing submitted orders; settlement paying the realized cell; no debt/margin/leverage/liquidation) are a description of a hypothetical instrument's terms in a research design. The pool-coverage sentence is expressly "by construction of the terms". | `body.typ:63-117` (the "A worked example" section); design source `/Users/ember/dev/dragons-clutch/PROJECT.md` sections 1, 4, 5, 6. | PROPOSED design description; B only for the prototype checks in V-17. | Never present the terms as an operating or enforced system, a solvency guarantee, an offer, or a collateral-sufficiency proof for production; the Draft 3 ledger's downgrade table ("Production clearing, matching, settlement, collateral sufficiency…") still governs. CHECKED: every operative sentence is in design register ("The market's terms are frozen when it is created", "the terms allow", "The design creates no debt…"), and the section closes with the not-deployed paragraph. No venue launch request, reporting-architecture detail, or project economics appears anywhere in the filing (keep-out column respected). |
| V-19 | The Regulation 40.11 scope disclaimer: current 17 C.F.R. 40.11 plus the pending June 2026 prediction-markets proposal address event contracts and public-interest review; the filing's examples are chosen to stay away from that boundary. | `body.typ:109-117` with `note_ref(5)` and `note_ref(6)`; `sources.typ` entries 5 and 6 (wording copied from the already-audited IAC sources entries 10 and 11; both retrieved 2026-08-17). | SOURCED (cited primary sources, retrieved 2026-08-17). | Entry 6 must keep "Proposed, not current law." Both entries carry "recheck before filing" / retrieval dates; the Draft 3 current-docket gate applies unchanged. CHECKED: wording is identical to the IAC filing's audited entries; the body sentence claims only what the notice addresses, plus a scope choice, plus the express non-claim about the CEA. |
| V-20 | The traceability appendix replaces inline labels and must itself be accurate: each row's stated basis must be the true strongest basis. | `body.typ:431-453` ("Appendix: basis of material technical claims"). | Document-scope check (R). | Row-by-row check performed 2026-08-18: rows 1-5 cite source notes 1-6 (all present in `sources.typ` with retrieval dates); row 6 = V-03 ceiling; row 7 = V-06 ceiling; row 8 = V-17 ceiling; row 9 = V-01 ceiling with the third-party limitation stated in-row. The appendix's definitions of "model theorem" and "prototype test" say "reviewed by the submitter" and expressly deny deployment and independent audit. CHECKED: no appendix row states a stronger basis than this ledger allows. |
| V-21 | Identifier preservation: the Draft 4 filing retains the exact joint title, CFTC RIN 3038-AF71, SEC File S7-2026-21, SEC RIN 3235-AN79, FR Doc. 2026-12743, and the note_ref/source structure (entries 1-4 unchanged except FR Doc. number added to entry 1's detail; entries 5-6 added). | `metadata.typ`; `sources.typ` entries 1-6; PDF text check of `output/pdf/joint-definitions-comment-draft-4.pdf` on 2026-08-18. | R (mechanical text check). | The final-source gate note in `sources.typ` is unchanged. Current-docket revalidation before filing remains a human gate. CHECKED. |

## Claims that are not locally VERIFIED (unchanged families)

The Draft 3 ledger's downgrade table continues to govern. For this filing the
live families are:

| Claim family | Draft 4 status | Required gate |
|---|---|---|
| Identity, affiliation, contact, signature | Placeholders retained (`[FULL NAME]` etc.). | Human identity/authority gate. |
| Any legal classification, registration, CEA/Exchange Act, or 40.11 conclusion | Draft 4 states questions, factual criteria, and express non-claims only; first-person requests, no conclusions of law. | Human counsel gate. |
| Current deadlines, docket identifiers, current rules | Sources retrieved 2026-08-17 with "recheck before filing" markers. | Current-docket gate immediately before filing. |
| Independent public reproducibility of the research artifacts | Sources section still conditions public citation on frozen public commits "by filing time"; body claims only submitter review. | Freeze commits/paths/hashes or keep the limited description. |
| Production clearing, settlement, collateral sufficiency, oracle validity, liveness | Stated only as design terms (V-18) or prototype checks (V-17); the filing's element 5 and request 5 expressly condition any weight on "correctly specified, soundly proved, correctly implemented, and bound to settlement". | No local gate closes these; keep the conditional wording. |

## Mechanical checks recorded for this audit

- `cargo test --manifest-path /Users/ember/dev/dragons-clutch/crates/<c>/Cargo.toml`
  for clutch-kernel, clutch-accumulator, clutch-batch: 7 + 10 + 9 = 26 tests
  passed, 0 failed, on 2026-08-18 at commit
  `fa4efb4e5a5a3ef14c6b8b33a949525928ae5a70` (clean tree). This is the offline
  prototype gate only (V-17); it verifies toy arithmetic and refusal cases,
  not deployment, custody, oracle authenticity, or solvency of anything real.
- `./scripts/build-regulatory-pdfs.sh` (typst 0.15.0): all four filings built
  2026-08-18. Definitions output renamed to
  `output/pdf/joint-definitions-comment-draft-4.pdf` so the archived Draft 3
  PDF is not overwritten. Draft 4 PDF: 9 pages, SHA-256
  `51530c2699f3bc9e0dfc97149be84ae4715e2450fab40fa59c70c99aa786be68`.
- PDF text check on 2026-08-18: no `SOURCED:`/`VERIFIED`/`INFERRED:`/`PROPOSED:`
  label strings remain in the rendered filing; "DRAFT 4 FOR REVIEW - NOT FILED"
  appears in the header of every page; RIN/File/FR Doc. identifiers present.
- No network, regulator contact, filing, deployment, key access, or external
  publication was performed for this audit.

## Pre-filing disposition

The strongest honest overall description of the Draft 4 definitions filing is:
**a plain-English public-comment draft grounded in source-cited regulatory
material, Lean model theorems, and one offline deterministic Rust prototype,
with every deployment, verification, and legal conclusion expressly
disclaimed**. Identity, counsel, current docket, copyright/provenance, final
build, and public-disclosure decisions remain human gates. An adversarial
re-verification lane should start from the traceability appendix
(`body.typ:431-453`), this ledger's V-17/V-18 rows, and the Draft 3 ledger's
downgrade table.

## Post-audit addendum (2026-08-18)

An independent adversarial audit
([DRAFT4_DEFINITIONS_AUDIT_VERDICT.md](DRAFT4_DEFINITIONS_AUDIT_VERDICT.md))
returned PASS WITH WARNINGS. All six proposed fixes were applied to
`typst/definitions/body.typ`: F-1 ("necessarily" restored, twice), F-2
("can materially change" restored), F-3 (funded deposit described as locking
collateral, not as binding), F-4 ("whatever instrument was created earlier"),
F-6 ("onchain digital-asset price", dropping the commodity characterization),
F-7 (reporting-discretion clause narrowed and expressly not a
manipulation-immunity claim). F-5 is left for counsel per the verdict.

N-1: the V-17 evidence was independently re-verified by the auditor at
dragons-clutch `245c965` (26/26 tests; zero source changes since
`fa4efb4`). Correction to V-17's environment note: Verus and Rocq are now
installed and pinned (dragons-clutch `toolchain/PINNED_PROOF_TOOLS.md`);
the filing claim remains exactly "tested, not formally verified" and must not
be upgraded on installed tools alone.

Rebuilt definitions PDF after fixes: 9 pages, SHA-256 `d7b4ab9e6a94b1514915a0ac1f5b935a48da9ac66269c33abd37cc1d042b3f6e`.
