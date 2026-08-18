# CFTC IAC regulatory research packet

**VERIFIED (local repository state):** Draft 1 research memoranda, Draft 2 review PDFs, and Draft 3 Typst filing-candidate review editions were prepared August 17-18, 2026. Nothing in this directory has been filed, sent to a regulator, deployed, or represented as legal advice.

Claim labels follow the repository convention: **VERIFIED**, **SOURCED**, **INFERRED**, **PROPOSED**, **CHAT-REPORTED**, and **REJECTED**. A label introducing a paragraph, list, or subsection governs the material claims within it until another label appears. Current agency and legal sources were retrieved August 17, 2026 unless a different retrieval date is stated.

## Contents

**VERIFIED (local repository state):**

- [`CFTC_IAC_PUBLIC_COMMENT_DRAFT.md`](./CFTC_IAC_PUBLIC_COMMENT_DRAFT.md) is a filing-oriented written statement for the August 20, 2026 CFTC Innovation Advisory Committee meeting. It contains visible identity placeholders that must be completed before filing.
- [`CFTC_IAC_MEETING_BRIEF.md`](./CFTC_IAC_MEETING_BRIEF.md) is an internal discussion brief: classification map, privacy/auditability design requirements, a 90-second opening, questions to ask, and filing/public-communications cautions.
- [`JOINT_DEFINITIONS_RFC_COMMENT_DRAFT.md`](./JOINT_DEFINITIONS_RFC_COMMENT_DRAFT.md) answers the joint CFTC/SEC definitions request, principally Question 1, using guarded commitments and candidate results as a staged-classification problem.
- [`JOINT_DATA_REPORTING_RFC_COMMENT_DRAFT.md`](./JOINT_DATA_REPORTING_RFC_COMMENT_DRAFT.md) answers the joint CFTC/SEC data request, principally Questions 3, 8, and 19, on blockchain reporting, public identity/strategy leakage, and machine-readable reporting logic.
- [`SUBMISSION_WEEK_PLAN.md`](./SUBMISSION_WEEK_PLAN.md) distinguishes the August 24 joint-comment deadline from the August 27 IAC deadline and provides a non-executing review/filing checklist.

## Draft 3 Typst review editions

**VERIFIED (local repository state):** The Markdown filings above remain the
Draft 1 research record, and the rendered Draft 2 PDFs remain archived. The
current third-draft sources live under [`typst/`](./typst/), with
independent `main.typ`, `metadata.typ`, `body.typ`, and `sources.typ` files for
each report and the IAC cover statement, plus a shared presentation module only.
They add the Regulation 40.11 boundary, early-exit lifecycle milestone, exact
deadline instructions, restrained Dark research framing, and a one-page
plain-English question cover. They remain visibly marked
`DRAFT 3 FOR REVIEW - NOT FILED`.

The current Draft 3 review PDFs are:

- [`joint-definitions-comment-draft-3.pdf`](../../output/pdf/joint-definitions-comment-draft-3.pdf)
- [`joint-data-reporting-comment-draft-3.pdf`](../../output/pdf/joint-data-reporting-comment-draft-3.pdf)
- [`cftc-iac-written-statement-draft-3.pdf`](../../output/pdf/cftc-iac-written-statement-draft-3.pdf)
- [`cftc-iac-cover-statement-draft-3.pdf`](../../output/pdf/cftc-iac-cover-statement-draft-3.pdf)

Build all four from the repository root with
`./scripts/build-regulatory-pdfs.sh`. Rendering a review PDF is not authorization
to file it. The source repository currently has no committed baseline, the
identity fields remain unresolved, and a filing edition still requires a frozen
source commit, exact build manifest and hashes, live-docket revalidation, public
disclosure review, and appropriate legal review.

## Verified open processes

**SOURCED:** Two joint CFTC/SEC requests for comment close **August 24, 2026**:

- FR Doc. 2026-12743, CFTC RIN 3038-AF71, SEC File S7-2026-21, asks for principled criteria distinguishing swaps, mixed swaps, SBS, securities, and excluded instruments in innovative product structures. [Official Federal Register page](https://www.federalregister.gov/documents/2026/06/24/2026-12743/joint-request-for-comment-on-further-definition-of-swap-and-security-based-swap-and-on-alternative).
- FR Doc. 2026-12742, CFTC RIN 3038-AF70, SEC File S7-2026-22, asks about blockchain reporting, public identity/strategy leakage, machine-readable reporting rules, and other data issues. [Official Federal Register page](https://www.federalregister.gov/documents/2026/06/24/2026-12742/joint-request-for-comment-on-swap-and-security-based-swap-data-reporting).

Both sources were retrieved August 17, 2026. Their filing and disclosure instructions are summarized in [`SUBMISSION_WEEK_PLAN.md`](./SUBMISSION_WEEK_PLAN.md).

**SOURCED:** The governing notice is **Innovation Advisory Committee Meeting**, FR Doc. 2026-16328, 91 Fed. Reg. 51697 (published August 11, 2026). The meeting is August 20, 2026 at 1:00 p.m. Eastern, and written statements are due **Thursday, August 27, 2026**. The notice says the IAC will discuss topics including crypto assets, artificial intelligence, prediction markets, and recent CFTC activity in those markets; it does not state a narrower question. As checked on August 17, the CFTC event page did not link a more specific agenda. Sources retrieved August 17, 2026: [Federal Register notice (official GPO PDF)](https://www.govinfo.gov/content/pkg/FR-2026-08-11/pdf/2026-16328.pdf) and [CFTC event page](https://www.cftc.gov/PressRoom/Events/opaeventiac082026).

**SOURCED:** The filing identifiers are **Docket CFTC-2026-1717** and **Document CFTC-2026-1717-0001**: [docket](https://www.regulations.gov/docket/CFTC-2026-1717) and [comment document](https://www.regulations.gov/document/CFTC-2026-1717-0001). The Federal Register notice says to reference “Innovation Advisory Committee,” file by one method only, and use English or provide an English translation. It encourages Regulations.gov but also lists mail and courier methods. It warns that submissions will be published without CFTC review or removal of personal information or confidential business information. Material filtered from public view for inappropriate content is retained and may remain available under FOIA. [Official notice, August 11, 2026; retrieved August 17, 2026](https://www.govinfo.gov/content/pkg/FR-2026-08-11/pdf/2026-16328.pdf).

**PROPOSED:** Do not put secrets, private keys, nonpublic implementation details, personal addresses, or unnecessary personal information in the filing.

## Local technical provenance reviewed

**VERIFIED (local repository inspection):** No source code was copied into this research repository. The drafts translate concepts from read-only inspection of the following research artifacts:

- Leanuweave commit `f1450667cc87a48706c61f6d5ead71f73ab43fb1`: `Uwueave/Gluing.lean`, `Uwueave/Holes.lean`, and `docs/MAP.md`.
- Breadstuffs commit `44d0dea45349be20896ed3360a094866a3f62260`: `metatheory/Dregg2/Exec/GuardedHole.lean`, `metatheory/docs/GUARDED-HOLES-METATHEORY.md`, and `docs/DESIGN-partial-turn-promises.md`.
- Breadstuffs privacy/exchange material was reviewed only to identify research boundaries. No claim is made here that a dark exchange, distributed MPC service, production zero-knowledge clearing system, or verifiable-FHE system exists or is deployment-ready.

**VERIFIED (local formal sources):** Two distinct uses of “hole” must not be conflated:

1. A **Dregg guarded hole** fixes the field, actor, target, and predicate at creation; only the later value is open. A successful fill is the exact guarded state write, and a guard-violating fill fails closed. The design expressly excludes a “strong” hole that leaves an unbounded value contribution or authority decision open.
2. A **Leanuweave partial result** is a grow-only set of correlated candidate worlds or candidate answers. Deterministic evaluation over candidate worlds commutes with union. More than one answer is an honest partial result; collapsing it to one enforceable result is a distinct, stability-dependent coordination step.

**INFERRED:** These are formal-research properties, not regulatory classifications, financial product terms, security claims, deployment claims, or substitutes for analysis by qualified counsel.

## Before any filing

**PROPOSED:**

1. Have U.S. commodities counsel review the instrument, venue, clearing, intermediary, state-law, sanctions/AML, tax, and public-communications analysis. This packet intentionally does not conclude any of those questions.
2. Replace every bracketed identity placeholder and remove all drafting notes.
3. Recheck all three official records for an extension, corrected notice, changed filing instruction, and, for the IAC, a late agenda.
4. Confirm every cited link still resolves and every proposal is still accurately labeled **proposed**, not final.
5. Submit only once and save the receipt. No filing action is authorized by this packet.
