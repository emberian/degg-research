# Committee-questions research memos

**Internal research memo set — not a filing, not legal advice.** These are
working notes on the questions in the two joint CFTC/SEC requests for comment
that the program's current filings do not answer. They are raw material the
author may or may not choose to develop. Nothing here has been filed, and
nothing here is a legal conclusion.

Written 2026-08-18.

## What is here

The two joint notices, published together at 91 Fed. Reg. 37873 and 37877
(June 24, 2026), ask 42 numbered questions between them (45 counting three
lettered sub-parts). The program's filings answer four of them: the definitions
comment answers Question 1 of FR Doc 2026-12743, and the data comment answers
Questions 3, 8, and 19 of FR Doc 2026-12742. This directory triages the other
thirty-eight and develops the ones where the research program has something to
say.

Question text throughout is quoted verbatim from the Government Publishing
Office text of each notice, with the Federal Register's typographic quotation
marks normalized to ordinary ones. Citations are limited to the two notices
themselves and to authorities on the verified ledger in
[../LEGAL_ANALYSIS.md](../LEGAL_ANALYSIS.md) section 9. Where a position would
need a source that is not on that ledger, the memo says so under a
"Needs verification" heading rather than citing it.

Where a memo states a fact about this program's own artifacts, it does so once,
at section level, and states the evidence class exactly: an offline research
prototype with passing deterministic tests is tested, not formally verified, and
not deployed; a model theorem is a machine-checked statement about a simplified
formal model and not a property of any running system. No memo upgrades a claim
beyond what the filings already support.

## Triage classes

- **(a)** — this research answers the question directly.
- **(b)** — this research partially informs the question.
- **(c)** — out of this program's lane; the one-line reason is given.

Counts across the 42 numbered questions: **(a) 6, (b) 27, (c) 9.** Six of the
(a) and (b) questions are already answered in filings; the rest are triaged
below and eight memos develop the strongest of them.

## The memos

| Memo | Questions | Verdict |
|---|---|---|
| [definitions-q8-event-contracts-and-options-on-securities.md](definitions-q8-event-contracts-and-options-on-securities.md) | Definitions 8 (touching 2, 3, 4) | **Strong** — classification should be invariant under bundling and unbundling; argued entirely from verified statutory text. |
| [definitions-q5-narrow-based-index-status.md](definitions-q5-narrow-based-index-status.md) | Definitions 5 | **Strong** on the composition-versus-price distinction and on requiring the status rule in the contract's terms; **needs evidence** before claiming what existing tolerance rules already cover. |
| [definitions-q7-q10-terms-readable-tests.md](definitions-q7-q10-terms-readable-tests.md) | Definitions 7 and 10 | **Needs evidence** for Question 7 (the CDS half cannot be answered without reading the 2012 guidance); **interesting but weak** for Question 10. |
| [definitions-q15-reference-integrity.md](definitions-q15-reference-integrity.md) | Definitions 15 (touching 14) | **Strong** as a position; **needs evidence** before any quantitative claim. Note: falls inside the definitions filing's express no-position range. |
| [cross-framework-equivalence.md](cross-framework-equivalence.md) | Definitions 12-14 and Data 4, 4a, 4b (touching Data 14) | **Strong as method** on the data notice's Question 4; **out of character** for the definitions filing as currently scoped. |
| [data-q2-q20-reporting-party.md](data-q2-q20-reporting-party.md) | Data 2, 2a, 20 | **Strong** — allocate the reporting obligation by record custody; requires reading Regulation SBSR Rule 901 before filing. |
| [data-q12-q18-data-quality.md](data-q12-q18-data-quality.md) | Data 10, 12, 15, 17, 18 | **Strong** — data quality is a specification problem, and "unsupported" should be a reportable value. |
| [data-q21-q25-identity-and-reference-data.md](data-q21-q25-identity-and-reference-data.md) | Data 21-25 | **Strong, conditional** on reading the UPI designation order — the argument turns on what an attribute-based identifier can express. |

## Question inventory: definitions notice

FR Doc 2026-12743, RIN 3038-AF71, SEC File S7-2026-21, 91 Fed. Reg. 37873.
Comments due August 24, 2026.

| # | Subject | Class | Disposition |
|---|---|---|---|
| 1 | Products raising interpretive questions; principled objective criteria; new or revised rules or interpretations | (a) | **Answered** by the definitions filing. |
| 2 | Whether the CEA 1a(47)(B) exclusions need clarity | (b) | Partly covered by the filing's position that prefunding and determinism are risk facts, not exclusions; the (B)(iii) half is developed in the Question 8 memo. |
| 3 | Lines between swaps, SBS, and mixed swaps; scope of the three SBS prongs | (b) | Covered by the filing's reference-object position; the prong-level material is in the Question 8 and Question 5 memos. |
| 4 | When an instrument is based on "any interest" in a security, particularly when not based on its value | (b) | Touched by the Question 8 memo (settlement function as the discriminator); not developed separately. |
| 5 | SBS narrow-based-index prong; composition changes versus price changes; tolerance and grace periods | (b) | **Memo.** |
| 6 | SBS single-security prong | (b) | Not developed: this program's only contribution is the reference-object point already in the filing, and the rest is a securities characterization question with no structural angle. |
| 7 | SBS event prong; when an event "directly affects" issuer financials; relation to CDS guidance | (b) | **Memo** (with Question 10). |
| 8 | Event contracts referencing securities; the options exclusion; distinguishing from listed binary options | (a) | **Memo.** |
| 9 | Notes, bonds, and evidence of indebtedness that are securities | (b) | Not developed. One line worth keeping: a fully collateralized claim set has no obligor whose performance can fail, and an instrument with no obligor is a poor fit for "evidence of indebtedness." The rest of the question is a structured-notes matter this program has no experience with. |
| 10 | Security forwards "intended to be physically settled" | (b) | **Memo** (with Question 7). |
| 11 | Futures and security futures; cash-settled "perpetual" contracts; effects on liquidity, price discovery, and hedging | (c) | Out of lane: the second half is market-data economics on markets this program has neither data about nor experience in, and the perpetual-contract characterization question turns on features (no maturity, periodic funding) the program's designs do not have. |
| 12 | Alternative compliance; what "substantially similar" should mean | (b) | **Memo.** Note: the filing expressly takes no position on 12-15. |
| 13 | Joint or coordinated registration, tailored rules, deemed filing | (b) | Touched by the same memo (shared semantics as the prerequisite); the institutional design question is out of lane. |
| 14 | Surveillance, examination, and enforcement under alternative compliance; enhanced data sharing | (b) | Touched by the same memo. |
| 15 | Deterring manipulation and trading on material non-public information; cross-market oversight | (b) | **Memo.** |

The notice also contains an unnumbered general request for comment and data
(section IV) encouraging data-driven input. The experiments proposed in the
memos are the program's answer to that request; none has been run.

## Question inventory: data reporting notice

FR Doc 2026-12742, RIN 3038-AF70, SEC File S7-2026-22, 91 Fed. Reg. 37877.
Comments due August 24, 2026.

| # | Subject | Class | Disposition |
|---|---|---|---|
| 1 | Which requirements or elements would benefit from harmonization | (c) | Out of lane: requires reporting experience and element-level cost data this program does not have. |
| 2 | Should the platform report platform-executed, non-cleared trades | (b) | **Memo.** |
| 2a | Reporting party for exchange-effected SBS issued by a clearing agency; platform versus counterparty | (b) | **Memo** (same). |
| 3 | Reporting for transactions occurring on a blockchain | (a) | **Answered** by the data filing. |
| 4 | Whether the SEC should amend its SBS reporting rules to harmonize | (b) | **Memo** (cross-framework equivalence). |
| 4a | Scope of such amendments; identical technical specifications; staying harmonized over time | (b) | **Memo** (same); the staying-harmonized half is the strongest part. |
| 4b | If not, what amendments should be considered | (b) | Not developed separately. |
| 5 | Elements of limited practical utility relative to cost | (c) | Out of lane: no reporting history and no cost data. |
| 6 | Categories of elements that could be combined or eliminated | (c) | Same reason. |
| 7 | SBS dissemination of large notional transactions; the credit size cap | (c) | Out of lane: cap calibration is market-data economics requiring transaction data this program has none of. |
| 8 | Whether dissemination affects liquidity or discloses identity and strategy | (a) | **Answered** by the data filing. |
| 9 | Which disseminated elements are most useful; changes improving price transparency | (b) | Not developed: the program's contribution is the method already filed under Question 8 — each public field carries a stated purpose and a re-identification review — and the substance requires being a market-data user. |
| 10 | Elements never or rarely populated in practice | (b) | Touched by the data-quality memo (an element nobody can compute is not a compliance problem). |
| 11 | Duplicative elements | (c) | Out of lane: requires the technical specification and reporting experience. |
| 12 | Elements persistently hard to report consistently; static versus changing; practical steps | (b) | **Memo.** |
| 13 | Changes to reporting deadlines or dissemination methods | (b) | Not developed: the onchain-specific part (finality timing, correction on reorganization) is already in the data filing; deadline calibration is out of lane. |
| 14 | Separate dissemination by each repository; price-discovery effects | (b) | Touched by the equivalence memo: a shared event model makes cross-repository aggregation mechanical. |
| 15 | Additional repository validation of reported data | (b) | **Memo** (data quality). |
| 16 | Most operationally complex aspects of the frameworks | (c) | Out of lane: this is an operator's answer and the program has never operated a reporting obligation. |
| 17 | Validation rules and lifecycle-event obligations that could be simplified | (b) | **Memo** (data quality). |
| 18 | Materiality or de minimis threshold for error correction | (b) | **Memo** (data quality). |
| 19 | Machine-readable rule structures and standardized reporting logic | (a) | **Answered** by the data filing. |
| 20 | Reporting hierarchies determining the reporting counterparty | (b) | **Memo** (reporting party). |
| 21 | Limitations of the UPI and other standardized product identifiers | (a) | **Memo.** |
| 22 | When a standard other than the UPI should identify products | (b) | **Memo** (same). |
| 23 | Additional opportunities for standardized and static reference data | (b) | **Memo** (same). |
| 24 | When a standard other than the LEI should identify counterparties | (b) | **Memo** (same) — a wallet address is an account, not a counterparty. |
| 25 | Trade-by-trade information better captured as reference data | (b) | **Memo** (same). |
| 26 | Timelines and sequencing for implementation | (c) | Out of lane: implementation planning for systems the program has no view into. |
| 27 | Structuring implementation to minimize compliance costs | (c) | Out of lane: no cost data. |

The notice also contains an unnumbered request for data (section III) seeking
compliance and operational costs, error and rejection rates, and correction
frequency and latency. The program has none of those figures and should not
imply otherwise.

## Sources this memo set would need before any of it is filed

None of the following is on the verified ledger in
[../LEGAL_ANALYSIS.md](../LEGAL_ANALYSIS.md) section 9. Each is relied on in
these memos only as characterized or quoted by the notice that cites it, and
each is flagged in the memo that touches it.

| Source | Needed by | Why it matters |
|---|---|---|
| 17 C.F.R. 240.3a55-2, 240.3a55-3, 17 C.F.R. part 41 (index tolerance and grace periods); Exchange Act section 3(a)(55); 15 U.S.C. 78c(a)(68)(E) | Definitions Question 5 memo | The existing rules may already resolve part of the proposal. |
| Product Definitions Adopting Release, 77 Fed. Reg. at 48267 (CDS rules and guidance) | Definitions Question 7 memo | The ledger records this release as fetched for title, date, agencies, and action only; its substance was not read. |
| SEC-CFTC Memorandum of Understanding (Mar. 11, 2026) | Cross-framework equivalence memo | Cited in both notices; may already contain a coordination framework the memo would duplicate. |
| Regulation SBSR, 17 C.F.R. 242.900-909 (esp. Rule 901); SBSDR rules, 17 C.F.R. 240.13n-1 to 13n-12; 17 C.F.R. 45.3(a), 45.8, 43.3; 15 U.S.C. 78f(l) | Reporting-party memo | A position on how Rule 901 should be amended requires reading Rule 901. |
| 2019 Compliance Statement (85 Fed. Reg. 6270, 6346-49) and its extension | Cross-framework equivalence memo | Relied on as the notice's characterization only. |
| CFTC Parts 43 and 45 Technical Specification (Mar. 2023) | Data-quality memo | The memo deliberately names no element; a filing that names one must read the specification. |
| UPI designation order, 88 Fed. Reg. 11790, and the DSB product classification system | Reference-data memo | The memo's central factual claim is about what an attribute-based identifier can express. |
| 7 U.S.C. 2(a)(13); 15 U.S.C. 78m(m) | Background in several memos | Cited in the data notice for the statutory reporting and public-availability mandates; not independently retrieved and not relied on for any position. |

## Related documents

- [../LEGAL_ANALYSIS.md](../LEGAL_ANALYSIS.md) — in-house legal analysis and
  the citation ledger these memos draw from.
- [../../research/GUARDED_EVENT_FOUNDATIONS.md](../../research/GUARDED_EVENT_FOUNDATIONS.md)
  — the technical exposition behind the milestone vocabulary.
- [../../VERDICTS.md](../../VERDICTS.md) — current status of every claim about
  the program's artifacts.
