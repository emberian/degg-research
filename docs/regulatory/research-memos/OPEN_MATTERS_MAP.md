# Open matters map — what else the CFTC has open, and what this program could answer

Internal research memo, prepared 2026-08-18 by the open-matters research
lane. See [README.md](README.md) for status and citation rules. This memo is
a survey of the public record; it is not legal advice, a filing
authorization, or a commitment to file anything. Nothing in this lane was
filed, submitted, or sent; no regulator was contacted.

Companion to [FILED_COMMENTS_LANDSCAPE.md](FILED_COMMENTS_LANDSCAPE.md),
which surveys *who else is on the three dockets this program is already
working*. This memo asks the complementary question: **what else is open at
all.** It does not re-cover the joint definitions RFC, the joint data
reporting RFC, or the substance of other filers' comments.

**All retrievals in this memo were performed 2026-08-18** (verified against
`TZ=America/New_York date` at the start of the lane: Tue Aug 18 04:16 EDT
2026). Deadlines move and dockets fill; every count and every "nothing open"
finding below is a snapshot of that morning and should be re-run before it is
relied on.

---

## Part 0 — Method, and exactly what was blocked

What worked:

- **federalregister.gov API v1**, `https://www.federalregister.gov/api/v1/documents.json`
  filtered by `conditions[agencies][]=commodity-futures-trading-commission`.
  This is the authoritative source for comment deadlines and it required no
  key and imposed no rate limit. 61 CFTC documents published in 2026 were
  enumerated; 43 of them carry a `comments_close_on` date.
- **federalregister.gov per-document API**, `.../documents/<FR-doc>.json?fields[]=regulations_dot_gov_info`,
  which returns the regulations.gov docket ID, the comment-form document ID,
  and a comment count for each Federal Register document. This is how the
  docket IDs and counts below were obtained.
- **Full text of Federal Register documents** via
  `https://www.federalregister.gov/documents/full_text/text/<yyyy>/<mm>/<dd>/<doc>.txt`.
  Read in full or near-full: FR Docs 2026-12784 (24/7 and perpetuals RFC),
  2026-15216 (its extension), 2026-13182 (portfolio margining), 2026-15948
  (Conflicts and Affiliations), 2026-16328 (IAC meeting notice), 2026-16423
  (IAC renewal), and the summary blocks of the PRA notices.
- **cftc.gov** press releases, event pages, advisory-committee pages, and
  media downloads, with a browser user agent.

What was blocked, precisely:

- **regulations.gov API v4 returned `OVER_RATE_LIMIT` (HTTP 429) on every
  request today**, including from a backoff loop retrying at 5-minute
  intervals, which was still returning 429 when this memo was finished. The
  public demo key (`api_key=<DEMO_KEY>`) appears to have been exhausted
  before this lane started. Consequence: **no comment bodies were
  read and no docket listings were retrieved from regulations.gov.** Every
  comment count below comes from the Federal Register API's
  `regulations_dot_gov_info.comments_count` field, which may lag. A
  registered API key would close this gap.
- **sec.gov returned HTTP 403 "Request Rate Threshold Exceeded" on every path
  tried**, through both a plain client and a browser user agent, and through
  the WebFetch tool. Five URLs were attempted. Consequence: **Part 5's SEC
  paragraph rests on search-result metadata only and no SEC page was read
  today.** It is marked accordingly.
- **cftc.gov/LawRegulation/PublicComments/ and .../FederalRegister/ProposedRules/
  return Cloudflare HTTP 403** to this client. The Federal Register API
  substituted for both, and is strictly better for deadlines.

Nothing in this memo is inferred from an unread document. Where a document
was not read, the memo says so.

---

## Part 1 — The IAC meeting: the agenda is published, and the event page does not say so

### The finding that changes the drafting brief

**An agenda for the August 20 meeting was published on 2026-08-13 and it is
not linked from the event page.** This resolves the open item in
[../CFTC_IAC_MEETING_BRIEF.md](../CFTC_IAC_MEETING_BRIEF.md) — which records
"As checked on August 17, the event page did not link a narrower agenda" and
carries "any late agenda have been rechecked" as an unticked pre-filing
checklist item. That observation was correct and is *still* correct: a
re-check of <https://www.cftc.gov/PressRoom/Events/opaeventiac082026> on
2026-08-18 shows no agenda, no panel list, and no mention of AI, crypto, or
prediction markets. The agenda exists anyway, in a separate press release:

- **Press Release 9283-26**, "Chairman Selig Announces Agenda for August 20
  Innovation Advisory Committee Meeting in Washington," August 13, 2026,
  <https://www.cftc.gov/PressRoom/PressReleases/9283-26> (retrieved
  2026-08-18): "Attendees will discuss topics related to the regulation of
  crypto assets, artificial intelligence, and prediction markets."
- **The agenda PDF itself**, <https://www.cftc.gov/media/14476/IACMeetingAgenda082026/download>
  (retrieved 2026-08-18, 1 p.), which the press release links and the event
  page does not.

The earlier announcement, **Press Release 9279-26** (August 10, 2026,
<https://www.cftc.gov/PressRoom/PressReleases/9279-26>), carries no agenda —
it is the text the event page mirrors.

### The agenda, verbatim in structure

Inaugural Meeting: Innovation Advisory Committee, August 20, 2026,
1:00 pm - 4:00 pm Eastern.

| Time | Segment |
|---|---|
| 1:00 - 1:30 | Introductions and Opening Remarks — Michael J. Passalacqua (DFO); **Walt Lukken, Chair, Innovation Advisory Committee**; Michael S. Selig, Chairman and Sponsor |
| 1:30 - 2:20 | **Session I — Crypto's Regulatory Evolution: From Uncertainty to Clarity** |
| 2:20 - 2:30 | Break |
| 2:30 - 3:05 | **Session II — Artificial Intelligence: Preparing for Intelligent Markets** |
| 3:05 - 3:55 | **Session III — Prediction Markets: Innovation, Jurisdiction, and the Future of Event Contracts** |
| 3:55 - 4:00 | Closing Remarks — Chairman Selig |

The sub-bullets are what matter. Quoted from the agenda PDF:

**Session II (AI), 35 minutes** — three headings:

- "The AI Revolution in Derivatives Markets": "The rapid evolution of
  artificial intelligence across financial services"; "Emerging applications
  in trading, compliance, surveillance, and risk management."
- "**The Rise of Agentic Finance**": "**Autonomous agents capable of executing
  financial transactions and managing portfolios**"; "Intersection of crypto
  and AI."
- "Preparing for the Future": "**How existing regulatory principles apply to
  AI-enabled market participants**"; "**Whether additional guidance or best
  practices would promote responsible innovation**."

**Session III (prediction markets), 50 minutes** — the two headings that
reach this program:

- "Building a Durable Regulatory Framework": "**Product design principles for
  event contracts**"; "Expectations for exchanges listing innovative
  products"; "**Market surveillance, manipulation concerns, and customer
  protections**."
- "The Road Ahead": "**Identifying principles that can guide the next
  generation of event-based products**."

**Session I (crypto), 50 minutes** — one heading reaches this program:
"Cybersecurity, operational resilience, and crypto asset infrastructure as
foundational elements of trusted markets."

### Supplementary IAC materials

- **Meeting notice, 91 FR 51697** (FR Doc 2026-16328, August 11, 2026),
  <https://www.federalregister.gov/documents/2026/08/11/2026-16328/innovation-advisory-committee>:
  "the IAC will discuss topics including crypto assets, artificial
  intelligence, and prediction markets, along with recent CFTC activity in
  these markets." Written statements due **Thursday, August 27, 2026**;
  submission via regulations.gov, mail, or hand delivery; comments "will be
  published without review for, and without removal of, any personal
  identifying information."
- **Renewal notice, 91 FR 52047** (FR Doc 2026-16423, August 12, 2026),
  <https://www.federalregister.gov/documents/2026/08/12/2026-16423/renewal-of-the-innovation-advisory-committee>.
  Two things in it are directly useful. First, the committee's stated scope
  includes advice on "**the appropriate level of investment in technology at
  the Commission to meet its surveillance and enforcement responsibilities**"
  — an express invitation for the position about what surveillance
  obligations actually consume. Second, its justification names the
  innovation set: "blockchain technologies, artificial intelligence, machine
  learning, prediction markets, and cybersecurity."
- **No new subcommittee reports.** The IAC page
  (<https://www.cftc.gov/About/AdvisoryCommittees/IAC>, retrieved 2026-08-18)
  still lists only the two TAC-era reports — "Responsible Artificial
  Intelligence in Financial Markets" (approved 2024-05-02) and
  "Decentralized Finance" (approved 2024-01-08). No background paper, panel
  list, or discussion draft has been posted for the August 20 meeting.
- **Membership: 43 members listed**, 41 industry principals and two academics
  (Professors Harry Crane and Carla Reyes). Two entries are worth flagging to
  the drafting lane, both verified from that page: **Raghu Yarlagadda,
  Co-Founder and CEO of FalconX**, is a member — FalconX Bravo, Inc. filed
  the one live counter-position to P-D7 on the definitions docket
  (CFTC-2026-1355-0006, per the landscape memo) — and **Sergey Nazarov, CEO
  of Chainlink Labs**, is a member, which places a reference-price
  infrastructure vendor in the room for any argument about how settlement
  references should be specified.

### Docket state

Docket **CFTC-2026-1717**, comment document **CFTC-2026-1717-0001**.
Comment count as reported by the Federal Register API on 2026-08-18: **2** —
unchanged from the landscape memo's survey earlier the same day, and still
one procedural comment plus one non-comment. **There is still no substantive
technical statement on the IAC docket.**

### What this does to the IAC statement

The landscape memo's recommendation 3 (render N-1/N-2/N-3 as a labelled
falsifier table; add the manipulation-cost position) was written before the
agenda was visible. The agenda **confirms and sharpens it**, and adds one
demand it did not anticipate:

1. Session III's "product design principles for event contracts" and "market
   surveillance, manipulation concerns" is the exact frame the
   manipulation-cost / reference-specification-in-the-terms position argues
   in. That position is now aimed at a named agenda item rather than at a
   general committee.
2. Session II names **agentic finance and autonomous execution** explicitly,
   and asks "how existing regulatory principles apply to AI-enabled market
   participants." This program's discretion-freedom material — a checkable
   property of an instrument's own terms, with machine-checked negatives —
   is the closest thing it has to an answer, and it is an unusual answer:
   most of the room will discuss governing the *model*, and this program can
   argue for constraining the *instrument the model trades*, which is
   checkable in a way a model is not. **Nothing in the current IAC body is
   addressed to Session II** — a case-insensitive grep of
   [../typst/iac/body.typ](../typst/iac/body.typ) for "artificial
   intelligence", "AI", "agentic", "autonomous agent", "machine learning"
   and "automated trading" returns zero hits (checked 2026-08-18). That is
   the gap the agenda opens.

   **Coordination note.** A sibling lane is already in this gap: as of this
   memo's completion, [OPERATORLESS_AGENTS.md](OPERATORLESS_AGENTS.md)
   exists in this directory and states that it produced a candidate IAC
   addendum, gated on the author's express go. This lane did not read it
   beyond its header and takes no view on its content; the point of
   recording it here is that **the Session II gap identified from the
   agenda and the addendum that lane is drafting are the same gap**, and
   the coordinator should treat them as one decision rather than two.
3. Session I's "operational resilience" bullet is the natural home for the
   fail-closed and abort-behaviour material, but it is the weakest of the
   three fits and should not displace either of the others in a 6-page
   budget.

---

## Part 2 — Every CFTC comment period open on 2026-08-18

Complete enumeration from the Federal Register API. **Twelve matters are
open.** Eleven of them carry a `comments_close_on` date in the API; the
twelfth is the IAC meeting notice, whose August 27 statement deadline
appears only in the notice's DATES prose and not in that field — which is
worth knowing, because a deadline query alone does not surface
advisory-committee statement windows. Two of the twelve (the joint
definitions and joint data RFCs, both due August 24) are this program's
existing filings and are not re-analysed here.

| Deadline | Matter | FR cite / RIN | Docket | Comments | Fit |
|---|---|---|---|---|---|
| **Mon Aug 24** | Joint RFC, further definition of swap and SBS | 91 FR 37873, FR Doc 2026-12743, RIN 3038-AF71 / S7-2026-21 | CFTC-2026-1355 | 5 | *existing filing* |
| **Mon Aug 24** | Joint RFC, swap and SBS data reporting | 91 FR 37877, FR Doc 2026-12742, RIN 3038-AF70 / S7-2026-22 | CFTC-2026-1354 | 4 | *existing filing* |
| **Wed Aug 26** | RFC, extension of standard futures to 24/7 trading and perpetual contracts on physically delivered or storable energy commodities | 91 FR 38334 (FR Doc 2026-12784), extended by 91 FR 47158 (FR Doc 2026-15216), RIN 3038-AF75 | CFTC-2026-1388 | 86 + 4 | **strong, narrow** |
| **Thu Aug 27** | IAC written statements | 91 FR 51697, FR Doc 2026-16328 | CFTC-2026-1717 | 2 | **strong** |
| **Mon Aug 31** | Joint RFC, further implementation of portfolio margining and cross-margining of securities and derivatives | 91 FR 39579, FR Doc 2026-13182, RIN 3038-AF72 / S7-2026-23 | CFTC-2026-1420 | 2 | possible |
| **Mon Sep 28** | PRA 3038-0097, process for review of swaps for mandatory clearing | 91 FR 47225, FR Doc 2026-15166 | CFTC-2026-1651 | 1 | out-of-lane |
| **Mon Oct 5** | **Conflicts and Affiliations** (NPRM) | 91 FR 50926, FR Doc 2026-15948, RIN 3038-AF76 | CFTC-2026-1686 | 6 | possible |
| **Mon Oct 5** | PRA 3038-0103, Ownership and Control Reports, Forms 102/102S, 40/40S, 71 (trader and account identification) | 91 FR 50813, FR Doc 2026-16040 | CFTC-2026-1687 | 0 | possible, small |
| **Mon Oct 5** | PRA 3038-0076, requirements for derivatives clearing organizations | 91 FR 50816, FR Doc 2026-16048 | CFTC-2026-1685 | 0 | out-of-lane |
| **Mon Oct 5** | PRA 3038-0091, disclosure and retention re cleared swaps customer collateral | 91 FR 50817, FR Doc 2026-16074 | CFTC-2026-1684 | 2 | out-of-lane |
| **Mon Oct 5** | PRA 3038-0092, customer clearing documentation and timing of acceptance | 91 FR 50814, FR Doc 2026-16038 | CFTC-2026-1688 | 0 | out-of-lane |
| **Tue Oct 13** | PRA 3038-0005, CPO/CTA rules and FCM monthly reporting | 91 FR 52677, FR Doc 2026-16631 | CFTC-2026-1783 | 0 | out-of-lane |

Comment counts are the Federal Register API's figures on 2026-08-18 and were
not cross-checked against regulations.gov (Part 0).

### 2.1 — 24/7 trading and perpetual energy contracts (due Wed Aug 26)

**What it is.** A Commission-level request for comment on two things: listing
standard energy futures for continuous 24/7 trading without changing
expiration or settlement, and listing perpetual contracts — no fixed
expiration, funding-rate convergence — on physically delivered or storable
energy commodities such as crude oil. 67 numbered questions. The comment
period was extended thirty days on July 28 and the extension added three
further topics, including comment on the **NYMEX 10-Barrel WTI Crude Oil
Futures contract self-certified July 8, 2026 and stayed by the Commission
July 9, 2026** under Regulation 40.2(c). The stay was entered, per the
extension notice, to examine "the reliability and manipulation-resistance
of reference prices and . . . a contract market's capacity to surveil
trading."

**Deadline.** August 26, 2026 (91 FR 47158).

**What this program could uniquely contribute.** Section F ("Reference Price
and Continuous Observability," Questions 38-41) and Section I
("Susceptibility to Manipulation, Surveillance, and the Compliance
Demonstration," Questions 50-53) ask, in the Commission's own words, for
exactly the thing the Question 15 memo argues. Question 40: "Are there
reference-price methodologies . . . which could provide 24/7,
manipulation-resistant observability at every funding interval . . . ?"
Question 50: "the reference price must be reliable at every funding interval
on a continuous basis rather than at a single settlement. What features would
such a contract require to satisfy Core Principle 3 on that continuous
basis?" Question 53: "What would a DCM be required to demonstrate . . . and
if so, on what evidentiary basis?" Question 66 asks for "objective and
generally applicable criteria," naming "demonstrable continuous
manipulation-resistance" as a candidate. Question 41 asks "what additional
cross-market manipulation concerns arise from the funding linkage itself." Question 62
asks what automatic liquidation mechanisms are appropriate and how to keep
them from amplifying stress.

The program's answer set maps onto that with unusual precision: put the
reference specification — venue, statistic, sampling grid, window,
source-failure rule — in the contract's terms; make the cost of moving the
settlement statistic far enough to change the payout computable in advance
from those terms; treat the demonstration as a recomputation rather than a
format check; and state a fail-closed rule for the source-unavailable case
rather than leaving it to discretion. **Nobody has to have energy data to
argue that the demonstration Question 53 asks for should be a stated,
recomputable predicate rather than a narrative.**

**Fit: strong, narrow.** Strong because Questions 38-41, 44, 50-53, 62 and 66
are specification-and-verification questions and this program's whole method
is specification and verification; narrow because the RFC twice states it
wants "data, empirical analysis, transaction- or market-level statistics . . .
rather than . . . conclusory assertions," and this program has **no energy
market data at all** and no view of Cushing storage, deliverable supply, or
weekend liquidity. A filing here would have to answer a named subset,
disclaim the data questions in terms at least as explicit as Draft 6's
"Limits" section, and take no position on whether such a contract should be
listed. It would also be entering a docket carrying roughly ninety comments
already (86 on the original RFC, 4 on the extension), almost certainly from
energy trade associations and exchanges with the data the program lacks — so it must be the specification argument nobody else is
making, not a thinner version of theirs.

### 2.2 — Joint portfolio margining and cross-margining (due Mon Aug 31)

**What it is.** A joint CFTC/SEC request for comment on expanding portfolio
margining and cross-margining across securities and derivatives, including
across cleared and uncleared swaps and security-based swaps, and on account
types and bankruptcy treatment. 19 numbered questions plus a request for
data. Same joint posture, same two agencies, same "Request for Data" closing
section as the two RFCs this program is already answering.

**Deadline.** August 31, 2026.

**What this program could uniquely contribute.** Question 19 — "Should the
Commissions consider distinguishing between single-stock and narrow-based
index futures in considering potential approval of further implementation of
portfolio margining or cross-margining of securities and derivatives?" — is
the narrow-based-index line reappearing in a margin setting, and the
definitions Question 5 memo's composition-versus-price distinction transfers
directly: a margin framework that keys off narrow-based-index status inherits
whatever instability that status has, so the status rule has to be stated in
terms the margin system can evaluate mechanically. The cross-framework
equivalence memo's strongest half — how two frameworks *stay* aligned after
they are aligned once — also applies: the notice contemplates two agencies
approving a shared treatment and says nothing about drift.

**Fit: possible.** Possible rather than strong because the other eighteen
questions are margin-model calibration, collateral usage, account-type and
bankruptcy-treatment questions that need operating and legal experience the
program does not have, and because Section III expressly asks for "empirical
data and quantitative analysis relating to margin efficiency, collateral
usage, liquidity effects, operational costs." A two-question comment on a
nineteen-question notice is a legitimate form, but the marginal contribution
is thin next to the two matters above.

### 2.3 — Conflicts and Affiliations (due Mon Oct 5)

**What it is.** A 70-page NPRM (91 FR 50926-50995) amending parts 1, 37, 38
and 39. It covers SRO/DSRO financial oversight of FCMs, FCM disclosure of
affiliate relationships with a SEF/DCM/DCO, conflicts-of-interest
requirements for SEFs/DCMs/DCOs regarding affiliate FCMs and affiliated
principal trading firms, and guidance on "the sharing of resources including
staffing, technology, and office space, and limitations on the sharing of
non-public information." A regex count over its plain text finds 48
sentences in the form "the Commission requests / seeks / invites comment".

**Deadline.** October 5, 2026.

**What this program could uniquely contribute.** One seam, and it is real.
The document uses "information barrier" 28 times and "non-public information"
81 times, and asks — among other things — "whether reliance on existing
voluntary practice is sufficient" and that exchanges "describe the policies,
procedures, governance separations, information barriers, and disclosures
currently in place." Every one of those is a *program* an entity attests to
having. This program's Clear / Shielded / Dark vocabulary exists precisely to
say which actor learns what, under which corruption model, as a property of a
system rather than as a description of a policy — and the landscape memo
records that no filer on any of the three current dockets distinguishes
who-learns-what. A short comment arguing that an information-barrier
obligation stated as a checkable information-flow property is auditable in a
way a policies-and-procedures attestation is not would be in character and,
on the evidence available, unclaimed.

**Fit: possible, mostly out-of-lane.** The other 47 comment requests are
institutional design — whether a DCM may be DSRO for its own affiliate FCM,
whether affiliate market makers should be prohibited or capped, capital
independence standards, certification frequency, transition periods. The
memo set's own README classifies institutional-design questions as out of
lane, and answering one seam in a rulemaking whose centre of gravity is
elsewhere risks reading as an outsider commenting on a fight it is not in.
The compensating fact is the calendar: October 5 is the only relevant
deadline that does not collide with anything, so this is the one that can be
decided in September on its merits rather than under time pressure.

### 2.4 — PRA 3038-0103, Ownership and Control Reports (due Mon Oct 5)

**What it is.** A Paperwork Reduction Act renewal notice for the collection
behind Forms 102/102S, 40/40S and 71 — the trader and account identification
reports. Its own summary describes the underlying rules as ones "the
Commission adopted to enhance its identification of futures and swap market
participants." **Zero comments filed.**

**Deadline.** October 5, 2026.

**What this program could uniquely contribute.** The identity and
reference-data memo's central claim — that a wallet address is an account,
not a counterparty, and that an attribute-based identifier can only express
what its attribute scheme can express — is about exactly this collection's
subject matter. A PRA notice invites comment on necessity, burden, and
"ways to enhance the quality, utility, and clarity of the information to be
collected," which is a legitimate hook for a short note on what an ownership
and control record can and cannot establish when the controlling party is a
key rather than a person.

**Fit: possible, small.** Small because PRA comments are burden-focused and
short by convention, and because the memo's own README flags that the
reference-data position is "**strong, conditional** on reading the UPI
designation order" — the same conditionality applies here, and the underlying
form instructions would have to be read before anything is asserted about
what the forms currently capture. The upside is asymmetric: zero comments, a
low bar, and a docket that is squarely on the program's identity ground.

### 2.5 — The rest

PRA 3038-0097 (mandatory clearing review), 3038-0076 (DCO requirements),
3038-0091 (cleared swaps customer collateral), 3038-0092 (customer clearing
documentation), and 3038-0005 (CPO/CTA rules and FCM monthly reporting) are
**out-of-lane**: each asks for burden-hour estimates and practical utility
judgments about collections the program has never been subject to and has no
operating experience with. Filing burden estimates it does not have would be
the exact failure mode the "Limits" discipline exists to prevent.

---

## Part 3 — AI at the CFTC: the chain, and the honest answer about what is open

### The answer

**No AI-related comment window is open at the CFTC on 2026-08-18.** A Federal
Register sweep for "artificial intelligence" across all CFTC documents
published since 2023-06-01 returns eight documents. The only two published in
2026 with any comment deadline are the prediction-markets NPRM (closed July
27) and the IAC-related notices (no deadline of their own; the IAC statement
deadline is August 27). **The August 27 IAC written-statement docket is the
only currently open public channel to CFTC AI policy.** That is the single
most consequential finding in this memo for the calendar.

### The chain, with dates

1. **2024-01-25 — staff Request for Comment on the Use of Artificial
   Intelligence in CFTC-Regulated Markets.** Issued by the Divisions of
   Market Oversight, Clearing and Risk, Market Participants, and Data and the
   Office of Technology Innovation. **Never published in the Federal
   Register** — it was a press-release-and-PDF instrument with comments routed
   to the legacy comments.cftc.gov system, which now returns 403 (landscape
   memo, Part 0). PDF: <https://www.cftc.gov/media/10156/AI_RFC_012524/download>
   (retrieved 2026-08-18); the PDF's own deadline line is an unfilled
   placeholder, "[90 days from publication]", and the actual deadline appears
   only in **Press Release 8853-24**
   (<https://www.cftc.gov/PressRoom/PressReleases/8853-24>, retrieved
   2026-08-18): "Comments will be accepted until April 24, 2024." Subject
   matter: AI in "trading, risk management, compliance, cybersecurity,
   recordkeeping, data processing and analytics, and customer interactions,"
   and risks including "market manipulation and fraud, governance,
   explainability, data quality, concentration, bias, privacy and
   confidentiality and customer protection."
2. **2024-05-02 — TAC report "Responsible Artificial Intelligence in
   Financial Markets"**, approved at AI Day
   (<https://www.cftc.gov/media/10626/TAC_AIReport050224/download>). Per the
   IAC renewal notice, the committee approved five AI recommendations in FY
   2024.
3. **2024-12-05 — CFTC Staff Advisory, Letter 24-17**, on the use of AI by
   registered entities and registrants, issued by the Divisions of Clearing
   and Risk, Data, Market Oversight, and Market Participants
   (<https://www.cftc.gov/csl/24-17/download>; announcement, **Press Release
   9013-24**, <https://www.cftc.gov/PressRoom/PressReleases/9013-24>,
   retrieved 2026-08-18). The announcement states it "is informed, in part, by
   public comments received in response to the staff's January 25, 2024
   Request for Comment on AI." **Needs verification before it is cited for
   anything substantive:** the advisory's own text was not read in this lane,
   and no document was found either confirming or withdrawing it under the
   current Chairman. Do not assert that it remains in effect.
4. **2026-03-24 — Innovation Task Force**, launched by Chairman Selig
   (**Press Release 9201-26**,
   <https://www.cftc.gov/PressRoom/PressReleases/9201-26>, retrieved
   2026-08-18). It names three workstreams: "(i) crypto assets and blockchain
   technologies; (ii) **artificial intelligence and autonomous systems**; and
   (iii) prediction markets and event contracts," led by Michael J.
   Passalacqua — the same person who is IAC Designated Federal Officer. It
   "will work with the Commission to develop a clear regulatory framework for
   innovators" and "will coordinate with . . . the U.S. Securities and
   Exchange Commission and its Crypto Task Force." **No dedicated page for
   the Innovation Task Force exists on cftc.gov** — four candidate URLs were
   tried and all returned 404 — and **no public input channel for it has been
   published.** The Task Force is the body that would draft AI guidance, and
   the IAC is the only place it has said it will take public input through.
5. **cftc.gov/ai is not a policy page.** Retrieved 2026-08-18, it is entirely
   about the agency's *internal* use of AI under OMB Memorandum M-25-21: a
   CFTC compliance plan, an AI Use Case Inventory (CSV), and an Acting Chief
   Artificial Intelligence Officer (Matthew Kennedy). No open request for
   input.

### Regulation AT's ghost, and electronic-trading risk

A Federal Register sweep for "automated trading" across CFTC documents since
2024-01-01 returns **exactly one document**: the **Operational Resilience
Framework for Futures Commission Merchants, Swap Dealers, and Major Swap
Participants** NPRM (89 FR 4706, FR Doc 2023-28745, published 2024-01-24;
comments closed 2024-03-02). That proposal was **formally withdrawn on
2025-09-11** (90 FR 43949, FR Doc 2025-17555, "Notice of withdrawal of
proposed rules"): "The Commission does
not intend to issue final rules with respect to this proposal. If the
Commission decides to pursue future regulatory action in this area, it will
issue new proposed rules."
<https://www.federalregister.gov/documents/2025/09/11/2025-17555/operational-resilience-framework-for-futures-commission-merchants-swap-dealers-and-major-swap>
(retrieved 2026-08-18).

So: Regulation AT was withdrawn years ago, its nearest successor on the
technology-risk side was withdrawn eleven months ago, and **nothing is
pending on automated-trading or electronic-trading risk.** Anyone looking for
an open rulemaking to attach an autonomous-agents argument to will not find
one. The argument has to go somewhere else, and the only somewhere else with
an open window is the IAC statement.

### The one place the CFTC has written about trading agents in 2026

The **prediction-markets NPRM, 91 FR 35806** (FR Doc 2026-11854, June 12,
2026; comments closed July 27, 2026) contains the Commission's only 2026
Federal Register discussion of autonomous trading agents, in its
self-regulatory-capacity factor:

> "advancements in artificial intelligence tools, such as model-driven
> trading agents capable of generating rapid, correlated trading patterns,
> may strain conventional surveillance models, increasing the need for
> predictive analytics and real-time anomaly detection"

and in its cost discussion: "innovations in the marketplace, including growth
in artificial intelligence driven trading, may entail continuous surveillance
upgrades, raising ongoing operational expenditures."

That is the Commission's own framing of the problem, in a closed docket, in
language that maps onto the IAC's Session II. It is the sentence an IAC
statement should engage: the Commission's stated worry is that agent-driven
trading strains *surveillance*, and this program's position — that removing
settlement discretion relocates manipulation risk rather than removing it,
and that the cure is putting the reference specification in the instrument's
terms so surveillance knows which venue to watch during which minutes — is a
direct, non-obvious response to that worry rather than a change of subject.

---

## Part 4 — Other advisory committees: GMAC and MRAC no longer exist

**The Global Markets Advisory Committee and the Market Risk Advisory
Committee are gone.** The evidence is direct and primary:

- The **IAC renewal notice, 91 FR 52047** (2026-08-12), enumerates the
  agency's committees under the FACA public-interest determination: "In
  addition to the IAC, the CFTC has one statutory committee that Congress has
  exempted from the FACA and one discretionary FACA committee: (a)
  Agricultural Advisory Committee (discretionary) (b) Energy and
  Environmental Markets Advisory Committee (statutory)." **GMAC and MRAC are
  not in that list.** This is the agency's own current enumeration, published
  six days ago.
- Corroborating Federal Register history: the last GMAC meeting notice was
  published 2024-10-25 and its last renewal 2024-08-20; the last MRAC meeting
  notice was published 2024-11-05 and its last renewal 2024-04-18. No renewal
  notice for either appears in 2025 or 2026. FACA charters run two years.
- The cftc.gov pages for both still exist and still list members and
  subcommittees, with meeting histories that stop at **2024-11-21 (GMAC)** and
  **2024-12-10 (MRAC)**. The pages are stale, not evidence of activity.

Consequence: **there is no GMAC or MRAC digital-asset or AI workstream to
address, and no GMAC or MRAC written-statement window.** Anyone planning
around those committees is planning around committees that were not renewed.

The two surviving committees:

- **Agricultural Advisory Committee** — renewed 2026-07-20 (91 FR 45265),
  meeting notice 2026-07-21 (91 FR 45795), met 2026-07-29 (Press Release
  9275-26). Active, and **out-of-lane**: its subject is agricultural
  commodities, and this program has nothing to say about them.
- **Energy and Environmental Markets Advisory Committee** — statutory,
  exempt from FACA per the IAC renewal notice. Its last meeting on the
  cftc.gov events list was **2025-05-28**, and no notice of an upcoming
  meeting was found in the Federal Register. **Nothing open.**

**Net finding for item 4: the IAC is the only CFTC advisory committee with an
open written-statement window, and it closes August 27, 2026.** That is an
honest "nothing else open," and it raises the stakes on the IAC statement
rather than lowering them.

---

## Part 5 — Near-adjacent

### The June 2026 prediction-markets proposed rule is closed

**91 FR 35806, "Prediction Markets; Public Interest Determinations"** (FR Doc
2026-11854, RIN not stated in the API record; docket **CFTC-2026-1189**),
published June 12, 2026. **Comments closed July 27, 2026** — twenty-two days
before this memo. **1,454 comments** as reported by the Federal Register API.
The window is not open and the scope-disclaimer question does not arise.

For completeness, the surrounding prediction-markets and event-contract
record, all closed:

| Matter | FR cite | Deadline | Docket | Comments |
|---|---|---|---|---|
| Prediction Markets (ANPRM) | 91 FR 12516, FR Doc 2026-05105 | closed 2026-04-30 | CFTC-2026-0331 | 3,561 |
| Prediction Markets; Public Interest Determinations | 91 FR 35806, FR Doc 2026-11854 | closed 2026-07-27 | CFTC-2026-1189 | 1,454 |
| **Data Reporting Requirements for Certain Event Contracts** | 91 FR 40102, FR Doc 2026-13239, RIN 3038-AF73 | **closed 2026-07-31** | CFTC-2026-1453 | 34 |
| RFI, Identifying Regulations To Facilitate Innovation and Competition to Financial Technology Firms | 91 FR 36774, FR Doc 2026-12337 | closed 2026-07-09 | CFTC-2026-1321 | 22 |
| Event Contracts (2024 proposal), withdrawal notice | 91 FR 5386, FR Doc 2026-02454 | proposal withdrawn 2026-02-06 | — | — |

**The near-miss worth recording.** "Data Reporting Requirements for Certain
Event Contracts" (RIN 3038-AF73) proposed an alternate reporting framework
for **fully collateralized event contracts**, routing them to parts 15-18
instead of parts 38, 39, 43 and 45. That is this program's exact subject
matter — prefunded, fully collateralized conditional claims and how they
should be reported — with only 34 comments on it, and it closed **eighteen
days ago**. Nothing can be done about it now. It should be a standing watch
item: a final rule may follow, and any final rule, re-proposal, or request
for further comment on AF73 reopens the single best-fitting docket this
survey found.

### SEC — what could and could not be established

**sec.gov returned HTTP 403 to every request from this lane today** (Part 0),
so no SEC page was read and nothing below is a retrieval. What can be stated:
web-search metadata on 2026-08-18 shows SEC pages titled "Crypto Task Force
Written Input," "Submit Written Input to the Crypto Task Force," and "Crypto
Task Force Meetings," with individual submissions dated through mid-2025,
which is consistent with an open-ended rolling input channel rather than a
docketed comment period — and CFTC Press Release 9201-26 independently
confirms the Task Force exists and that the CFTC's Innovation Task Force
"will coordinate with . . . the SEC and its Crypto Task Force." **Whether
that channel is currently accepting input, and on what questions, was not
established and must be re-checked from sec.gov itself before anything is
planned around it.** Separately, the open joint portfolio-margining RFC
(Part 2.2) carries SEC File No. S7-2026-23 and is the one matter where the
existing joint-filing posture extends with no new argument required; per the
landscape memo the SEC does not publish letter indexes for the S7-2026-2x
file numbers, so the CFTC docket remains the only public view.

---

## Part 6 — Ranked shortlist, and what it does to the calendar

### The shortlist

Ranked on fit (Part 2's ratings) weighted by feasibility inside the calendar
in the next subsection. Nothing here authorises a filing; the ranking is a
recommendation about where drafting attention would earn the most.

**1. The IAC written statement — Docket CFTC-2026-1717, due Thursday,
August 27.** Already committed, but the published agenda changes what it
should say. Sessions II and III name agentic finance and event-contract
product-design-and-manipulation as agenda items; the renewal notice names
Commission surveillance investment as within scope; the docket still has zero
substantive technical statements; and Part 3 establishes that this is the
**only** open channel to CFTC AI policy anywhere in the agency. Reasoning for
the rank: highest fit, lowest marginal cost (the document exists), and the
only one where being second is impossible.

**2. The 24/7 and perpetual-contracts RFC — Docket CFTC-2026-1388, due
Wednesday, August 26.** The best-fitting *new* matter found. Questions 38-41,
44, 50-53, 62 and 66 ask for a stated, continuously-checkable
manipulation-resistance demonstration, which is the Question 15 memo's
argument in the Commission's own words. Reasoning for the rank: strong fit on
a named question set, but it demands an explicit no-energy-data disclaimer,
it lands in a docket already carrying roughly ninety comments, and its
deadline sits inside the worst week of the calendar.

**3. Conflicts and Affiliations — Docket CFTC-2026-1686, due Monday,
October 5.** Ranked third not on fit — it is weaker than either above — but
because it is the only candidate whose deadline permits an unhurried
decision, and because its information-barrier seam is the one place outside
the current dockets where the Clear/Shielded/Dark taxonomy answers a question
the Commission actually asked. Reasoning for the rank: real but narrow
purchase, and no calendar conflict at all.

Runners-up, with reasons for not making the three: **joint portfolio
margining** (August 31) — a genuine joint-posture extension and only two
comments filed, but the contribution reduces to Question 19 plus a
framework-drift observation, and the notice asks for margin data the program
does not have; **PRA 3038-0103, Ownership and Control Reports** (October 5,
zero comments) — squarely on the identity ground and nearly free to file, but
PRA comments are burden-shaped and the memo's underlying position is
conditional on reading source material nobody has read yet.

### Calendar consequences

Today is Tuesday, August 18. Every deadline below is a hard receipt date.

| Date | Event | Drafting window if taken |
|---|---|---|
| Thu Aug 20, 1:00-4:00 pm ET | IAC inaugural meeting | listen; capture Session II and III language for the statement |
| **Mon Aug 24** | Joint definitions + joint data RFCs due | already drafted; the landscape memo's edits (promote bundling-invariance, answer FalconX by name, cede the generic P-R6 layer to Ariadne) must land **Aug 18-22** |
| **Wed Aug 26** | 24/7 / perpetuals RFC due | **Aug 18-25 — in direct collision with the above.** A go/no-go is needed within about 24 hours, because a 4-6 page scoped comment cannot start on Aug 24 |
| **Thu Aug 27** | IAC written statement due | **Aug 20-26**, after the meeting; the agenda adds a Session II demand the current draft does not answer |
| **Mon Aug 31** | Joint portfolio margining due | Aug 28-30 only, i.e. entirely after the IAC filing; feasible but thin |
| Mon Sep 28 - Tue Oct 13 | PRA windows and Conflicts and Affiliations | decide in September, on merits, with no time pressure |

The one genuinely hard call is item 2. **The 24/7 RFC deadline (Aug 26) falls
between the joint-RFC deadline (Aug 24) and the IAC deadline (Aug 27)**, so
taking it means drafting a fourth document across the same seven days as the
edits to two documents and the rewrite of a third. Three observations bearing
on that call, offered without recommending the outcome:

- The argument is already written. The Question 15 memo's position — reference
  specification in the terms; manipulation cost computable in advance from
  those terms; the inputs, not the conclusion — transfers to Questions 38-41
  and 50-53 with no new research, and the memo's own counterargument section
  transfers with it.
- It is the same argument the IAC statement is being asked to carry for
  Session III. Writing it once for two audiences is cheaper than the calendar
  makes it look, and the two filings would reinforce rather than duplicate
  (one aimed at Core Principle 3 for energy perpetuals, one at event-contract
  product design principles).
- Against: the RFC's repeated demand for data is not a formality, roughly
  ninety filers are ahead, and a fourth document filed thin in the same week
  as three others is the failure mode the packet's length discipline was
  adopted to avoid.

### Re-run checklist

1. **After the August 20 meeting**, re-check
   <https://www.cftc.gov/PressRoom/Events/opaeventiac082026> and the IAC
   committee page for a transcript, minutes, presentations, or any newly
   posted statement — the written-statement window stays open a week past the
   meeting, and the TAC precedent (landscape memo, Part 1) is that those
   materials appear on the event page.
2. **Watch the press-release feed, not just the event page.** This memo's
   central Part 1 finding is that the agenda was published in a press release
   and never linked from the event page. Assume the same for any supplementary
   material.
3. **Re-run the Federal Register API sweep on Aug 24 and again in early
   September.** Comment periods get extended (the 24/7 RFC already was, by
   thirty days) and new matters publish weekly.
4. **Re-check regulations.gov with a registered API key.** Every comment count
   in this memo is the Federal Register API's figure and none was verified
   against the docket listing.
5. **Re-check sec.gov from an unblocked client** for the Crypto Task Force
   input channel and for any S7-2026-23 comment index.
6. **Standing watch: RIN 3038-AF73** (data reporting for certain event
   contracts). It is the best-fitting docket this survey found and it closed
   on July 31; a final rule, a re-proposal, or a request for further comment
   reopens it.

---

## Related documents

- [FILED_COMMENTS_LANDSCAPE.md](FILED_COMMENTS_LANDSCAPE.md) — who else is on
  the three current dockets and what is unclaimed there.
- [OPERATORLESS_AGENTS.md](OPERATORLESS_AGENTS.md) — a sibling lane's memo
  on agentic market participants, which names a candidate IAC addendum.
  Read only its header in this lane; see the coordination note in Part 1.
- [../CFTC_IAC_MEETING_BRIEF.md](../CFTC_IAC_MEETING_BRIEF.md) — the
  meeting brief whose "any late agenda" checklist item Part 1 resolves.
- [README.md](README.md) — the committee-question memo set, its triage
  classes, and the sources it would need before filing.
- [../LEGAL_ANALYSIS.md](../LEGAL_ANALYSIS.md) — the citation ledger. **None
  of the Federal Register documents surveyed in this memo is on that ledger**;
  each is cited here by FR volume, page, FR document number, and URL with a
  2026-08-18 retrieval date, and anything relied on in a filing must be added
  to the ledger first.
- [../SUBMISSION_WEEK_PLAN.md](../SUBMISSION_WEEK_PLAN.md) — the existing
  week plan, which Part 6 supplements rather than replaces.
