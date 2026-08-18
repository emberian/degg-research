#import "../shared/template.typ": review_note, source_entry

#set text(size: 9.4pt)
#set par(leading: 0.55em)

= Source notes

// Draft 8 layout note: the twelve source entries are set in two columns,
// recovering about a third of a page toward absorbing the operatorless-agent
// insertion. Entry text and numbering are unchanged; layout only.
#columns(2, gutter: 16pt)[

#source_entry(
  1,
  "Prediction Markets, Advance Notice of Proposed Rulemaking",
  "https://www.cftc.gov/LawRegulation/FederalRegister/proposedrules/2026-05105.html",
  [91 Fed. Reg. 12516 (March 16, 2026), cited for the Commission's public description of event-contract terminology and product and venue categories; its comment period is closed. Retrieved August 17, 2026.],
)

#source_entry(
  2,
  "Commodity Exchange Act definitions",
  "https://uscode.house.gov/view.xhtml?edition=prelim&num=0&req=granuleid%3AUSC-prelim-title7-section1a",
  [7 U.S.C. section 1a, including derivatives clearing organization, swap execution facility, and trading facility. Current preliminary edition retrieved August 17, 2026.],
)

#source_entry(
  3,
  "CFTC Staff Letter No. 26-09",
  "https://www.cftc.gov/csl/26-09/download",
  [Market Participants Division, March 17, 2026. Fact-specific, conditional, nonbinding staff no-action position involving a frontend and registered DCM collaborators. Retrieved August 17, 2026.],
)

#source_entry(
  4,
  "CFTC derivatives clearing organization requirements",
  "https://www.ecfr.gov/current/title-17/chapter-I/part-39",
  [17 C.F.R. part 39, including fully collateralized positions. Current through August 14, 2026 and retrieved August 17, 2026; recheck before filing.],
)

#source_entry(
  5,
  "CFTC designated contract market requirements",
  "https://www.ecfr.gov/current/title-17/chapter-I/part-38",
  [17 C.F.R. part 38, including surveillance, monitoring, audit trail, recordkeeping, and system safeguards provisions. Current through August 14, 2026 and retrieved August 17, 2026; recheck before filing.],
)

#source_entry(
  6,
  "CFTC regulation on use of data collected for regulatory purposes",
  "https://www.ecfr.gov/current/title-17/chapter-I/part-38/section-38.7",
  [17 C.F.R. section 38.7, current through August 14, 2026 and retrieved August 17, 2026. Recheck before filing.],
)

#source_entry(
  7,
  "Innovation at the CFTC",
  "https://www.cftc.gov/About/Innovation",
  [CFTC public description of Innovation Advisory Committee input, meetings, and Innovation Task Force engagement. Retrieved August 17, 2026.],
)

#source_entry(
  8,
  "Innovation Advisory Committee Charter",
  "https://www.cftc.gov/media/13366/IAC_Charter030326/download",
  [Amended March 3, 2026. The Committee is advisory. Retrieved August 17, 2026.],
)

#source_entry(
  9,
  "Innovation Advisory Committee Meeting",
  "https://www.govinfo.gov/content/pkg/FR-2026-08-11/pdf/2026-16328.pdf",
  [91 Fed. Reg. 51697 (August 11, 2026), FR Doc. 2026-16328; written statements must be received by August 27, 2026. Retrieved August 17, 2026.],
)

#source_entry(
  10,
  "CFTC special rule concerning event contracts",
  "https://www.ecfr.gov/current/title-17/chapter-I/part-40/section-40.11",
  [17 C.F.R. section 40.11. Current through August 14, 2026 and retrieved August 17, 2026; recheck before filing.],
)

#source_entry(
  11,
  "Prediction Markets; Public Interest Determinations",
  "https://www.govinfo.gov/content/pkg/FR-2026-06-12/pdf/2026-11854.pdf",
  [91 Fed. Reg. 35806 (June 12, 2026), proposing amendments to Regulation 40.11 and a new Appendix F to part 40. Proposed, not current law. Retrieved August 17, 2026.],
)

#source_entry(
  12,
  "Innovation Advisory Committee Meeting Agenda",
  "https://www.cftc.gov/media/14476/IACMeetingAgenda082026/download",
  [Agenda for the August 20, 2026 inaugural meeting, announced by Press Release 9283-26 (August 13, 2026). Session III, "Prediction Markets: Innovation, Jurisdiction, and the Future of Event Contracts," lists market surveillance, manipulation concerns, and customer protections among its topics. Retrieved August 18, 2026.],
)

]

== Local technical materials inspected

The guarded-commitment and candidate-result descriptions derive from research
models reviewed by the submitter, including a frozen Breadstuffs prototype and
current Minidregg and Leanuweave formal research. The worked market's
accounting derives from the Dragon's Clutch offline conditional-asset
prototype (a pure-Rust research kernel with passing deterministic tests;
tested, not formally verified; not a deployed system, product, or offer). A
final public edition should cite frozen public repository commits and exact
paths if those materials are public by filing time; otherwise it should
retain this limited description and avoid claims of independent public
reproducibility. No source code is included. The descriptions do not claim
production readiness, cryptographic security of a composed system, or a
legal classification.

#review_note[
  *Final-source gate.* The notice requires receipt by August 27, 2026, a reference
  to "Innovation Advisory Committee," one listed submission method, and English
  or an English translation. Recheck the live docket and meeting materials.
  Public comments may be permanently posted; remove unnecessary personal data,
  confidential business information, private keys, and unpublished vulnerability
  details.
]
