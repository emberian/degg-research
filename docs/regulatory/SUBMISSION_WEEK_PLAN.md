# Regulatory submission week plan

Status: **PROPOSED** workflow prepared August 17, 2026. No filing, regulator contact, meeting request, deployment, or public announcement is authorized by this plan.

Claim-label convention: **VERIFIED** is directly inspected; **SOURCED** is supported by a cited authoritative primary source; **INFERRED** is a deduction rather than a legal conclusion; and **PROPOSED** is an action or drafting choice. A label introducing a paragraph, list, or table governs its material claims until the next label. Current sources were retrieved August 17, 2026.

## 1. Three separate records and two deadlines

**SOURCED:** Two joint CFTC/SEC requests for comment close on **Monday, August 24, 2026**:

1. **Definitions / alternative compliance:** “Joint Request for Comment on Further Definition of ‘Swap’ and ‘Security-Based Swap’ and on Alternative Compliance,” CFTC RIN **3038-AF71**, SEC File **S7-2026-21**, SEC RIN **3235-AN79**, FR Doc. **2026-12743**. [Official Federal Register page](https://www.federalregister.gov/documents/2026/06/24/2026-12743/joint-request-for-comment-on-further-definition-of-swap-and-security-based-swap-and-on-alternative) and [official GPO PDF](https://www.govinfo.gov/content/pkg/FR-2026-06-24/pdf/2026-12743.pdf).
2. **Data reporting:** “Joint Request for Comment on Swap and Security-Based Swap Data Reporting,” CFTC RIN **3038-AF70**, SEC File **S7-2026-22**, SEC RIN **3235-AN78**, FR Doc. **2026-12742**. [Official Federal Register page](https://www.federalregister.gov/documents/2026/06/24/2026-12742/joint-request-for-comment-on-swap-and-security-based-swap-data-reporting) and [official GPO PDF](https://www.govinfo.gov/content/pkg/FR-2026-06-24/pdf/2026-12742.pdf).

**SOURCED:** A separate CFTC Innovation Advisory Committee (“IAC”) record closes on **Thursday, August 27, 2026**. The public virtual meeting begins August 20, 2026 at 1:00 p.m. Eastern. The filing references are **Docket CFTC-2026-1717** and **Document CFTC-2026-1717-0001**. [Official Federal Register notice, Aug. 11, 2026](https://www.govinfo.gov/content/pkg/FR-2026-08-11/pdf/2026-16328.pdf) and [CFTC event page](https://www.cftc.gov/PressRoom/Events/opaeventiac082026).

**INFERRED:** These are not interchangeable dockets. The two August 24 comments answer exact joint CFTC/SEC questions. The August 27 statement addresses the IAC’s broader innovation discussion and may reflect what is learned at the August 20 meeting.

## 2. Draft-to-docket map

**PROPOSED:**

| Deadline | Artifact | Targeted prompt | Keep out |
|---|---|---|---|
| Aug. 24 | [`JOINT_DEFINITIONS_RFC_COMMENT_DRAFT.md`](./JOINT_DEFINITIONS_RFC_COMMENT_DRAFT.md) | Joint Definitions RFC Question 1, with related formation/automation observations | Reporting architecture detail, venue launch request, project economics |
| Aug. 24 | [`JOINT_DATA_REPORTING_RFC_COMMENT_DRAFT.md`](./JOINT_DATA_REPORTING_RFC_COMMENT_DRAFT.md) | Joint Data RFC Questions 3, 8, and 19 | Product-classification conclusion, a claim that Dark satisfies current rules |
| Aug. 27 | [`CFTC_IAC_PUBLIC_COMMENT_DRAFT.md`](./CFTC_IAC_PUBLIC_COMMENT_DRAFT.md) | Broad IAC innovation discussion; milestone taxonomy and privacy-compatible auditability | Duplicated full RFC answers, confidential implementation detail, transaction-specific approval request |
| Meeting preparation | [`CFTC_IAC_MEETING_BRIEF.md`](./CFTC_IAC_MEETING_BRIEF.md) | Internal framing, questions, architecture, public-communications discipline | Submission as-is; it is not a public comment |

**PROPOSED:** Each public draft must remain independently intelligible. Reuse the same defined concepts and factual claims, but do not paste the full background or recommendation section from one filing into another. If one filing is cited in another, cite its public docket version only after it actually exists; do not cite a local draft.

## 3. Daily sequence

### August 17: factual lock

**VERIFIED (local repository state):** Drafts exist for the three public records and an internal meeting brief. No filing has occurred.

**PROPOSED:**

- Freeze the reviewed local-source commits and paths in [`README.md`](./README.md).
- Confirm that every current regulatory claim has an official primary-source citation and retrieval date.
- Confirm the exact Clear/Shielded/Dark terminology against `../PRIVACY_MODES.md`.
- Run the repository checker and link checker; record unresolved links rather than silently deleting support.
- Search all drafts for private keys, addresses, token holdings, revenue plans, counterparties, unannounced security details, home addresses, and unnecessary personal facts.

### August 18: legal-analysis and technical review

**PROPOSED:**

- Complete and keep current the in-house legal analysis
  ([`LEGAL_ANALYSIS.md`](./LEGAL_ANALYSIS.md)): the comment-filing act, the
  instrument/venue/intermediary/clearing exposure map, the event-contract
  boundary, publication-vs-operation, and the project risk register — every
  legal citation verified against a fetched primary source with a recorded
  retrieval date. There is no retained counsel; this analysis is research
  work-product, not legal advice.
- Send the user's designated reviewer the courtesy-review packet
  ([`JOHN_REVIEW_PACKET.md`](./JOHN_REVIEW_PACKET.md)) with the exact review
  artifacts; the packet preprocesses everything so the review is a short,
  bounded favor answering the specific questions it poses.
- Ask a technical reviewer to challenge the guarded-hole and candidate-result descriptions against the frozen commits.
- Ask a privacy/security reviewer to challenge the leakage claims, disclosure governance, key recovery, finality, and failure modes.
- Resolve whether “maximum possible value movement” is actually fixed by every application described; narrow any statement that overgeneralizes the primitive.

### August 19: IAC preparation

**PROPOSED:**

- Recheck the IAC page for an official agenda or corrected notice.
- Reduce the oral opening to 90 seconds and the follow-up to three questions: formation milestone, atomic settlement/DCO boundary, and regulator-observable privacy architecture.
- Do not ask the IAC to approve a protocol or provide individualized relief.
- Prepare a contemporaneous note template separating official statements, panelist views, and personal inference.

### August 20: IAC meeting

**PROPOSED:**

- Attend or monitor the public session without contacting or representing the project unless separately authorized.
- Capture exact speaker, timestamp, and wording for material points; later verify any legal claim against an official source before relying on it.
- Record which questions the meeting actually answers and which remain open.
- Do not live-tweet claims of approval, compliance, regulator interest, or an imminent launch.

### August 21–22: revise the joint RFC comments

**PROPOSED:**

- Definitions draft: preserve its narrow answer to Question 1; make the formation criteria and contrasting examples the center.
- Data draft: preserve its narrow answers to Questions 3, 8, and 19; make the public/confidential/machine-readable separation the center.
- Incorporate IAC material only if it is authoritative, directly relevant, and accurately attributed. A panelist statement is not a Commission rule or interpretation.
- Remove repo-specific jargon that is not defined in one sentence.
- Keep “Dark” exact: no actor learns beyond the frozen leakage contract and authorized local output. A due-process decryption path is regulator-observable Shielded, not Dark.

### August 23: filing rehearsal

**PROPOSED:**

- Replace all identity/date placeholders.
- Delete every pre-filing note block, unless the user affirmatively decides the research-label note should be retained.
- Render or convert each comment to the intended submission format and visually inspect headings, tables, footnotes, URLs, and page breaks.
- Compare each title, RIN, SEC file number, and question number against the official notice.
- Recheck the deadline and submission page.
- Create final immutable copies and hashes; do not include hashes or local paths in the public filing unless useful.
- Obtain explicit approval of the exact final artifact and exact filing channel. This plan itself supplies no authority.

### August 24: joint RFC deadline

**SOURCED:** Both joint notices say comments must be received on or before August 24, 2026. Each notice provides a CFTC route and an SEC route, tells CFTC commenters to use one listed method, tells SEC commenters to use one method, and describes public posting. [Definitions notice](https://www.govinfo.gov/content/pkg/FR-2026-06-24/pdf/2026-12743.pdf); [data notice](https://www.govinfo.gov/content/pkg/FR-2026-06-24/pdf/2026-12742.pdf).

**PROPOSED:**

- Before filing, the user decides which agency route or routes to use for each joint comment, informed by [`LEGAL_ANALYSIS.md`](./LEGAL_ANALYSIS.md) §7 item 3 and the designated reviewer’s answer. Do not assume cross-posting from the notice’s silence.
- Use only the authorized method for the selected route; do not send duplicate copies through multiple channels absent an intentional, reviewed reason.
- Confirm that the Definitions filing references the exact joint title and **RIN 3038-AF71** if using the CFTC route, or **File No. S7-2026-21** if using the SEC route.
- Confirm that the Data filing references the exact joint title and **RIN 3038-AF70** if using the CFTC route, or **File No. S7-2026-22** if using the SEC route.
- Save the confirmation/receipt and the exact filed artifact. No filing is authorized until the user explicitly approves it.

### August 25–26: finalize the IAC statement

**PROPOSED:**

- Update the IAC draft with only the most relevant verified meeting information.
- Remove detailed responses already made in the two joint RFC comments; summarize and link to the public comments only if they are publicly accessible.
- Preserve the IAC statement’s distinct ask: a milestone taxonomy, privacy-compatible audit criteria, proof/control test objectives, and functional developer/interface guidance.
- Re-run the in-house legal, privacy, source, and public-disclosure review; confirm [`LEGAL_ANALYSIS.md`](./LEGAL_ANALYSIS.md) is still current for the final IAC text.

### August 27: IAC statement deadline

**SOURCED:** The IAC notice says written statements are due August 27, 2026; should reference “Innovation Advisory Committee”; should be submitted through one listed method; and will be made public under the notice’s stated conditions. [Official notice](https://www.govinfo.gov/content/pkg/FR-2026-08-11/pdf/2026-16328.pdf).

**PROPOSED:** After explicit approval, file only the final IAC artifact through one authorized method and save the receipt. Do not attach the internal meeting brief.

## 4. Shared release gate

**PROPOSED:** No draft advances to filing until every box is checked:

- [ ] Exact docket/RIN/file number and deadline reverified on the filing day.
- [ ] Identity and affiliation complete; unnecessary personal data removed.
- [ ] In-house legal analysis ([`LEGAL_ANALYSIS.md`](./LEGAL_ANALYSIS.md)) complete and current for the final text, not an earlier version, with every legal citation verified against a fetched primary source.
- [ ] Final courtesy review by the user’s designated reviewer completed on the exact final artifacts, and the reviewer’s open questions ([`JOHN_REVIEW_PACKET.md`](./JOHN_REVIEW_PACKET.md) §3) answered — silence is not sign-off.
- [ ] The user has reviewed the risk register and approved filing without retained counsel.
- [ ] Current legal claims cite official primary sources with retrieval dates.
- [ ] Proposals are labeled as proposals; staff letters are not called Commission rules; pending proposals are not called final.
- [ ] Local theorem claims match frozen source commits and are not described as product readiness.
- [ ] Clear, Shielded, Dark, and regulator-observable variants use the repository’s exact definitions.
- [ ] No “operatorless,” “approved,” “compliant,” “safe harbor,” “fully verified,” or “cannot be manipulated” claim appears without an exact supported basis.
- [ ] No private key, exploit detail, confidential business information, unpublished security mechanism, or unnecessary personal/financial fact appears.
- [ ] All placeholders and drafting notes are removed.
- [ ] Formatting, links, and repository checks pass.
- [ ] The user has approved the exact artifact and exact filing channel.

## 5. Public communication after filing

**PROPOSED:** If a build-in-public post is later authorized, link to the actual public docket artifact and describe it as a comment or research submission. Do not imply that submission means endorsement, review, approval, relief, or a meeting commitment. Say what questions were asked, identify the relevant docket, and preserve the distinction between public research, a prototype, and an operated venue.
