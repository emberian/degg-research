# Draft 5 packet — consolidated adversarial audit verdict (found-and-fixed)

Status: adversarial audit lane verdict, 2026-08-18. Target: the complete
Draft 5 packet at degg-research commit `2c00da2` (all four filings'
`docs/regulatory/typst/{definitions,data-reporting,iac,iac-cover}/` sources
and the four `output/pdf/*-draft-5.pdf` review PDFs). Mid-audit the mandate
changed from audit-and-propose to audit-and-repair: findings with clear
minimal fixes were applied directly to the Draft 5 sources, the PDFs were
rebuilt, and the consolidated claim ledger was created. Nothing was
committed (the coordinator commits), and nothing was filed, sent, or
published. This is a local research control, not legal advice or a filing
authorization.

Lane discipline: this lane wrote none of the Draft 5 text under audit; the
repairs below are the audit's own minimal fixes, each recorded with the
offending text, the governing ceiling, and the applied edit. Draft marks
("DRAFT 5 FOR REVIEW - NOT FILED"), [FULL NAME] placeholders, and every
claim ceiling are unchanged.

## Summary table

| # | Category | Definitions | Data-reporting | IAC statement | IAC cover |
|---|---|---|---|---|---|
| 1 | Ceiling compliance | **PASS** | **PASS** | **PASS** | **PASS** |
| 2 | Cross-document consistency | **PASS** | **FIXED** (C2-1, article alignment) | **FIXED** (C2-1) | **PASS** |
| 3 | Artifact truth / monotone coupling | **FIXED** (M-1) | **FIXED** (M-2, M-3, M-9) | **FIXED** (M-4, M-5, M-6) | **PASS** |
| 4 | Identifier exactness | **PASS** | **PASS** | **PASS** | **PASS** |
| 5 | Note/source integrity | **PASS** | **PASS** | **PASS** | **PASS** |
| 6 | Keep-outs and safety marks | **PASS** | **PASS** | **PASS** | **PASS** |
| 7 | Substance regression vs Draft 4 | **PASS** | **PASS** | **PASS** | **PASS** |
| 8 | Register | **PASS** | **PASS** | **PASS** | **PASS** |
| 9 | Ledger consolidation | **BUILT** — `DRAFT5_CLAIM_LEDGER.md` | | | |

Overall: **PASS AFTER REPAIRS.** No ceiling breach, no lost hedge, no
keep-out violation, no identifier error, no safety-mark failure, and no
analytical substance lost in the Draft 5 tightening. The earned findings are
one record-integrity defect in the Draft 4 ledger chain (already surfaced by
the rewriter's own delta and verified here), one cross-document article
inconsistency, and a set of engineering-state couplings under the
monotone-coupling rule — all fixed in place.

## Verification of the rewriter's delta ledger (`DRAFT5_CLAIM_DELTA.md`)

Every checkable claim in the delta was independently verified before any
repair was made:

- **Word counts**: `wc -w` on `git show` of the four bodies reproduces
  3,853→3,750 / 4,476→4,366 / 4,288→4,247 / 386→386 exactly; `pdftotext`
  word counts reproduce 4,321 / 4,923 / 4,935 / 480 exactly.
- **PDF hashes and pages**: all four committed Draft 5 PDFs match the
  delta's SHA-256 values and page counts (9/10/9/1); Draft 2/3/4 PDFs are
  untouched on disk (mtimes pre-date the Draft 5 build). Rebuilding all four
  from the committed sources reproduced byte-identical extracted text.
- **"Limitations moved: none"**: verified by full `--word-diff` of each body
  `f087fea..2c00da2`. Every cut is meta-discourse, restatement, dash
  re-encoding, or an intensifier; every protected phrase in the delta's
  verbatim lists was found in source and rendered text. The delta's
  itemized non-material cuts are a complete inventory of the diffs.
- **Three material changes** verified: (1) definitions Criterion 4
  "commodity price" → "onchain digital-asset price" (fix F-6) with the
  paired-example row's "commodity price" correctly retained; (2) the F-9
  universal ("Any staged automated product can be located on it") deleted;
  (3) the template cover-warning identity-association sentence removed —
  and the delta's claim that the Draft 4 addendum's F-6 record was wrong is
  TRUE: `git show f087fea:…definitions/body.typ` line 293 and the rebuilt
  Draft 4 PDF (`d7b4ab9e…`) both still read "commodity price" (finding R-1).
- **Dash encoding**: zero literal em-dash characters remain in any typst
  source; `./scripts/check.sh` passes.
- **note_ref integrity**: definitions uses refs {1×2, 2, 3, 4, 5, 6} against
  entries 1-6; data-reporting {1, 2, 3, 4} against 1-4, each once; IAC all
  eleven entries (11 twice, as recorded); cover its single footnote. No
  orphans in either direction. All four `sources.typ` files are byte-
  unchanged from Draft 4 (verified with `git diff --quiet`).

## Findings, by severity

### R-1 (Medium, record integrity — fixed by annotation): the Draft 4 addendum recorded F-6 as applied when it was not

- Where: `DRAFT4_CLAIM_AUDIT.md` post-audit addendum ("All six proposed
  fixes were applied … F-6"). The committed Draft 4 definitions body
  (`f087fea`, line 293) and the rebuilt Draft 4 PDF on disk both still read
  "commodity price"; F-1 through F-4 and F-7 were applied, F-6 was not.
- Governing rule: the ledger chain must be accurate about itself
  (DRAFT3 V-16 family: register/record changes require audit).
- Found by: `DRAFT5_CLAIM_DELTA.md` (honestly disclosed); independently
  verified here against `git show f087fea` and `pdftotext` of the Draft 4
  PDF.
- Fix applied: Draft 5 makes the actual edit (verified in source line
  `definitions/body.typ:284` and rendered text); a record-correction note
  was added to the supersession pointer atop `DRAFT4_CLAIM_AUDIT.md`, and
  consolidated row V-35 in `DRAFT5_CLAIM_LEDGER.md` carries the corrected
  history. The three Draft 4 ledgers were annotated, not rewritten.

### M-group (Medium, monotone-coupling rule): filing sentences coupled to mutable engineering state — fixed

Rule applied (coordinator directive): a filing may depend on engineering
state through exactly one monotone claim — "I have built an offline research
prototype of this design's accounting; it is tested, not formally verified"
— plus the ceiling-mandated present-tense negatives, which the filing-day
gate re-verifies. Sentences whose truth tracked the repositories' current
API surface or simple-present behavior were rewritten to built-anchored
(historically true forever) or design-anchored (frozen inside the filing)
form. The live demonstration of the risk: dragons-clutch HEAD moved to
`d60ccf3`, whose kernel added `transfer_internal` and `redeem_complete_set`
— the Draft 4 writer had curated an operation list against the kernel of
that hour ("transfer" deliberately removed because the kernel then lacked
it); the kernel now has a transfer transition. No Draft 5 sentence was
actually false at HEAD (verified: both new transitions call
`check_invariants` before their first write, so even the universal "around
every transition" happened to hold), but the sentence-shapes were the
defect. All fixes are narrowings or register decouplings; no ceiling moves.

| ID | Where (post-fix line) | Offending Draft 5 text | Fix applied |
|---|---|---|---|
| M-1 | `definitions/body.typ:100-102` | "…kernel with integer-exact arithmetic covering deposit, recombination, claim materialization, resolution, and redemption, plus its conservation and pool-coverage checks…" — prototype-API enumeration ("claim materialization" appears nowhere in the filing's own worked example; the list invites re-curation per commit) | "…covering the worked example's accounting --- deposit, recombination, resolution, redemption --- with its conservation and pool-coverage checks…" (list now names design terms frozen in the filing itself) |
| M-1 | `definitions/body.typ:444`, `data-reporting/body.typ:509` (appendix rows) | same enumeration + "runs offline with passing deterministic tests" (simple present) | "claim materialization" dropped; "has been implemented offline with passing deterministic tests" (perfect-tense anchor) |
| M-2 | `data-reporting/body.typ:106-109` | "In my research prototype, the batch verifier accepts a submitted clearing only if recomputation from the frozen book reproduces it exactly; it never trusts the submitter's claimed quantities." (simple-present behavior; V-23 wording — and the batch crate has in-flight uncommitted `relation_v1` work) | "In my research prototype, I built the batch verifier to accept a submitted clearing only if recomputation from the frozen book reproduces it exactly, never trusting the submitter's claimed quantities." Consolidated ledger V-38(b) records this as the superseding allowed wording. |
| M-2 | `data-reporting/body.typ:510` | "The prototype's batch verifier accepts…" | "…was built to accept…" |
| M-3 | `data-reporting/body.typ:400-402` | "my observation accumulator refuses to answer a question its retained information cannot support, rather than approximating it" (simple present; V-24 wording) | "I built the observation accumulator to refuse a question its retained information cannot support rather than approximate it." V-38(c). |
| M-3 | `data-reporting/body.typ:511` | "The prototype's observation accumulator refuses questions…" | "…was built to refuse questions…" |
| M-4 | `iac/body.typ:139-143` | "In my prototype the check is structural: … and any state whose collateral falls below that maximum is refused as an invariant violation --- at market construction and around every transition." — an unanchored universal over the prototype's API surface (exactly the sentence `transfer_internal` could have silently falsified) | "In my prototype I made the check structural: … and I built every transition --- market construction included --- to refuse, as an invariant violation, any state whose collateral falls below that maximum." (universal now quantifies over what was built) V-38(d). |
| M-5 | `iac/body.typ:170-174` | "My batch prototype relies on exactly this: it freezes its price grid, tie rule, and remainder rule …, derives …, and accepts … The submitter's claimed quantities are never trusted." | "I built my batch prototype to do exactly this: freeze …, derive …, and accept a submitted clearing candidate only if it matches what the frozen book itself determines, recomputed from scratch --- never trusting the submitter's claimed quantities." V-38(e). |
| M-6 | `iac/body.typ:484` (appendix) | "an observation accumulator that refuses questions its retained information cannot support" | "an observation accumulator built to refuse questions its retained information cannot support" |
| M-9 | `data-reporting/body.typ:512` (appendix) | "The leakage laboratory replays four synthetic traces … and separates …" (simple present; the lab is mutable engineering state in this repository) | "The leakage laboratory was built to replay four synthetic traces against three transcript designs, separating …" (the body's own "I have built a small deterministic laboratory that replays…" at `:281-287` is already perfect-tense-anchored and was left alone) |

Reviewed and deliberately RETAINED (with rationale, so the fix round does
not "fix" them):

- **M-7 — "Its deterministic tests pass" / "with passing deterministic
  tests"** (`definitions/body.typ:103`, `iac/body.typ:97`, appendix bases,
  `sources.typ` local-materials paragraphs). This is the ceilings' own
  allowed formula (V-17: "passing deterministic tests"), it is anchored by
  the document's draft date, and the filing-day gate re-runs the tests and
  re-pins the ledger once. Converting it to past tense would read as
  distancing rather than accuracy.
- **M-8 — the present-tense negatives** ("It is not a deployed system, a
  product, or an offer"; "no deployed program, no keys, no customers, and no
  funds"; "do not presently compose"; "none of them is deployed, funded,
  offered, or operating"). These are mandated by ceilings V-01/V-08/V-16,
  their meaning is inherently current-state, their risk direction is safe,
  and they are re-verified at the filing-day gate. Removing or
  past-tensing them would weaken required disclaimers.
- **Lean-model present tense** ("In my Lean models … an accepted fill is
  exactly the committed transition"). A proved theorem is a fact about a
  mathematical object; the claim does not decay with repository state, and
  the internal ledgers pin the inspection commits. No change.

### C2-1 (Low, cross-document consistency — fixed): Dark-mode definition article

- Where: `iac/body.typ:315` defined Dark as "beyond **the** frozen leakage
  function" while `data-reporting/body.typ:316` and the IAC's own question 6
  (`iac/body.typ:299`) say "beyond **a** frozen leakage function".
- Governing rule: SUBMISSION_WEEK_PLAN release gate — the three privacy
  modes use the repository's exact definitions; the two filings must define
  the term identically.
- Fix applied: `iac/body.typ:315` now reads "beyond a frozen leakage
  function", matching the data-reporting definition and the IAC's internal
  usage. (The indefinite article is also the more accurate general
  definition; the definite form presupposed a particular function.) The
  frozen framing sentence ("Dark is retained solely as a long-horizon
  research boundary…") is untouched in both IAC documents.

### L-group (legal-inference sentences): resolved with recorded reasoning, no edits needed

The counsel-deferral category was dissolved mid-audit (legal analysis is
in-house; a final courtesy review of the finished packet follows). The
sentences the prior audit flagged as closest to legal conclusions were
therefore re-analyzed on the merits rather than deferred. None requires an
edit; the reasoning is recorded here for the final reviewer:

- **L-1** (`definitions/body.typ:122-124`, the former F-5): "Whatever this
  text is, it does not yet look like an agreement, contract, or transaction,
  because there are no parties." Retained. It is doubly hedged ("Whatever
  this text is", "does not yet look like"), grounded entirely in the
  hypothetical's stipulated facts (nobody has signed, deposited, or
  promised, and nobody can cause value to move), and even read strongly it
  asserts only the absence of formation elements no theory supplies without
  parties. The stage table separately keeps the question open.
- **L-2** (`definitions/body.typ:250-251`, F-3 as applied): "the published
  template binds nobody, while a funded deposit locks collateral that only
  the market's terms can move." Retained. The sentence states the stipulated
  mechanical fact (a code-enforced lock) and deliberately avoids asserting
  that funding creates an enforceable right — which Criterion 1's own text
  poses as the question.
- **L-3** (`definitions/body.typ:151-153`, F-4 as applied): "On its face
  this is performance of whatever instrument was created earlier…" Retained:
  "on its face" plus "whatever instrument" avoids presupposing that or when
  an instrument arose, and the counter-design clause preserves the open
  question.
- **L-4** (`definitions/body.typ:283-284`, F-6 as applied in Draft 5): "The
  worked example references an onchain digital-asset price" is a factual
  description, not a legal characterization; the paired-example row's
  "commodity price" (`:332`) is the varied hypothesis of a jurisdictional
  question inside an expressly hypothetical table row, which the prior
  verdict allows.
- **L-5** (`definitions/body.typ:86-89`, F-7 as applied): "This removes
  reporting discretion; it does not prevent trading from influencing the
  underlying price." Retained — the manipulation-immunity disclaimer the
  release gate requires.
- No filing contains counsel-deference language of any kind (verified by
  grep across all typst sources), so no crutch removal was needed. The
  pre-filing legal-review gate itself is carried forward unchanged in
  `DRAFT5_CLAIM_LEDGER.md`; this audit changed who is recorded as doing the
  analysis, not whether the final text gets legal review before filing.

## Category detail

### 1. Ceiling compliance — PASS (all four)

Every material claim was extracted and checked against the strongest allowed
wording in DRAFT3 V-01…V-16 (and downgrade table), DRAFT4 V-17…V-21,
DRAFT4-DATA V-22…V-28, DRAFT4-IAC V-22…V-27, and GUARDED_EVENT_FOUNDATIONS
[C-01]…[C-25]. Specifically hunted and not found: hedges deleted inline
whose limitation survives nowhere (every protected phrase grep-verified in
source and rendered PDF); qualifiers lost in compression (full word-diffs
read line by line — the only strength-adjacent deltas are "can always be
recombined"→"is recombinable" and "possibly much later"→"a later moment",
both downgrades or neutral); table cells asserting more than the prose
(both definitions tables carried forward unchanged; the IAC milestone
table's right column remains questions; appendix rows checked row by row).
The two Draft 4 audit-fix regressions that Draft 5 was obligated to
preserve (F-1 "necessarily" ×2, F-2 "can materially change") are present,
as are F-3, F-4, F-7, and the F-5 sentence. The Draft 5 tightening
introduced zero upgrades.

### 2. Cross-document consistency — PASS after C2-1

Worked-example facts are identical wherever they appear (five bands,
exhaustive disjoint partition, TWAP of a specific digital asset in a
specific onchain pool, one-unit deposit for a complete set, recombination
before resolution, batch venue with stated close and frozen deterministic
clearing rule, frozen observation program, repair period, realized cell paid
from the market-local pool, others expire worthless, no
debt/margin/leverage/liquidation). Guarded-commitment and candidate-result
descriptions are consistent across definitions/data/IAC. Clear/Shielded
definitions are identical; Dark now identical after C2-1. No filing cites
another local draft (grep-verified); the data-reporting classification
scope-out and the definitions registration-status scope-out correctly
partition the two dockets; the IAC statement and cover contradict neither.

### 3. Artifact truth — PASS after M-group; independent re-verification

- `cargo test --offline --locked` re-run by this lane in
  `/Users/ember/dev/dragons-clutch` (HEAD `d60ccf3`, working tree dirty):
  clutch-kernel **16**, clutch-accumulator **10**, clutch-batch **28** —
  **54 passed, 0 failed**. Committed-state counts by source inspection:
  16 + 10 + 9 = 35 (the batch surplus is uncommitted `relation_v1` work —
  19 new tests, a `pub mod` line, and one visibility change; the
  `propose`/`verify` semantics the filings describe are untouched by the
  dirty diff, and the 9 committed batch tests are among the 28 passing).
- The Draft 4 ledgers' pins are stale as expected: kernel is 16 tests, not
  7; `check_invariants` moved to `lib.rs:290`; the kernel API now includes
  `redeem_complete_set` and `transfer_internal`. **No filing states a test
  count** ("deterministic tests pass" only, verified in all four PDFs), and
  **no filing sentence is invalidated by `transfer_internal`** — no "the
  prototype has no transfer" sentence survives anywhere, and the operation
  lists were non-exhaustive even before the M-1 fix. Both new transitions
  call `check_invariants` before their first write (verified in source), so
  the pre-fix "around every transition" was still true; it is now anchored
  anyway (M-4).
- "Tested, not formally verified" remains exactly correct at HEAD: the
  pinned Verus/Rocq record no verification result
  (`toolchain/PINNED_PROOF_TOOLS.md`: "It records no verification
  result."), and the `verus/` tree carries no passing proof. Do not upgrade
  on installed tools. Cross-repo note (not a filing defect): dragons-clutch
  `README.md` "Status" still says "Verus is not yet installed or pinned,"
  contradicting its own toolchain record; the filings do not repeat the
  claim in either direction.
- Batch-verifier and accumulator behaviors re-verified against source
  (`clutch-batch` `propose`/`verify` recompute from the frozen book and
  compare exactly; the accumulator's refusal cases are in its tests);
  required-collateral semantics at HEAD are unchanged from the audited
  commit (max over the immutable payout set in the active phase, realized
  vector after resolution, rounding against the protocol) — identical at
  `fa4efb4`, so the funding-milestone sentence carries no regression.
- Evidence re-pin recorded once in `DRAFT5_CLAIM_LEDGER.md`; after the
  M-group fixes the filings carry no per-commit specifics, which is the
  monotone-coupling rule's intended end state.

### 4. Identifier exactness — PASS (all four)

Checked character-for-character against SUBMISSION_WEEK_PLAN §1:
definitions `CFTC RIN 3038-AF71 | SEC File S7-2026-21 | SEC RIN 3235-AN79`,
proceeding title, FR Doc. 2026-12743 and the August 24 receipt deadline in
source entry 1; data `CFTC RIN 3038-AF70 | SEC File S7-2026-22 | SEC RIN
3235-AN78`, FR Doc. 2026-12742, August 24 in entry 1 and the final-source
gate; IAC `Docket CFTC-2026-1717 | Document CFTC-2026-1717-0001`, the
August 20 meeting in both `proceeding` fields, and the August 27
written-statement deadline in the body (note 9, 91 Fed. Reg. 51697 / FR
Doc. 2026-16328). Question numbers: definitions answers Question 1 and
expressly takes no position on 12-15; data answers 3, 8, 19 only. The
straight-vs-typographic quote glyphs in the definitions `proceeding` string
remain assigned to the filing-day current-docket check, as in the prior
verdict. All identifier lines are byte-identical to Draft 4.

### 5. Note/source integrity — PASS (all four)

See the delta-verification section: every `note_ref` resolves, no orphan
entries, all `sources.typ` files byte-unchanged from their audited Draft 4
state, entry-level "Proposed, not current law" and "recheck before filing"
markers intact, final-source gate notes unchanged.

### 6. Keep-outs and safety marks — PASS (all four)

- Keep-outs (SUBMISSION_WEEK_PLAN §2): definitions contains no reporting-
  architecture detail, venue launch request, or project economics; data
  contains no product-classification position (express scope-out) and no
  claim that Dark satisfies current rules (express denial including the
  submitter's own research); the IAC documents duplicate no full RFC
  answers, contain no confidential implementation detail, and request no
  transaction-specific approval. No DREGG/fee/treasury economics anywhere;
  no "operatorless/approved/compliant/safe harbor/fully verified/cannot be
  manipulated" phrasing (the observation bullet retains the F-7
  discretion-not-immunity split).
- Safety marks, re-verified on the REBUILT PDFs: "DRAFT 5 FOR REVIEW - NOT
  FILED" renders on every page of all four (9/10/9/1 pages, mark count
  equals page count); `[FULL NAME]` placeholders in metadata block and
  signature of each; zero claim-label strings; the removed
  identity-association warning ("permanently link") absent from all four;
  the remaining cover warning and the main public-review warning render;
  no local absolute paths, secrets, or personal data in any source or PDF.
- Template: the only change in `shared/template.typ` since Draft 4 is the
  cover-warning sentence removal; the `filing()` review block never
  contained that sentence and is untouched — verified by direct diff.

### 7. Substance regression vs Draft 4 — PASS

Full word-diffs `f087fea..2c00da2` of all four bodies, read hunk by hunk
(the definitions Draft 4 baseline includes the d2cd46c audit fixes).
Everything analytically load-bearing survives: both definitions tables (9
and 6 rows), the seven criteria, the six requested work products, the
five-element analysis with element 5's conditional wording, both key
points, the data-reporting three-record framework, direct answers, field
and purpose tables, ten-item minimum package, governance failure list,
seven requested actions, the IAC five milestones, both key points, four
separated questions, seven questions for guidance, the reference
architecture, proof/test objectives, publication-to-operation factors, and
all four Limits sections. The only deletions are the delta's inventoried
meta-discourse, intensifiers, and the two deliberate downgrades (F-9
universal; "not hypothetical"). The cover's seven questions are verbatim.

### 8. Register — PASS

No label prefixes, no meta-discourse about the documents' own epistemics
in body prose, no editorializing intensifiers left that earn a finding.
Optional candidates a future pass could weigh, deliberately NOT applied to
avoid churning audited text for taste: "narrower and real" (`iac:37`),
"The stages are not a blur" (`iac:27`), "made cheap" (`iac:166`), "Not
stale --- false." (`iac:185`), "--- a central economic fact about them"
(`definitions:294`), "which is exactly where a regulator wants it"
(`data:376`). Each is doing rhetorical work its sentence arguably needs;
none asserts anything.

### 9. Ledger consolidation — BUILT

`docs/regulatory/DRAFT5_CLAIM_LEDGER.md` created: one ID space resolving
the V-22+ collision (data keeps V-22…V-28; IAC's V-22…V-27 become
V-29…V-34), new rows V-35…V-38 for the Draft 5 changes and this audit's
repairs, a single evidence re-pin section (re-pinned once on filing day),
and the five pre-filing gates carried forward. The three DRAFT4_* ledgers
are marked with supersession pointers (content preserved;
`DRAFT4_CLAIM_AUDIT.md`'s pointer carries the R-1 record correction).

## Mechanical checks recorded for this audit

- Independent test run: `cargo test --offline --locked` per crate manifest
  in dragons-clutch, 2026-08-18 — 16 + 10 + 28 = 54 passed, 0 failed (see
  category 3 for the committed/dirty split). Offline prototype gate only.
- Pre-repair reproducibility: `typst compile` (0.15.0) of all four filings
  from the committed `2c00da2` sources reproduced text byte-identical to
  the committed Draft 5 PDFs, whose SHA-256 values match
  `DRAFT5_CLAIM_DELTA.md` exactly.
- Post-repair rebuild: `./scripts/build-regulatory-pdfs.sh` run 2026-08-18;
  `./scripts/check.sh` passes. Final review PDFs:
  - `joint-definitions-comment-draft-5.pdf` — 9 pages, SHA-256
    `cbe79eddd4df29b47ded5e4350121e772b949591fa713d19a7f8dd3c17eb17a5`
  - `joint-data-reporting-comment-draft-5.pdf` — 10 pages, SHA-256
    `22165e87ed4848bcadfc8fd7dfd16a8f9bca1301c8cdf35d19b6c8fa6d4d6a8b`
  - `cftc-iac-written-statement-draft-5.pdf` — 9 pages, SHA-256
    `680141fc9677f4fb9852384c9a19402c29920baf08f01ac111cf929c68cdb7dd`
  - `cftc-iac-cover-statement-draft-5.pdf` — 1 page, SHA-256
    `58ff8aadd30113e8f8184ecb2e726e8e018ec6300f51ec336b083913baee99f1`
    (cover source unchanged; hash differs from the delta's only through
    rebuild metadata — extracted text is identical)
- Post-repair PDF text checks: all safety marks, placeholders, protected
  phrases, and the M-group replacement sentences verified in the rendered
  text (table cells checked in layout mode; pdftotext's default reading
  order interleaves table columns and soft-hyphenates, which produced two
  false-negative greps resolved by layout-mode extraction).
- Files changed by this lane: `typst/definitions/body.typ` (M-1 ×2),
  `typst/data-reporting/body.typ` (M-2 ×2, M-3 ×2, M-9),
  `typst/iac/body.typ` (M-4, M-5, M-6, C2-1), the four rebuilt draft-5
  PDFs, supersession pointers in the three DRAFT4_* ledgers,
  `DRAFT5_CLAIM_LEDGER.md` (new), and this verdict. `metadata.typ`,
  `sources.typ`, `template.typ`, `main.typ`, the delta ledger, and every
  earlier draft PDF are untouched by this lane. Nothing committed.
- No network, regulator contact, filing, deployment, key access, or
  external publication was performed for this audit.

## Disposition

The Draft 5 packet survives adversarial reading, and the repairs bring it
to the intended end state: every claim at or below its ceiling, hedges
stated once in the right place, filings coupled to engineering state only
through the monotone existence claim plus gate-checked negatives, one
consolidated ledger owning the ID space, and a clean rebuild. Before any
filing edition, the standing gates apply: identity placeholders (user),
in-house legal record plus final courtesy review of the exact final text,
filing-day docket/identifier revalidation with the evidence re-pin, and
the disclosure scan. Nothing found blocks Draft 5 from proceeding to those
gates.
