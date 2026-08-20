#import "../shared/template.typ": review_note, source_entry

#set text(size: 9.4pt)
#set par(leading: 0.55em)

= Source notes

// The eight source entries are set in two columns, as in the IAC statement.
#columns(2, gutter: 16pt)[

#source_entry(
  1,
  "Joint Request for Comment on Further Definition of Swap and Security-Based Swap and on Alternative Compliance",
  "https://www.federalregister.gov/documents/2026/06/24/2026-12743/joint-request-for-comment-on-further-definition-of-swap-and-security-based-swap-and-on-alternative",
  [91 Fed. Reg. 37873 (June 24, 2026), FR Doc. 2026-12743, especially Questions 1 through 8. Comments must be received on or before August 24, 2026. CFTC RIN 3038-AF71; SEC File S7-2026-21. Retrieved August 17, 2026.],
)

#source_entry(
  2,
  "Commodity Exchange Act definitions",
  "https://uscode.house.gov/view.xhtml?edition=prelim&num=0&req=granuleid%3AUSC-prelim-title7-section1a",
  [7 U.S.C. section 1a, including the definitions of derivatives clearing organization, swap execution facility, and trading facility, and the excluded-commodity definition in section 1a(19). Current preliminary edition retrieved August 17, 2026.],
)

#source_entry(
  3,
  "Further Definition of Swap, Security-Based Swap, and Security-Based Swap Agreement",
  "https://www.govinfo.gov/content/pkg/FR-2012-08-13/pdf/2012-18003.pdf",
  [77 Fed. Reg. 48208 (August 13, 2012), the joint Product Definitions Adopting Release. Retrieved August 17, 2026.],
)

#source_entry(
  4,
  "Exchange Act definition of security-based swap",
  "https://uscode.house.gov/view.xhtml?req=granuleid:USC-prelim-title15-section78c&num=0&edition=prelim",
  [15 U.S.C. section 78c(a)(68), including its security-, security-index-, loan-, and issuer-event reference prongs. Current preliminary edition retrieved August 17, 2026; recheck before filing. This source establishes the definition, not the legal status of any actual digital asset.],
)

#source_entry(
  5,
  "CFTC special rule concerning event contracts",
  "https://www.ecfr.gov/current/title-17/chapter-I/part-40/section-40.11",
  [17 C.F.R. section 40.11. Current through August 14, 2026 and retrieved August 17, 2026; recheck before filing.],
)

#source_entry(
  6,
  "Prediction Markets; Public Interest Determinations",
  "https://www.govinfo.gov/content/pkg/FR-2026-06-12/pdf/2026-11854.pdf",
  [91 Fed. Reg. 35806 (June 12, 2026), proposing amendments to Regulation 40.11 and a new Appendix F to part 40. Proposed, not current law. Retrieved August 17, 2026.],
)

#source_entry(
  7,
  "Comment of FalconX Bravo, Inc. on the joint definitions request",
  "https://www.regulations.gov/comment/CFTC-2026-1355-0006",
  [Comment CFTC-2026-1355-0006, Docket CFTC-2026-1355 (posted August 17, 2026). Primary source only for FalconX Bravo, Inc.'s own listing-status proposal described in position 7; not cited as legal authority. Retrieved August 18, 2026.],
)

#source_entry(
  8,
  "Pyth price-update uniqueness rule (crossing rule)",
  "https://github.com/pyth-network/pyth-crosschain/blob/main/pythnet/pythnet_sdk/src/messages.rs",
  [The provider's published price-message interface documentation states that for any time t, the unique update is the one with prev_publish_time < t <= publish_time. Reviewed at pinned commit ec456fca86adf2ab451ef3622833097c2a36ab00; also described at docs.pyth.network. Retrieved August 19, 2026; recheck before filing.],
)

]

== Local technical materials inspected August 20, 2026

*VERIFIED (local inspection).* The appendix gives the basis of each claim; this
note gives their provenance. The technical descriptions were prepared from
separately scoped artifacts reviewed by the submitter: guarded-commitment and
candidate-result formal models; exact pure-Rust categorical and degree-one
through degree-three B-spline kernels; 212 zero-sorry Lean theorems; one narrow
Verus-checked internal-transfer seam; and focused local-Agave/SBF construction,
custody, resolution, redemption, reservation, staged-resolution, clearing, and
settlement campaigns. The strongest end-to-end evidence is the 44-transaction
signed loopback-validator walk of August 20, 2026, driving the general clearing
lifecycle from policy artifact through portfolio full-pair settlement against
the sealed, independently attested build; the strongest market-construction
evidence begins with a bank containing only the program ELF and uses
ordinary-wallet transactions to create typed sealed artifacts and categorical
or native markets; the strongest native execution evidence covers
source-receipt-authenticated point resolution with internal and exact-lot
bearer redemption for degrees one through three, plus one measured staged
prepaid resolution route inside the selected compute-headroom policy. *STOP:*
this edition does not claim a production provider authenticator or parser,
production price history, fractional-lot bearer exit, or an end-to-end
permissionless lifecycle; permissionless source construction executes only
against a deliberately non-production stand-in provider, and the default build
registers no provider or parser release and fails closed before source state
or value admission. A final public edition should cite frozen public
repository commits and exact paths if the repositories are public by filing
time.

#review_note[
  *Final-source gate.* The notice requires receipt on or before August 24, 2026.
  A CFTC submission must reference the exact joint-request title and RIN
  3038-AF71; an SEC submission must include File S7-2026-21. Use only one listed
  method for each agency, satisfy the English-language instructions, recheck the
  live submission pages, and include only information intended for permanent
  public disclosure.
]
