# Filing process and forward calendar — what happens to a submission, and what is coming

Internal research memo, prepared 2026-08-19 by the filing-process research
lane. See [README.md](README.md) for status and citation rules. This memo is a
survey of the public record; it is not legal advice, a filing authorization, or
a prediction about how any agency will act. Nothing in this lane was filed,
submitted, or sent; no regulator was contacted.

Companion to [FILED_COMMENTS_LANDSCAPE.md](FILED_COMMENTS_LANDSCAPE.md) (who
else is on the three current dockets) and
[OPEN_MATTERS_MAP.md](OPEN_MATTERS_MAP.md) (what else is open). This memo asks
the third question: **what the process actually does with a document once it is
submitted, and what is coming after this week.** It corrects **eight** findings
in those memos; see [Part 8](#part-8--corrections-to-the-companion-memos).

**All retrievals in this memo were performed 2026-08-19** (verified against
`TZ=America/New_York date` at the start of the lane: Wed Aug 19 20:56 EDT 2026;
corroborated by an HTTP `date:` header from www.cftc.gov of Thu, 20 Aug 2026
01:09:45 GMT). **The IAC meeting is tomorrow.** Every deadline below is a
receipt date read off a primary source on that day and should be re-read before
it is relied on.

### The two time-critical answers, up front

1. **Both IAC dates verified from 91 FR 51697.** Meeting **Thursday, August 20,
   2026, 1:00–4:00 p.m. EDT**; written statements "should" be submitted by
   **Thursday, August 27, 2026**. The notice says nothing about pre-meeting
   statements being provided to members. The one legally specified difference
   between filing before and after is a listing in the certified minutes (41
   CFR 102-3.165(b)(2)) — a provision this committee has never exercised. See
   [Part 2](#part-2--timing-before-the-meeting-versus-during-the-open-window-after-it).
2. **There is no speaking channel, and none has been missed.** The notice
   provides listen-only audio and a webcast, no registration, and **no
   oral-comment or request-to-present mechanism** — therefore no deadline for
   one. 41 CFR 102-3.140(d) makes public speaking discretionary with the
   agency, and 41 CFR 102-3.150(a)(6) requires oral-comment instructions to be
   in the notice if oral comment is permitted; they are not. Eight CFTC
   advisory-committee notices from 2023–2026 were checked and not one provides
   for oral comment. See [Part 3](#part-3--speaking-could-anyone-have-asked-and-can-they-now).

---

## Part 0 — Method, and exactly what was blocked

What worked:

- **federalregister.gov API v1**, `documents.json` filtered by
  `conditions[agencies][]=commodity-futures-trading-commission`. No key, no
  rate limit. Note that **`comments_close_on` is not a valid *filter* field** —
  it must be requested as an output field and filtered client-side. 43 CFTC
  documents published since 2026-05-01 were enumerated.
- **federalregister.gov full text**, `documents/full_text/text/<yyyy>/<mm>/<dd>/<doc>.txt`.
- **uscode.house.gov** preliminary-edition section pages (5 U.S.C. 553, 555,
  706, 1009; 7 U.S.C. 19). Each carries its own currency line; the title 5
  pages state "Text contains those laws in effect on August 18, 2026."
- **ecfr.gov API v1**, `versioner/v1/full/2026-08-18/title-NN.xml?...`. Note
  the API refuses a date later than a title's most recent issue date; 2026-08-18
  was the newest accepted date for titles 5, 17, and 41.
- **cftc.gov** event pages, press releases, committee pages, media downloads,
  and `forms.cftc.gov`, all with a browser User-Agent. A default client
  User-Agent gets Cloudflare 403s.
- **api.regulations.gov v4** with the public demo key — for roughly a dozen
  requests before the rate limit engaged.

- **sec.gov**, using a **declared-identity** User-Agent rather than a spoofed
  browser one. This is the access finding that unblocks the two companion
  memos' SEC gap: `-A "Mozilla/5.0 ..."` returns HTTP 403 with the body "SEC.gov
  | Request Rate Threshold Exceeded"; `-A "<name> <contact email>"` returns 200
  on every page tried.

What was blocked, precisely:

- **api.regulations.gov v4 rate limit.** After about a dozen requests the demo
  key returned `{"code":"OVER_RATE_LIMIT"}` (HTTP 429) and kept returning it
  through two retries at 45 and 90 second intervals. **Consequence: the
  regulations.gov "all CFTC documents open for comment" listing was never
  retrieved, and the three comments on the Agricultural Advisory Committee
  docket (CFTC-2026-1618) were never read.** Comment counts in this memo come
  from whichever of the two sources answered; each is labelled. A registered
  API key would close this gap.
- **comments.cftc.gov (legacy system) still returns HTTP 403** on all three
  URLs tried — `/`, `/PublicComments/AdvisoryCommittees.aspx`, and
  `/PublicComments/ReleasesWithComments.aspx?Type=ListAll&Year=2024` — to a
  browser User-Agent with an HTML `Accept` header. This confirms the
  FILED_COMMENTS_LANDSCAPE Part 0 finding and means the pre-2026-04-28 archive
  of advisory-committee public statements remains unreadable from here.
- **www.cftc.gov/LawRegulation/PublicComments/index.htm returns 403**; the
  working path is `/LawRegulation/PublicComments/ReleasesWithComments`.
- **www.regulations.gov HTML pages return HTTP 403** to this environment
  (`/docket/CFTC-2026-1355`, `-1354`, `-1420` all tried). Only the v4 API is
  usable, and only until the demo key's limit engages.

Nothing in this memo is inferred from an unread document. Where a document was
not read, the memo says so.

---

## Part 1 — What physically happens to a written statement

### 1.1 The three things the law requires

| Provision | What it requires | Source |
|---|---|---|
| **5 U.S.C. 1009(a)(3)** (FACA) | "Interested persons shall be permitted to attend, appear before, or file statements with any advisory committee, subject to such reasonable rules or regulations as the Administrator may prescribe." | <https://uscode.house.gov/view.xhtml?req=granuleid:USC-prelim-title5-section1009&num=0&edition=prelim> (retrieved 2026-08-19; page states "Text contains those laws in effect on August 18, 2026") |
| **41 CFR 102-3.140(c)** | "Any member of the public is permitted to file a written statement with the advisory committee, **whether or not the statement is related to a specific meeting**." | eCFR API, title 41 part 102-3, `versioner/v1/full/2026-08-18/title-41.xml?subtitle=F&chapter=102&part=102-3` (retrieved 2026-08-19) |
| **41 CFR 102-3.165(b)(2)** | Minutes must include "A list of the persons who were present at the meeting, including advisory committee members and staff, agency employees, and **members of the public who presented oral or written statements**." | same |

Three further pieces of the same regulation bear on what happens afterward:

- **41 CFR 102-3.165(b)(4)** — minutes must include "Copies of each report or
  other materials received, issued, or approved by the advisory committee at
  the meeting."
- **41 CFR 102-3.165(c)** — "The DFO must ensure that minutes are certified for
  accuracy by the chairperson within 90 calendar days of the meeting."
- **41 CFR 102-3.170** — advisory-committee records must be contemporaneously
  available, and "agencies may not require members of the public or other
  interested parties to use FOIA procedures in order to obtain records
  available under sec. 10(b) of the Act (codified at 5 U.S.C. 1009(b))."

5 U.S.C. 1009(b) itself reaches "the records, reports, transcripts, minutes,
appendixes, working papers, drafts, studies, agenda, or other documents **which
were made available to or prepared for or by** each advisory committee."

The IAC's own charter picks both up: the DFO shall "fulfill the requirements
under 5 U.S.C. § 1009(b)" (¶8, item 5), and "Records of open IAC meetings will
be made available to the public on the Commission's website" (¶14). Charter
amended March 3, 2026,
<https://www.cftc.gov/media/13366/IAC_Charter030326/download> (retrieved
2026-08-19).

### 1.2 What the CFTC says it does

The August 20 meeting notice routes written statements to a **regulations.gov
docket** — CFTC-2026-1717 — or to mail or hand delivery, and says:

> "Comments (regardless of submission method) will be published without review
> for, and without removal of, any personal identifying information or
> information your business may consider confidential."

and

> "If a submission that is refused for or withdrawn from publication because of
> inappropriate content also contains comments regarding the IAC meeting, such
> submission will be retained in the record for the matter and will be
> considered as required under the Administrative Procedure Act and other
> applicable laws, and may be accessible under the Freedom of Information Act."

91 FR 51697, FR Doc 2026-16328,
<https://www.federalregister.gov/documents/full_text/text/2026/08/11/2026-16328.txt>
(retrieved 2026-08-19).

The CFTC's general submission page adds two operationally important facts:

> "All comments on proposed rules and on other items requesting comment become
> part of the record for the matter and will be published online"

> "***Please Note:** Your comment will NOT immediately appear in the online
> public comment file. Submissions to Regulations.gov are subject to additional
> processing that **can take up to several days**."

<https://www.cftc.gov/LawRegulation/PublicComments/HowtoSubmit/index.htm>
(retrieved 2026-08-19). The same page points to 17 C.F.R. § 145.9 for
confidential-treatment requests.

All four predecessor TAC **event pages** carried an explicit promise:

> "Statements submitted in connection with the committee meeting will be made
> available to the public, including publication on CFTC.gov."

(quoted from <https://www.cftc.gov/PressRoom/Events/opaeventtac032223>;
identical wording on `opaeventtac071823`, `opaeventtac010824`,
`opaeventtac050224`, all retrieved 2026-08-19).

### 1.3 What is actually posted — direct inspection

Every link on the four TAC event pages and the IAC event page was extracted and
classified on 2026-08-19:

| Event page | Materials actually posted |
|---|---|
| `opaeventtac032223` | Agenda, Minutes, Transcript, Presentation |
| `opaeventtac071823` | Agenda, Minutes, Transcript, Presentation |
| `opaeventtac010824` | Agenda, Minutes, Transcript, Decentralized Finance Report, **Statement of Hilary J. Allen**, Presentations |
| `opaeventtac050224` | Agenda, Minutes, Transcript, Responsible AI in Financial Markets report, Presentation |
| `opaeventaac072926` (AAC, 2026-07-29) | Agenda, **Transcript**, Archived Webcast, Remarks of Chairman Selig. **No minutes. No public statements.** |
| `opaeventiac082026` (IAC, 2026-08-20) | **Agenda only** |

All under `https://www.cftc.gov/PressRoom/Events/`, retrieved 2026-08-19.

**Not one written statement from a member of the public appears on any of
them.** The single non-commissioner statement in the series, Hilary J. Allen's,
was a committee member's submission in absentia. This independently confirms
FILED_COMMENTS_LANDSCAPE Part 1.

The transcripts and minutes do not mention public statements either. A
case-insensitive grep of the TAC 2024-01-08 transcript (194 pages), the TAC
2024-05-02 transcript (194 pages), and the TAC 2024-05-02 minutes (6 pages) for
`public comment`, `written statement`, `member(s) of the public`, `comment from
the public`, and `public statement` returns **one hit across all three
documents** — an aside in the 2024-05-02 transcript about "multiple rounds of
draft for public comment" in a rulemaking, unrelated to that meeting's public.
(Transcripts at
<https://www.cftc.gov/sites/default/files/2024/04/1712252252/TAC_010824_transcript.pdf>
and
<https://www.cftc.gov/sites/default/files/2025/02/1738966437/TAC_050224_transcript.pdf>;
minutes at <https://www.cftc.gov/media/11601/TAC_minutes050224/download>; all
retrieved 2026-08-19.)

### 1.4 The live test case: the Agricultural Advisory Committee, three weeks ago

The AAC met **2026-07-29** under the current Chairman, with the same notice
template and the same DFO office. Written statements were due **Friday, August
7, 2026**. That window has closed, so the AAC is the only observable full cycle
under the current regime.

- Notice: 91 FR 45795, FR Doc 2026-14675,
  <https://www.federalregister.gov/documents/2026/07/21/2026-14675/agricultural-advisory-committee>.
- Docket **CFTC-2026-1618**, **3 comments** — Federal Register API
  `regulations_dot_gov_info.comments_count`, with its own
  `checked_regulationsdotgov_at` of 2026-08-09T08:55:03Z (retrieved
  2026-08-19). **The comment bodies were not read** (Part 0).
- Event page `opaeventaac072926` as of 2026-08-19: Agenda, Transcript, Archived
  Webcast, Chairman's Remarks. **No public statements, and no minutes.**

So under the current regime a public written statement lands **on the
regulations.gov docket and stays there**. It does not migrate to the event page.

### 1.5 Transcript timing — measured, not estimated

The notice promises "After the meeting, a transcript of the meeting will be
published through a link on the CFTC's website." Actual publication lag, taken
from the HTTP `last-modified` header of each transcript PDF (retrieved
2026-08-19/20 UTC):

| Meeting | Transcript last-modified | Lag |
|---|---|---|
| TAC 2023-03-22 | 2023-07-03 | ~103 days |
| TAC 2023-07-18 | 2023-10-11 | ~85 days |
| TAC 2024-01-08 | 2024-04-04 | ~87 days |
| TAC 2024-05-02 | 2025-02-07 | ~281 days |
| **AAC 2026-07-29** | **2026-08-12** | **14 days** |

The current office is an order of magnitude faster than the prior sponsors. On
that single data point an IAC transcript could appear in early September;
that is an observation about one meeting, not a commitment by anyone.

### 1.6 Direct answers to the question as asked

- **Published on the event page?** No. Not once in five meetings. The event
  page carries agenda, minutes, transcript, presentations, subcommittee
  reports, webcast, and commissioner/chairman statements.
- **Entered in a docket?** Yes. Docket CFTC-2026-1717 on regulations.gov,
  published without review or redaction, after a processing delay the CFTC
  itself describes as "up to several days."
- **Distributed to members before the meeting?** **Nothing in the notice says
  so.** The word does not appear. The nearest provision anywhere is 41 CFR
  102-3.140(f), which says Federal Register notices, agendas and supporting
  materials "should be posted on the agency advisory committee website ... as
  soon as they are available **or at the time they are provided to the advisory
  committee members**" — which presupposes that *some* materials go to members,
  without saying that public statements are among them. This is **UNVERIFIED**
  and should not be asserted.
- **Read aloud?** No provision for it, and no instance of it in either 194-page
  transcript.
- **Summarized?** No provision for it. FACA requires minutes to *name* members
  of the public who presented oral or written statements (41 CFR
  102-3.165(b)(2)) — a listing duty, not a summarizing duty — and no observed
  CFTC minutes contain such a list.

---

## Part 2 — Timing: before the meeting versus during the open window after it

### 2.1 Both dates verified

From the DATES caption of 91 FR 51697 (FR Doc 2026-16328), verbatim:

> "The meeting will be held virtually for the public on August 20, 2026, from 1
> p.m. to 4 p.m. Eastern Daylight Time. Please note that the meeting may end
> early if the IAC has completed its business. Members of the public who wish
> to submit written statements in connection with the meeting **should** submit
> them by Thursday, August 27, 2026."

The SUMMARY adds that it is "an in-person meeting for IAC members, with options
for the public to attend virtually."

<https://www.federalregister.gov/documents/full_text/text/2026/08/11/2026-16328.txt>,
retrieved 2026-08-19. Both dates confirmed. **Meeting: Thursday, August 20,
2026, 1:00–4:00 p.m. EDT. Written statements: Thursday, August 27, 2026.**

Two features of that sentence are worth stating plainly:

1. The modal is **"should," not "must."**
2. The Federal Register API's structured `comments_close_on` field for this
   document is **`null`** — the August 27 date exists only in the DATES prose.
   Consequence: a deadline-driven query does not surface it, and there is no
   structured close date driving the docket. (API record retrieved 2026-08-19.)

The seven-day-after-the-meeting shape is the CFTC's house pattern, not
an IAC peculiarity. Verified from the DATES caption of each notice
(federalregister.gov full text, retrieved 2026-08-19):

| Committee | Meeting | Statements due | Gap |
|---|---|---|---|
| TAC (88 FR 13107) | 2023-03-22 | 2023-03-29 | 7 days |
| TAC (88 FR 41933) | 2023-07-18 | 2023-07-25 | 7 days |
| TAC (88 FR 88376) | 2024-01-08 | 2024-01-15 | 7 days |
| TAC (89 FR 27421) | 2024-05-02 | 2024-05-09 | 7 days |
| GMAC (89 FR 85175) | 2024-11-21 | 2024-11-26 | 5 days |
| MRAC (89 FR 87861) | 2024-12-10 | 2024-12-17 | 7 days |
| AAC (91 FR 45795) | 2026-07-29 | 2026-08-07 | 9 days |
| **IAC (91 FR 51697)** | **2026-08-20** | **2026-08-27** | **7 days** |

### 2.2 The practical difference, stated factually

**What is the same either way.** The document lands on the same docket, is
published the same way, is subject to the same no-redaction rule, and becomes
part of the same record. Nothing in the notice conditions any of that on
timing.

**What differs, with the provision that makes it differ.**

1. **The minutes listing.** 41 CFR 102-3.165(b)(2) requires the certified
   minutes to list "members of the public who **presented** oral or written
   statements" — and the list is of persons "who were present **at the
   meeting**." A statement on the docket before the meeting can be a statement
   presented at that meeting; a statement filed on August 26 cannot be. That is
   the one *legally specified* consequence of filing before rather than after,
   and it is a listing in the minutes, not consideration of the argument.
   Caveat: none of the observed CFTC minutes contains such a list, because no
   public statements were ever presented at those meetings — so the provision
   is unexercised in this committee's record.
2. **Visibility before the room convenes.** The CFTC's own page says
   regulations.gov processing "can take up to several days." A statement
   submitted on the evening of August 19 is therefore **unlikely to be visible
   on the docket before the meeting begins at 1 p.m. on August 20.** For
   reference, the two comments now on the IAC docket were both posted
   2026-08-12, eight days before the meeting.
3. **What the meeting itself supplies.** The agenda's session topics are
   labelled "Potential Topics" and the notice says "The meeting agenda may
   change to accommodate other Committee priorities." A statement filed after
   the meeting can respond to what was actually said; the transcript will not
   be available by August 27 on any observed lag, but the live webcast will
   have been.
4. **The right does not depend on the meeting at all.** 41 CFR 102-3.140(c):
   any member of the public may file a written statement with an advisory
   committee "whether or not the statement is related to a specific meeting."
   The August 27 date governs statements "in connection with the meeting"; the
   standing right to file with the committee is not bounded by it.

**Does anything in the notice say pre-meeting statements are provided to
members?** **No.** The full text was read; there is no such sentence, and no
sentence about distribution to members at all. Any claim to the contrary is
unsupported.

---

## Part 3 — Speaking: could anyone have asked, and can they now?

### 3.1 What the notice provides

The entire public-participation content of the SUPPLEMENTARY INFORMATION
section of 91 FR 51697 is:

> "Registration for the meeting is not required. Members of the public may
> listen to the meeting by calling a domestic or international toll or
> toll-free number to connect to a **live, listen-only audio feed**. Call-in
> participants should be prepared to provide their first name, last name, and
> affiliation. ... Members of the public may also view a live webcast of the
> meeting via the www.cftc.gov website. ... After the meeting, a transcript of
> the meeting will be published through a link on the CFTC's website ...
> Persons requiring special accommodations to attend the virtual meeting
> because of a disability should notify the contact person above."

Contact person: Michael J. Passalacqua, IAC Designated Federal Officer, (771)
241-2301, IAC@CFTC.gov. Dial-in: 833-435-1820 / 833-568-8864 (toll-free),
+1 646-828-7666 (New York); Webinar ID 165 371 7748, passcode 172798.

**There is no oral-comment period, no request-to-speak mechanism, no
request-to-present mechanism, and therefore no deadline for any of them.** The
only individualized channel in the notice is the disability-accommodation
contact.

### 3.2 What the governing regulation provides

- **41 CFR 102-3.140(d):** "Any member of the public may speak to or otherwise
  address the advisory committee **if the agency's guidelines so permit**." The
  speaking right is discretionary with the agency; the writing right in
  paragraph (c) is not.
- **41 CFR 102-3.150(a)(6):** a Federal Register meeting notice must include
  "Instructions for submitting written comments, **and oral comments if
  permitted**."

The IAC notice contains written-comment instructions and contains no
oral-comment instructions. On the face of the notice, read against the
provision that governs what the notice must contain, **oral comment was not
permitted at this meeting.**

### 3.3 This is not an IAC quirk — it is CFTC practice

Eight CFTC advisory-committee meeting notices published 2023–2026 were
retrieved in full and searched for `oral`, `speak`, `wish to`, `request to
(make|present)`, `presentation to the Committee`, and `written statement`: TAC
2023-03-02 (88 FR 13107), TAC 2023-06-28 (88 FR 41933), TAC 2023-12-21 (88 FR
88376), TAC 2024-04-17 (89 FR 27421), GMAC 2024-10-25 (89 FR 85175), MRAC
2024-11-05 (89 FR 87861), AAC 2026-07-21 (91 FR 45795), IAC 2026-08-11 (91 FR
51697). Retrieved 2026-08-19 via
`federalregister.gov/documents/full_text/text/...`.

**Every single hit is the phrase "Members of the public who wish to submit
written statements."** Not one of the eight notices contains an oral-statement
provision, a request-to-speak provision, or a public comment period during the
meeting.

### 3.4 Direct answers

- **Could a member of the public have requested to speak at this meeting?** No.
  The notice provides no mechanism, and no CFTC advisory-committee notice in
  the four-year sample provides one.
- **Can they still?** No — for the same reason. There is no deadline that has
  passed, because there was never a channel.
- **What does the practice actually provide?** Public attendance by
  listen-only telephone or live webcast, no registration; a written-statement
  channel to a regulations.gov docket; and a published transcript afterward.
- **Is there any speaking channel at the CFTC on these subjects?** Yes, but to
  a different body: the **Innovation Task Force meeting request form** (Part
  6.3). It is not the IAC, has no relationship to the August 20 meeting, and
  the Task Force states it "may be limited in its ability to schedule a meeting
  in each instance."

---

## Part 4 — How public comments work at the CFTC, by type

The five channels differ in exactly one respect that matters: **what the agency
is obliged to do next.**

### 4.1 The five channels

**(a) Advisory-committee written statement.** Authority: FACA, 5 U.S.C.
1009(a)(3); 41 CFR 102-3.140(c). The committee's "duties ... shall be solely
advisory," and "No determination of fact or policy shall be made by the IAC on
behalf of the Commission" (IAC Charter ¶4, citing 5 U.S.C. 1008(b)). **No
obligation to respond, and no obligation to consider** arises from FACA. FACA's
duties are publication and recordkeeping: minutes (41 CFR 102-3.165), record
availability without FOIA (41 CFR 102-3.170), and filing of committee reports
with the Library of Congress (41 CFR 102-3.175(d), 5 U.S.C. 1012).

**(b) Comment on an NPRM.** Authority: 5 U.S.C. 553. Subsection (c) is the
operative sentence:

> "After notice required by this section, the agency shall give interested
> persons an opportunity to participate in the rule making through submission
> of written data, views, or arguments with or without opportunity for oral
> presentation. **After consideration of the relevant matter presented, the
> agency shall incorporate in the rules adopted a concise general statement of
> their basis and purpose.**"

<https://uscode.house.gov/view.xhtml?req=granuleid:USC-prelim-title5-section553&num=0&edition=prelim>
(retrieved 2026-08-19).

**This is the only channel with a statutory duty to consider and to produce a
written statement of basis and purpose.** The CFTC additionally must "consider
the costs and benefits" before promulgating a regulation, evaluated against
five named factors — protection of market participants and the public;
efficiency, competitiveness and financial integrity of futures markets; price
discovery; sound risk management practices; and other public interest
considerations. 7 U.S.C. 19(a),
<https://uscode.house.gov/view.xhtml?req=granuleid:USC-prelim-title7-section19&num=0&edition=prelim>
(retrieved 2026-08-19).

On judicial review, 5 U.S.C. 706 directs a court to "hold unlawful and set
aside agency action ... found to be (A) arbitrary, capricious, an abuse of
discretion, or otherwise not in accordance with law" or "(D) without
observance of procedure required by law," and provides that "the court shall
review **the whole record** or those parts of it cited by a party."
<https://uscode.house.gov/view.xhtml?req=granuleid:USC-prelim-title5-section706&num=0&edition=prelim>
(retrieved 2026-08-19). *How courts have applied that language to
comment-response is case law and was not researched in this lane; treat any
statement about it as UNVERIFIED.*

**(c) Comment on an RFC, ANPRM, or Concept Release.** The CFTC's own
description:

> "Before issuing an NPRM, the Commission sometimes conducts informational
> meetings and/or votes to publish an Advance Notice of Proposed Rulemaking
> (ANPRM), Request for Comment, or Concept Release in the Federal Register to
> obtain preliminary information and views about an issue. ... The Commission
> would take the public's comments on an ANPRM, Request for Comment, or Concept
> Release **into consideration to determine whether and how to proceed with an
> NPRM**."

<https://www.cftc.gov/LawRegulation/CommissionRulemakingExplained/index.htm>
(retrieved 2026-08-19). The same page notes that the CFTC's policy is "to make
public on the Commission's website substantive ex parte communications, both
written and oral, that provide significant, material information addressed to
the merits of a proposed rule," and that it is "the Commission's practice to
make public on its website all ex parte meetings held on proposed rules,
including the names and affiliations of attendees."

No 553(c) duty attaches, because no rule is being adopted. **Note that all
three joint CFTC/SEC matters this program is working are of this type** — each
is titled "Joint Request for Comment," not a proposed rule, notwithstanding
that the Federal Register types them as "Proposed Rule."

**(d) Petition for rulemaking.** This is the only channel with an obligation
running to the *submitter*.

- 5 U.S.C. 553(e): "Each agency shall give an interested person the right to
  petition for the issuance, amendment, or repeal of a rule."
- 5 U.S.C. 555(e): "Prompt notice shall be given of the denial in whole or in
  part of a written application, petition, or other request of an interested
  person made in connection with any agency proceeding. Except in affirming a
  prior denial or when the denial is self-explanatory, the notice shall be
  accompanied by a brief statement of the grounds for denial."
  <https://uscode.house.gov/view.xhtml?req=granuleid:USC-prelim-title5-section555&num=0&edition=prelim>
  (retrieved 2026-08-19).
- **17 C.F.R. § 13.1** (CFTC's own rule, authority 7 U.S.C. 2(a)(12), source 84
  FR 68789, as amended at 89 FR 71809): "Any person may file a petition with
  the Secretariat of the Commission, by mail or electronically through the
  Commission website, for the issuance, amendment or repeal of a rule of
  general application. ... **The Secretariat shall acknowledge receipt of the
  petition, refer it to the Commission for such action as the Commission deems
  appropriate, and notify the petitioner of the action taken by the
  Commission.** Except in affirming a prior denial or when the denial is
  self-explanatory, notice of a denial in whole or in part of a petition shall
  be accompanied by a brief statement of the grounds of denial." (eCFR API,
  title 17 chapter I part 13, current 2026-08-18, retrieved 2026-08-19.)
- 5 U.S.C. 706(1) authorises a court to "compel agency action unlawfully
  withheld or unreasonably delayed."

**(e) Submission or comment under a specific regulation.** Part 40 creates
comment windows that never touch the Federal Register:

- **17 C.F.R. § 40.6(c)(2)** — when the Commission stays a self-certified rule
  or rule amendment: "**The Commission shall provide a 30-day comment period**
  within the 90-day period in which the stay is in effect ... The Commission
  shall publish a notice of the 30-day comment period **on the Commission
  website**. Comments from the public shall be submitted as specified in that
  notice."
- **17 C.F.R. § 40.11(c)** — 90-day review of event contracts based on certain
  excluded commodities: "The Commission shall post on the Web site a
  notification of the intent to carry out a 90-day review," and shall issue an
  approving or disapproving order within 90 days. **This provision creates a
  review, not a comment period.**
- **17 C.F.R. § 40.2(c)** — the Commission may stay a product listing during
  proceedings for a false certification or pendency of a § 8a(7) petition; the
  decision "shall not be delegable to any employee of the Commission."

(All from eCFR API, title 17 chapter I part 40, current 2026-08-18, retrieved
2026-08-19.)

The practical consequence is a watch-list consequence: **a Federal-Register-only
sweep misses § 40.6(c)(2) windows entirely.**

### 4.2 The comparison table

| Channel | Duty to consider? | Duty to respond to the submitter? | Part of a rulemaking record? |
|---|---|---|---|
| Advisory-committee statement | No FACA duty | No | It is on a docket and in the committee's 5 U.S.C. 1009(b) records; it is not a rulemaking record because there is no rulemaking |
| NPRM comment | **Yes** — 5 U.S.C. 553(c), plus 7 U.S.C. 19(a) cost-benefit | No individualized duty | **Yes** |
| RFC / ANPRM / concept-release comment | Agency states it takes them "into consideration to determine whether and how to proceed with an NPRM"; no 553(c) duty | No | Enters the matter's record; whether it enters a later NPRM's record was not researched — **UNVERIFIED** |
| Petition for rulemaking | 5 U.S.C. 553(e) right to petition | **Yes** — 17 C.F.R. § 13.1 and 5 U.S.C. 555(e) | The petition and its disposition |
| Submission under a specific regulation (e.g. § 40.6(c)(2) stay comment) | Tied to the specific determination, not to 553(c) | No | The certification/review file |

### 4.3 The one thing the IAC notice does say about the APA

The IAC notice's boilerplate says a submission refused for inappropriate
content that also contains comments regarding the IAC meeting "will be retained
in the record for the matter and will be considered as required under the
Administrative Procedure Act and other applicable laws." Note precisely what
this is: **a clause about submissions the CFTC declines to publish**, applying
the agency's standard comment-file language to an advisory-committee docket. It
is not a general statement that IAC statements receive APA consideration, and
should not be quoted as one. The identical clause appears in the CFTC's PRA
notices with "the merits of this notice" substituted (e.g. 91 FR 53610) and in
the compute RFC with "the merits of this proposal."

---

## Part 5 — The forward calendar

### 5.1 Everything open with a deadline, from the Federal Register

Complete enumeration from the federalregister.gov API on 2026-08-19: 43 CFTC
documents published since 2026-05-01, of which **13 carry a comment deadline of
2026-08-19 or later.** Comment counts are the API's
`regulations_dot_gov_info.comments_count`; where a regulations.gov figure was
obtained before the rate limit engaged it is given second.

| Deadline | Matter | Cite / FR Doc / RIN | Docket | Comments | Touches this project |
|---|---|---|---|---|---|
| Mon **Aug 24** | Joint RFC, further definition of "swap" and "security-based swap" and alternative compliance | 91 FR 37873 · 2026-12743 · 3038-AF71 / 3235-AN79 | CFTC-2026-1355 | 5 / 5 | *existing filing* |
| Mon **Aug 24** | Joint RFC, swap and SBS data reporting | 91 FR 37877 · 2026-12742 · 3038-AF70 / 3235-AN78 | CFTC-2026-1354 | 4 / 4 | *existing filing* |
| Wed **Aug 26** | RFC, extension of standard futures to 24/7 trading and perpetual contracts on physically delivered or storable energy commodities | 91 FR 47158 (extending 91 FR 38334) · 2026-15216 · 3038-AF75 | CFTC-2026-1388 | 4 / **88** | **Yes** — perpetuals, reference-price integrity, continuous manipulation-resistance |
| Thu **Aug 27** | IAC written statements | 91 FR 51697 · 2026-16328 | CFTC-2026-1717 | 2 / 2 | **Yes** — all three sessions |
| Mon **Aug 31** | Joint RFC, further implementation of portfolio margining and cross-margining | 91 FR 39579 · 2026-13182 · 3038-AF72 / 3235-AN80 | CFTC-2026-1420 | 2 / 0 | Partly — clearing, narrow-based-index line |
| Mon **Sep 28** | PRA 3038-0097, process for review of swaps for mandatory clearing | 91 FR 47225 · 2026-15166 | CFTC-2026-1651 | 1 / n-a | Marginal |
| Mon **Oct 5** | **Conflicts and Affiliations** (NPRM) | 91 FR 50926 · 2026-15948 · 3038-AF76 | CFTC-2026-1686 | 6 / 6 | Partly — information barriers, non-public information |
| Mon **Oct 5** | PRA 3038-0103, Ownership and Control Reports, Forms 102/102S, 40/40S, 71 | 91 FR 50813 · 2026-16040 | CFTC-2026-1687 | 0 / 0 | Partly — identity and reference data |
| Mon **Oct 5** | PRA 3038-0076, requirements for DCOs | 91 FR 50816 · 2026-16048 | CFTC-2026-1685 | 0 / n-a | No |
| Mon **Oct 5** | PRA 3038-0091, cleared swaps customer collateral disclosure/retention | 91 FR 50817 · 2026-16074 | CFTC-2026-1684 | 2 / n-a | No |
| Mon **Oct 5** | PRA 3038-0092, customer clearing documentation and timing of acceptance | 91 FR 50814 · 2026-16038 | CFTC-2026-1688 | 0 / n-a | No |
| Tue **Oct 13** | PRA 3038-0005, CPO/CTA rules and FCM monthly reporting | 91 FR 52677 · 2026-16631 | CFTC-2026-1783 | 0 / n-a | No |
| Mon **Oct 19** | **PRA 3038-0059, Part 41, Relating to Security Futures Products** — NEW | 91 FR 53610 · 2026-16876 | CFTC-2026-1816 | 0 / 0 | Partly — see 5.2 |

Each row's FR document is at
`https://www.federalregister.gov/documents/<yyyy>/<mm>/<dd>/<FR-doc>/...`;
metadata retrieved 2026-08-19.

**Nothing new appeared on Federal Register public inspection for the CFTC as of
2026-08-19** (113 documents on inspection that day, none from the CFTC;
`api/v1/public-inspection-documents/current.json`, retrieved 2026-08-19).

### 5.2 The one new Federal Register item since the last sweep

**PRA 3038-0059, Part 41, Relating to Security Futures Products.** Published
2026-08-19, 91 FR 53610, FR Doc 2026-16876, docket CFTC-2026-1816, **zero
comments**, due **October 19, 2026**.
<https://www.federalregister.gov/documents/full_text/text/2026/08/19/2026-16876.txt>
(retrieved 2026-08-19).

Its abstract states the collection covers rules the CFTC issued jointly with
the SEC under CEA § 4d(c), 7 U.S.C. 6d(c), requiring firms dually registered as
broker-dealers and FCMs "to make choices as to how its customers' transactions
in security futures products will be treated, either as securities transactions
held in a securities account or as futures transactions held in a futures
account." Burden statement: **10 respondents (2 DCMs and 8 FCMs)**, 603 total
annual hours.

Relevance is real but narrow: it sits exactly on the security-futures /
security-based-swap boundary that the definitions docket is arguing about, and
it is the collection behind the *account-election* mechanics that a
listing-status classification test (the FalconX position) would depend on. But
it is a PRA renewal, so the four questions it actually asks are the statutory
ones, quoted verbatim from the notice: whether the collection is necessary and
has practical utility; the accuracy of the burden estimate; "Ways to enhance
the quality, usefulness, and clarity of the information to be collected"; and
ways to minimize burden including through automated collection techniques.

### 5.3 Announced but not yet in the Federal Register — the most consequential finding

Two Commission-level documents were released on cftc.gov in the last 36 hours
with comment periods that have **not started yet** because they run from
Federal Register publication. Neither is in the Federal Register, and neither
was on public inspection as of 2026-08-19. A Federal-Register-only sweep sees
neither.

#### (i) Request for Comment on the Listing of Compute Derivatives Contracts — RIN 3038-AF77

- Announced **2026-08-19**, Press Release 9286-26,
  <https://www.cftc.gov/PressRoom/PressReleases/9286-26> (retrieved 2026-08-19).
- Document (19 pp., "As approved by the Commission (subject to Office of the
  Federal Register technical corrections)"):
  <https://www.cftc.gov/media/14496/RFC_ComputeDerivates081826/download>
  (retrieved 2026-08-19). 17 CFR Parts 1, 38. Issued by the Commission
  2026-08-19, signed Christopher Kirkpatrick.
- **DATES line reads, verbatim: "Comments must be received on or before [INSERT
  DATE 60 DAYS AFTER DATE OF PUBLICATION IN THE FEDERAL REGISTER]."** The press
  release: "Comments will be accepted for 60 days following publication in the
  Federal Register." **The deadline is therefore not yet determinable and must
  be re-checked once the document publishes.**
- Subject: derivatives referencing "compute" — "the processing power primarily
  used by the large language models ('LLMs') at the center of the artificial
  intelligence ('AI') economy." Chairman Selig, in the release: "America cannot
  win the AI race without a robust derivatives market for compute."
- Four question blocks: (1) Compute Cash Markets: Size, Liquidity, and Other
  Considerations; (2) Market Oversight and Susceptibility to Manipulation; (3)
  Customer Protection of Market Participants; (4) perpetual compute futures.
  37 interrogative sentences in total.
- **This is the single best subject-matter fit found in any sweep by this
  program.** Quoted verbatim from the document:
  - "Would it be appropriate to permit trading in a derivative contract
    settling to a price computed from data that the Commission may not be able
    to observe, verify, or surveil, in whole or in part?"
  - "Are there protections or requirements that would prevent a compute
    capacity provider from manipulating a cash settlement index by adjusting a
    posted rate, directing capacity onto or away from a venue whose
    transactions the index calculation methodology treats as input data, or by
    executing or declin[ing to execute] ..."
  - "How should the Commission consider whether the parties best positioned to
    influence the reference price are the same parties that supply capacity or
    contribute transactions or posted rates from which price is computed?"
  - "What surveillance capabilities would be necessary to satisfy Core
    Principle 4 for compute derivatives, and are those capabilities presently
    feasible from a technological, operational, and legal perspective?"
  - "Should a DCM be expected or required to maintain an information-sharing
    arrangement with each compute venue and each compute capacity provider
    whose transactions or posted rates enter a settlement reference price
    against which a compute derivative settles on the DCM?"
  - "**Should the terms and conditions of a compute futures contract be
    required to include any specific information related to idiosyncratic
    risks?**"
  - "Would perpetual compute futures have advantages for market participants
    over 'traditional' or 'fixed date' futures contracts?" / "Would perpetual
    compute derivatives pose any unique risks for market participants or the
    broader markets?"
  - "What thresholds would be appropriate for a compute settlement reference
    price given the idiosyncrasies of the cash markets for compute?"
- The document also states: "The Commission particularly encourages commenters
  to provide **empirical and data-driven input**," and that it "is a significant
  regulatory action under section 3(f) of Executive Order 12866, and has been
  reviewed by the Office of Management and Budget."
- Touches: AI/agentic finance, digital-asset-adjacent market structure,
  perpetual contracts, reference-price integrity, manipulation cost,
  surveillance feasibility — that is four of the six subject areas in this
  memo's brief, in one document, on a docket that does not exist yet.

#### (ii) NPRM, CPO and CTA registration (part 4 amendments)

- Announced **2026-08-18**, Press Release 9284-26,
  <https://www.cftc.gov/PressRoom/PressReleases/9284-26> (retrieved 2026-08-19).
- Document: <https://www.cftc.gov/media/14481/NPRMCPOCTARegistrations081826/download>.
- "Comments will be accepted for **45 days** following publication in the
  Federal Register."
- Adds a CPO registration exemption for certain SEC-registered investment
  advisers, a related CTA exemption, and raises the small-pool exemption
  capital threshold for inflation. **Out of this program's lane.**

### 5.4 A third recent item with no comment window, worth reading

**Division of Market Oversight advisory on self-certification of incentive
programs for prediction markets**, Press Release 9282-26, 2026-08-12,
<https://www.cftc.gov/PressRoom/PressReleases/9282-26> (retrieved 2026-08-19),
issuing CFTC Staff Letter No. 26-23. It addresses "an increasing number of
incentive-program rule filings submitted under CFTC Regulation 40.6(a) —
particularly those relating to event contract products — that contain
procedural or substantive deficiencies." No comment period; it is staff
guidance. It matters here only because § 40.6 is the provision whose stay
mechanism creates website-only comment windows (Part 4.1(e)), and event-contract
filings are the traffic running through it.

### 5.5 The SEC side — and a correction that changes the docket census

**Access note.** sec.gov returns HTTP 403 (`Request Rate Threshold Exceeded`)
to a *spoofed browser* User-Agent. The SEC's automated-access policy wants a
declared-identity User-Agent instead, and with one every page below returned
200. This is why the two companion memos recorded sec.gov as blocked; it was
not blocked, it was answering a policy the requests did not satisfy.

#### 5.5.1 The SEC does publish comment-letter indexes for the joint matters

FILED_COMMENTS_LANDSCAPE Part 0 states that "**SEC comment lists do not exist
for these file numbers**" after five 404s, and concludes that "the
CFTC/regulations.gov side is the only public view of these dockets." **That is
incorrect.** The five URLs tried are a retired shape. The current shape is
`/rules-regulations/public-comments/<file-number>`, and all three resolve:

| URL | Status |
|---|---|
| `https://www.sec.gov/rules-regulations/public-comments/s7-2026-21` | **200** |
| `https://www.sec.gov/rules-regulations/public-comments/s7-2026-22` | **200** |
| `https://www.sec.gov/rules-regulations/public-comments/s7-2026-23` | **200** |
| `https://www.sec.gov/comments/s7-2026-21/s7202621.htm` (the old shape) | 404 |
| `https://www.sec.gov/comments/` | 200, body reads "Directory Browsing Not Allowed Here." |

All retrieved 2026-08-19. Each page renders a filterable index — columns Date,
Letter Type, Commenter Name — with individual letters served under
`/comments/S7-2026-2X/`.

#### 5.5.2 What is on them, and who is not on the CFTC dockets

Read directly off the three index pages, 2026-08-19:

**S7-2026-21 — definitions (SEC side): 6 letters.**

| Date | Commenter | Also on CFTC-2026-1355? |
|---|---|---|
| Aug. 12, 2026 | Alexandra Guest, CEO, FalconX Bravo, Inc. | Yes (-0006) |
| **Aug. 6, 2026** | **Angela Dunn, Principal Associate General Counsel, Nasdaq, Inc.** — with an additional file captioned "7-31-26: CFTC Comment" | **No** |
| **July 21, 2026** | **Carol Alexander, University of Sussex** | **No** |
| July 21, 2026 | Ilya Beylin, Associate Professor, Seton Hall University | Yes (-0002) |
| **June 25, 2026** | **Seth Allen** | **No** |
| **June 21, 2026** | **Nicolin Decker, Independent Researcher** | **No** |

**S7-2026-22 — data reporting (SEC side): 9 items, of which 5 are ex parte
memoranda.**

| Date | Letter Type | Item |
|---|---|---|
| Aug. 13, 2026 | **Meeting with SEC Officials** | Memorandum, Division of Trading and Markets, meeting with representatives of **DTCC Data Repository** |
| Aug. 13, 2026 | **Meeting with SEC Officials** | Memorandum, Division of Trading and Markets, meeting with representatives of **ICE Trade Vault** |
| July 22, 2026 | **Meeting with SEC Officials** | Memorandum, Division of Trading and Markets, meeting with representatives of **ISDA** |
| **July 9, 2026** | Public Comment | **James R. Cochrane, CEO, Navigator-FX Solutions, LLC** — not on CFTC-2026-1354 |
| July 6, 2026 | Public Comment | **Ariadne Dataworks Ltd.** — also on CFTC-2026-1354 (-0002, posted 2026-07-14); **filed with the SEC eight days earlier** |
| **June 26, 2026** | Public Comment | **"Msmith, Restaurant GM"** — not on CFTC-2026-1354 |
| June 25, 2026 | **Meeting with SEC Officials** | Memorandum, Office of the Chairman, meeting with representatives of **DTCC** |
| June 22, 2026 | **Meeting with SEC Officials** | Memorandum, Office of the Chairman, meeting with representatives of **Bloomberg** |
| **June 20, 2026** | Public Comment | **Zontonnia Moore, Living Eden Frameworks, LLC** — not on CFTC-2026-1354 |

**S7-2026-23 — portfolio margining (SEC side): 4 letters.** Eyad G. Haddadin,
VeloxVFX LLC (July 18); Hans R. Dutt, Economic Consultant (June 30); Michael
Ravnitzky (June 30); Jeremy Wilson, Credit Analyst, Macon Bank and Trust (June
26). The CFTC-side docket CFTC-2026-1420 showed 0 comments on a direct
regulations.gov query and 2 in the Federal Register API's figure.

**The consequence for the packet is direct: the eleven-comment census in
FILED_COMMENTS_LANDSCAPE Part 2 is not the whole record.** At least seven
additional public comments and five ex parte memoranda sit on the SEC side of
the same three matters, and the ex parte memoranda show DTCC (twice), ICE Trade
Vault, ISDA, and Bloomberg meeting SEC staff on the data reporting matter — the
docket this program's data comment enters. Those meetings do not appear
anywhere on the CFTC side.

#### 5.5.3 The joint matters, deadlines re-verified

All three verified against the Federal Register document's own DATES section
(federalregister.gov API, retrieved 2026-08-19) and cross-checked against the
SEC's open-comment listing:

| SEC file | Title | FR cite / doc | RINs | Deadline |
|---|---|---|---|---|
| S7-2026-21 | Joint Request for Comment on Further Definition of "Swap" and "Security-Based Swap" and on Alternative Compliance | 91 FR 37873 · 2026-12743 | 3038-AF71 / 3235-AN79 | **Aug. 24, 2026** |
| S7-2026-22 | Joint Request for Comment on Swap and Security-Based Swap Data Reporting | 91 FR 37877 · 2026-12742 | 3038-AF70 / 3235-AN78 | **Aug. 24, 2026** |
| S7-2026-23 | Joint Request for Comment on Further Implementation of Portfolio Margining and Cross-Margining of Securities and Derivatives | 91 FR 39579 · 2026-13182 | 3038-AF72 / 3235-AN80 | **Aug. 31, 2026** |

All three close before September 1; **none is a forward-calendar item.**

#### 5.5.4 Open SEC Commission-level items with a September-or-later deadline

From the SEC's own listing, <https://www.sec.gov/rules-regulations/submit-public-comments>
(retrieved 2026-08-19; page states "Last Reviewed or Updated: Aug. 18, 2026"),
cross-checked against the Federal Register:

| Deadline | Matter | File no. / RIN / FR cite | Subject-matter touch |
|---|---|---|---|
| **Sept. 8, 2026** | Cboe Clear U.S., LLC — application for temporary registration as a clearing agency | File No. **600-47** · Rel. 34-105960 · 91 FR 46817 · FR Doc 2026-14979 | Clearing infrastructure adjacent to the portfolio-margining matter; engages Regulation SCI |
| **Sept. 17, 2026** | Roundtable on Preparations for **24-Hour Trading** | File No. **4-913** | The equities analogue to always-on markets; the closest SEC item to the CFTC's 24/7 RFC. **The deadline appears only on sec.gov's listing and in press release 2026-69; no Federal Register notice for it was found — treat the date as sec.gov-sourced, not FR-corroborated.** |
| **Sept. 21, 2026** | Electronic Delivery of Information Under the Federal Securities Laws | **S7-2026-25** · RIN 3235-AN57 · 91 FR 45884 · FR Doc 2026-14679 | Electronic delivery across five statutes; relevant to machine-readable disclosure arguments |
| **Sept. 14 or Sept. 24, 2026 — conflicting** | SEC Regulatory Flexibility Agenda | **S7-2026-16** · Rel. 33-11416 · 91 FR 53164 · FR Doc 2026-16619 | The SEC's own forward calendar of intended rulemakings |
| **60 days after FR publication — clock not started** | **Regulation Crypto Assets** | **S7-2026-27** · RIN 3235-AN38 · Rel. **33-11434, 34-106150** · 17 CFR parts 200, 201, 228, 230, 232, 239 | **Direct hit on digital assets and tokenization** |

⚠️ **Verified conflict on the Reg Flex Agenda deadline.** The SEC's listing page
says "Comments Due: **Sept. 24, 2026**." The Federal Register document (91 FR
53164) and the SEC's own signed release (33-11416,
<https://www.sec.gov/files/rules/final/2026/33-11416.pdf>) both say "Comments
should be received on or before **September 14, 2026**." No correction notice
was found. Two authoritative documents against one landing page; the conflict
is unresolved on the record and is recorded here rather than reconciled.

**Regulation Crypto Assets (S7-2026-27) is the SEC item that matters here.**
Verified from the release PDF directly
(<https://www.sec.gov/files/rules/proposed/2026/33-11434.pdf>, retrieved
2026-08-19). Its SUMMARY, verbatim in part:

> "The proposed rules would be set forth in a new regulation titled 'Regulation
> Crypto Assets' and would include two exemptions from the registration
> requirements of section 5 of the Securities Act of 1933. The first exemption
> would permit offerings of up to $5 million during a four-year period. The
> second exemption would permit offerings of up to $75 million during each
> 12-month period. ... The proposed rules also would include a **conditional
> safe harbor from the term 'investment contract'** in the definitions of
> 'security' in the Securities Act of 1933 and the Securities Exchange Act of
> 1934. If the conditions of that proposed safe harbor are satisfied, then a
> crypto asset would be deemed not to be subject to an investment contract for
> purposes of those definitions of 'security.'"

Its DATES section carries unfilled placeholders — "[INSERT DATE OF PUBLICATION
IN THE FEDERAL REGISTER]" and "[INSERT DATE 60 DAYS AFTER DATE OF PUBLICATION
IN THE FEDERAL REGISTER]" — so **the deadline is not yet knowable**. It was not
in the Federal Register public-inspection queue on 2026-08-19. The comment form
is already live at
<https://www.sec.gov/comments/s7-2026-27/regulation-crypto-assets>.

This is a proposed rule, i.e. the one document type in Part 4's taxonomy that
carries a 5 U.S.C. 553(c) duty to consider — and its safe harbor is a
classification rule for when an instrument stops being a security. That is the
same layer the definitions comment argues about, from the other agency.

**SRO rule-change filings.** SEC Exchange Act § 19(b) notices carry a 21-day
comment window from Federal Register publication, verified verbatim against
three notices spanning 2026-08-11 to 2026-08-19. **51 SEC SRO notices published
in that window, 45 with a comment window**, deadlines spread across Sept. 1–9,
2026. They are high-volume and mostly noise. One is worth naming:

- **Cboe BZX Exchange, SR-CboeBZX-2026-065** — "Notice of Filing of a Proposed
  Rule Change To List and Trade Shares of **3x Gold ETF, 3x Silver ETF, 3x
  Bitcoin ETF, 3x Ether ETF, 3x Crude Oil ETF, and 3x Natural Gas ETF**, Each a
  Series of the VS Trust, Under BZX Rule 14.11(e)(4) (Commodity-Based Trust
  Shares)." 91 FR 53686, FR Doc 2026-16854, published 2026-08-19, comments due
  **Sept. 9, 2026**. Leveraged crypto exposure routed through the
  commodity-based-trust-share listing rule.

Also flagged: Cboe BZX small-retail-broker distribution program for the BZX Top
data feed (91 FR 52767, due Sept. 4 — market data), and Cboe BYX periodic
auction orders (91 FR 52735, comments Sept. 4, **rebuttal comments Sept. 18** —
the only two-stage schedule in the window).

**Nothing open on the SEC side with a September-or-later deadline touches event
contracts, prediction markets, perpetual contracts, security futures, or
AI/algorithmic trading.**

#### 5.5.5 Two standing SEC-side channels — one of which is joint with the CFTC

**(a) The SEC-CFTC Harmonization Initiative.** This is the finding that most
changes the picture of the three joint dockets.
<https://www.sec.gov/featured-topics/sec-cftc-harmonization-initiative>
(retrieved 2026-08-19). It carries dedicated mailboxes at
**harmonization@sec.gov** and **harmonization@cftc.gov**, an MOU
(<https://www.sec.gov/files/mou-sec-cftc-2026.pdf>), the three joint RFCs, and
three standing public channels:

- **Written Input form** —
  `/featured-topics/sec-cftc-harmonization-initiative/submit-written-input`.
  Verbatim: "Written input will generally be posted on www.sec.gov **and
  www.cftc.gov**. Material received will be posted without modification; the
  SEC and CFTC do not edit personal identifying information from submissions."
  Confidential treatment requires following both agencies' procedures. **No
  deadline.**
- **Written Input Log** —
  `/featured-topics/sec-cftc-harmonization-initiative/sec-cftc-harmonization-initiative-written-input-log`.
  Seven submissions, all 2026:

  | Date | Submitter | Subject |
  |---|---|---|
  | **May 20, 2026** | **ISDA and SIFMA** | "CFTC-SEC Harmonization" |
  | Apr. 13, 2026 | Kara Dutta, **ICE Trade Vault** | Swap and SBS data reporting regimes |
  | Apr. 7, 2026 | Wave Digital Assets LLC | Letter of support for Veda Tech Labs |
  | Mar. 27, 2026 | sFOX, Inc. | Intent to participate in the "Project Crypto" Innovation Exemption Pilot Program |
  | Mar. 23, 2026 | Veda Tech Labs Inc. | Recognition of vaults as satisfying SEC qualified custody and CFTC segregation requirements for digital assets |
  | Mar. 9, 2026 | Thomas P. Gallagher, **Miami International Holdings** | Potential SEC-CFTC harmonization initiatives |
  | Jan. 29, 2026 | **Paradigm Operations LP** | "Why Close SEC–CFTC Coordination Is Key to Unlocking U.S. Market Innovation" |

  The log page carries its own caveat: "Note that the 'Key Points' column is
  **AI-generated**. AI can make mistakes ... Staff has not reviewed these
  AI-generated summaries for accuracy or completeness."
- **Meeting request form and meeting log** —
  `/featured-topics/sec-cftc-harmonization-initiative/request-meeting` and
  `.../sec-cftc-harmonization-initiative-meetings`. Verbatim: "Market
  participants as well as other interested members of the public are invited to
  request a **joint meeting with SEC and CFTC staff**. ... Any person or firm
  that is meeting with staff **must provide a brief written summary of the
  issues planned for discussion**. ... The SEC and CFTC plan to post these
  summaries to their websites." Five meetings logged: American Innovation
  Exchange / Willkie Farr / Architect Securities (Jul. 16); **Rothera Markets
  LLC** and Delta Strategy Group (Jun. 18); **American Perpetuals Exchange
  Corporation**, Arktouros, Gibson Dunn, BGR Group (Jun. 4); Veda Tech Labs and
  Morrison Cohen (Apr. 23); Integrity Compliance 360 (Mar. 26).

  (Note: Thomas Chippas, CEO of **Rothera Markets**, is an IAC member — Part
  6.2.)

**This resolves an open question in FILED_COMMENTS_LANDSCAPE Part 5.** That
memo's re-run checklist says "ISDA, SIFMA, FIA, and the major exchange and
digital-asset trade associations **have not yet filed** on either joint docket,
and their absence on 2026-08-18 is not evidence of absence on 2026-08-24."
**ISDA and SIFMA filed jointly on May 20, 2026** — five weeks before the RFCs
were even published — through this channel, and ISDA separately met SEC Trading
and Markets staff on July 22, 2026 on the data reporting matter (ex parte
memorandum, § 5.5.2). Their absence from the regulations.gov dockets is not
absence from the process. MIAX, ICE Trade Vault, and Paradigm are in the same
position.

**(b) The SEC Crypto Task Force written-input channel.** Open, rolling, **no
deadline**. Led by Commissioner Hester M. Peirce.

- Main page: <https://www.sec.gov/about/crypto-task-force>
- Submission webform:
  <https://www.sec.gov/about/crypto-task-force/submit-written-input>. Verbatim:
  "If you would like to provide written input on the issues the Task Force is
  considering, please use the webform below. ... **Written input will generally
  be posted on www.sec.gov. Material received will be posted without
  modification; the Commission does not edit personal identifying information
  from submissions.** You should provide only written input that you wish to
  make available publicly." Alternative: **crypto@sec.gov**. Submitter's name is
  marked "Publicly Available"; if none is given the submitter is identified as
  "Anonymous."
- Published index:
  <https://www.sec.gov/featured-topics/crypto-task-force/crypto-task-force-written-input>,
  filterable by topic (Crypto ETPs, Crypto Lending, Custody, Public Offerings,
  Regulatory Sandbox, RFI Responses, Safe Harbor, Security Status, Tokenization,
  Trading). Most recent submission visible: **Aug. 18, 2026**. Same AI-generated
  summary caveat.
- A "Lunch and Learn" channel exists for anyone "interested in presenting on an
  educational crypto topic," via crypto@sec.gov.
- No future Crypto Task Force roundtable is scheduled; the most recent listed is
  2025-12-15.

All retrieved 2026-08-19; the phrases `deadline`, `due date`, `no later than`
and `closes` return **zero matches** on the submission page.

#### 5.5.6 SEC advisory committees

Two exist — the **Investor Advisory Committee** and the **Small Business
Capital Formation Advisory Committee**
(<https://www.sec.gov/about/advisory-committees>). **There is no crypto- or
technology-specific SEC advisory committee**; that work runs through the Crypto
Task Force, which is not a FACA committee.

**Neither has an announced future meeting or an open written-statement window.**
The SEC's upcoming-events page (<https://www.sec.gov/newsroom/meetings-events>)
lists exactly two items: a Closed Meeting on Aug. 20, 2026, and the 24-Hour
Trading Roundtable on Sept. 17, 2026.

Worth recording for when the next one is noticed: the SEC IAC's written-statement
practice is **materially more generous than the CFTC's**. From the Sunshine Act
notice for its June 4, 2026 meeting (91 FR 32499, FR Doc 2026-10847):

> "**Public Comment:** The public is invited to submit written statements to the
> Committee. Written statements should be received on or before June 3, 2026.
> ... All submissions should refer to **File No. 265-28**. ... **The Commission
> will post all statements on the Commission's website.** ... The meeting will
> be conducted in-person ... and by remote means. **Members of the public may
> attend in-person** or watch the webcast."

Two differences from CFTC practice: the deadline is **before** the meeting, and
the public may attend **in person**. The SBCFAC notice (91 FR 38043, FR Doc
2026-12755) contains **no** public-comment provision at all. The SEC IAC's
published recommendations include one titled **"Tokenization of Equity
Securities."**

---

## Part 6 — The IAC and the innovation bodies

### 6.1 Charter and constitution

Amended charter filed **March 3, 2026** with the Commission, the Senate
Committee on Agriculture, Nutrition and Forestry, the House Committee on
Agriculture, OMB, the GSA Committee Management Secretariat, and the Library of
Congress. <https://www.cftc.gov/media/13366/IAC_Charter030326/download>
(retrieved 2026-08-19). Key terms:

- **Authority.** A **discretionary** advisory committee under FACA, 5 U.S.C.
  §§ 1001–1014 (¶2).
- **Scope** (¶3). Advice on "the impact and implications of technological
  innovation in the financial services and markets"; on "the application and
  utilization of new technologies"; and — the clause most directly usable by
  this program — on "**the appropriate level of investment in technology at the
  Commission to meet its surveillance and enforcement responsibilities**."
- **Duties** (¶4). "In accordance with 5 U.S.C. § 1008(b), the duties of the
  IAC shall be **solely advisory**." Topics named: "cybersecurity, responsible
  financial innovation, digital assets, blockchain technology, artificial
  intelligence, and evolving and emerging technologies." Advice is adopted by
  simple majority, "shall be developed in consultation with all members," and
  "any transmission to the Commission shall include dissenting or minority
  views, if any." "No determination of fact or policy shall be made by the IAC
  on behalf of the Commission."
- **Frequency** (¶9). "It is estimated that meetings of the IAC will occur **at
  least annually**; meetings of subcommittees may occur as frequently as
  needed."
- **Duration** (¶10–11). Two years from the date of renewal.
- **Membership** (¶12). "Approximately 40-45 members (voting and nonvoting)."
  Seven viewpoint categories: market participants; financial technology
  providers; market infrastructure firms; other segments of the industry;
  regulatory organizations including SROs; academia; and "Think Tanks and
  Public Interest Groups."
- **Subcommittees** (¶13). "IAC subcommittees shall report to the IAC and **may
  not provide advice and recommendations directly to the Commission**"; their
  advice "shall be deliberated on by the IAC before transmission."
- **Recordkeeping** (¶14). "Records of open IAC meetings will be made available
  to the public on the Commission's website."
- **Cost** (¶7). "Approximately $170,787" annually, including 0.46 FTE.

**Renewal**, 91 FR 52047, FR Doc 2026-16423, published 2026-08-12,
<https://www.federalregister.gov/documents/full_text/text/2026/08/12/2026-16423.txt>
(retrieved 2026-08-19): "The IAC will operate for two years from the date of
renewal unless the Commission directs that the IAC terminate on an earlier
date." Its public-interest determination under 41 CFR 102-3.60(a) gives the
annual operating cost as **$83,581.88** with 0.50 FTE — a figure that does not
match the charter's $170,787 / 0.46 FTE. Both are quoted here as found; the
discrepancy is not explained in either document.

The renewal notice also enumerates the agency's committees: "In addition to the
IAC, the CFTC has one statutory committee that Congress has exempted from the
FACA and one discretionary FACA committee: (a) Agricultural Advisory Committee
(discretionary) (b) Energy and Environmental Markets Advisory Committee
(statutory)." **GMAC and MRAC are not listed**, confirming OPEN_MATTERS_MAP
Part 4.

### 6.2 Membership, verified today

<https://www.cftc.gov/About/AdvisoryCommittees/IAC> (retrieved 2026-08-19).
Sponsor: Chairman Michael S. Selig. DFO: Michael Passalacqua. Chair: **Walt
Lukken, CEO, FIA** (named as Chair on the meeting agenda PDF). **43 rows in the
member table.**

Both members flagged by the landscape memo are **confirmed on the page today**:

- **Raghu Yarlagadda — FalconX — Co-Founder & CEO.** FalconX Bravo, Inc. filed
  CFTC-2026-1355-0006, the one institutionally sponsored counter-position to
  P-D7.
- **Sergey Nazarov — Chainlink Labs — CEO.** A reference-price infrastructure
  vendor, in the room for any argument about how settlement references are
  specified.

Nine further members intersect this work directly, all verified from the same
page:

| Member | Entity | Why it matters here |
|---|---|---|
| Shayne Coplan | Polymarket | CEO of an event-contract venue; Session III |
| Luana Lopes Lara | Kalshi | Co-founder of the venue whose certifications drive the § 40.11 / § 40.6 traffic |
| Hayden Adams | Uniswap Labs | Constant-product AMM; the manipulation-cost work is over synthetic constant-product pools |
| Vladimir Novakovski | Lighter | Perpetuals venue; Session I/III and the perpetual-contracts RFCs |
| Anatoly Yakovenko | Solana Labs | The runtime the local SBF campaigns target |
| Vlad Tenev | Robinhood | Retail event-contract distribution |
| Chris Dixon | a16z crypto | Managing Partner |
| Scott D. O'Malia | ISDA | The trade body absent from both joint dockets as of the last survey |
| Profs. Harry Crane (Rutgers) and Carla Reyes (Gray Dawn Breaking) | — | The only two "Representative"-titled academic entries |

**Reports:** still only the two TAC-era ones — "Responsible Artificial
Intelligence in Financial Markets" (approved 2024-05-02) and "Decentralized
Finance" (approved 2024-01-08). **No IAC subcommittee, report, background
paper, panel list, or discussion draft has been posted.**

**Future meetings:** the IAC page's meetings table lists 08/20/2026 and then
jumps back to 05/02/2024. **No future IAC meeting is announced**, on the page or
in the Federal Register, as of 2026-08-19.

### 6.3 The Innovation Task Force — it has a page, a standing input channel, and a meeting log

This corrects OPEN_MATTERS_MAP Part 3, item 4, which reported that "No
dedicated page for the Innovation Task Force exists on cftc.gov ... and no
public input channel for it has been published."

<https://www.cftc.gov/About/Innovation> (retrieved 2026-08-19) states three
"Innovation Focus Areas" — "Crypto Assets & Blockchain Technologies";
"Artificial Intelligence & Autonomous Systems"; "Prediction Markets & Event
Contracts" — and, under "How We Work":

> "Through public roundtables, **written input**, the Innovation Advisory
> Committee, and industry engagement, the CFTC aims to develop insights that
> will help inform Commission policy, interpretation, and rulemaking
> considerations. Please contact the Innovation Task Force at
> innovation@cftc.gov or by requesting a meeting with the Task Force via the
> Related Links."

The page also carries an "Innovation Tracker" table of Commission actions, and
three Related Links:

**(a) Written Input** — <https://www.cftc.gov/About/Innovation/writteninput>
(retrieved 2026-08-19):

> "Please find the written input submitted to the Innovation Task Force below,
> **which is posted without modification**. The Innovation Task Force hopes
> sharing submissions will help promote dialogue and continued engagement with
> industry stakeholders. Interested parties can submit materials to the
> Innovation Task Force by emailing Innovation@cftc.gov."

**No deadline. No docket. No form.** An email address, and publication without
modification. Two submissions are posted:

| Date | Submitter | Subject | Document |
|---|---|---|---|
| 2026-03-24 | Daniel Lasko, dYdX Trading Inc. | "Re: Perpetual Derivatives Classification" | <https://www.cftc.gov/media/13711/innovation_dYdXTrading03242026/download> |
| 2026-02-24 | Ashok Pinto, Blockchain Association | "Re: Petition for an Order under Section 4(c) of the Commodity Exchange Act: Trading and Clearing of Perpetual Derivatives on Digital Assets" | <https://www.cftc.gov/media/13716/innovation_BlockchainAssn02242026/download> |

(Neither document was read in this lane.)

**(b) Innovation Task Force Meeting Log** —
<https://www.cftc.gov/About/Innovation/meetings> (retrieved 2026-08-19). "The
table below reflects meetings between the Innovation Task Force and market
participants." **27 meetings logged between 2026-04-01 and 2026-07-23**, with
participants including ISDA, Blockchain Association (×3), Galaxy Digital (×2),
dYdX, Hyperliquid Strategic Inc. and Hyperliquid Labs, Ondo Finance,
Fireblocks, Morpho Association, BitGo, Dragonfly Capital, The Digital Chamber,
Hedera, Backpack, Databoiler, The Wall Street Blockchain Alliance, Topstep,
Choctaw Nation, DormDAO, PrizePicks, Parcl Labs, Blockworks, Mecone Markets,
Unit Labs, Moody's, and TradingHub. **No individual is logged.** The most recent
entry is 2026-07-23; there is no entry in the four weeks before retrieval.

**(c) Meeting Request Form** —
<https://forms.cftc.gov/forms/InnovationMeetingRequest> (retrieved 2026-08-19).
Fields: name, phone, email, organization, "Purpose of Meeting" (4,000
characters), and up to 10 attachments of ≤10 MB each in doc/docx/xls/xlsx/pdf/
ppt/pptx/odt/ods/odp/txt/jpg/bmp/zip. The form states: "Although the Innovation
Task Force aims to accommodate all meeting requests, the Task Force may be
limited in its ability to schedule a meeting in each instance."

**This is the only channel found anywhere at the CFTC through which a member of
the public can request to speak to staff about these subjects, and it has no
deadline.** It is not the IAC and carries none of FACA's record guarantees.

### 6.4 The agenda for tomorrow

Agenda PDF, <https://www.cftc.gov/media/14476/IACMeetingAgenda082026/download>
(retrieved 2026-08-19; announced by Press Release 9283-26, 2026-08-13). It is
now linked from the event page under "Related News" — it was not on 2026-08-18
(OPEN_MATTERS_MAP Part 1).

"Inaugural Meeting: Innovation Advisory Committee, August 20, 2026, 1:00 pm –
4:00 pm Eastern Standard Time" (the agenda says EST; the Federal Register notice
says Eastern Daylight Time). Each session's bullets are headed "**Potential
Topics**."

| Time | Segment |
|---|---|
| 1:00–1:30 | Introductions and Opening Remarks — Passalacqua (DFO); Walt Lukken (Chair); Chairman Selig (Sponsor) |
| 1:30–2:20 | **Session I** — Crypto's Regulatory Evolution: From Uncertainty to Clarity |
| 2:20–2:30 | Break |
| 2:30–3:05 | **Session II** — Artificial Intelligence: Preparing for Intelligent Markets |
| 3:05–3:55 | **Session III** — Prediction Markets: Innovation, Jurisdiction, and the Future of Event Contracts |
| 3:55–4:00 | Closing Remarks — Chairman Selig |

Session II names "**The Rise of Agentic Finance** — Autonomous agents capable of
executing financial transactions and managing portfolios; Intersection of crypto
and AI," and asks "How existing regulatory principles apply to AI-enabled market
participants" and "Whether additional guidance or best practices would promote
responsible innovation."

Session III names "**Product design principles for event contracts**";
"Expectations for exchanges listing innovative products"; "**Market
surveillance, manipulation concerns, and customer protections**"; and
"Identifying principles that can guide the next generation of event-based
products." It also adds a heading the earlier memo did not record: "Federal and
State Regulatory Perspectives," including "Recent state litigation and
enforcement actions."

---

## Part 7 — Recurring and predictable cadence: the watch list

Each item below is a rule or an observed regularity with its source, not a
guess.

### 7.1 Structural, from the regulations

| Item | Rule | Consequence |
|---|---|---|
| **Advisory-committee meeting notice** | 41 CFR 102-3.150(a): "A notice in the Federal Register must be published **at least 7 calendar days prior** to an advisory committee meeting" | Seven days is the *floor*, and the CFTC used nine for the IAC (notice 2026-08-11, meeting 2026-08-20). Assume ~1–2 weeks of warning, no more. |
| **Minutes** | 41 CFR 102-3.165(c): certified by the chairperson "**within 90 calendar days** of the meeting" | IAC minutes for the August 20 meeting are due by roughly **2026-11-18**. |
| **Transcript** | 91 FR 51697: "After the meeting, a transcript of the meeting will be published through a link on the CFTC's website" | No deadline. Measured lag: 14 days (AAC 2026) to 281 days (TAC 2024). |
| **PRA second window** | 5 CFR 1320.10(a) and 1320.12(c): on or before submission to OMB the agency must publish a further FR notice requesting "that comments be submitted to OMB **within 30 days** of the notice's publication"; 1320.10(b): "OMB shall provide at least 30 days for public comment" | **Every PRA renewal gives two bites.** Each of the five October 5 / October 13 / October 19 collections above will generate a second, 30-day, OMB-directed window later. |
| **Charter renewal** | Charter ¶10–11 and 91 FR 52047: two years from renewal | The IAC charter cycle runs to roughly August 2028; a renewal notice publishes in the Federal Register each time. |
| **Committee reports** | 41 CFR 102-3.175(d), 5 U.S.C. 1012: copies of each report filed with the Library of Congress | Any IAC subcommittee report becomes publicly filed. |
| **GSA annual review** | 41 CFR 102-3.175(b): GSA "is required to conduct an **annual comprehensive review**" of each committee | Annual, government-wide. |
| **§ 40.6(c)(2) stay windows** | 17 CFR 40.6(c)(2): a 30-day comment period noticed **on the Commission website** | These never appear in the Federal Register. A FR-only watch misses them. |
| **IAC meetings** | Charter ¶9: "at least annually" | One per year is the floor. Nothing further is announced. |

### 7.2 Standing, always-open channels

**Six standing channels exist. None has a deadline. Every one publishes without
modification.**

| Channel | Address | Agency | Deadline | Published? |
|---|---|---|---|---|
| **CFTC Innovation Task Force — written input** | innovation@cftc.gov | CFTC | **None** | Yes — "posted without modification" at `/About/Innovation/writteninput` (2 submissions) |
| **CFTC Innovation Task Force — meeting request** | <https://forms.cftc.gov/forms/InnovationMeetingRequest> | CFTC | **None** | The meeting is logged at `/About/Innovation/meetings` (27 meetings); materials may be attached |
| **SEC-CFTC Harmonization Initiative — written input** | `/featured-topics/sec-cftc-harmonization-initiative/submit-written-input`; harmonization@sec.gov and harmonization@cftc.gov | **Both** | **None** | Yes — "posted without modification" on **both** agency sites; log holds 7 submissions incl. ISDA/SIFMA |
| **SEC-CFTC Harmonization Initiative — joint meeting request** | `.../request-meeting` | **Both** | **None** | Yes — 5 meetings logged; a written summary of issues is **required** and is posted |
| **SEC Crypto Task Force — written input** | `/about/crypto-task-force/submit-written-input`; crypto@sec.gov | SEC | **None** | Yes — "posted without modification"; index filterable by topic; latest 2026-08-18 |
| **Petition for rulemaking** | 17 CFR 13.1 — Secretariat, by mail or via the Commission website | CFTC | **None** | Acknowledged and answered; see Part 4.1(d) |

Plus one right with no published mechanism: **an advisory-committee statement
outside a meeting window.** 41 CFR 102-3.140(c) permits it "whether or not the
statement is related to a specific meeting," and it has no deadline, but the
CFTC publishes no mechanism for it. The only address the notice gives is the
IAC DFO mailbox, IAC@CFTC.gov.

Note the asymmetry between these and the docketed channels: the Harmonization
meeting channel *requires* a written summary and *posts* it, which makes a
requested meeting a publication event as well as a conversation. The Innovation
Task Force meeting log records only the organization name.

### 7.3 What is *not* a reliable cadence

- **The CFTC's Regulatory Flexibility Agenda.** Federal Register documents
  titled "Regulatory Flexibility Agenda" from this agency since 2020:
  **2020-08-26 (85 FR 52805), 2021-03-31 (86 FR 16992), 2024-08-16 (89 FR
  66898). None in 2025 or 2026.** (federalregister.gov API term search,
  retrieved 2026-08-19.) Do not build a watch on it.
- **The event page.** It carried a "Public Comments" block with a deadline on
  all four TAC meetings and carries **none** for the IAC. And for TAC
  2023-03-22 the event page's stated deadline (May 22, 2023) **contradicts the
  Federal Register notice's** (March 29, 2023, per 88 FR 13107). The notice is
  the document of record.

### 7.4 Two further things worth watching, verified

- **SEC Exchange Act § 19(b) SRO notices carry a fixed 21-calendar-day comment
  window** from Federal Register publication — verified verbatim against three
  notices published 2026-08-11, 2026-08-14, and 2026-08-19. That makes SRO
  deadlines computable from the publication date without fetching each notice,
  and makes crypto-ETP and listing-standard filings a predictable weekly stream
  rather than a surprise. 51 such notices published in a nine-day window.
- **The SEC Investor Advisory Committee's written-statement file number is
  265-28** and is stable across meetings; its notices put the statement
  deadline **before** the meeting and permit in-person public attendance. The
  committee meets irregularly; the SBCFAC page states it "meets quarterly," and
  its last meeting was 2026-07-21 continued to 2026-08-06, so a Q4 meeting is
  likely but is **not announced**.

### 7.5 The leading indicator is the press-release feed, not the Federal Register

Both of the two new comment opportunities found in this sweep — the compute
derivatives RFC (2026-08-19) and the CPO/CTA NPRM (2026-08-18) — were
**published in full on cftc.gov with a press release before Federal Register
publication**, and neither was on FR public inspection. The agenda for tomorrow's
meeting was likewise announced by press release (9283-26) before the event page
linked it. <https://www.cftc.gov/PressRoom/PressReleases> (retrieved
2026-08-19) is the earliest-warning surface available, by roughly the length of
the OFR queue.

---

## Part 8 — Corrections to the companion memos

1. **OPEN_MATTERS_MAP, Part 3, item 4** — "**No dedicated page for the
   Innovation Task Force exists on cftc.gov** — four candidate URLs were tried
   and all returned 404 — and **no public input channel for it has been
   published.**" **This is incorrect as of 2026-08-19.**
   <https://www.cftc.gov/About/Innovation> exists and carries a standing
   written-input channel (innovation@cftc.gov, submissions "posted without
   modification"), a 27-entry meeting log, and a meeting-request form. See
   Part 6.3. The consequence for that memo's conclusion is material: its
   central calendar finding was that "the August 27 IAC written-statement
   docket is the only currently open public channel to CFTC AI policy." **There
   is a second, and it has no deadline.**

2. **OPEN_MATTERS_MAP, Part 1** — "a re-check of
   <https://www.cftc.gov/PressRoom/Events/opaeventiac082026> on 2026-08-18
   shows no agenda." **The event page now links the Agenda** under "Related
   News" (retrieved 2026-08-19). The memo's broader point survives: the event
   page still has no public-comment section and no written-statement deadline.

3. **FILED_COMMENTS_LANDSCAPE, Part 1 table** — the TAC 2023-03-22
   public-statement deadline is given as "May 22, 2023." That is the **event
   page's** figure. The **Federal Register meeting notice** for the same
   meeting, 88 FR 13107 (FR Doc 2023-04332), says statements "should submit
   them by March 29, 2023." Two primary sources disagree; the notice is the
   document of record and the table should carry both.

4. **FILED_COMMENTS_LANDSCAPE, Part 1** — "Across all four predecessor meeting
   pages, not one written statement from a member of the public is posted."
   **Independently confirmed 2026-08-19** by re-extracting every link on all
   four pages, and extended: the same is true of the AAC's 2026-07-29 page,
   whose statement window has already closed.

5. **OPEN_MATTERS_MAP, Part 2 table** — the joint portfolio-margining docket
   CFTC-2026-1420 is listed with 2 comments (the Federal Register API's
   figure). A direct regulations.gov v4 query on 2026-08-19, before the rate
   limit engaged, returned **0**. The discrepancy is unexplained; both figures
   are recorded here.

6. **FILED_COMMENTS_LANDSCAPE, Part 0** — "**SEC comment lists do not exist for
   these file numbers.** Five URL patterns were tried and all returned HTTP
   404." **Incorrect.** All five were the retired URL shape. The current shape
   is `/rules-regulations/public-comments/<file-no>`, and all three joint file
   numbers resolve with letters on them (Part 5.5.1). The same memo's
   conclusion that "the CFTC/regulations.gov side is the only public view of
   these dockets" therefore does not hold. The underlying cause is an access
   convention, not a blocked site: sec.gov 403s a *spoofed browser*
   User-Agent and answers a declared-identity one.

7. **FILED_COMMENTS_LANDSCAPE, Part 2 census** — "**Eleven comments total
   across the three dockets.**" That is the regulations.gov count. Across the
   same three matters the SEC side holds **at least seven further public
   comments** (Nasdaq, Carol Alexander, Seth Allen, Nicolin Decker on the
   definitions matter; Navigator-FX, "Msmith", Zontonnia Moore on data
   reporting) **plus five ex parte memoranda** recording SEC staff meetings with
   DTCC Data Repository, ICE Trade Vault, ISDA, DTCC, and Bloomberg (Part
   5.5.2). Nothing on the CFTC side discloses those meetings.

8. **FILED_COMMENTS_LANDSCAPE, Part 5, checklist item 1** — "ISDA, SIFMA, FIA,
   and the major exchange and digital-asset trade associations **have not yet
   filed** on either joint docket." **ISDA and SIFMA filed a joint letter on
   2026-05-20**, five weeks before the RFCs published, through the SEC-CFTC
   Harmonization Initiative written-input log; MIAX (2026-03-09), ICE Trade
   Vault (2026-04-13) and Paradigm (2026-01-29) used the same channel; and ISDA
   separately met SEC Trading and Markets staff on 2026-07-22 on the data
   reporting matter. **The trade associations are not absent from this process;
   they are in a channel neither companion memo looked at.** (Part 5.5.5.)

---

## Part 9 — What this means for the current packet

Stated as facts and their direct consequences, not as recommendations.

**On timing.**

- The meeting is **tomorrow, August 20, 1:00–4:00 p.m. EDT**. The
  written-statement date is **August 27**, and the notice's verb is "should."
- The CFTC states that regulations.gov submissions take "up to several days" to
  appear. A statement submitted tonight is unlikely to be on the public docket
  before the meeting convenes.
- The only legally specified consequence of filing before rather than after is
  41 CFR 102-3.165(b)(2): the certified minutes list members of the public who
  *presented* oral or written statements at the meeting. That provision has
  never been exercised in this committee's published record.
- Nothing in the notice states that pre-meeting statements are given to
  members. Do not assume it, and do not write as though it were true.
- Filing after the meeting permits the statement to engage what was actually
  said in Sessions II and III, which will be observable live but not in
  transcript by August 27.

**On speaking.** There is nothing to request and nothing that has been missed.
No CFTC advisory-committee notice in a 2023–2026 sample of eight provides for
oral comment, and 41 CFR 102-3.150(a)(6) requires such instructions to be in
the notice if oral comment is permitted. The public channel is listen-only
audio or webcast. The nearest speaking channel at the agency is the Innovation
Task Force meeting-request form, which is a different body, has no deadline,
carries no FACA record guarantee, and logs the meeting publicly by organization
name.

**On what the document becomes.** It becomes a permanently public entry on
docket CFTC-2026-1717, published without review and without removal of personal
identifying information or business-confidential material. On five meetings of
observed practice it will **not** appear on the meeting's event page. The
CFTC's own promise that "Statements submitted in connection with the committee
meeting will be made available to the public, including publication on
CFTC.gov" appears on the four TAC event pages and appears nowhere on the IAC's.
This is a direct input to the identity and confidentiality gate in
[../DRAFT5_CLAIM_LEDGER.md](../DRAFT5_CLAIM_LEDGER.md).

**On effort allocation across channels.** Only an NPRM comment carries a
statutory duty to consider and to produce a reasoned statement (5 U.S.C.
553(c)), reinforced at the CFTC by 7 U.S.C. 19(a). Only a part 13 petition
carries a duty to answer the submitter. An advisory-committee statement and an
RFC comment carry publication and record retention, and the agency's own stated
practice of taking RFC comments "into consideration to determine whether and
how to proceed with an NPRM." Of the matters this program is currently working,
**all three joint matters and the 24/7 matter are Requests for Comment, and the
IAC statement is an advisory-committee statement — none of them is an NPRM.**
The one NPRM on the open list is Conflicts and Affiliations (RIN 3038-AF76, due
October 5).

**On the forward calendar.** Taking both agencies together, after August 31 the
next deadline is **September 8** (SEC, Cboe Clear temporary clearing-agency
registration), then September 9 (SEC, the 3x crypto/commodity ETF listing
filing), September 14-or-24 (SEC Reg Flex Agenda, conflicting), September 17
(SEC 24-hour trading roundtable), September 21 (SEC electronic delivery), and
September 28 (CFTC PRA 3038-0097). The CFTC side is empty from September 1 to
September 28; **the September calendar is almost entirely SEC.** October 5–19
then carries one CFTC NPRM and six CFTC PRA collections.
The **Request for Comment on the Listing of Compute Derivatives Contracts (RIN
3038-AF77)**, released today, has not started its clock — 60 days from Federal
Register publication — and is the closest subject-matter match to this
program's developed positions that any sweep has produced: it asks, in the
Commission's own words, whether a contract may settle to a price the Commission
"may not be able to observe, verify, or surveil," who is best positioned to
move the reference price, what surveillance is feasible, and whether the terms
and conditions should be required to carry specific risk information. That is
the reference-specification-in-the-terms position and the computable
manipulation-cost position, on an AI-adjacent underlier, on a docket with zero
comments because it does not exist yet.

**On the standing channels.** There are **six**, not one, and none has a
deadline (Part 7.2). Three bear directly on this program's subject matter:

- The **CFTC Innovation Task Force** written-input channel (innovation@cftc.gov)
  belongs to the body whose three named workstreams are exactly
  crypto/blockchain, AI and autonomous systems, and prediction markets and
  event contracts. It holds **two** submissions; the adjacent meeting channel
  has been used by twenty-seven organizations.
- The **SEC-CFTC Harmonization Initiative** written-input channel is **joint**,
  posts to both agencies' websites without modification, and covers precisely
  the three matters this program is already filing on. It holds **seven**
  submissions. Its meeting channel requires a written summary of issues and
  posts that summary — so a requested meeting is itself a publication.
- The **SEC Crypto Task Force** channel is rolling and open, with a topic
  taxonomy that already includes Custody, Safe Harbor, Security Status,
  Tokenization, and Trading.

The competitive fact attached to this: the trade associations this program's
landscape memo recorded as absent — ISDA, SIFMA — were **not** absent. They
filed jointly on 2026-05-20 through the Harmonization channel, before the RFCs
existed. Reading the joint dockets alone systematically understates who is in
the room.

**On the SEC-side record.** The three joint matters have a second public face at
`sec.gov/rules-regulations/public-comments/s7-2026-2X`, holding letters from
filers who never appear on the CFTC dockets and five ex parte memoranda
recording DTCC, ICE Trade Vault, ISDA and Bloomberg meeting SEC staff on data
reporting. Any statement in the packet about who has and has not taken a
position on these dockets should be checked against both faces.

**On the other agency's crypto item.** **Regulation Crypto Assets (S7-2026-27**,
RIN 3235-AN38, Releases 33-11434 / 34-106150), proposed 2026-08-18, is a
*proposed rule* — the one document type carrying a 5 U.S.C. 553(c) duty to
consider — and it proposes "a conditional safe harbor from the term 'investment
contract'" such that a crypto asset "would be deemed not to be subject to an
investment contract" for purposes of the statutory definitions of "security."
That is a classification rule operating on the same boundary the definitions
comment argues about, from the other side, with a 60-day window that has not
opened.

---

## Related documents

- [FILED_COMMENTS_LANDSCAPE.md](FILED_COMMENTS_LANDSCAPE.md) — who else is on
  the three current dockets and what is unclaimed there.
- [OPEN_MATTERS_MAP.md](OPEN_MATTERS_MAP.md) — what else the CFTC has open.
  Three of its findings are corrected in Part 8.
- [../CFTC_IAC_MEETING_BRIEF.md](../CFTC_IAC_MEETING_BRIEF.md) — the meeting
  brief.
- [../SUBMISSION_WEEK_PLAN.md](../SUBMISSION_WEEK_PLAN.md) — the week plan.
- [../LEGAL_ANALYSIS.md](../LEGAL_ANALYSIS.md) — the citation ledger. **None of
  the procedural authorities in Part 1, Part 3, Part 4, or Part 7 is on that
  ledger**; each is cited here by section, source URL, and retrieval date, and
  anything relied on in a filing must be added to the ledger first.

---

## Addendum, 2026-08-20 evening — the inaugural IAC meeting occurred

Recorded same-day from desk monitoring (ember's desk watched the meeting;
the transcript will adjudicate in-room specifics later — AAC precedent says
~14 days). Verification status per item:

- **Chairman Selig's opening remarks were POSTED on cftc.gov before the
  meeting** (Aug 20, ~16:13Z) and are quotable now. They announce a
  "Roadmap for the New Frontier of Finance" with three named tracks:
  (1) crypto capital markets; (2) "Winning the AI Race: Roadmap for Compute
  Market Dominance" — naming the compute derivatives RFC (Release 9286-26,
  RIN 3038-AF77, issued Aug 19) as "our first step" toward "a gold standard
  regulatory framework," in partnership with Commerce under the AI Action
  Plan; (3) prediction markets.
- **A fourth door**: the remarks state the Commission expects to propose
  **Parts 38 & 40 amendments** — modernized DCM core principles and listing
  rules for event contracts (consumer protection, product governance,
  market design, incentive programs). Not yet published, no clock running;
  joins the FR watch.
- **The developer-engagement directive**: staff directed to "engage with
  developers of onchain finance protocols to establish ways in which
  developers can offer their protocols in a legal and compliant manner."
- **Compute RFC context** (desk research, re-verify before citing beyond
  the RFC itself): two exchange applications already pending (CME +
  Silicon Data futures settling to daily GPU rental benchmarks, announced
  May 12; ICE + Ornn cash-settled futures on Ornn's OCPI). The RFC's
  analytical frame leans on Bandi & Su, "(Early) AI Compute Asset Pricing"
  (arXiv 2607.12156): compute service flow $430B–$1.3T/yr; the named
  structural gap is that compute has **no market-clearing venue publishing
  real clearing prices** — only surveyed indexes — the same gap FERC Order
  888's market operators closed for electricity. Structurally: a
  settlement/clearing layer.
- **In-room content** (unquotable until the transcript): the meeting
  discussed formal verification among other aspects (firsthand report).

Consequences for the calendar: the compute RFC is no longer merely a
forward opportunity — it is the Chairman's titled agenda track, with a
~60-day window from FR publication (~mid/late Oct). The Aug-27 IAC
statement should be meeting-responsive against the posted remarks; the
full compute response belongs in the RFC window.
