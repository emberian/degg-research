#import "../shared/template.typ": review_note, source_entry

#set text(size: 9.4pt)
#set par(leading: 0.55em)

= Source notes

#source_entry(
  1,
  "Request for Comment on the Extension of Standard Futures Contracts to 24/7 Trading and on Perpetual Contracts",
  "https://www.regulations.gov/docket/CFTC-2026-1388",
  [CFTC RIN 3038-AF75; Docket CFTC-2026-1388, Document
  CFTC-2026-1388-0001. Comments due August 26, 2026.],
)

#source_entry(
  2,
  "Companion filings, via the same facilitator",
  "https://comments.cftc.gov",
  [The author's written statement to the Innovation Advisory Committee
  (Docket CFTC-2026-1717); and comments on the joint definitions
  (CFTC-2026-1355 / SEC S7-2026-21) and data-reporting
  (CFTC-2026-1354 / SEC S7-2026-22) releases, co-authored by Claude Opus
  4.6, Kimi K3, and GPT-5.6-Sol, three AI systems. The four documents share
  one witness system and one evidence trail.],
)

#source_entry(
  3,
  "Dragon's Clutch: public literate explanation",
  "https://emberian.github.io/dragons-clutch/",
  [The author's public explanation of the witness system, with its Evidence
  page labeling each verification claim by the object it binds. The
  manipulation-cost table of Position 2 (1,080 parameterized window-design
  rows, including the correction it forced on an earlier draft of this
  comment) is published whole at
  `https://emberian.github.io/dragons-clutch/artifacts/manipulation-cost-v1.txt`,
  and its off-hours sequel (8,640 rows adding a session-thinness
  dimension, with its limitations stated in its own header) at
  `.../artifacts/manipulation-cost-v2-offhours.txt`; the remaining
  artifacts are available to Commission staff on request.],
)

#source_entry(
  4,
  "Extension of comment period and further request for comment",
  "https://www.federalregister.gov/documents/2026/07/28",
  [91 Fed. Reg. 47158 (July 28, 2026; FR Doc. 2026-15216): extends this
  window through August 26, 2026, and further requests comment on the
  self-certified 24/7 NYMEX crude-oil contract listed July 8, 2026 and
  stayed by the Commission July 9, 2026.],
)

// DAY-OF GATE (crib): DEPLOY_1.md must be on the repository's public head
// before this files --- push the dclutch subtree first. Optionally replace
// "the repository's public head of the filing date" with a second pinned
// /tree/<hash> URL once that push lands, and rebuild.
#source_entry(
  5,
  "dClutch repository: evidence documents at a pinned snapshot",
  "https://github.com/emberian/dragons-clutch/tree/03442ff97e44184d1b5e9421e3f526298b3c1f52",
  [At this exact public commit: the whole-life campaign and its seven-law
  conservation ledger (`dclutch/tools/gauntlet/journey/README.md`), and the
  public test-network rehearsal that proved the deploy-and-byte-verification
  mechanics (`dclutch/docs/evidence/DEVNET_SMOKE_0.md`). The record of the
  August 27 devnet deployment itself --- each program's on-chain address,
  its deployment slot, and the byte-verification it passed --- is
  `dclutch/docs/evidence/DEPLOY_1.md`, at the repository's public head as
  of the filing date. Underlying run artifacts are
  available to Commission staff on request. Retrieved August 27, 2026.],
)
