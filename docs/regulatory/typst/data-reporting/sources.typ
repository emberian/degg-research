#import "../shared/template.typ": review_note, source_entry

= Source notes

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

== Local technical materials inspected

The technical descriptions were prepared from research artifacts reviewed by
the submitter: the guarded-commitment and candidate-result formal models, the
Dragon's Clutch offline conditional-asset prototype (a pure-Rust research
kernel, observation accumulator, and batch-clearing verifier with passing
deterministic tests; tested, not formally verified; not a deployed system,
product, or offer), and a deterministic synthetic-transcript leakage
laboratory. The Clear, Shielded, and Dark taxonomy is used as a proposed
analytical distinction, not a claim of a deployed confidential venue, complete
compliance system, production proof stack, or vFHE system. The local prototype
history includes serious FHE, MPC, threshold, and private-proof experiments,
but its strongest composed paths remain Shielded rather than end-to-end Dark.
A final public edition should cite frozen public repository commits and exact
paths if those repositories are public by filing time; otherwise it should
retain this limited description and avoid claims of independent public
reproducibility.

#review_note[
  *Final-source gate.* The notice requires receipt on or before August 24, 2026.
  A CFTC submission must reference the exact joint-request title and RIN
  3038-AF70; an SEC submission must include File S7-2026-22. Use only one listed
  method for each agency, satisfy the English-language instructions, recheck the
  live submission pages, and exclude confidential business information,
  unnecessary personal data, private keys, and unpublished vulnerability details.
]
