# Filed-comments landscape — who else is on our three dockets, and what is left unclaimed

Internal research memo, prepared 2026-08-18 by the comparative-filings
research lane. See [README.md](README.md) for status and citation rules.
This memo is a survey of the public record; it is not legal advice, a filing
authorization, or a claim about any third party's positions beyond what the
quoted text says.

**All retrievals in this memo were performed 2026-08-18** (verified against
`TZ=America/New_York date` at the start of the lane: Tue Aug 18 03:54 EDT
2026). Retrieval dates are not currency: both joint-RFC deadlines are
**August 24, 2026** and the IAC written-statement deadline is **August 27,
2026**, so every count below is an *early-filer snapshot* and every
white-space finding in Part 3 is provisional. Re-run this survey on
2026-08-25 and 2026-08-28 before treating any absence as final.

---

## Part 0 — Method, and exactly what was blocked

What worked:

- **regulations.gov v4 API**, `https://api.regulations.gov/v4/comments`
  filtered by `filter[docketId]`, with the public demo key
  (`api_key=<DEMO_KEY>`). Full docket listings and one comment detail record
  retrieved before the rate limit engaged.
- **Attachment downloads**, `https://downloads.regulations.gov/<COMMENT-ID>/attachment_<n>.pdf`.
  No API key required. Returns HTTP 403 to a default client user agent and
  HTTP 200 with a browser user agent. 13 attachment PDFs retrieved and text
  extracted with `pdftotext -layout`.
- **cftc.gov** event, press-release, and media-download pages.
- **govinfo.gov** Federal Register PDFs (FR Docs 2026-12743, 2026-12742,
  2026-16328).

What was blocked, precisely:

- **regulations.gov API rate limit.** After roughly 16 requests the demo key
  returned `{"error":{"code":"OVER_RATE_LIMIT"}}`. A backoff-and-retry loop
  (8 attempts at 5-minute intervals) was run. Consequence: **one comment of
  eleven, CFTC-2026-1354-0003 (Nathan Fry), is an inline comment with no
  attachment and its body text was not retrieved.** Its metadata (name, date,
  docket) came from the listing endpoint, which succeeded. A registered API
  key would close this gap in one request.
- **regulations.gov HTML view.** `https://www.regulations.gov/comment/CFTC-2026-1354-0003`
  returns HTTP 403 to non-browser clients; it is a JavaScript application
  with no server-rendered comment body.
- **SEC comment lists do not exist for these file numbers.** Five URL
  patterns were tried and all returned HTTP 404:
  `sec.gov/comments/s7-2026-21/s7202621.htm`,
  `sec.gov/comments/s7-2026-21/s7-2026-21.htm`,
  `sec.gov/comments/s7-2026-21/`,
  `sec.gov/comments/s7-2026-22/s7202622.htm`, and
  `sec.gov/comments/s7-2026-22/`. The URLs the Federal Register notices
  actually give (for example
  `https://www.sec.gov/comments/s7-2026-21/joint-request-comment-further-definition-swap-security-based-swap-alternative-compliance`)
  return HTTP 200 but are **submission forms**, not letter indexes. As of
  2026-08-18 the SEC is not publishing letters for S7-2026-21 or S7-2026-22
  at any discoverable location, so the CFTC/regulations.gov side is the only
  public view of these dockets.
- **comments.cftc.gov (legacy system) returns HTTP 403** on all three URLs
  tried (`/`, `/PublicComments/ReleasesWithComments.aspx`,
  `/PublicComments/AdvisoryCommittees.aspx`), to both a plain client and a
  browser user agent. Per
  <https://www.cftc.gov/LawRegulation/PublicComments/ReleasesWithComments>
  (retrieved 2026-08-18), the CFTC transitioned to regulations.gov for
  comment files dated **2026-04-28 onward**, with earlier files remaining in
  the legacy system. This matters for Part 1: the historical archive of
  advisory-committee public statements is behind that 403.

Nothing in this memo is inferred, extrapolated, or reconstructed. Where a
document was not read, the memo says so.

---

## Part 1 — The prior IAC/TAC record, and what a submitted statement looks like

### The committee is not new; the *name* is

The premise that the IAC "existed before the 2026-08-20 meeting" is correct,
but with a wrinkle that matters for what precedent exists. The Innovation
Advisory Committee is the **renamed Technology Advisory Committee (TAC)**.
Per the CFTC's IAC page (<https://www.cftc.gov/About/AdvisoryCommittees/IAC>,
retrieved 2026-08-18), the committee "was created to advise the Commission on
complex issues at the intersection of technology, law, policy, and finance,"
and the page carries the TAC's meeting history alongside the new IAC meeting.

- Chairman Michael S. Selig launched/renamed the committee on **January 12,
  2026** (Press Release 9167-26,
  <https://www.cftc.gov/PressRoom/PressReleases/9167-26>), stating an
  intent to nominate CEO Innovation Council participants as charter members
  and soliciting nominations from industry, academia, public interest
  groups, and market infrastructure firms.
- The charter was renewed by FR Doc 2026-16423 (August 12, 2026).
- The **August 20, 2026 meeting is described as the IAC's inaugural
  meeting** (Press Release 9283-26,
  <https://www.cftc.gov/PressRoom/PressReleases/9283-26>; event page
  <https://www.cftc.gov/PressRoom/Events/opaeventiac082026>, both retrieved
  2026-08-18). Members include CEOs of Coinbase, CME Group, Nasdaq, Uniswap
  Labs, and Kraken, plus two academics (Professors Harry Crane and Carla
  Reyes).

So: **under the IAC name there is no prior meeting.** Under the TAC name
there are four recent ones.

### The four recent predecessor meetings and their published materials

| Meeting | Public-statement deadline | Materials published on the event page |
|---|---|---|
| 2023-03-22 (TAC inaugural, Goldsmith Romero) | May 22, 2023 | agenda, minutes, transcript, presentation, 4 commissioner statements |
| 2023-07-18 | not stated on page | agenda, presentation, minutes, transcript, 4 commissioner statements, archived webcast |
| 2024-01-08 | January 15, 2024 | agenda, minutes, transcript, **Decentralized Finance report**, **Statement of Hilary J. Allen**, presentations, 2 commissioner statements |
| 2024-05-02 ("AI Day") | May 9, 2024 | agenda, minutes, transcript, **Responsible AI in Financial Markets report**, presentation, 3 commissioner statements |

Event pages: `opaeventtac032223`, `opaeventtac071823`, `opaeventtac010824`,
`opaeventtac050224`, all under
`https://www.cftc.gov/PressRoom/Events/` and retrieved 2026-08-18. Meetings
index: <https://cftc.gov/About/CFTCCommittees/TechnologyAdvisory/tac_meetings.html>.
The TAC's own history runs further back (meetings in 2012, 2016, 2018, 2019
under earlier sponsors), but those pages carry the same material types.

### The finding that actually matters: public written statements are not archived

**Across all four predecessor meeting pages, not one written statement from a
member of the public is posted.** The published record is agenda, minutes,
transcript, presentations, subcommittee reports, and commissioner statements.
The January 8, 2024 minutes (<https://www.cftc.gov/media/10656/TAC_minutes010824/download>,
6 pages, retrieved 2026-08-18) contain **no occurrence** of "public comment",
"written statement", or "member of the public". Public statements were routed
to comments.cftc.gov, which now returns 403 (Part 0).

The single non-commissioner written statement published in this series is
**Hilary J. Allen's** (<https://www.cftc.gov/media/10111/TAC_HilaryAllen010824/download>,
retrieved 2026-08-18) — and she was a committee member submitting in absentia,
not a member of the public. It is the only concrete model available, so it is
worth characterising exactly:

- **1 page, 470 words.**
- No caption, no letterhead, no "Re:" line, no addressee block. It opens
  "I apologize that I could not be there today."
- Register is first-person evaluative throughout: "I would like to applaud
  the Subcommittee for their hard work"; "Ultimately, however, I cannot
  support this report's recommendations"; "I question the report's
  recommendations."
- **One footnote**, to her own SSRN paper.
- Structure: praise the specific technical strengths, then state the
  disagreement, then give the reason (economic incentives; opportunity cost
  of scarce regulatory resources), then a one-paragraph close.
- It is a *position* document, not a question document. It disagrees with a
  committee work product in public and says why.

### What this means for our IAC packet

1. **There is no visible corpus of public written statements to this
   committee to conform to.** Anyone claiming to know the house norm for a
   public IAC submission is guessing. The only published model is a member's
   1-page prose note.
2. **There is also no visible competition.** Statements to this committee
   have historically been invisible; on the new regulations.gov docket they
   are visible immediately (Part 2). That is a change in the visibility
   regime in our favour, and an argument for filing early rather than at the
   August 27 deadline.
3. The IAC notice (91 FR 51697, FR Doc 2026-16328, retrieved 2026-08-18 via
   <https://www.govinfo.gov/content/pkg/FR-2026-08-11/pdf/2026-16328.pdf>)
   imposes **no format, length, or structure requirement** on written
   statements. It does state, verbatim, that comments "will be published
   without review for, and without removal of, any personal identifying
   information or information your business may consider confidential" — a
   direct input to the identity gate in
   [../DRAFT5_CLAIM_LEDGER.md](../DRAFT5_CLAIM_LEDGER.md).

---

## Part 2 — What has actually been filed

**Eleven comments total across the three dockets as of 2026-08-18.**

### Docket CFTC-2026-1717 — IAC written statements (deadline Aug 27, 2026)

Two comments, both posted 2026-08-12.

| ID | Filer | Type | Posted | One-line thesis |
|---|---|---|---|---|
| CFTC-2026-1717-0002 | steven brown | individual | 2026-08-12 | The notice does not say which questions the Committee will actually consider, and "the agenda may change" plus listen-only audio means the public cannot participate meaningfully; specify the questions or the resulting record will be easy to challenge later. |
| CFTC-2026-1717-0003 | (no submitter name recorded; text signed "Daniel B Leahy") | non-substantive | 2026-08-12 | Not a comment: a scraped fragment of the Federal Register notice header plus a name and an email address. |

The brown comment is ~460 words of inline text, no citations, purely
procedural/administrative-record in character. It is the only substantive
filing on the IAC docket.

**Finding: as of 2026-08-18 there is no technical or substantive policy
statement on the IAC docket at all.** Our IAC statement would be the first.

### Docket CFTC-2026-1355 — Definitions RFC, RIN 3038-AF71 / S7-2026-21 (deadline Aug 24, 2026)

Five comments.

| ID | Filer | Type | Posted | Length | One-line thesis |
|---|---|---|---|---|---|
| -0002 | **Ilya Beylin**, Associate Professor, Seton Hall Law School (personal capacity) | academic | 2026-07-20 | 4 pp, ~1,640 w, 7 footnotes | The 2012 release's consumer-transaction carve-out appears to exclude wagering from "swap" yet Kalshi certifies wagers as swaps — clarify that; and clarify when an event contract meets the SBS event-contract prong, using named live contracts (Tesla deliveries, Apple iPhone launch, Tesla/SpaceX merger, sports outcomes, drug trials). |
| -0003 | **Timothy G. Massad**, Harvard Kennedy School (former CFTC Chairman) | former regulator / academic | 2026-07-22 | 11 pp, ~5,460 w, 33 footnotes | "The Commodity Futures Trading Commission has lost its way": it mischaracterises its own mission, reads "swap" and "price information" out of context, reinterprets the Special Rule without justification, and builds an illusory review process; it should not become the national sports-betting regulator. |
| -0004 | **VeloxVFX LLC** (Eyad G. Haddadin, Managing Member, Bountiful, UT) | small firm / individual | 2026-07-31 | 7 pp, ~2,300 w, 0 citations | Classify by product-level criteria — referenced object, economic exposure, event dependency, issuer nexus, index breadth, ownership transfer, settlement, duration, funding, lifecycle character change — recorded in a "Product Classification Record", with the legal determination reserved to the authorised Commission. |
| -0005 | **Conan Mak**, for "APES TOGETHER STRONG" | retail advocacy | 2026-08-10 | 78 + 85 + 36 pp (3 attachments) | Any instrument referencing a single equity, NBSI, or private valuation — however styled ("token", "SPV interest", "crypto-asset") — is inherently an SBS. Includes conspiracy allegations (Epstein-linked networks; a "$1.187 sextillion" derivatives figure) and two OCR'd exhibit dumps. |
| -0006 | **FalconX Bravo, Inc.** (Alexandra Guest, CEO) | registered swap dealer (digital-asset focused) | 2026-08-17 | 12 pp, 30 footnotes, cc to 9 named SEC/CFTC officials | Three targeted asks: classify cash-settled perpetual contracts on a single security/NBSI by whether they are *listed* under the security-futures framework (otherwise SBS — expressly including contracts offered through a DeFi protocol); eliminate "arranged, negotiated, or executed" as an independent SEC cross-border trigger; relax SEC Rule 18a-10's alternative-compliance 10% condition to 49%. |

### Docket CFTC-2026-1354 — Data Reporting RFC, RIN 3038-AF70 / S7-2026-22 (deadline Aug 24, 2026)

Four comments.

| ID | Filer | Type | Posted | Length | One-line thesis |
|---|---|---|---|---|---|
| -0002 | **Ariadne Dataworks Ltd.** (boutique regtech consultancy) | industry consultancy | 2026-07-14 | 15 pp, ~5,960 w, 4 figures, inline legal citations | **Publish the reporting framework itself as structured data**: one joint machine-readable field dictionary (R1), validation logic in executable openly-licensed form with a public conformance corpus (R2), a normative lifecycle event model with worked examples (R3), governed free/bulk reference data (R4), one public dissemination dialect with uniform access terms (R5), versioned convergence rather than a cutover (R6). |
| -0003 | **Nathan Fry** | individual | 2026-07-27 | inline, no attachment | **Text not retrieved** — see Part 0. Metadata only. |
| -0004 | **VeloxVFX LLC** | small firm / individual | 2026-07-31 | 9 pp, ~3,315 w, 0 citations (attachments 1 and 2 are byte-identical duplicates) | A common core reporting model with agency-specific extensions, event-level reporting responsibility, controlled lifecycle lineage, versioned validation, governed reference data, and distinct treatment of public and confidential information; onchain evidence supports reconciliation but does not replace required reporting. |
| -0005 | **Conan Mak**, for "APES TOGETHER STRONG" | retail advocacy | 2026-08-10 | 75 + 85 + 36 pp | Same filing family as CFTC-2026-1355-0005; attachments 2 and 3 are the identical exhibit dumps. |

### Representative full texts read

Read in full or near-full: Beylin (4 pp), Massad (11 pp), FalconX
(12 pp, sections I-III), Ariadne (15 pp, sections A-F), VeloxVFX data
(9 pp), VeloxVFX definitions (7 pp), brown (IAC, full inline text), Mak
(sampled across all three attachments and characterised; the bulk is OCR'd
exhibit imagery, not argument). That is 7 of the 11 read substantively,
covering every filer type present.

---

## Part 3 — Comparison against Draft 6

Sources for our side: [../typst/definitions/body.typ](../typst/definitions/body.typ),
[../typst/data-reporting/body.typ](../typst/data-reporting/body.typ),
[../typst/iac/body.typ](../typst/iac/body.typ),
[../typst/iac-cover/body.typ](../typst/iac-cover/body.typ), and the Draft 6
position register in [../DRAFT5_CLAIM_LEDGER.md](../DRAFT5_CLAIM_LEDGER.md).

### 3.1 Corpus-wide term sweep

A case-insensitive grep across all 13 retrieved attachment texts plus the two
IAC inline comments. Zero hits means the term appears nowhere in any filed
comment on any of the three dockets.

| Term | Hits in filed comments | Hits in our Draft 6 bodies |
|---|---|---|
| `bundl` (bundling-invariance) | **0** | **0** |
| `milestone` | **0** | 21 |
| `prefund` / `pre-fund` | **0** | 12 |
| `manipulation cost` / `cost of manipulation` | **0** | **0** |
| `formal verification` / `formally verif` | **0** | (uses "not formally verified") |
| `oracle` | **0** | 0 (uses "observation program") |
| `zero-knowledge` | 2 (Mak exhibits only, incidental) | 0 |
| `encrypt` | 2 (Mak exhibits only, incidental) | present (IAC P-I5/P-I6) |
| `leakage` | 2 (VeloxVFX data, Q7/Q8) | 13 |
| `machine-check` | 1 (Ariadne, "machine-checkable at submission time") | 8 |
| `validator` | 1 (Ariadne) | 4 |
| `conformance` | 3 (Ariadne, VeloxVFX) | 5 |
| `executable` | 1 (Ariadne) | present |
| `discretion` | 7 files, all in *institutional* senses (agency discretion, per-repository discretion) | 8, in the *instrument-terms* sense |

**Two corrections to the coordinator's distinctiveness list — and the
correction is good news, not bad:**

Both bundling-invariance and computable manipulation cost are **corpus-wide
zeros in the filed comments AND absent from Draft 6's filed text** — but
neither is absent from this program. Both are fully argued positions sitting
one layer down, in the memo set:

- **Bundling-invariance** is position 1 of
  [definitions-q8-event-contracts-and-options-on-securities.md](definitions-q8-event-contracts-and-options-on-securities.md),
  rated **"Strong — classification should be invariant under bundling and
  unbundling; argued entirely from verified statutory text"** in
  [README.md](README.md). It targets definitions Question 8, which Draft 6's
  definitions comment does not reach (that filing answers Question 1 and
  expressly takes no position on Questions 12-15). The memo already answers
  its own strongest objection: administrability, met with "under invariance,
  neither relabeling changes the answer, so neither is worth doing."
- **Computable manipulation cost** is position 1 of
  [definitions-q15-reference-integrity.md](definitions-q15-reference-integrity.md):
  "the cost of moving the settlement statistic far enough to change which
  outcome pays can be calculated by anyone, in advance, and compared to the
  amount at stake." It carries a scoped counterargument (a computed cost is a
  model output, so require the *inputs* in the terms, not the *conclusion*)
  and a bounded, buildable evidence deliverable (a manipulation-cost table
  over a synthetic constant-product pool). It targets Question 15, which the
  definitions comment's own declared no-position range blocks — but nothing
  blocks it in the IAC statement, whose P-I5 is already about what
  surveillance obligations consume.

So the accurate statement is not "we do not have this ground." It is: **we
have two developed, corpus-unclaimed positions that Draft 6 left on the
floor.** See recommendations 2 and 3.

### 3.2 Theme frequency — positions others also argue

These are the empirically "obvious" ones. Position IDs from the Draft 6
register.

| Our position | Also argued by | How close |
|---|---|---|
| **P-D1** classify by facts, not labels | VeloxVFX (1355-0004, Q1) | Close on the principle: "use product-level criteria rather than labels or technology… and whether the product changes character during its lifecycle", with "reassessment after a material change". **But the named milestone taxonomy is ours alone** — `milestone` is a corpus-wide zero. |
| **P-D5** the reference object and payout allocate the category | **Beylin, VeloxVFX, FalconX, Mak — 4 of 5 definitions filers** | This is the most crowded position in the docket. Beylin asks precisely our question about the 78c(a)(68) event prong applied to issuer-specific references; VeloxVFX proposes a "dominant-reference and material-exposure analysis"; FalconX routes single-security/NBSI references to SBS; Mak asserts it categorically. **P-D5 is not distinctive.** |
| **P-R1** report normalised economic events; provenance adds, never replaces | VeloxVFX (1354-0004, Q3); Ariadne (Section C) | VeloxVFX states our exact counterargument answer: onchain data "may omit legal identity, amendments, offchain terms, allocations, collateral" and should be "supporting evidence, not… an automatic substitute". Ariadne argues the same normalisation from the "unit of report" angle without the chain framing. **Substantially covered.** |
| **P-R3** corrections supersede, never overwrite | VeloxVFX ("preserve validation and correction history"; every lifecycle event "preserved through closure"); Ariadne (warns that deleting validations "convert[s] visible rejections into invisible data corruption"; asks for a normative event vocabulary including "correct") | Direction shared by both; **neither states the rule as sharply as we do.** Partially covered. |
| **P-R4** dissemination as an explicit versioned leakage policy | VeloxVFX (Q7, Q8): calibrated caps, delays, aggregation, masking of "highly identifying combinations", full regulatory data preserved | Same structure, minus the versioning, the per-field stated purpose, and any onchain leakage-surface analysis. **Partially covered.** |
| **P-R6** publish reporting rules as governed executable validators | **Ariadne (R1, R2, R3) — filed 2026-07-14, six weeks ahead of us** | This is the headline. Ariadne's R2 is "validation logic published in executable form, with a public conformance corpus"; R1 is the field dictionary "as a versioned schema rather than a PDF"; R3 is "a normative lifecycle event model with worked examples". That covers items 1, 2, 3, 4, 5, and 7 of our seven-item package, **with more governance detail** (semantic versioning, published release calendar, machine-readable diffs between versions, joint change governance) **and real empirical backing** (an archive of disseminated SDR files; corpus searches; record counts). **P-R6 is not white space and we are second to it.** |

### 3.3 White space — positions nobody else filed

| Our position | Status | Note |
|---|---|---|
| **P-D2** publication of market software, without more, is not a regulated activity | **Unclaimed** | Nobody in either docket addresses software publication versus operation. |
| **P-D3** formation where binding effect and consideration coincide | **Unclaimed** | No filer analyses formation timing at all. |
| **P-D4** instrument formation and contingent exposure are separate findings; a complete set plus recombination is fully hedged | **Unclaimed** | The hedged-whole analysis appears nowhere. |
| **P-D6** prefunding, loss ceilings, fail-closed terms are risk facts, not classification exclusions | **Unclaimed** | `prefund` is a corpus-wide zero. |
| **P-D7** separate instrument / venue / intermediary / clearing findings | **Unclaimed, and actively contradicted** | See 3.4 — FalconX proposes the opposite. |
| **P-R2** hedged complete-set funding is not a contingent-position print | **Unclaimed** | |
| **P-R5** a proof satisfies a reporting element only where the required proposition is exactly what its statement establishes | **Unclaimed** | Cryptographic proofs appear in the corpus only inside Mak's OCR exhibits. |
| **P-I1 … P-I7** (all seven IAC positions) | **Unclaimed on the IAC docket** | The docket has one procedural comment and one non-comment. |
| Clear / Shielded / Dark information-flow taxonomy | **Unclaimed** | No filer distinguishes who-learns-what. |
| Onchain leakage-surface analysis (mempool, fee payer, funding graph, failed instruction, settlement graph) | **Unclaimed** | The only two `leakage` hits are VeloxVFX's generic dissemination-risk answers. |
| Three-record separation (public transparency / exact confidential / machine-verifiable validation package) | **Unclaimed** | Ariadne separates public dissemination from regulatory data implicitly but never names a third verification record. |
| "The rule is the check" — a validator that re-derives the answer by full recomputation rather than checking a format | **Unclaimed** | Ariadne's conformance corpus checks *outcomes against expected outcomes*; it does not propose recomputation as the validation form. |
| Rejected-versus-backend-unavailable as distinct reporting states | **Unclaimed** | |
| Discretion-freedom as a checkable property of the instrument's own terms | **Unclaimed** | All seven `discretion` hits elsewhere are institutional (agency discretion, per-repository discretion). Ours is the only filing treating the absence of a human chooser inside the terms as a property one can check. |
| A machine-checked property of the submitter's own artifact, offered as evidence | **Unclaimed** | No filer in either docket cites a machine-checked property of anything they built. |
| **Bundling-invariance of classification** (memo layer, not in Draft 6) | **Unclaimed corpus-wide** | `bundl` is a zero across all 13 attachments and both inline comments. Argued and rated Strong in the memo set; unfiled. |
| **Computable manipulation cost / reference-specification integrity** (memo layer, not in Draft 6) | **Unclaimed corpus-wide** | Massad treats susceptibility to manipulation as a review criterion and VeloxVFX lists "manipulation-pathway reconstruction", but nobody computes a cost or asks for the reference specification in the terms. |

### 3.4 The one live counter-position

**FalconX Bravo (CFTC-2026-1355-0006) proposes that a Perpetual Securities
Contract be classified by whether it is *listed* under the security-futures
framework** — listed there, it is a security futures product; offered
bilaterally, over-the-counter, on a non-U.S. venue, or "through a
decentralized finance ('DeFi') protocol", it is an SBS. FalconX defends this
as "a clear, objective, and readily verifiable basis" for classification.

That makes the **venue finding dispositive of the instrument finding** — the
precise collapse P-D7 rejects, argued by a registered swap dealer with a
CEO signature and a cc list to nine named SEC and CFTC officials. It is the
only institutionally sponsored position in either docket that our positions
directly contradict, and it is the newest filing on the docket (2026-08-17).

### 3.5 Style norms, and where Draft 6 sits

| Dimension | The distribution | Draft 6 |
|---|---|---|
| **Length** | Substantive letters cluster at **4-15 pp / 1.6k-6k words**; median ~9-11 pp. Outliers: Allen's 1-page member statement; Mak's ~200 pp of exhibits. | 6 / 7 / 6 / 1 pp; 2,695 / 2,968 / 2,708 / 462 words. **Median-to-short end of normal.** Length is not a problem; we are within the band and the hard budget was the right call. |
| **Structure** | Three shapes: (a) letterhead + narrative argument + numbered footnotes (Beylin, Massad, FalconX); (b) question-by-question with the question quoted above the answer (Ariadne, VeloxVFX, Beylin partly); (c) a recommendations table up front, then sections (Ariadne R1-R6, VeloxVFX "Executive Direct Answers", FalconX's (i)/(ii)/(iii)). | "Summary of positions" list, argued sections, "Specific requests", "Limits", claim-basis appendix. That is (c) + (a). **Structurally normal**, with one addition nobody else has: the per-claim evidentiary-basis appendix. |
| **Citation practice** | **Bimodal.** Lawyers/institutions use Bluebook footnotes with FR/USC/CFR pincites and URLs: Massad 33 footnotes, FalconX 30, Beylin 7. Non-lawyers use zero (VeloxVFX, both letters) or inline citations without footnotes (Ariadne). | 6 / 4 / 11 distinct source notes, each with URL **and retrieval date**. Fewer than the lawyers, far more disciplined than the non-lawyers. **The retrieval-date practice appears in no other filing.** |
| **Tone** | Two poles. Adversarial (Massad: "just plain stupid"; "Well, thank goodness it's willing to draw[] the line somewhere"; "Doesn't it have better things to do?"). Deferential-modal (Ariadne: "we respectfully suggest", "the Commissions may wish to consider"; VeloxVFX's "Scope and Institutional Boundary" disclaimer). | Declarative but not adversarial: "The Commission should adopt…", followed by the strongest objection named and answered in text. **This state-the-counterargument-and-answer-it move appears in no other filing.** Ariadne comes closest (it concedes the advantages of per-repository dissemination before recommending against consolidation). Genuine differentiator. |
| **Self-limitation** | Two filers carry explicit boundary blocks: VeloxVFX's "Scope and Institutional Boundary" paragraph, Ariadne's soft modals. | Our "Limits" section plus a per-claim basis appendix is **more granular than anyone's**. Nobody else publishes a claim-basis table. |
| **Evidence** | Only Ariadne offers original empirical data (an SDR-file archive, corpus searches, record counts, storage estimates, observed access blocks). Nobody offers built artifacts. | We offer built artifacts and machine-checked model properties. **Nobody in either docket offers both empirics and artifacts** — and Ariadne's letter shows how much credibility original data buys. |

**Summary verdict.** Draft 6's length, structure, and register are all inside
the norm — the packet does not look eccentric. Our *distinctiveness* is
narrower than the position register implies on the data docket (P-R6 is a
crowded second place behind Ariadne, P-R1/P-R3/P-R4 are partly covered, P-D5
is the most-argued position in the definitions docket) and **wider than
expected everywhere else** (P-D2, P-D3, P-D4, P-D6, P-D7, P-R2, P-R5, all
seven IAC positions, and the entire Clear/Shielded/Dark and
leakage-surface apparatus are unclaimed).

The sharpest single result: **the two positions this program developed and
then did not file — bundling-invariance and computable manipulation cost —
are both corpus-wide zeros.** The distinctiveness gap is not a research gap;
it is a scoping decision, and it is reversible before the deadline.

---

## Part 4 — Three recommendations

### 1. Cede the generic layer of P-R6 to Ariadne by name, and spend the space on the onchain and confidentiality layers nobody filed

Ariadne Dataworks filed the schema-first / executable-validators /
public-conformance-corpus argument on **2026-07-14**, in 15 pages, with more
governance machinery than our seven-item package and with an empirical
archive behind it. Filing the same argument second, thinner, and without data
converts our strongest-sounding data position into a weaker echo — and a
staff reader who has read both will notice.

Concretely: **cite CFTC-2026-1354-0002 by docket number and expressly endorse
R1-R3 rather than restating them.** An endorsement costs no credibility,
strengthens the record for a position we actually want adopted, and buys back
roughly a page. Spend that page on what nobody filed: the three-record
separation, the "rule is the check" full-recomputation validator form, the
rejected-versus-backend-unavailable failure states, and the Question 8
leakage-surface enumeration (mempool, fee payer, funding graph, failed
instruction, settlement graph). That last item is the only technical content
in either docket that no other filer touches at all.

### 2. Rebalance the definitions comment: demote P-D5, promote bundling-invariance from the memo layer, and answer FalconX by name

Three moves, none of which costs length or touches a ceiling.

**Demote P-D5.** It is argued by four of the five definitions filers — a law
professor, a swap dealer, a small firm, and a retail-advocacy letter. Leading
with it places us in the middle of the crowd. **P-D2, P-D3, P-D4, P-D6, and
P-D7 are argued by nobody in either docket** and should lead instead.
Reordering is free.

**Promote bundling-invariance.** The Question 8 memo's position — a rule that
answers differently for a bundle and for its parts "can be defeated by an
operation these designs perform routinely, at will, and for no cost" — is
rated Strong, is argued entirely from verified statutory text, and is a
corpus-wide zero. It is also the *strongest available attack on the crowded
ground*: everyone else is arguing about which reference object lands a
contract in which category, and this position says the category rule must
survive an operation that every one of those filers' products can perform.
Draft 6's definitions comment currently reaches only Question 1; adding the
Question 8 position is the single highest-yield distinctiveness move
available, and the argument is already written.

**Answer FalconX by name.** Its listing-status test (filed 2026-08-17) is a
live, institutionally sponsored contradiction of P-D7: it makes the venue
finding dispositive of the instrument finding, expressly including
DeFi-protocol offerings. Answering it — the venue fact is real and separately
findable, but it cannot carry the instrument category, because the same
instrument offered two ways is the same instrument — converts our most
abstract position into the docket's live disagreement, which is exactly what
staff attorneys read for. Note that bundling-invariance and the FalconX
answer reinforce each other: both say a classification rule must not be
defeasible by a costless repackaging.

### 3. Load the IAC statement with the two things the corpus lacks entirely: a rendered machine-checked negative, and the manipulation-cost position

The IAC docket has two comments, one procedural and one not a comment. There
is no archive of prior public statements to this committee. We are not
competing for attention there; we are the only technical statement, and the
Committee's first meeting has no established submission norm to violate.

That makes the IAC statement the right place to spend half a page on the
thing **no filing in either docket has**: a concrete falsifying example from
the submitter's own work. We already have three, currently buried as
one-clause credentials inside prose — N-1 (a fill outside the committed
limits fails and changes nothing), N-2 (declaring an outcome before the
evidence window closes is a declaration the remaining evidence can falsify),
N-3 (two withdrawals can each be valid against the same pool and jointly
overdraw it). Rendered as a small labelled table — *property / the
counterexample / what it forces the design to do* — they become the
statement's signature, they are the only demonstration in the corpus of
discretion-freedom as a checkable property, and they give the Committee
something quotable in a transcript. The existing N-1/N-2/N-3 ceiling wording
carries over verbatim; this is a rendering change, not a claim change.

**Second half of the same recommendation: give the IAC statement the
manipulation-cost position.** The Question 15 memo is scope-blocked in the
definitions comment by that filing's own declared no-position range on
Questions 12-15 — but nothing blocks it here, and it lands directly on P-I5,
which is already about which fields surveillance obligations actually
consume. The position pairs with the machine-checked negatives above and
makes the same underlying point in the supervisory register: **removing
settlement discretion does not remove manipulation risk, it relocates it**,
from a reporter who could lie to an attacker who now knows exactly which
statistic over exactly which window decides the payout. That is a
self-critical observation about our own design pattern, which is precisely
the register that distinguishes a research statement from an advocacy letter,
and no filer in either docket makes a comparable one. The ask that follows is
narrow and cheap for the Commission: require the reference specification —
venue, statistic, sampling grid, window, source-failure rule — in the terms,
which converts "watch for manipulation" into "watch this venue during these
minutes."

Sequencing note: the memo's buildable deliverable (a manipulation-cost table
over a synthetic constant-product pool) is **not** required to file the
position, and should not gate it — the position asks for the *inputs* in the
terms, not for a *conclusion*, and the memo's own counterargument section
says why. If the table gets built before August 27 it strengthens the
statement; if it does not, the position stands on the drafting argument
alone.

---

## Part 5 — Re-run checklist before filing

1. Re-query both joint dockets and the IAC docket. Comment dockets fill in
   the final 72 hours; ISDA, SIFMA, FIA, and the major exchange and
   digital-asset trade associations have not yet filed on either joint
   docket, and their absence on 2026-08-18 is not evidence of absence on
   2026-08-24.
2. Re-check the SEC comment pages — if the SEC begins publishing letters for
   S7-2026-21/22, the visible landscape may change substantially, since some
   filers submit only to the SEC.
3. Retrieve CFTC-2026-1354-0003 (Nathan Fry) with a registered API key; it is
   the one gap in this survey.
4. Re-check the IAC event page after the August 20 meeting for a posted
   agenda, transcript, and any published statements — the deadline is August
   27, so the record is still open after the meeting.
