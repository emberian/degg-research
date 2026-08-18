# Draft 4 definitions comment — adversarial claim-audit verdict

Status: adversarial audit lane verdict, 2026-08-18. Target: the Draft 4
definitions filing at degg-research commit `fba1105`
(`docs/regulatory/typst/definitions/{main,metadata,body,sources}.typ` and
`output/pdf/joint-definitions-comment-draft-4.pdf`). The working tree for
these files matches `fba1105` (only `data-reporting/*` files are dirty, owned
by another lane). This lane did not edit the filing and commits nothing.
This is a local research control, not legal advice or a filing authorization.

## Summary table

| # | Category | Verdict |
|---|---|---|
| 1 | Ceiling compliance | **WARN** — no artifact-ceiling breach; two unrecorded hedge-strengthenings and several legal-adjacent pre-answers (F-1 … F-7) |
| 2 | Artifact truth | **PASS** — 26/26 tests independently re-run green; kernel API confirmed; one stale-pin note (N-1) |
| 3 | Identifier exactness | **PASS** — one quote-glyph nit (N-3) |
| 4 | Note/source integrity | **PASS** |
| 5 | Keep-outs | **PASS** |
| 6 | Safety marks | **PASS** |
| 7 | Substance regression | **PASS** — nothing substantive lost; the one dropped word is F-1 |
| 8 | Register spot-check | **PASS** |

Overall: **PASS WITH WARNINGS.** No fabricated evidence, no artifact
overclaim, no keep-out breach, no identifier error, no safety-mark failure.
The earned findings are register upgrades the writer's own ledger
(`DRAFT4_CLAIM_AUDIT.md`) failed to record, plus a handful of sentences that
pre-answer classification questions the filing elsewhere insists on leaving
open. All have one-line fixes.

## Findings, ordered by severity

### F-1 (Medium): "necessarily" silently dropped, twice — unrecorded register upgrade

- Where: `docs/regulatory/typst/definitions/body.typ:236` — "Technical
  incompleteness is not economic incompleteness." — and `body.typ:410-411` —
  "technical incompleteness is not economic incompleteness."
- Draft 3 (both occurrences, `git show HEAD~1` lines 107 and 259): "Technical
  incompleteness is not **necessarily** economic incompleteness."
- Governing ceiling: DRAFT3 V-16 — "Moving a claim to a stronger register is a
  material upgrade requiring audit." `DRAFT4_CLAIM_AUDIT.md` does not record
  this change anywhere.
- Assessment: on the merits the flat form is defensible — the same key_point
  immediately supplies the existence proof in both directions (economics fixed
  while computation incomplete; identical arithmetic unfunded and inert). But
  it is exactly the kind of subtle strengthening the ledger exists to catch,
  and it was not caught.
- Minimal fix: restore "necessarily" in both places, **or** add a ledger row
  recording and justifying the upgrade. Either closes the finding.

### F-2 (Medium): "Prefunding materially changes" vs Draft 3 "can materially change" — unrecorded upgrade

- Where: `body.typ:268-269` — "Prefunding materially changes credit and
  customer-protection risk, but it should not be assumed to decide the product
  category by itself."
- Draft 3 (line 130): "Prefunding **can** materially change credit and
  customer-protection risk…"
- Governing ceiling: DRAFT3 V-16 (register upgrade requires audit); the claim
  is INFERRED-family economics with no local gate.
- Assessment: economically uncontroversial for full prefunding, but flatly
  asserted where Draft 3 hedged, and unrecorded in the Draft 4 ledger.
- Minimal fix: restore "can materially change", or record the upgrade.

### F-3 (Medium): "a funded deposit does [bind]" — a formation conclusion inside the criterion that asks the formation question

- Where: `body.typ:257-258` — "In the worked example, the published template
  binds nobody, while a funded deposit does."
- Governing ceiling: DRAFT4 ledger, not-locally-VERIFIED table — "Any legal
  classification … conclusion: Draft 4 states questions, factual criteria, and
  express non-claims only … Human counsel gate." Criterion 1's own text
  (`body.typ:252-255`) says binding effect is the thing to be *asked*.
- Assessment: new in Draft 4. Whether funding creates an *enforceable* right is
  precisely the legal fact the criterion says should be determined; the
  sentence answers it for the worked example. A hypothetical's author may
  stipulate facts, and "binds" here can be read as the stipulated
  code-enforced lock — but the word chosen is the legal one.
- Minimal fix: "…while a funded deposit locks collateral that only the
  market's terms can move." (states the stipulated fact, leaves "binding" to
  the criterion), or flag for counsel review explicitly.

### F-4 (Low): "performance of the instrument created earlier" presupposes instrument creation

- Where: `body.typ:154-157` — "On its face this is performance of the
  instrument created earlier, not the creation of a new one…"
- Governing ceiling: same counsel-gate family as F-3; the stage table's own
  settlement row (`body.typ:173`) poses this as an open question.
- Assessment: hedged by "On its face" and the immediately following
  counter-design clause; still pre-answers the question and presupposes "the
  instrument" exists. Earlier stages carefully treated *whether/when* an
  instrument arises as open.
- Minimal fix: "performance of whatever instrument was created earlier".

### F-5 (Low): "does not yet look like an agreement, contract, or transaction"

- Where: `body.typ:126-127` — "Whatever this text is, it does not yet look
  like an agreement, contract, or transaction, because there are no parties."
- Governing ceiling: counsel-gate family (legal inference stated in prose).
- Assessment: doubly hedged ("Whatever this text is", "look like") and grounded
  in stipulated facts (nobody signed, deposited, promised, or can cause value
  to move). Acceptable, but it is the filing's first pre-answer; counsel
  should see it.
- Minimal fix: none required; optionally recast as the stage-table question.

### F-6 (Low): "The worked example references a commodity price" — flat legal characterization

- Where: `body.typ:288-289` — "The worked example references a commodity
  price; the identical program pointed at a single issuer's security would
  raise a different nexus."
- Governing ceiling: counsel gate. Draft 3 used "commodity price" only inside
  a hypothetical paired-example row; Draft 4 applies it to the specific worked
  design (a digital-asset TWAP in an onchain pool).
- Assessment: this characterization *concedes* CFTC-relevant facts rather than
  claiming an exemption, so the risk direction is benign, but it is still a
  legal label asserted as fact.
- Minimal fix: "The worked example references a digital-asset price…" (keeps
  the nexus contrast, drops the characterization).

### F-7 (Low): "No person chooses the value" brushes the forbidden "cannot be manipulated" family

- Where: `body.typ:88-89` — "No person chooses the value: a transaction either
  carries evidence that satisfies the frozen rule or it is rejected."
- Governing ceiling: SUBMISSION_WEEK_PLAN shared release gate — no
  "cannot be manipulated" claim without an exact supported basis; DRAFT3
  downgrade table — oracle validity has no local gate.
- Assessment: within the frozen-terms bullet list, and the colon-clause
  narrows the meaning to observation-step non-discretion, which the design
  does stipulate. But flatly read, a well-funded trader *can* steer the
  underlying TWAP; only oracle discretion is excluded, not influence over the
  referenced price. The finality stage (`body.typ:147-152`) partially
  recovers this by asking what happens when the source fails.
- Minimal fix: "No person exercises discretion in the observation: …"

### F-8 (Nit): "as the worked example's frozen edge-case rules do"

- Where: `body.typ:279-280`, claiming the example's terms specify ambiguity,
  non-fill, correction, fork, dispute, and expiry behavior. The narrated terms
  give three examples plus the universal stipulation "a deterministic rule for
  every edge case" (`body.typ:68-70`); dispute/repair machinery appears only
  inside the finality-stage *question* (`body.typ:151-153`). Covered by the
  stipulation; a fork rule is never actually exhibited. No fix required;
  aware-and-accepted is fine.

### F-9 (Nit): "Any staged automated product can be located on it"

- Where: `body.typ:159-160`. Universal framework-generality claim, new in
  Draft 4 (Draft 3 said only "the same control structure can appear at
  materially different stages"). Rhetorical, not artifact-backed; low risk.

## Notes (not findings)

### N-1: dragons-clutch has moved past the ledger's pinned commit — evidence still transfers

`DRAFT4_CLAIM_AUDIT.md` V-17 pins commit `fa4efb4`. The repo is now at
`245c965` (two commits later: `4b0df95` BatchRelationV1 design PROPOSED,
`245c965` toolchain-status sync). Verified: `fa4efb4` is an ancestor of HEAD
and `git diff --stat fa4efb4..HEAD` touches only docs, READMEs, `toolchain/`,
and `verus/` READMEs — **zero** changes under `crates/*/src` or any Cargo
manifest. I independently re-ran the V-17 evidence at HEAD `245c965`
(clean tree), with `--offline --locked`:

- `crates/clutch-kernel`: **7 passed, 0 failed**
- `crates/clutch-accumulator`: **10 passed, 0 failed**
- `crates/clutch-batch`: **9 passed, 0 failed**

26/26 as claimed. Recommend the ledger note the re-pin or the transfer.

Sub-note: V-17's parenthetical "(neither toolchain is installed per the
README)" is now **stale** — Verus 0.2026.08.15 is installed and pinned per
`/Users/ember/dev/dragons-clutch/toolchain/PINNED_PROOF_TOOLS.md`, which
itself states "It records no verification." The filing's "tested, not
formally verified" remains exactly correct and must NOT be upgraded on the
strength of installed tools.

### N-2: kernel API and check attribution verified

`grep 'pub fn' /Users/ember/dev/dragons-clutch/crates/clutch-kernel/src/lib.rs`:
`validate`, `new`, `required_collateral`, `check_invariants`, `split`,
`merge`, `materialize`, `dematerialize`, `resolve`, `redeem_internal`,
`redeem_external`. Case-insensitive grep for "transfer": **zero hits** — the
writer's removal of "transfer" from the accounting list is justified, and the
body's list (deposit, recombination, claim materialization, resolution,
redemption; `body.typ:100-102`) matches real transitions (omitting
`dematerialize` is a narrowing, permissible). "Conservation and pool-coverage
checks" is supported: pool coverage in `check_invariants`
(`lib.rs:253-264`, `collateral < required` refused), conservation in kernel
tests (`complete_split_merge_preserves_claims_and_collateral` `lib.rs:507`,
`materialization_is_supply_neutral_and_round_trips` `lib.rs:523`) and in
batch verification (`ConservationFailure`, batch `lib.rs:212,297`).
`#![no_std]` and `#![forbid(unsafe_code)]` at kernel `lib.rs:1-2` as the
ledger claims. PDF SHA-256 matches the ledger
(`51530c26…be68`), 9 pages, and the Draft 3/Draft 2 definitions PDFs are
untouched on disk.

### N-3: identifier and quote-glyph nit

`metadata.typ:7-8`: joint title, `CFTC RIN 3038-AF71 | SEC File S7-2026-21 |
SEC RIN 3235-AN79` all exact against SUBMISSION_WEEK_PLAN §1;
`FR Doc. 2026-12743` present in `sources.typ:9`; deadline "on or before
August 24, 2026" in entry 1 and the final-source gate. Nit: the proceeding
string quotes "Swap" with straight double quotes where the plan renders the
official title with single quotes inside the outer quotation — glyph-level
title exactness is already assigned to the pre-filing current-docket human
gate; noting only so the gate checks it. The Fed. Reg. page numbers
(91 FR 37873, 91 FR 35806) are post-training-cutoff and not locally
verifiable; they carry retrieval dates and "recheck before filing" markers,
which is the correct register.

### N-4: category 4/5/6 mechanics

- Every `note_ref(n)` in `body.typ` (1×2, 2, 3, 4, 5, 6) resolves to source
  entry *n*; no orphan entries in either direction. Entry 6 carries
  "Proposed, not current law." (`sources.typ:44`). Entries 5-6 are
  character-identical to the already-audited IAC entries 10-11. V-21's claim
  "entries 1-4 unchanged except FR Doc. added to entry 1" confirmed by
  `git diff HEAD~1..HEAD`.
- Keep-outs: zero occurrences of reporting architecture (no "report" token in
  the filing at all), no venue launch request (registration-status disclaimer
  intact, `body.typ:348-350`), no fees/revenue/token/treasury economics
  ("Dregg" appears once as a project name, `sources.typ:50`), no
  Dark/Shielded taxonomy, no exemption argument (expressly disclaimed,
  `body.typ:116, 398-401`).
- Safety: "DRAFT 4 FOR REVIEW - NOT FILED" renders in the header of all 9
  PDF pages (pdftotext count: 9) via `template.typ` header; the public-review
  warning block renders; identity placeholders intact in `metadata.typ:9-11`,
  `body.typ:426-428`, and in the rendered PDF; secret/personal-data scans of
  `definitions/` and `shared/` are clean; no local absolute paths in the
  filing; no claim-label strings in source or PDF.

### N-5: substance-regression inventory (category 7)

Checked element-by-element against `git show HEAD~1:…/body.typ`: the staged
formation table (9 rows, unchanged), the five-element analysis (element 5's
conditional wording intact), the milestone-matrix recommendation key_point,
the core-distinction key_point (now concretized; sole loss is F-1's
"necessarily"), the limits section (strengthened: adds "none of their
properties is offered as determining a legal classification"), the
paired-examples table (6 rows, unchanged), all seven criteria, the 6-item
requested work product, and the instrument/venue/intermediary/clearing
separation with the software-activity paragraph — all present. Dropped: only
Draft 3's opening context sentence (its "event contracts" content resurfaces
in the scope paragraph) and the sentence "Neither object is offered as a
legal category," whose substance survives distributed at `body.typ:204-208`
and `body.typ:403-405`. No substantive analytical element lost; the worked
example, stage narrative, scope/40.11 paragraph, and traceability appendix
are additive.

### N-6: register spot-check (category 8)

No label prefixes anywhere; no meta-commentary about the research's own
epistemics in body prose (the appendix's basis-recording is the sanctioned
mechanism and stays in appendix register); the document leads with the
concrete example and every criterion is grounded in it. "an honest partial
state" (`body.typ:220-221`) is Draft 3 carry-over describing model semantics
— fine. The writer's line anchors in `DRAFT4_CLAIM_AUDIT.md` were spot-checked
and are accurate.

## Disposition

The filing survives adversarial reading. Before the human gates: fix or
ledger-record F-1 and F-2 (one word each), consider the one-line fixes for
F-3, F-6, and F-7, and hand F-3 … F-6 to counsel as the sentences closest to
legal conclusions. Re-pin or annotate V-17's commit reference (N-1). Nothing
found blocks Draft 4 from proceeding to counsel and identity gates.
