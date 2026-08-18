# Courtesy-review memo — ROUND 1 of 2

Prepared 2026-08-18 (updated 2026-08-19) for the courtesy review. The
deadlines have real runway, so the review is structured as **two rounds**:
this memo is **round 1 — substance**: the positions, the judgment table,
and the specific questions, sent early so your feedback can be incorporated
before anything freezes. Round 2 will be a short delta memo (protocol in
Section 4). Everything below is preprocessed: the in-house analysis
(`LEGAL_ANALYSIS.md`, attached) records the reasoning and primary-source
verification behind each judgment, so each row here should cost a glance
and only Section 3 asks for your time. Thank you — this is a favor and it
is built to respect your hour.

## 1. What is being filed

I am filing four public comment documents as an independent researcher, in
my own name, with no client, product, offer, or deployed system involved.
Two are responses to joint CFTC/SEC requests for comment due **Monday,
August 24, 2026** — one on product definitions (CFTC RIN 3038-AF71 / SEC
File S7-2026-21, answering Questions 1 and 8) and one on data reporting
(RIN 3038-AF70 / S7-2026-22, answering Questions 3, 8, and 19). The third,
new since the first version of this memo, is a narrow CFTC-only comment on
the 24/7-trading and perpetual-contracts RFC (RIN 3038-AF75, docket
CFTC-2026-1388), due **Wednesday, August 26, 2026** — it answers only the
reference-price and observability questions, expressly takes no position on
the energy-market, margin, or listing questions, and states that I have no
energy market data. The fourth is a written statement to the CFTC's
Innovation Advisory Committee, docket CFTC-2026-1717, due **Thursday,
August 27, 2026** (hard electronic cutoff 11:59 p.m. ET; the docket does
not accept late comments); it now carries a short agenda-responsive
"operatorless agent" section that requests approval of nothing and asks the
Committee to take up one question.

Draft 6 changes the register: the filings now take **argued analytical
positions** — for example, that publication of market software, without
more, creates no agreement, contract, or transaction; that exposure arises
at funding; that prefunded atomic settlement performs no credit
intermediation — each argued from primary text verified in the attached
analysis. The positions are the commenter's own advocacy, which the comment
process invites (5 U.S.C. 553(c): "written data, views, or arguments").
None classifies a deployed product, claims any design of mine is exempt or
compliant, or requests relief. The full position register, with each
position's statutory basis and the strongest counterargument it answers, is
the Draft 6 section of `DRAFT5_CLAIM_LEDGER.md`, as amended by the Draft 7
section — **the positions are now the main thing to eyeball.**

Draft 7 recenters the filings on positions no other filer on these dockets
argues (a survey of all eleven filed comments is in
`research-memos/FILED_COMMENTS_LANDSCAPE.md`), and for the first time
engages two other filed comments **by name and docket number**: the
definitions comment answers FalconX Bravo, Inc.'s proposed listing-status
classification (CFTC-2026-1355-0006) directly and respectfully, and the
data comment endorses — rather than restates — Ariadne Dataworks Ltd.'s
machine-readable-reporting recommendations (CFTC-2026-1354-0002). The
definitions comment adds a bundling-invariance position answering
Question 8; the IAC statement adds a deliberately self-critical
manipulation-cost position (no computed number appears) and cites the
Committee's published meeting agenda.

## 2. Material legal judgments made in-house

Bases cite sections of the attached `LEGAL_ANALYSIS.md` (LA). Every legal
citation behind these rows was verified against the primary source on
2026-08-18; LA §9 is the per-citation ledger.

| Judgment | Basis | Confidence |
|---|---|---|
| Anyone may file these comments; no counsel or credential is required; filing pro se is not practicing law | LA §1 (5 U.S.C. 553(c); the notices' own invitations; 5 U.S.C. 1009(a)(3) for the IAC) | High |
| Comments are published permanently, unredacted, without agency review of PII/CBI; mitigation is what we omit, and nothing sensitive is in the drafts | LA §1, §6 R-6/R-7 (both notices' ADDRESSES terms, verified verbatim) | High |
| 18 U.S.C. 1001 applies to statements in these submissions; the maintained claim-audit ledger and hash-frozen final texts are the accuracy control; residual risk low | LA §1, §6 R-2 | High |
| The definitions comment classifies no deployed product and claims no exemption; its Draft 6 positions are analytical advocacy about a class of staged structures, inclusionary where they approach classification (e.g., no SBS prong implicated on the worked example's face — LA §2's own INFERRED analysis) | LA §2; Draft 6 position register in `DRAFT5_CLAIM_LEDGER.md` | High |
| Taking argued positions is invited comment content, not practice of law or a request for relief; each position's basis is a fetch-verified primary source, and no position crosses into "my design is therefore exempt/compliant" | LA §1 (5 U.S.C. 553(c)); position register (P-D1..8, P-R1..6, P-I1..8) | High |
| Engaging other filed comments by name and docket number (answering FalconX Bravo's CFTC-2026-1355-0006; endorsing Ariadne Dataworks' CFTC-2026-1354-0002) is ordinary notice-and-comment practice; both characterizations are limited to what the cited comments' filed text says, and both citations carry regulations.gov URLs with retrieval dates | Draft 7 section of `DRAFT5_CLAIM_LEDGER.md`; `research-memos/FILED_COMMENTS_LANDSCAPE.md` (the comments read in full) | High |
| The IAC statement's new manipulation-cost position is self-critical analysis of the commenter's own design pattern, states no computed number, names no reference-data or oracle provider, and asserts nothing about what will occur at the August 20 meeting; the published agenda is cited only for its listed Session III topics | Draft 7 section of `DRAFT5_CLAIM_LEDGER.md`; `research-memos/definitions-q15-reference-integrity.md`; `research-memos/OPEN_MATTERS_MAP.md` Part 1 | High |
| The filings' legal recitals (swap definition, SBS prongs, mixed swaps, 2012 release, 40.11 + June 2026 proposal, facility definitions) are accurate as written | LA §2, §3, §5 (all recitals checked against fetched primary text) | High |
| The crypto-native-objective-events scope keeps the examples clear of the 40.11/CEA 5c(c)(5)(C) enumerated activities without claiming any exemption — the correct ceiling | LA §3 | High |
| The IAC statement is input to a solely-advisory FACA committee; it creates no status and requests only that the Committee recommend work | LA §4 (charter, 5 U.S.C. ch. 10, meeting notice) | High |
| Publication-vs-operation is genuinely unsettled (functional triggers; one nonbinding staff letter; FinCEN analogy), so the filings' request for guidance is well-founded — and the filings never claim publication is safe | LA §5 | High |
| Filing creates no meaningful exposure for the researcher: no solicitation/offer content, no operating activity described, prototype descriptions carry audited evidence ceilings, no CBI/PII | LA §6 risk register R-1..R-11 | High |
| Two defects found and fixed before this memo was finalized: a source note misattributed CFTC Staff Letter 26-09 to the wrong division (sources.typ corrected, IAC PDF rebuilt); the repo README named a superseded retained-counsel gate (aligned to the actual in-house + courtesy-review process) | LA §8 | High |
| **Filing a fourth comment** (24/7/perpetuals RFC) was a weighed go. For, in one line: the Commission asks this program's exact question in its own words (Questions 40, 50, 53, 66), the argument was already written, and it reinforces the IAC statement's Position 8 for a second audience. Against, in one line: an energy-adjacent docket with ~90 filers ahead where the program has no data, an RFC that asks for data, and Question 40 brackets the digital-asset context — so the comment argues the property set, not the substrate, scoped by an express no-energy-data disclaimer and keep-outs | `research-memos/CANDIDATE_247_PERPETUALS_COMMENT.md` (go/no-go); perpetuals section of `DRAFT5_CLAIM_LEDGER.md`; LA §9 rows 29-30 | High |
| **Including the operatorless-agent addendum** in the IAC statement is agenda-responsive (the meeting notice's topics include artificial intelligence), takes no numbered position, requests approval of nothing, and every artifact sentence carries the independent artifact survey's ceiling wording, including the corrected live-session sentence; its one filing gate (the recorded suite re-run) was met before insertion | `research-memos/IAC_ADDENDUM_CANDIDATE.md`; Draft 8 section of `DRAFT5_CLAIM_LEDGER.md` | High |

## 3. Round-1 questions for you (each ~1 minute; tentative answers included)

These are the round-1 questions; anything arising from your round-1
feedback becomes the round-2 list.

1. **The position register.** Draft 6 converts the old hedged observations
   into openly argued positions — most prominently: "Publication of market
   software, without more, should not itself be a regulated activity,
   because it creates no agreement, contract, or transaction." This is now
   deliberate advocacy, argued from the statutory predicate (LA §2), not an
   inadvertent legal conclusion. Two-part question: (a) are you comfortable
   with an independent researcher filing argued positions of this kind at
   all, and (b) does any single position in the register
   (`DRAFT5_CLAIM_LEDGER.md`, Draft 6 section) read to an agency lawyer as
   a claim about *my* conduct or products rather than as proposed general
   analysis? A per-position skim is the review; each is one line.
2. **Signature block.** I plan to sign name + "independent researcher" + one
   durable email, omitting postal address and phone from the permanently
   public artifacts. Anything imprudent in that presentation?
3. **Routing.** The joint notices' "use only one method" language is stated
   per agency, so I read them as permitting one submission via the CFTC route
   *and* one via the SEC route for each joint comment (identical artifact,
   once per agency), which puts the comment in both records. Tentative plan:
   file both routes. Any reason to prefer single-route instead? (LA §1, §7.3.)
4. **Risk register.** Does anything in LA §6 strike you as wrong, mis-weighted,
   or missing — especially R-2 (resting the 1001 accuracy control on the
   maintained claim-audit ledger) and R-3/R-4 (that the filings cannot
   reasonably be read as an offer or as operating activity)?
5. **Engaging named commenters.** Draft 7 answers FalconX Bravo, Inc.'s
   filed definitions comment by name (a CEO-signed filing that directly
   contradicts one of my positions; its founder sits on the IAC) and
   endorses Ariadne Dataworks Ltd.'s data-reporting recommendations by
   name. My tentative answer is that this is appropriate and normal:
   responding to other comments on the same public record is ordinary
   notice-and-comment practice, the FalconX answer is respectful and
   credits the rule's objectivity before disagreeing, the endorsement
   adopts rather than affiliates, and both cite the comments' public
   docket numbers. Flag if either engagement reads to you as adversarial,
   as implying affiliation, or as otherwise imprudent for a pro se filer.

A nod or a one-line reply per item is exactly the right amount of effort; if
any answer is "needs a conversation," say so and I will not treat silence as
sign-off.

## 4. Round 2 protocol

Round 2 will be a **delta memo**: what changed since round 1, item by item
— each change in one line with why, nothing restated — with the final
artifacts attached, so your second pass is a diff-read costing minutes, not
a re-read. Between the rounds the claim ledgers re-pin at the filing-day
freeze (final commits, artifact hashes, and docket revalidation), so the
delta memo's evidence references are to frozen objects. The round-2
question list is whatever your round-1 feedback raises; if it raises
nothing, round 2 is a confirmation that the deltas are as described.

## 5. Attachments

1. `joint-definitions-comment-draft-7.pdf` (7 pp.)
2. `joint-data-reporting-comment-draft-7.pdf` (8 pp.)
3. `cftc-perpetuals-comment-draft-1.pdf` (4 pp.; the fourth filing, new in
   this round)
4. `cftc-iac-written-statement-draft-8.pdf` (8 pp.; the reserved half page
   now carries the operatorless-agent section — the eighth page holds only
   source apparatus, and whether to compress back to 7 pp. is an open
   layout call noted in the ledger)
5. `cftc-iac-cover-statement-draft-8.pdf` (1 p.)
6. `LEGAL_ANALYSIS.md` (the in-house analysis this memo indexes; its
   filing-text review describes Draft 5 — the Draft 6 register change, the
   Draft 7 rebalance, the Draft 8 insertion, and the perpetuals conversion,
   with their per-position bases, are recorded in the corresponding
   sections of `DRAFT5_CLAIM_LEDGER.md`)

Every material technical claim in the five PDFs carries a one-line
evidentiary basis in its own appendix and is governed by a maintained
evidence ledger in the repository (claim audits for Drafts 3-8 and the
perpetuals Draft 1; artifact hashes re-pin at the filing-day freeze); you
do not need to verify any technical claim to review the legal posture.
Every page of every draft is watermarked "FOR REVIEW - NOT FILED" with its
draft number, and identity placeholders remain visibly unresolved until
the final identity gate.
