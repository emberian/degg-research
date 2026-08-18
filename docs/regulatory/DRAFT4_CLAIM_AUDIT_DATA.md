# Draft 4 filing-preflight claim ledger — Data-reporting comment

Status: filing-preflight audit of the Draft 4 DATA-REPORTING rewrite, prepared
2026-08-18. This is a local research control, not legal advice, a filing
authorization, a source audit for the sibling repositories, or a conclusion
about any product or jurisdiction.

Scope: this ledger covers only the Data-reporting filing rewritten in Draft 4
(`docs/regulatory/typst/data-reporting/{main,metadata,body,sources}.typ` and
the review PDF `output/pdf/joint-data-reporting-comment-draft-4.pdf`). The
Definitions filing has its own Draft 4 ledger (`DRAFT4_CLAIM_AUDIT.md`); the
IAC documents remain covered by `DRAFT3_CLAIM_AUDIT.md`. Row IDs continue the
shared numbering: V-17 through V-21 belong to the Definitions ledger; this
ledger adds V-22 through V-28.

## What changed in Draft 4 and what the audit must check

Draft 4 removes the inline claim-label prefixes (`SOURCED:`, `VERIFIED (local
research object):`, `INFERRED:`, `PROPOSED:`) from the body prose and
restructures the comment concrete-first: the fully collateralized price-band
market is introduced in the executive summary and walked milestone by
milestone (`= The worked example, viewed as records`, `body.typ:65-138`) as a
reporting scenario — which records exist at each milestone, which are public
ledger bytes, which are confidential linked records, and what a machine could
validate — before the Question 3/8/19 answers generalize it. The claim
discipline moved into:

1. natural hedging language in the prose ("I have built an offline research
   prototype…", "In my Lean models…", "In my research prototype…", "I offer
   these as risk analysis", "It is transcript bookkeeping over synthetic
   data"); and
2. a traceability appendix ("Appendix: basis of material technical claims",
   `body.typ:495-520`) mapping each material technical claim to its basis.

The audit question for every row below is: **does the Draft 4 sentence,
without its old label, still sit at or below the Draft 3 support ceiling?**

Gate letters are the same as the Draft 3 ledger:

| Gate | Meaning | Does not mean |
|---|---|---|
| S | Lean definition/theorem about a model, with stated premises | deployed cryptography, physical execution, or legal effect |
| A | Lean-owned admission/controller decision over a modeled request | signature security, custody, consensus, or physical CAS |
| B | Reproducible matched measurement or deterministic test corpus | security, anonymity, liveness, or production readiness |
| R | Read-only repository/provenance inspection | independent audit, clean-room status, or public reproducibility |

## Carried-forward rows (Draft 3 ledger, re-checked against Draft 4 wording)

| ID | Ceiling (from Draft 3 ledger) | Draft 4 wording and check |
|---|---|---|
| V-01 | Bounded local-state negatives only; never a universal negative. | Body: "It is not a deployed system, a product, or an offer" (`body.typ:25-27`); Limits: "the artifacts behind this comment are Lean models and offline research prototypes, not a reporting system, and none of them is deployed, funded, offered, or operating" (`body.typ:458-461`); the appendix's final row limits the negative to "the submitter's own artifacts, not… any third party" (`body.typ:519`). CHECKED: at ceiling. |
| V-02 | Review artifacts only; a PDF is not a filing. | `metadata.typ` now reads "JOINT PUBLIC COMMENT - DRAFT 4" / "DRAFT 4 FOR REVIEW - NOT FILED"; the template's public-review warning block is unchanged; the Draft 2 and Draft 3 data-reporting PDFs are untouched on disk (`output/pdf/joint-data-reporting-comment-draft-{2,3}.pdf`, mtimes pre-dating this rewrite) and Draft 3 sources are preserved in git history. CHECKED: at ceiling. |
| V-11 | Leakage lab: deterministic projection of four synthetic traces onto three transcript surfaces; never an anonymity, noninterference, cryptographic-leakage, timing, endpoint, settlement, or real-market measurement. Gate B. | Body (`body.typ:283-296`): "a small deterministic laboratory that replays four synthetic trading traces and records, for each of three transcript designs… which fields the design mechanically reveals and which deductions those fields enable… It is transcript bookkeeping over synthetic data. It measures no anonymity, no cryptographic leakage, no timing behavior, no endpoints, and no real market." Appendix row repeats the denial (`body.typ:517`). CHECKED: at ceiling — the Dark surface is described as "one hypothetical design with a fixed disclosure budget," matching the harness's PROPOSED Dark row. |
| V-12 | Proposed leakage budgets are surface contracts; roots are transcript fields, not verified commitments; a label alone does not establish confidentiality. | Draft 4 does not restate the budget contract; it uses the lab only for the revealed-vs-inferred discipline and recommends "enumerate the fields, then defend each one" (`body.typ:294-296`). CHECKED: below ceiling (claim narrowed). |
| V-15 | Guarded-update/candidate-state paragraph: S/A model evidence only; a design pattern, not a deployed reporting adapter, compliance implementation, regulator-access mechanism, or accepted reporting schema. | Body (`body.typ:383-408`): "In my Lean models of guarded commitments, an update's shape… is fixed before the late value arrives; an accepted update is exactly the committed transition, and a violating one fails closed… These are theorems about modeled state machines. They are not a deployed reporting adapter, a compliance implementation, or a proposal that the Commissions adopt any research calculus." CHECKED: at ceiling — the old label's limitation now lives in the prose itself, and the candidate-collapse sentence keeps the explicit-precondition hedge (V-06 ceiling: no oracle validity, legal finality, or enforceable selection). |
| V-16 | Preserve the express disclaimers; moving a claim to a stronger register is a material upgrade requiring audit. | Draft 4 preserves, in plain first person: no reduced-access argument from "onchain" (`body.typ:453-455`); a transaction hash is not a complete report; a proof does not establish the report behind it; zero-knowledge does not eliminate recordkeeping; encryption does not place records beyond lawful process (`body.typ:455-458` and the conclusion's four "should not mean" sentences, `body.typ:469-474`); no claim that any presently available Dark architecture satisfies existing reporting obligations, expressly including the submitter's own research (`body.typ:458-461`); no product-classification position ("classification is the subject of a separate joint request and is outside this comment's scope", `body.typ:461-463` — the SUBMISSION_WEEK_PLAN §2 keep-out for this filing); no request to adopt a proof system, blockchain, or research formalism (`body.typ:463-465`). CHECKED: all Draft 3 limits survive; the "my own research included" clause is an added restriction, not an upgrade. |

Rows V-03/V-04/V-06 (the Lean-model ceilings) govern the guarded-update and
candidate-state sentences via V-15 above. Row V-08's bounded-composition
ceiling is engaged once, in the privacy section: "My own research has not
produced an end-to-end Dark system; its strongest composed paths remain
Shielded" (`body.typ:339-341`) — present-tense, bounded to the submitter's
research, matching the sources.typ local-materials paragraph. Rows V-05, V-09,
V-10, V-13, V-14 are not engaged: Draft 4 does not restate the Minidregg
private-turn model, the CLEARING_V0 relation, the dark-FBA artifact, the
Breadstuffs FHE measurements, or any provenance claim. The batch-clearing
sentences now cite the Dragon's Clutch prototype instead (new row V-23).

## New rows for Draft 4

| ID | Material claim family and allowed wording | Exact local artifact/path | Strongest actual gate | Filing boundary |
|---|---|---|---|---|
| V-22 | The worked-example-as-records walk: the price-band market's terms (segregated pool, complete sets, recombination, batch close, frozen observation program, settlement) restated through a reporting lens, with a hypothetical ledger placement ("this comment places one concrete design on a public ledger… I use it here only as a lens", `body.typ:13-16,26-28`) and per-milestone claims about which records would exist, which would be public bytes, and which would exist only if required to be kept. Milestone vocabulary (publication/funding/close/finality/settlement) follows the committed foundations exposition. | `body.typ:65-138`; design source `/Users/ember/dev/dragons-clutch/PROJECT.md` §§1, 4-6; milestone taxonomy `docs/research/GUARDED_EVENT_FOUNDATIONS.md` §5 (C-24, PROPOSED framing) | PROPOSED design/reporting analysis; B only for the prototype checks in V-17/V-23/V-24 | Never present the walk as a description of an operating venue or an actual reporting system. CHECKED: every milestone paragraph is conditional-design register ("The ledger can carry…", "may never become ledger bytes", "What supervision needs…"); the hedged-position point at funding ("a complete set plus the right to recombine… is fully hedged") matches the Definitions filing's audited V-18 wording; no DREGG/fee/treasury economics, no venue launch request. The Definitions ledger's V-17 wording ceiling for the prototype ("offline research prototype… not a deployed system, a product, or an offer"; tested, not formally verified) is reproduced verbatim in the executive summary (`body.typ:23-27`) and appendix (`body.typ:514`). |
| V-23 | The batch prototype verifies a submitted clearing by full recomputation from the frozen book, never trusting the submitter's claimed quantities. Allowed wording: "In my research prototype, the batch verifier accepts a submitted clearing only if recomputation from the frozen book reproduces it exactly" — offline research code, not a deployed venue. | `body.typ:108-112,377-381` (generalized as "checkable by anyone through full recomputation"), appendix row `body.typ:515`; source `/Users/ember/dev/dragons-clutch/crates/clutch-batch/src/lib.rs` (header, `propose`/`verify`); GUARDED_EVENT_FOUNDATIONS C-19. Tests re-run 2026-08-18: 9 passed at dragons-clutch commit `245c965c559c3d83ad65fa9a9f288a9201b76419`; `crates/clutch-batch/src/lib.rs` unchanged since the audited commit `fa4efb4e` (only `crates/README.md` prose changed between them, and it still states "none of which is verified, deployed"). | B (deterministic tests) + R (source inspection) | Do NOT say "verified", "production matching", "deployed venue", or that recomputation-checking is itself a regulatory requirement satisfied by anything real. CHECKED: both body uses carry "in my research prototype"; the appendix basis says "offline research code, not a deployed venue". |
| V-24 | The observation-accumulator prototype refuses questions its retained information cannot support, rather than approximating; used to motivate distinguishing "the rule rejects this" from "the backend cannot answer this" in the failure taxonomy. | `body.typ:404-408`, appendix row `body.typ:516`; source `/Users/ember/dev/dragons-clutch/crates/clutch-accumulator/src/lib.rs` (header: information-theoretic boundary; refusal cases); GUARDED_EVENT_FOUNDATIONS C-20. Tests re-run 2026-08-18: 10 passed at commit `245c965c`; file unchanged since `fa4efb4e`. | B + R | Prototype behavior only; not an oracle-validity, data-quality, or completeness claim about any real feed. CHECKED: framed as "one further small lesson from my prototypes". |
| V-25 | The leakage-laboratory description and the discipline drawn from it (enumerate revealed fields; separate revealed from inferred). | `body.typ:283-296`; `experiments/leakage-lab/{leakage_lab.py,test_leakage_lab.py,README.md}`; Draft 3 ledger V-11/V-12 | B for synthetic transcript accounting only | See carried-forward V-11/V-12 rows: at ceiling. The identity/strategy leakage analysis itself (`body.typ:231-246`) stays expressly hedged: "I offer these as risk analysis — reasoning about what public fields make inferable — not as measurements of any real market" — satisfying the Draft 3 downgrade-table requirement that empirical leakage claims remain INFERRED/PROPOSED risk analysis. |
| V-26 | Sourced regulatory characterizations newly load-bearing in this filing: part 43 real-time public reporting "already delay[s] the public print of certain large trades and cap[s] the disseminated notional" (`note_ref(2)`); part 45 lifecycle reporting "expects exact creation and continuation data with counterparty and transaction identifiers" (`note_ref(3)`); reported data "held under repository confidentiality and access rules" (`note_ref(4)`). | `body.typ:264-267,275-278`; `sources.typ` entries 2-4 (17 C.F.R. parts 43, 45, 49; retrieved 2026-08-17, "recheck immediately before filing"); appendix rows 2-4 (`body.typ:509-511`) | SOURCED (cited primary sources) | Draft 3 carried entries 2-4 without body references; Draft 4 uses each once, at the level of what the parts facially provide (block-trade time delays and notional cap dissemination under part 43; creation/continuation data and identifiers under part 45; SDR confidentiality/access duties under part 49). No section-level citation, effectiveness claim, or compliance conclusion is drawn. The current-docket gate applies unchanged. CHECKED. |
| V-27 | The traceability appendix replaces inline labels and must itself be accurate: each row's stated basis must be the true strongest basis. | `body.typ:495-520` ("Appendix: basis of material technical claims") | Document-scope check (R) | Row-by-row check 2026-08-18: rows 1-4 cite source notes 1-4 (all present in `sources.typ` with retrieval dates; note_ref/source_entry integrity verified 1-4 ↔ 1-4, each used exactly once); row 5 = V-15/V-03 ceiling; row 6 = V-06 ceiling ("no oracle, finality process, or enforceable selection is implemented or validated"); row 7 = Definitions-ledger V-17 ceiling verbatim; rows 8-9 = V-23/V-24 ceilings; row 10 = V-11 ceiling with the full denial list; row 11 = taxonomy as "proposed analytical terminology… no claim that any Dark system exists, is deployed, or satisfies current rules"; row 12 = V-01 ceiling with the third-party limitation in-row (`body.typ:519`). CHECKED: no appendix row states a stronger basis than this ledger allows. |
| V-28 | Identifier preservation: the Draft 4 filing retains the exact joint title "Joint Request for Comment on Swap and Security-Based Swap Data Reporting", CFTC RIN 3038-AF70, SEC File S7-2026-22, SEC RIN 3235-AN78, and FR Doc. 2026-12742; identity placeholders retained; draft marks updated. | `metadata.typ` (identifiers line unchanged; document_kind/review_label now DRAFT 4); `sources.typ` entry 1 (FR Doc. 2026-12742 added to the detail, mirroring the Definitions ledger's V-21 treatment; entries 2-4 byte-identical to Draft 3; final-source gate note unchanged); PDF text check 2026-08-18 of `output/pdf/joint-data-reporting-comment-draft-4.pdf`. | R (mechanical text check) | Current-docket revalidation before filing remains a human gate. CHECKED: RIN/File/RIN identifiers on the title block and in source note 1; FR Doc. number present; `[FULL NAME]` / `[AFFILIATION…]` / `[PUBLIC-CONTACT-SAFE EMAIL]` placeholders intact in metadata and signature block. |

## Claims that are not locally VERIFIED (unchanged families)

The Draft 3 ledger's downgrade table continues to govern. For this filing the
live families are:

| Claim family | Draft 4 status | Required gate |
|---|---|---|
| Identity, affiliation, contact, signature | Placeholders retained. | Human identity/authority gate. |
| Any legal classification, registration, or compliance conclusion | Draft 4 states recommendations, factual criteria, and express non-claims only; classification expressly out of scope (`body.typ:461-463`). | Human counsel gate. |
| Current deadlines, docket identifiers, current rules (parts 43/45/49 included) | Sources retrieved 2026-08-17 with "recheck before filing" markers; newly load-bearing note_refs 2-4 (V-26). | Current-docket gate immediately before filing. |
| Empirical identity/strategy leakage or privacy benefit | Hedged as risk analysis (`body.typ:243-246`); lab described as synthetic bookkeeping (V-25). | No local gate closes these; keep the hedges. |
| Clear/Shielded/Dark taxonomy and any end-to-end Dark statement | Proposed terminology; "I do not claim that question has been answered. My own research has not produced an end-to-end Dark system" (`body.typ:338-341`). | An implemented backend, frozen leakage function, corruption model, and full-surface analysis — none present. |
| Cryptographic commitment binding, proof soundness, confidential regulatory access | Stated only as what proofs *can* establish, with the proof-is-not-the-evidence paragraph and its authority/custody caveats intact (`body.typ:203-227`). | No local P/D gate closes these. |
| Proof establishes the report, onchain means reported, encryption means unavailable to lawful process | Explicitly denied (`body.typ:451-465,467-474`). | Preserve the denials; any contrary sentence is rejected/overbroad. |

## Mechanical checks recorded for this audit

- Build (exact command): `typst compile --root /Users/ember/dev/degg-research
  /Users/ember/dev/degg-research/docs/regulatory/typst/data-reporting/main.typ
  /Users/ember/dev/degg-research/output/pdf/joint-data-reporting-comment-draft-4.pdf`
  (typst 0.15.0), run 2026-08-18. Only this filing was built;
  `scripts/build-regulatory-pdfs.sh` was not run or modified, and no other
  filing's sources were touched. Draft 4 PDF: 10 pages, SHA-256
  `263a6f8a9d3c377f7affa09a470cbee440a0bd3675ae8b8c9395e0d811accd25`.
- PDF text check 2026-08-18: no `SOURCED:`/`VERIFIED`/`INFERRED:`/`PROPOSED:`
  label strings remain in the rendered filing; "DRAFT 4 FOR REVIEW - NOT
  FILED" appears in the header of all 10 pages; the joint title, RIN
  3038-AF70, File S7-2026-22, RIN 3235-AN78, and FR Doc. 2026-12742 are
  present.
- note_ref integrity: body uses `note_ref(1)`-`note_ref(4)` exactly once each;
  `sources.typ` defines entries 1-4. All Draft 3 source entries preserved
  (entry 1 detail extended with the FR Doc. number only).
- `cargo test --manifest-path /Users/ember/dev/dragons-clutch/crates/<c>/Cargo.toml`
  for clutch-kernel, clutch-accumulator, clutch-batch: 7 + 10 + 9 = 26 tests
  passed, 0 failed, on 2026-08-18 at dragons-clutch commit
  `245c965c559c3d83ad65fa9a9f288a9201b76419` (working tree dirty only in
  `research/economics/model.py`, which this filing does not cite). The three
  cited crate sources are unchanged since the commit audited in
  `DRAFT4_CLAIM_AUDIT.md` (`fa4efb4e`). This is the offline prototype gate
  only (V-17/V-23/V-24); it verifies toy arithmetic and refusal cases, not
  deployment, custody, oracle authenticity, or solvency of anything real.
- No network, regulator contact, filing, deployment, key access, or external
  publication was performed for this audit. Nothing was committed; the
  coordinator commits.

## Pre-filing disposition

The strongest honest overall description of the Draft 4 data-reporting filing
is: **a plain-English public-comment draft that walks one hypothetical
fully collateralized market through five reporting milestones, grounded in
source-cited regulatory material, Lean model theorems, offline deterministic
Rust prototypes, and one synthetic-transcript accounting exercise, with every
deployment, measurement, compliance, and classification conclusion expressly
disclaimed**. Identity, counsel, current docket, copyright/provenance, final
build, and public-disclosure decisions remain human gates. An adversarial
re-verification lane should start from the traceability appendix
(`body.typ:495-520`), this ledger's V-22 through V-26 rows, and the Draft 3
ledger's downgrade table.
