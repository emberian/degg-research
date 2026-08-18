# Draft 4 filing-preflight claim ledger — IAC written statement and cover

> **Superseded for ID purposes by [`DRAFT5_CLAIM_LEDGER.md`](DRAFT5_CLAIM_LEDGER.md)**
> (2026-08-18). This file's rows V-22 through V-27 collided with the
> data-reporting ledger's IDs and are renumbered V-29 through V-34 in the
> consolidated ledger; this file remains their authoritative full row text.
> The Draft 5 audit's V-38 supersedes the exact V-25 (now V-32) prototype
> phrasings with built-anchored forms.

Status: filing-preflight audit of the Draft 4 IAC rewrite, prepared
2026-08-18. This is a local research control, not legal advice, a filing
authorization, a source audit for the sibling repositories, or a conclusion
about any product or jurisdiction.

Scope: this ledger covers only the two IAC filings rewritten in Draft 4
(`docs/regulatory/typst/iac/{main,metadata,body,sources}.typ`,
`docs/regulatory/typst/iac-cover/{main,metadata,body,sources}.typ`, and the
review PDFs `output/pdf/cftc-iac-written-statement-draft-4.pdf` and
`output/pdf/cftc-iac-cover-statement-draft-4.pdf`). The definitions filing is
covered by `DRAFT4_CLAIM_AUDIT.md`; the data-reporting filing remains covered
by `DRAFT3_CLAIM_AUDIT.md` until its own Draft 4 ledger exists.

## What changed in Draft 4 and what the audit must check

Draft 4 removes the inline claim-label prefixes (`SOURCED:`, `VERIFIED (…):`,
`INFERRED:`, `PROPOSED:`) from both bodies. The claim discipline moved into:

1. natural hedging language in the prose ("In my earliest Lean prototype…",
   "In the model…", "tested, not formally verified", "by construction of the
   terms", "These are theorems about modeled state machines, not deployed
   controls"); and
2. a traceability appendix in the written statement ("Appendix: basis of
   material technical claims", `iac/body.typ:465-489`) mapping each material
   technical claim to its basis. The one-page cover carries no appendix; its
   only sourced claim keeps its inline footnote (cover `sources.typ`).

The statement was also restructured around the five-milestone taxonomy from
`docs/research/GUARDED_EVENT_FOUNDATIONS.md` (publication / funding / close /
finality / settlement), presented through the compressed price-band worked
market. That exposition's own ceilings ([C-01]–[C-25]) were treated as binding
alongside the Draft 3 ledger. The audit question for every row: **does the
Draft 4 sentence, without its old label, still sit at or below the Draft 3
support ceiling?**

Gate letters as in the prior ledgers:

| Gate | Meaning | Does not mean |
|---|---|---|
| S | Lean definition/theorem about a model, with stated premises | deployed cryptography, physical execution, or legal effect |
| A | Lean-owned admission/controller decision over a modeled request | signature security, custody, consensus, or physical CAS |
| B | Reproducible matched measurement or deterministic test corpus | security, anonymity, liveness, or production readiness |
| R | Read-only repository/provenance inspection | independent audit, clean-room status, or public reproducibility |

## Carried-forward rows (Draft 3 ledger, re-checked against Draft 4 wording)

| ID | Ceiling (from Draft 3 ledger) | Draft 4 wording and check |
|---|---|---|
| V-01 | Bounded local-state negatives only; never a universal negative. | Statement: "It is not a deployed system, a product, or an offer" (`iac/body.typ:101-102`); appendix closing row limits itself to "a statement about the submitter's own artifacts, not about any third party" (`iac/body.typ:488`). Cover: "It does not describe a deployed venue, accepted funds, or live orders" (cover `body.typ:4-6`). CHECKED: at ceiling; the third-party limitation is now explicit in the filing itself. |
| V-02 | Review artifacts only; a PDF is not a filing. | Both `metadata.typ` files now read "… - DRAFT 4" / "DRAFT 4 FOR REVIEW - NOT FILED"; the template's public-review warning blocks are unchanged; the Draft 3 and Draft 2 IAC PDFs are untouched on disk and Draft 3 sources survive in git history. CHECKED: at ceiling. |
| V-03 | Guarded-hole prototype: keep "weak"/"prototype"/"modeled"; fail-closed wording only; not a legal category or production control. Gate S. | Statement: "In my earliest Lean prototype of the pattern, a weak guarded hole fixes a field, an actor, a target cell, and a list of guard predicates, and only an integer arrives later; theorems state that an accepted fill is exactly the committed transition and that a guard-violating value fails closed, changing nothing." (`iac/body.typ:115-119`), closed by "These are theorems about modeled state machines, not deployed controls." (`iac/body.typ:127-128`). CHECKED: at ceiling. |
| V-04 | Minidregg eager advice/guarded reactions: source-inspection evidence at stated commit; never "deployed", "cryptographically bound", "physically atomic". Gate S/A. | Statement restates the generalization plainly: committed shape includes type/codec, pre-state root, abstract authority demand, finite write footprint, guard/effect commitments, deadline, replay domain; the late contribution is typed by that exact shape ("cannot be expressed at all", an inexpressibility claim, not a runtime-security claim); one-shot replay-key consumption and footprint framing appear in the settlement milestone (`iac/body.typ:120-128,202-208`). All under "In a current, more general model" / "in my models"; no "deployed"/"cryptographically bound"/"physically atomic" anywhere (PDF text check). The strong-hole anti-model wording ("given no primitive and is refuted in the model as an explicit anti-model") matches GUARDED_EVENT_FOUNDATIONS [C-04]/[C-08]. CHECKED: at ceiling; appendix row (`iac/body.typ:485`) records "source-inspected at stated commits; not deployed controls". |
| V-06 | Leanuweave candidate-result model: set/evaluation-commutes-with-union only; determinacy requires a separately supplied stability/coordination premise; no oracle validity, legal finality, or enforceable selection. Gate S. | Statement: "a partial result is a grow-only set of candidate worlds, and deterministic evaluation commutes with merging such sets by union… demanding one enforceable answer is a coordination requirement" (`iac/body.typ:160-168`); "it does not implement or validate an oracle, a legal finality process, or an enforceable selection" (`iac/body.typ:191-193`); appendix row repeats the limitation (`iac/body.typ:486`). CHECKED: at ceiling. |
| V-07 | Gluing results are `Spanning`-conditional; never "always glue/converge". | The statement makes only the NEGATIVE gluing claim — "balance-type conditions … are exactly the guards that provably do not merge coordination-free: two independently legal spending states can merge into an over-budget one" (`iac/body.typ:210-214`), per GUARDED_EVENT_FOUNDATIONS [C-16] and its budget witness. No positive "guards glue" claim is made, so the `Spanning` hypothesis is not needed for any stated sentence. CHECKED: below ceiling (only the counterexample direction is asserted). |
| V-08 | "Does not presently compose", never "cannot compose"; bounded local statement. | Statement: "Independently provenanced repositories contain separately scoped prototype clearing, proof, and privacy components; they do not presently compose into a production, permissionless, end-to-end Dark market system." (`iac/body.typ:430-433`). Appendix row mirrors it. No modal "cannot" added anywhere; the settlement section's "it predicts the opposite" is about the model's own theorem, not a deployment claim. CHECKED: at ceiling. |
| V-14 | No "clean room", "no third-party provenance", or "copyright cleared" claims. | Neither body makes provenance claims; `iac/sources.typ` still conditions public citation on frozen public commits "if those materials are public by filing time". CHECKED: at ceiling. |
| V-16 | Preserve the express disclaimers; moving a claim to a stronger register is a material upgrade requiring audit. | Draft 4 preserves, in plain first person: no jurisdictional shortcut ("They do not remove an instrument from the Commodity Exchange Act, make venue or clearing functions disappear, or replace surveillance, recordkeeping, customer protection, governance, or Commission access", `iac/body.typ:34-44`); no production infrastructure / customer funds / live orders / deployment-approval request ("Limits and current research status", `iac/body.typ:426-440`); no present Dark compliance ("Nothing in my research has produced a deployed Dark venue, and I make no claim that a Dark architecture satisfies current rules", `iac/body.typ:328-330`; "not a venue label, a compliance conclusion, or a deployment recommendation", `iac/body.typ:379-381`); "Formal verification does not prove legal compliance, and I do not claim that it does" (`iac/body.typ:434-436`); the 40.11 scope limitation with "This scope choice is not a claim that any example falls outside the CEA or any other law" (`iac/body.typ:67-75`); IAC-advisory / not-approval (`iac/body.typ:437-440`, notes 7-8). Cover keeps its own scope non-claim (cover `body.typ:42`) and the frozen Dark framing (cover `body.typ:44-47`). CHECKED: all Draft 3 limits survive the label removal; none moved to a stronger register. |

Rows V-05 and V-09 through V-13 concern FHE/private-turn, CLEARING_V0,
dark-FBA, and leakage-lab claims that the Draft 4 IAC filings do not make (the
Draft 4 statement dropped Draft 3's generic mention of nothing further; no new
claim in those families was added). Not applicable here. V-15 is the
data-reporting filing's row and is untouched by this lane.

## New rows for Draft 4

| ID | Material claim family and allowed wording | Exact local artifact/path | Strongest actual gate | Filing boundary |
|---|---|---|---|---|
| V-22 | The five-milestone taxonomy (publication fixes shape; funding creates exposure and fixes its ceiling; close freezes candidates; finality is a licensed collapse; settlement is a serialized guarded fill) presented as an interpretive framework through the compressed worked market. | `iac/body.typ:77-260` (worked-market walk and table); framing per `docs/research/GUARDED_EVENT_FOUNDATIONS.md` §5 and its [C-24] (PROPOSED: not a legal category, compliance conclusion, or property of a deployed system). | PROPOSED framing over S/A/B components audited separately (V-03/V-04/V-06/V-23/V-24). | The taxonomy is offered as questions for analysis, never as a classification conclusion; the table's right column is phrased as questions (`iac/body.typ:222-231`); the key_point ("What the timeline separates", `iac/body.typ:243-253`) states model-time structure only. CHECKED. |
| V-23 | Sealing/finality claims: in the model, sealing is licensed exactly by a stability premise; an unlicensed seal can be false of the merged state; input stability transports to result stability; determinacy does not survive coordination-free merging. | `iac/body.typ:160-196` ("In the model…" hedges throughout); evidence trail: GUARDED_EVENT_FOUNDATIONS [C-13]/[C-14] (VERIFIED source inspection: `/Users/ember/dev/leanuweave/Uwueave/Holes.lean:856,899,937-993` at commit `f1450667cc87a48706c61f6d5ead71f73ab43fb1`, clean tree). These lemmas extend beyond the Draft 3 V-06 line ranges; the foundations exposition's appendix is the audited basis. | S | Every sentence carries "in the model"; the agnosticism sentence ("deliberately agnostic about what real-world evidence discharges the license") and "The collapse is worth exactly what the evidence licensing it is worth" (`iac/body.typ:188-196`) keep the premise external. Do NOT add: oracle validity, enforceable selection, or any real-world finality claim. CHECKED: at ceiling. |
| V-24 | Balance-guard non-gluing claim: in the model, balance/quota/conservation-type guards provably do not merge coordination-free (two legal states merge over budget); the design therefore serializes settlement. | `iac/body.typ:209-218`; evidence trail: GUARDED_EVENT_FOUNDATIONS [C-16] (`/Users/ember/dev/leanuweave/Uwueave/Gluing.lean:606,627-643`; `Segmented.lean:76`, same commit). | S | Only the counterexample direction is asserted (see V-07 note); "the design accordingly treats settlement as one guarded fill" is a design statement, not a deployed-control claim. CHECKED: at ceiling. |
| V-25 | The compressed worked market and its prototype: five-band exhaustive partition, market-local pool, complete-set issuance and recombination, frozen observation program, batch close and frozen clearing rule, settlement from the pool, no debt/margin/leverage/liquidation; pool coverage "by construction of the terms"; core accounting exists as an offline pure-Rust integer-exact research prototype with passing deterministic tests, tested not formally verified; structural required-collateral refusal; batch verification by full recomputation, submitter quantities never trusted; accumulator refuses unsupported questions. | `iac/body.typ:80-108` (market terms and prototype status), `:141-147` (required-collateral check), `:172-177` (batch recomputation); design source `/Users/ember/dev/dragons-clutch/PROJECT.md`; code basis per GUARDED_EVENT_FOUNDATIONS [C-18]/[C-19]/[C-20]/[C-22] and DRAFT4_CLAIM_AUDIT.md V-17/V-18 (commit `fa4efb4e5a5a3ef14c6b8b33a949525928ae5a70`, 26 crate tests passing 2026-08-18; 52 repo-wide per [C-22]). | B for offline semantics; R for repo status; PROPOSED for the market terms as instrument description. | Allowed wording used verbatim: "offline research prototype", "pure Rust, integer-exact", "passing deterministic tests", "tested, not formally verified", "not a deployed system, a product, or an offer, and I do not ask the Commission to approve it" (`iac/body.typ:99-103`). "In my prototype the check is structural" hedge on the collateral claim. Do NOT add: "verified kernel", "solvency proven", "deployed", "operating", or DREGG/fee/treasury economics (SUBMISSION_WEEK_PLAN keep-out). The accumulator sentence deliberately omits "authenticated" to avoid a cryptographic claim. CHECKED: at ceiling. |
| V-26 | Identifier, deadline, and note_ref integrity: Docket CFTC-2026-1717 / Document CFTC-2026-1717-0001 retained in `iac/metadata.typ` identifiers (rendered on page 1); August 20 meeting in both `proceeding` fields; August 27, 2026 written-statement deadline now stated in the body with `note_ref(9)` (`iac/body.typ:5-8`) — Draft 3 left source entry 9 unreferenced; Draft 4 references all 11 source entries (1,11 exec; 10,11 scope; 2 four-questions; 3,4 seven-questions; 5,6 privacy; 7,8 limits; 9 exec). Cover references its single inline footnote (40.11 + June 2026 proposal) unchanged. | `iac/metadata.typ`; `iac/body.typ`; `iac/sources.typ` (11 entries unchanged; local-materials paragraph extended to name the Dragon's Clutch prototype with the V-17 limitation wording); cover `sources.typ` unchanged; PDF text check 2026-08-18. | R (mechanical text check) | The final-source gate note in `iac/sources.typ` is unchanged; entry 11 keeps "Proposed, not current law"; current-docket revalidation before filing remains a human gate. CHECKED. |
| V-27 | Cover register: the seven questions are preserved verbatim from Draft 3; the framing paragraph and scope section keep their substance with the `PROPOSED:`/`SOURCED:` prefixes removed; the frozen framing ("Regulator-observable Shielded is the practical reference architecture. Dark is retained solely as a long-horizon research boundary, not a venue label, compliance conclusion, or deployment recommendation.") survives unchanged; "Respectfully submitted, [FULL NAME]" retained; still one page. | `iac-cover/body.typ`; `output/pdf/cftc-iac-cover-statement-draft-4.pdf` (1 page). | R (document-scope check) | The only added clause is "and it proposes that the analysis attach to those milestones rather than to labels" — a framing sentence about the attachment, not a new technical or legal claim. CHECKED. |

## Claims that are not locally VERIFIED (unchanged families)

The Draft 3 ledger's downgrade table continues to govern. For these filings
the live families are:

| Claim family | Draft 4 status | Required gate |
|---|---|---|
| Clear/Shielded/Dark taxonomy and any end-to-end Dark statement | Stated as "information flows and design targets, not present implementation claims" (`iac/body.typ:314-330`); Dark held to the frozen long-horizon-research-boundary framing in both filings; no end-to-end Dark claim anywhere. | Remains design-target taxonomy only; a Dark filing claim would need the full evidence stack listed in the Draft 3 ledger. None is present. |
| Identity, affiliation, contact, signature | Placeholders retained (`[FULL NAME]` etc.) in both filings. | Human identity/authority gate. |
| Any legal classification, registration, CEA, or 40.11 conclusion | Both filings state questions, factual criteria, and express non-claims only. | Human counsel gate. |
| Current deadlines, docket identifiers, agendas, current rules | Sources retrieved 2026-08-17 with "recheck before filing" markers; meeting/deadline facts cited to note 9. | Current-docket gate immediately before filing. |
| Independent public reproducibility of the research artifacts | `iac/sources.typ` still conditions public citation on frozen public commits "by filing time"; bodies claim only submitter review. | Freeze commits/paths/hashes or keep the limited description. |
| Production clearing, settlement, collateral sufficiency, oracle validity, liveness | Stated only as design terms, model theorems, or prototype checks; the proof-objectives section conditions any weight on the exact verified statement (`iac/body.typ:366-370`). | No local gate closes these; keep the conditional wording. |

## Mechanical checks recorded for this audit

- `typst compile --root /Users/ember/dev/degg-research
  docs/regulatory/typst/iac/main.typ
  output/pdf/cftc-iac-written-statement-draft-4.pdf`: built 2026-08-18,
  10 pages, SHA-256
  `b23552add9b1dd36b0daa96f3b53905767b6904f2424fd0f80a54976c0c12edc`.
- `typst compile --root /Users/ember/dev/degg-research
  docs/regulatory/typst/iac-cover/main.typ
  output/pdf/cftc-iac-cover-statement-draft-4.pdf`: built 2026-08-18,
  1 page, SHA-256
  `93d42140168a3ca724d156a07c4a7b7a2ab614bd53189a3691bd11ff4a477086`.
- PDF text check on 2026-08-18: no `SOURCED:`/`VERIFIED`/`INFERRED:`/`PROPOSED:`
  label strings remain in either rendered filing; "DRAFT 4 FOR REVIEW - NOT
  FILED" appears in the running header; "CFTC-2026-1717" present in the
  statement; the frozen Dark framing, "do not presently compose", "tested, not
  formally verified", and "eligible contract participants" all present.
- Prototype evidence was not re-run by this lane; it relies on the 2026-08-18
  runs recorded in `DRAFT4_CLAIM_AUDIT.md` (26 crate tests) and
  `GUARDED_EVENT_FOUNDATIONS.md` [C-22] (52 tests repo-wide), both at
  dragons-clutch commit `fa4efb4e5a5a3ef14c6b8b33a949525928ae5a70`.
- No network, regulator contact, filing, deployment, key access, or external
  publication was performed for this rewrite or audit.

## Pre-filing disposition

The strongest honest overall description of the Draft 4 IAC packet is: **a
plain-English written statement and one-page cover, grounded in source-cited
regulatory material, Lean model theorems, and one offline deterministic Rust
prototype, organized around a five-milestone interpretive taxonomy, with every
deployment, verification, Dark-compliance, and legal conclusion expressly
disclaimed**. Identity, counsel, current docket, copyright/provenance, final
build, and public-disclosure decisions remain human gates. An adversarial
re-verification lane should start from the statement's traceability appendix
(`iac/body.typ:465-489`), rows V-22 through V-25 here, and the Draft 3
ledger's downgrade table.
