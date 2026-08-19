# Courtesy-review memo — ROUND 1 of 2 (Draft 10 review set)

Prepared 2026-08-18, rewritten 2026-08-19 for the Draft 10 review set; if an
earlier version of this memo (covering Drafts 6–8) reached you, this version
supersedes it. The deadlines still have real runway, so the review stays
structured as **two rounds**: this memo is **round 1 — substance**: the
documents, the judgment table, and five specific questions. Round 2 will be a
short delta memo (protocol in Section 4). Everything below is preprocessed:
each judgment row names where the claim appears and why we are confident, so a
row should cost a glance and only Section 3 asks for your time. Every legal
conclusion in this memo is **our in-house analysis, offered for your
sanity-check — nothing here reflects advice already received.** This memo
prepares a review and does nothing else: it files nothing, submits nothing,
and the documents it covers are watermarked drafts. Thank you — this is a
favor and it is built to respect your hour.

## 1. What is under review

Four public comment documents, prepared for possible filing by an independent
researcher, in the researcher's own name, with no client, product, offer, or
deployed system involved. All four are at **Draft 10** (committed at
`a1b8aea`), every page watermarked "DRAFT 10 FOR REVIEW - NOT FILED," with
identity placeholders ([FULL NAME] etc.) visibly unresolved until the final
identity gate:

1. **Definitions comment** (10 pp.) — joint CFTC/SEC request on product
   definitions, CFTC RIN 3038-AF71 / SEC File S7-2026-21, answering
   Questions 1 and 8; due **Monday, August 24, 2026**.
2. **Data-reporting comment** (12 pp.) — joint request on swap data
   reporting, RIN 3038-AF70 / S7-2026-22, answering Questions 3, 8, and 19;
   due **Monday, August 24, 2026**.
3. **IAC written statement** (12 pp.) — written statement to the CFTC's
   Innovation Advisory Committee, docket CFTC-2026-1717, for its August 20
   meeting; statements accepted through **Thursday, August 27, 2026** (hard
   electronic cutoff 11:59 p.m. ET). Carries eight argued positions, an
   agenda-responsive operatorless-agent section that requests approval of
   nothing, and a four-row table of machine-checked negatives.
4. **IAC cover statement** (2 pp.) — a one-page-plus-notes summary of the
   eight positions.

A fifth document, the narrow CFTC-only comment on the 24/7-trading and
perpetual-contracts RFC (RIN 3038-AF75, docket CFTC-2026-1388, due
**Wednesday, August 26, 2026**), stands at **Draft 2** — corrected earlier by
its own experiment (sampling structure, not window length) — and was
deliberately not reopened by the Draft 10 pass. A separately named "Energy
Draft 3" candidate for that docket exists only as an internal plan, has no
authored text, and is not before you.

**What Draft 10 changed.** Draft 10 is an engineering-accuracy pass, not a
change in legal register: the positions, the named-commenter engagements, and
every external citation are carried verbatim from Draft 9 (itself the audited
successor of the Draft 8 set you may have seen). The deltas replace stale
technical-status claims with the current committed evidence — chiefly a
corrected story about the resolution path's data source, two measured
compute-budget STOPs stated with exact numbers, one measured staged
alternative route, a bounded synthetic energy-dispatch example, a bounded
encrypted-validation (TFHE) example, and shared boundary sentences on
operatorlessness and formal proof. Each delta is a judgment row below,
because each embeds a call about how a research fact should read in a
permanent federal record. The change trail is
`docs/reviews/ENGINEERING_CLAIM_AUDIT_2026-08-19.md` (audit of Draft 8
against committed evidence) and
`research-memos/DRAFT10_ENGINEERING_CLAIM_DELTA_PLAN.md` (the executed plan);
you do not need either to review the legal posture.

## 2. Material judgments made in-house

Bases cite the attached `LEGAL_ANALYSIS.md` (LA; its per-citation ledger is
§9, verified against primary sources 2026-08-18 — the verbatim carry of all
citations through Drafts 9 and 10 is what keeps that verification current)
and the four Draft 10 documents (DEF = definitions, DATA = data-reporting,
IAC = written statement, COVER = cover statement).

| Position / claim as written | Where it appears | Judgment call embedded | Confidence and why |
|---|---|---|---|
| Anyone may file these comments pro se; taking argued analytical positions is invited comment content (5 U.S.C. 553(c) "written data, views, or arguments"), not practice of law or a request for relief | All four documents take positions; each states "the positions are my own analysis" | That an independent researcher's open advocacy stays on the safe side of the pro se line so long as no position claims *my* product is exempt or compliant — and none does | High — LA §1; the notices' own invitations, verified verbatim |
| Comments are published permanently, unredacted, with no agency PII/CBI screening; mitigation is what we omit | All four; placeholders and the identity gate | Resting privacy protection entirely on omission and a final identity gate rather than on any agency process | High — LA §1, §6 R-6/R-7 (ADDRESSES terms verified) |
| 18 U.S.C. 1001 accuracy control: every material technical claim carries its evidence class inline plus a one-line appendix basis; Draft 10's deltas were derived from a committed-evidence audit, and no new external citation was introduced | Appendices of DEF, DATA, IAC; audit + delta plan named above | Resting the accuracy control on maintained internal ledgers and audits, hash-frozen at filing, rather than external review | High — LA §1, §6 R-2; the Draft 10 pass tightened claims toward the evidence, never away |
| **Two-artifact source truth:** the default production ELF registers no source release and fails closed — refusing source construction and value admission with `SourceReleaseUnavailable` (0x79) — while permissionless source construction exists only in a deliberately non-production mock-provider ELF; production source remains STOP | IAC "Resolution is licensed by admitted evidence" + appendix; DEF Observation bullet; DATA worked example + Resolution; COVER scope paragraph | Replacing Draft 8's stale "no live archive-to-resolution join" story with a stronger capability statement, while stating in the same breath that the capability is mock-only and the production default refuses. Describing a working mock lifecycle must not read as an operating capability | High on accuracy (committed local-SBF evidence); the *impression* question is Q1 below |
| **ResolutionWork:** one measured, prepaid, SBF-executed staged resolution route whose measured rows all clear the selected 25-percent-headroom profile — "explicitly not a global liveness policy" — stated beside the monolithic occupation STOP (best measured initial row 1,236,364 CU against the 1,120,000-CU threshold under the 1,400,000-CU ceiling) | IAC "Operational readiness is measured, not assumed"; appendix rows in DEF and DATA | Admitting exactly one measured route and refusing the promotion to a liveness or production claim; keeping the admission and the STOP in the same paragraph | High — measured campaign rows; the non-promotion wording follows the delta plan's non-composition rule |
| **Direct V2 STOP:** full top-three selection reaches exactly the 1,400,000-CU ceiling and rolls back every watched byte and lamport — "a measured, non-promotable STOP" — submission max 1,194,085 CU; V3 is model/design only; any future selection claim capped at "best valid submitted candidate" before an immutable close boundary | IAC third compute-admission item; DATA "Close and match" | Publishing measured failure boundaries of our own prototype, with exact numbers, in a permanent record — framed as "a boundary, not a promise" | High on the numbers; the prudence-of-candor question is Q2 below |
| **The 22-transaction walk** is described as "local, signed, sequential, SBF-executed, genesis-assisted evidence, not a deployment or venue" | IAC Settlement; DEF local-SBF paragraph; DATA Settlement | The trailing negative ("not a deployment or venue") is a legal-adjacent characterization doing scoping work inside a factual sentence | High that the facts support it (nothing is deployed; no venue exists); whether the phrasing should stay legal-flavored is Q3 below |
| **TFHE candidate-only boundary:** a bounded encrypted evaluator validates a caller-supplied candidate's feasibility and exact conservation and detects a forged cost-59 settlement, holding no client key — with no global search, no optimality check, no vFHE proof, no custody/release/network privacy/settlement, "not described as Dark"; reproduced on arm64 and x86_64 CPUs | IAC negatives table + appendix; DATA "Close and match," Question 8 close, appendix | Describing encrypted-computation research in a CFTC record without implying a privacy product, an encrypted solver, or any surveillance-evading capability; the candidate-validator/solver distinction is drawn explicitly | High — every ceiling is stated in the same paragraph as the capability |
| **Clear-energy machine-checked negative:** a bounded synthetic dispatch relation selects canonical objective 56 and rejects a feasible, conserving cost-60 plan only by recomputing the frozen optimum — "recomputation of the complete rule, not predicate passing, is what verifies a selection" | IAC fourth machine-checked negative; DATA recomputation passage; deliberately absent from DEF | Using an energy-*flavored* synthetic example in two non-energy dockets while the perpetuals Draft 2 comment expressly disclaims energy-market data; consistency is preserved by express "not energy-market evidence" ceilings and by keeping it out of the definitions argument | High on accuracy (deterministic corpus); the cross-docket consistency read is Q4 below |
| **Operatorless boundary:** the IAC's agenda-responsive AI section requests approval of nothing, asks the Committee to take up one question, and denies the achievement — "no current artifact establishes an operatorless lifecycle," with seven named dependencies listed | IAC "The operatorless agent"; echoed in Limits | Posing publication-versus-operation in its sharpest hypothetical form while enumerating the operator functions that remain — the section must not read as announcing an autonomous agent | High — the denial sentence and dependency list come verbatim from the delta plan's strongest safe wording |
| **Formal-proof boundary sentence:** "Stated once, exactly: separate Lean modules prove named model properties, one pinned Verus run checks a narrow arithmetic seam ... No refinement proof connects all of those artifacts" | IAC Limits; DEF Position 6; DATA appendix | Crediting scoped machine-checked results without letting "verified" imply whole-system assurance — the guard against any "formally verified market" mischaracterization | High — the sentence lists what is *not* proved by name |
| Engaging other filed comments by name — answering FalconX Bravo, Inc. (CFTC-2026-1355-0006) in DEF Position 7; endorsing Ariadne Dataworks Ltd. (CFTC-2026-1354-0002) in DATA Question 19 | DEF Position 7; DATA Question 19 + Position 6 | Ordinary notice-and-comment engagement; both characterizations stay limited to the cited comments' filed text; carried unchanged in character since Draft 7 | High — LA-verified docket citations; the comments were read in full (`research-memos/FILED_COMMENTS_LANDSCAPE.md`) |
| The filings' legal recitals (swap definition, SBS prongs, 2012 release, 40.11 + June 2026 proposal expressly marked "proposed, not current law," facility and DCO definitions, Staff Letter 26-09 as narrow/conditional/nonbinding, parts 38/39/43/45/49) are accurate as written | Recitals and source notes of DEF, DATA, IAC | Verbatim carry from the verified Draft 9 text preserves the 2026-08-18 primary-source verification without re-fetching | High — LA §2, §3, §5, §9; `a1b8aea` introduced no new external citation |
| The IAC statement is input to a solely advisory FACA committee; it creates no status and requests only that the Committee recommend work | IAC Limits and Requested work products | Same judgment as prior rounds, unchanged by Draft 10 | High — LA §4 |
| Leaving the perpetuals comment at Draft 2 and not folding the new energy evidence into it | Perpetuals Draft 2 (attached); plan §5.4 | Not reopening a frozen, narrow, already-corrected comment five days before its deadline; the energy evidence enters the record through IAC/DATA instead, and any Energy Draft 3 would be a separately gated new document | Medium-high — a scheduling and scope call, flagged here so you can veto it |

## 3. Round-1 questions for you (five; each answerable in minutes from this packet)

Anything arising from your answers becomes the round-2 list. Tentative
in-house answers are included so a nod suffices where you agree.

1. **Does the two-artifact / fail-closed framing change any
   registration-trigger read?** Draft 10 now says, in all four documents,
   that the default program refuses source construction and value admission
   fail-closed (error 0x79) while "a separate, deliberately non-production
   mock-provider ELF" executes the full permissionless source lifecycle
   locally. Our analysis (LA §5) treats publication-versus-operation as
   turning on solicitation, order handling, custody, and control — none
   present. Question: could an agency reader treat "the mock build can do X
   permissionlessly" as closer to an *operating capability or offer* than
   Draft 8's weaker "no join exists" language, in a way that changes the
   publication-versus-operation analysis anywhere? Our tentative answer: no —
   nothing is deployed or offered, a fail-closed default is the opposite of
   operation, and every capability sentence carries its non-production
   restriction inline; but this is the delta most worth your eye.
2. **Are the measured STOPs prudent to publish?** The IAC statement prints
   exact failure numbers against our own prototype (initial row 1,236,364 CU
   missing a 1,120,000-CU threshold; selection halting at exactly
   1,400,000 CU) and calls each "a boundary, not a promise." Our tentative
   answer: this candor strengthens the 1001 accuracy posture and admits
   nothing *against* us because nothing operates — a STOP on a research
   prototype is a fact about the research, not about any product. Do you see
   a way these permanent statements could be turned against a later, changed
   deployment, and if so is a one-line "measured against the current
   decomposition, not a limit of the design" already adequate (it is present
   in substance: "a decomposition and admission-policy result, not a hardware
   impossibility")?
3. **Do any embedded negative characterizations read as legal conclusions?**
   Draft 10 repeatedly scopes facts with legal-flavored negatives: "not a
   deployment or venue," "not a global liveness policy," "publication ...
   is not operation of a venue or an intermediary," "not an admission of a
   complete venue." We intend these as factual scoping (each is supportable
   from the artifact record: nothing deployed, no venue, no policy adopted).
   Question: skimming the quoted phrases only, would any of them read to an
   agency lawyer as a self-serving legal conclusion that should be softened
   to pure factual form ("no deployment exists"), rather than as scoping? A
   flag on any specific phrase is a complete answer.
4. **Is the energy example consistent with the no-energy-data posture?** The
   perpetuals Draft 2 comment (attached) expressly states we have no energy
   market data and takes no energy-market position; Draft 10 separately adds
   a *synthetic* energy-dispatch example (canonical objective 56; TFHE
   validation) to the IAC statement and the data-reporting comment, each
   instance ceilinged as "synthetic ... not energy-market evidence" and kept
   out of the definitions comment. Our tentative answer: no contradiction —
   the example is about verification method, not energy markets, and the
   ceilings say so. Could an agency reader on the perpetuals docket
   nonetheless see tension between "no energy data" there and worked
   energy-dispatch arithmetic here?
5. **Carried forward unchanged, still needed before August 24 (two one-line
   items).** (a) *Signature block:* name + "independent researcher" + one
   durable email, omitting postal address and phone from the permanently
   public artifacts — anything imprudent? (b) *Routing:* the joint notices'
   "use only one method" language is stated per agency, so we read them as
   permitting one submission via the CFTC route *and* one via the SEC route
   per joint comment (identical artifact, once per agency); tentative plan is
   to file both routes — any reason to prefer single-route? (LA §1, §7.3.)

A nod or a one-line reply per item is exactly the right amount of effort; if
any answer is "needs a conversation," say so and we will not treat silence as
sign-off.

## 4. Round 2 protocol

Round 2 will be a **delta memo**: what changed since round 1, item by item —
each change in one line with why, nothing restated — with the final artifacts
attached, so your second pass is a diff-read costing minutes, not a re-read.
Between the rounds the claim ledgers re-pin at the filing-day freeze (final
commits, artifact hashes, docket revalidation), so the delta memo's evidence
references are to frozen objects. The round-2 question list is whatever your
round-1 feedback raises; if it raises nothing, round 2 is a confirmation that
the deltas are as described.

## 5. History of this memo and the drafts (context only; no action needed)

- **2026-08-18, first version (Drafts 6–7):** the filings converted hedged
  observations into argued positions (Draft 6) and recentered on positions no
  other filer argues, engaging FalconX Bravo and Ariadne Dataworks by name
  (Draft 7). Two defects were found and fixed before the memo finalized: a
  source note misattributed CFTC Staff Letter 26-09 to the wrong division
  (corrected, PDF rebuilt), and the repo README named a superseded
  retained-counsel gate (aligned to the actual in-house + courtesy-review
  process). LA §8 records both.
- **2026-08-19, second version (Draft 8 + perpetuals Draft 1):** the
  operatorless-agent addendum landed in the IAC statement; the fourth
  (perpetuals) comment was a weighed go, documented in
  `research-memos/CANDIDATE_247_PERPETUALS_COMMENT.md`. The perpetuals
  comment was then corrected by its own experiment (sampling structure, not
  window length) to Draft 2.
- **No round-1 feedback has been received or is recorded**; this Draft 10
  version supersedes both earlier versions before your clock starts. Three
  questions from the earlier list (comfort with the position register as
  such; the LA §6 risk-register weighting; the named-commenter engagements)
  are not re-asked: their substance is now carried by the judgment rows
  above, and all three registers are unchanged in character since Draft 7 —
  reopen any of them if a row bothers you.
- **Draft 9 (2026-08-19):** an engineering claim audit
  (`docs/reviews/ENGINEERING_CLAIM_AUDIT_2026-08-19.md`) rewrote the
  technical-status paragraphs to match committed evidence; Draft 9 sources
  and PDFs are frozen.
- **Draft 10 (2026-08-19, commit `a1b8aea`):** the engineering claim deltas
  described in Section 1, executed from
  `research-memos/DRAFT10_ENGINEERING_CLAIM_DELTA_PLAN.md` into new
  `*-draft-10` source directories; Draft 9 remains untouched. All external
  citations carried verbatim; no new external citation introduced.

## 6. Attachments

1. `joint-definitions-comment-draft-10.pdf` (10 pp.)
2. `joint-data-reporting-comment-draft-10.pdf` (12 pp.)
3. `cftc-iac-written-statement-draft-10.pdf` (12 pp.)
4. `cftc-iac-cover-statement-draft-10.pdf` (2 pp.)
5. `cftc-perpetuals-comment-draft-2.pdf` (6 pp.; frozen, not reopened by
   Draft 10 — see the last judgment row)
6. `LEGAL_ANALYSIS.md` (the in-house analysis this memo indexes; its
   filing-text review describes Draft 5 — the later register changes are
   recorded in `DRAFT5_CLAIM_LEDGER.md` through Draft 8, and the Draft 9/10
   technical deltas in the audit and plan named in Section 1)

Every material technical claim in the five PDFs carries a one-line
evidentiary basis in its own appendix and its evidence class inline in the
sentence that makes it; you do not need to verify any technical claim to
review the legal posture. Every page of every draft is watermarked with its
draft number and "FOR REVIEW - NOT FILED," and identity placeholders remain
visibly unresolved until the final identity gate.
