#import "../shared/template.typ": review_note, source_entry

= Source notes

// The seven source entries are set in two columns, as in the IAC statement.
#columns(2, gutter: 16pt)[

#source_entry(
  1,
  "Joint Request for Comment on Swap and Security-Based Swap Data Reporting",
  "https://www.federalregister.gov/documents/2026/06/24/2026-12742/joint-request-for-comment-on-swap-and-security-based-swap-data-reporting",
  [91 Fed. Reg. 37877 (June 24, 2026), FR Doc. 2026-12742, especially Questions 3, 8, and 19. Comments must be received on or before August 24, 2026. CFTC RIN 3038-AF70; SEC File S7-2026-22. Retrieved August 17, 2026.],
)

#source_entry(
  2,
  "CFTC real-time public reporting requirements",
  "https://www.ecfr.gov/current/title-17/chapter-I/part-43",
  [17 C.F.R. part 43, current through August 14, 2026 and retrieved August 17, 2026. Recheck immediately before filing.],
)

#source_entry(
  3,
  "CFTC swap data recordkeeping and reporting requirements",
  "https://www.ecfr.gov/current/title-17/chapter-I/part-45",
  [17 C.F.R. part 45, current through August 14, 2026 and retrieved August 17, 2026. Recheck immediately before filing.],
)

#source_entry(
  4,
  "CFTC swap data repository requirements",
  "https://www.ecfr.gov/current/title-17/chapter-I/part-49",
  [17 C.F.R. part 49, current through August 14, 2026 and retrieved August 17, 2026. Recheck immediately before filing.],
)

#source_entry(
  5,
  "Comment of Ariadne Dataworks Ltd. on the joint data-reporting request",
  "https://www.regulations.gov/comment/CFTC-2026-1354-0002",
  [Comment CFTC-2026-1354-0002, Docket CFTC-2026-1354 (posted July 14, 2026); the endorsed recommendations are characterized from this filed comment. Retrieved August 18, 2026.],
)

#source_entry(
  6,
  "Solana: Retrying Transactions",
  "https://solana.com/developers/cookbook/transactions/retry",
  [Official Solana Foundation developer documentation. It states that Solana has no mempool; describes submission through an RPC server or directly to leaders through a TPU client; distinguishes client, relaying-node, and leader awareness before processing; and explains that a transaction may be dropped before inclusion. Retrieved August 18, 2026.],
)

#source_entry(
  7,
  "Solana RPC JSON Structures",
  "https://solana.com/docs/rpc/json-structures",
  [Official Solana Foundation RPC documentation. Confirmed blocks include transaction records and transaction status metadata includes an error field, supporting the distinction between a landed failed transaction and one rejected or dropped before inclusion. Retrieved August 18, 2026.],
)

]

== Local technical materials inspected August 20, 2026

*VERIFIED (local inspection).* The appendix gives the basis of each claim; this
note gives their provenance. The technical descriptions were prepared from
separately scoped artifacts reviewed by the submitter: guarded-commitment and
candidate-result formal models; exact pure-Rust categorical,
observation-accumulator, batch, and B-spline kernels; 212 zero-sorry Lean
theorems; one narrow Verus-checked transfer seam; focused local-Agave/SBF
construction, custody, point-resolution, redemption, reservation,
staged-resolution, submission, clearing, settlement, and terminal-closure
campaigns, including the 44-transaction signed loopback-validator walk of
August 20, 2026 against the sealed, independently attested build; a
deterministic synthetic-transcript leakage laboratory; and, in separately
provenanced repositories, the bounded Clear energy-dispatch relation and the
encrypted candidate-validation experiment. None composes into a complete venue
or reporting system. *STOP:* no production provider authenticator, parser, or
deployable source pipeline exists; authenticated production price history
remains a qualifying design, not a current SBF property; permissionless source
construction executes only in a deliberately non-production mock-provider
build, and the default build registers no provider or parser release and fails
closed before source state or value admission. A final public edition should
cite frozen public repository commits and exact paths if those repositories
are public by filing time.

#review_note[
  *Final-source gate.* The notice requires receipt on or before August 24, 2026.
  A CFTC submission must reference the exact joint-request title and RIN
  3038-AF70; an SEC submission must include File S7-2026-22. Use only one listed
  method for each agency, satisfy the English-language instructions, recheck the
  live submission pages, and exclude confidential business information,
  unnecessary personal data, private keys, and unpublished vulnerability details.
]
