# Written statement for the CFTC Innovation Advisory Committee

**Re:** Innovation Advisory Committee; Docket No. CFTC-2026-1717

**Meeting:** August 20, 2026

**Submitted by:** [FULL NAME]

**Affiliation:** [INDEPENDENT RESEARCHER / AFFILIATION]

**Contact:** [PUBLIC-CONTACT-SAFE EMAIL OR OTHER REQUIRED FIELD]

> **Pre-filing note — delete this block before submission.** This is non-counsel technical and policy analysis, not a legal opinion or a request for approval of a transaction, product, facility, or deployment. Complete the identity fields, obtain legal review, recheck the docket, and remove any information that should not become permanently public. The Commission’s August 11 notice says comments are posted without review or removal of personal or confidential business information. It also says comments are due August 27, 2026, should reference “Innovation Advisory Committee,” and must be filed by only one listed method. [Official Federal Register notice, Aug. 11, 2026](https://www.govinfo.gov/content/pkg/FR-2026-08-11/pdf/2026-16328.pdf).

> **Research-label note — retain or delete after counsel review.** **VERIFIED** means directly inspected formal or local source; **SOURCED** means supported by a cited authoritative primary source; **INFERRED** is a stated deduction rather than a legal conclusion; and **PROPOSED** is a question or recommendation. A label introducing a paragraph, list, or subsection governs its material claims until the next label. Current legal and agency sources were retrieved August 17, 2026.

## Summary

**VERIFIED (technical research object):** I submit this statement as an independent software and formal-methods researcher. I am studying a class of programmable contingent transactions in which (1) a commitment can fix its authority, affected state, guard conditions, and maximum possible value movement before a later witness is supplied; (2) a computation may retain multiple provenance-bearing candidate results until a separately authorized finality event; and (3) execution can be studied in **Clear** mode (specified state and computation are public), **Shielded** mode (a named executor, committee, auditor, or disclosure coalition may learn private inputs), or **Dark** mode (within a stated corruption model, no actor learns beyond a frozen leakage function and its authorized local output).

**PROPOSED (scope):** This statement does **not** contend that these technical properties remove a transaction from the Commodity Exchange Act (“CEA”), eliminate the functions of a trading venue or clearing organization, or excuse surveillance, recordkeeping, customer protection, or access by the Commission. It asks the Commission to clarify *when* the legally relevant instrument, venue, clearing, and intermediary functions arise in a staged program, and *how* privacy-preserving systems can satisfy those functions without making commercially sensitive orders and strategies public to every participant or infrastructure operator.

**SOURCED:** The Commission’s current public description explains that “event contract” is not itself a defined CEA or CFTC-regulation term; an event contract may fall within the statutory definitions of a swap or a futures contract, and a prediction market offering swaps or futures to the general public must register as a designated contract market (“DCM”). It also explains that swap execution facility (“SEF”) participation is limited to eligible contract participants.[^anprm]

**INFERRED:** Accordingly, the proper analysis should begin with the economic terms and functions at each stage, not with labels such as “prediction,” “policy,” “token,” “smart contract,” “Dark,” or “decentralized.”

**PROPOSED:** I recommend that the Commission develop:

1. a **milestone-based functional taxonomy** for staged programmable contingent transactions;
2. a **privacy-compatible auditability framework** that distinguishes public market transparency from confidential, regulator-accessible regulatory data;
3. **testable control objectives** for proof-carrying matching and settlement; and
4. **functional-factor guidance for software developers and interfaces**, including what conduct changes publication of general-purpose code into operation, solicitation, order handling, intermediation, or another regulated function.

## I. The technical classification problem

### A. “Guarded holes”: late evidence inside an eagerly fixed authority envelope

**VERIFIED (local formal sources):** “Guarded hole” is research vocabulary, not a proposed legal category. In the Dregg prototype, a weak guarded hole fixes four things when created: the field to be written, the actor authorized to fill it, the target affected, and a list of guard predicates. Only the value is supplied later. A successful fill must be exactly the specified state transition and must satisfy every guard; a violating value fails closed. A deliberately excluded “strong hole” would leave the shape of the value transfer or authority itself open until later.

**INFERRED:** For regulatory analysis, the intended distinction is between:

- an **authored policy**, which may be only software text;
- a **signed but unfunded instruction**, which may or may not be binding under its terms;
- a **funded guarded commitment**, which escrows a bounded maximum loss and fixes authority and transition constraints;
- an **admitted fill**, where a later witness satisfies the guard;
- a **match or selection**, where multiple participants’ commitments interact; and
- **final settlement**, where balances or claims become final.

**INFERRED:** The technical discipline may improve customer protection by preventing an executor from inventing a new recipient, increasing the committed loss, changing the transition class, or accepting a value outside the guard. It does not answer whether a binding “agreement, contract, or transaction” already exists at signing or funding, whether the resulting economic exposure is a swap, option, future, or spot transaction, or whether a multi-participant system is a trading facility. Those are the classification questions on which guidance is requested.

### B. Partial computation over candidate results: preserving uncertainty before a priced collapse

**VERIFIED (local formal sources):** Leanuweave explores a separate concept: a partial result is a grow-only set of correlated candidate worlds or candidate answers. A deterministic function can be evaluated over every candidate world; the resulting candidate set commutes with union. This permits computation to continue without falsely selecting one result before the information needed for finality exists. A multiple-candidate answer is not treated as an execution failure. Converting it into one enforceable result is an explicit operation that requires a reason to believe later admissible information cannot invalidate the selection.

**INFERRED:** In market terms, candidate computation could model a rule that evaluates several possible oracle states, dispute outcomes, allocation states, or admissible matches before a finality certificate selects the settlement state. It could also allow a participant to inspect possible consequences of a policy without yet creating a funded or binding order. But the distinction is functional, not semantic magic: if pre-final candidate objects are transferable, funded, priced, redeemable, or otherwise create economic rights, they may themselves require instrument analysis. If a system makes multiple participants’ binding interests interact through a predetermined matching rule, calling the pre-settlement states “candidates” does not by itself answer the trading-facility question.

### C. Four questions should be kept separate

**SOURCED:** The CEA separately defines “trading facility” by reference to a system in which multiple participants can execute or trade agreements, contracts, or transactions through open bids or offers or through interaction in a predetermined, nondiscretionary matching algorithm. The definition excludes, among other things, facilities on which bids, offers, and acceptances are not binding.[^definitions]

**INFERRED:** That makes binding effect and system function central questions for a staged design.

**SOURCED:** The CEA separately defines a derivatives clearing organization (“DCO”) to include a clearing entity or system that substitutes its credit, arranges or provides multilateral settlement or netting, or otherwise provides arrangements that mutualize or transfer credit risk. The definition also contains exclusions, including bilateral settlement without a central counterparty and settlement of spot commodity sales.[^definitions]

**INFERRED:** Therefore atomic settlement, lack of credit extension, and full collateralization are important facts, but they should not be treated as self-executing answers to whether the system “arranges or provides” multilateral settlement.

**PROPOSED:** I suggest analyzing four axes independently:

1. **Instrument:** What enforceable economic rights and obligations exist, when do they arise, on what contingency do they depend, and what is delivered?
2. **Venue and intermediary:** Who or what solicits users, receives or routes orders, lets multiple interests interact, chooses or applies the matching rule, operates the interface, exercises discretion, receives transaction-linked compensation, or has continuing control?
3. **Clearing and custody:** Who holds collateral, becomes or does not become a counterparty, nets or settles obligations, handles defaults, and controls withdrawal or final settlement?
4. **Privacy and compliance:** Which data are public, which are visible to the regulated operator, which are encrypted, what must be reconstructible, and who can disclose required data to the Commission?

## II. Narrow classification questions for Commission consideration

**PROPOSED:** The questions in this section are requests for general guidance, not asserted legal conclusions.

### A. Instrument formation and the candidate-result boundary

1. For a staged program, which facts are most important in determining the point at which an “agreement, contract, or transaction” exists: publication of a reusable policy, signature, funding, admission of a later guarded witness, interaction with another participant, minting of a claim, or final settlement?
2. Does a policy that is nonbinding, unfunded, revocable, and incapable of execution without a later signed authorization remain outside the instrument until authorization? Which changes—escrowing funds, granting an irrevocable option, creating a transferable interest, or making the policy executable by another participant—alter that analysis?
3. If a funded commitment fixes its maximum loss, collateral, transition class, guard, and authorized filler but leaves a bounded value or external fact for later proof, which stage creates the contingent economic exposure?
4. If candidate computations are neither transferable nor independently funded and merely describe possible results, are they analytically different from priced or redeemable candidate claims? What economic features would make a candidate state itself relevant to classification?
5. When an outcome token or other claim is minted only after an oracle rule and dispute process have produced final settlement, what facts determine whether that token is delivery of the original instrument, a continuing derivative, or a separate spot asset?

### B. Venue, software, and intermediary functions

6. Which combination of functions causes a person or system to constitute, maintain, or provide a trading facility: maintaining code, controlling upgrades, operating a graphical interface, publishing markets, soliciting users, receiving signed commitments, routing messages, matching, selecting among candidate results, setting oracle or dispute rules, taking transaction-linked fees, or exercising emergency control?
7. How should the analysis distinguish publication of general-purpose open-source protocol code from operation of a live interface or system that invites, structures, and transmits customer transactions?
8. If anyone can independently execute a deterministic matching program and verify its proof, what governance, surveillance, disciplinary, and emergency powers must still be attributable to a DCM or other registered entity?
9. Does an interface that transmits user instructions directly to a registered venue, never controls funds, does not exercise routing or execution discretion, and provides no personalized trade signals require separate treatment when it markets the service or receives transaction-linked fees?

**SOURCED:** In CFTC Staff Letter No. 26-09, Division of Market Oversight staff provided a fact-specific, nonbinding no-action position for a front end that transmitted orders to registered DCM collaborators without custody, trade recommendations, or routing discretion. Staff nevertheless addressed facts including marketing, solicitation, and transaction fees and imposed conditions involving disclosures, records, collaborator relationships, policies, and jurisdiction.[^phantom]

**INFERRED:** The letter is useful evidence that the analysis is functional and fact-dependent; it is not a general exemption for software or interfaces.

### C. Clearing, collateral, and settlement

10. If a program requires proof of sufficient collateral for the maximum possible loss, locks that collateral, settles atomically, does not novate obligations, does not extend credit, and does not mutualize losses, what additional facts determine whether it nevertheless arranges or provides multilateral settlement within the DCO definition?
11. Does the answer differ if collateral is locked when a participant signs a guarded commitment, only when a match occurs, or only immediately before final settlement?
12. Which proof artifacts—conservation, full-collateralization, no-double-spend, segregation, valid payout vector, and settlement-source provenance—would be useful to a DCO or the Commission, even though proof of those facts cannot replace any required registration, governance, financial resources, or risk-management functions?

**SOURCED:** Part 39 currently defines a fully collateralized position by reference to the DCO holding, at all times, funds sufficient to cover the maximum possible loss and imposes requirements concerning participant and product eligibility, risk management, settlement, system safeguards, and records.[^part39]

**INFERRED:** Those requirements suggest that bounded-loss proofs can be valuable controls while leaving the institutional clearing analysis intact.

### D. Privacy, surveillance, and regulator-readable auditability

**SOURCED:** Current DCM rules require, among other things, automated surveillance capable of detecting anomalies and reconstructing activity; real-time monitoring; individual-trader and position data; records that permit reconstruction of all trading; customer-order tracking from receipt through fill, allocation, or disposition; unalterable sequential source records; and retention.[^part38]

**INFERRED:** A candidate architecture for a registered DCM therefore should not assume that a public transaction hash alone satisfies those functions. Under the research taxonomy, a true **Dark** system has no general decryption path. A design with due-process threshold disclosure is an explicitly regulator-observable **Shielded** variant, not Dark.

**SOURCED:** At the same time, current regulation recognizes that regulatory data and public commercial data need not be identical. Regulation 38.7 restricts a DCM’s use of proprietary and personal information collected for regulatory purposes for marketing absent consent and permits regulatory sharing with other DCMs and SEFs where necessary.[^part38data]

**INFERRED:** This separation could support a regulator-observable Shielded design: public price and volume data, protected participant and order data, and regulator-accessible audit information.

**SOURCED:** The Commission’s June 25, 2026 event-contract reporting proposal is not final. As proposed, it would require certain public execution data and would require a DCM to obtain specified identifying information for every trader and retain it through the contract’s life and for at least five years afterward; it explains that identifying information supports wash-trading, insider-trading, and cross-market surveillance. The proposal would also require relevant DCM and DCO records to remain open to specified regulators.[^reportingproposal]

**INFERRED:** Whether or not those provisions are adopted, they illustrate the concrete data-access question that privacy-preserving systems must answer.

I therefore ask:

13. Which order, identity, ownership, funding, position, timing, modification, cancellation, allocation, and settlement fields must a DCM be able to read in real time, and which may be encrypted if they remain timely retrievable and intelligible to the DCM and Commission?
14. Can an append-only sequence of commitments, encrypted owner-linked records, and zero-knowledge proofs satisfy audit-trail objectives if an authorized regulatory process can recover the underlying records and link them across accounts and markets?
15. Can a regulated entity use due-process threshold disclosure so that no single commercial operator routinely sees plaintext, while a defined compliance function and the Commission can obtain lawful, complete, timely disclosure? This would be a regulator-observable Shielded modality, not Dark. What key-recovery, rotation, retention, access-logging, and incident-response requirements would be necessary?
16. Which facts may appropriately be proven in zero knowledge—for example, identity eligibility, sanctions-screening status, access limits, position limits, sufficient collateral, conservation, rule-conforming matching, no double spend, or valid payout—while the underlying evidence remains available for authorized examination?
17. How can encrypted execution preserve a DCM’s ability to halt trading, cancel or correct transactions where authorized, investigate manipulation, discipline participants, and reconstruct the exact order lifecycle?
18. Would the Commission consider a registered-entity pilot or safe harbor focused on **privacy-compatible audit trails**, conditioned on regulator-readable data, independent security assessment, test vectors, key governance, incident reporting, and demonstrated surveillance rather than on permanent opacity?

## III. A concrete compliance-oriented technical architecture

**PROPOSED:** The following regulator-observable **Shielded** design is offered as a discussion target, not as a claim of present compliance or production readiness:

1. **Public market layer.** Publish product terms, resolution source and hierarchy, rule and circuit identifiers, settlement commitments, and whatever execution data must be public.
2. **Confidential regulatory layer.** Retain encrypted, owner-linked order and identity records with timestamps, sequence numbers, modifications, cancellations, matches, allocations, funding provenance, and settlement disposition.
3. **Threshold regulatory disclosure.** Separate ordinary commercial visibility from authorized disclosure; prevent one infrastructure operator from unilaterally reading all orders, but ensure a defined regulated function and the Commission can recover complete records.
4. **Proof-carrying execution.** Prove that accepted fills satisfy their precommitted guards; signatures and nullifiers are valid; collateral covers maximum loss; matching follows the disclosed rule; no order or collateral is reused; outputs conserve value; and settlement uses the specified finality certificate.
5. **Explicit finality.** Preserve candidate results until an identified oracle, dispute, causal, or governance rule supplies the evidence that licenses one settlement result. Record alternatives and the exact basis for selection.
6. **Operational control.** Maintain technically effective halt, correction, cancellation, position-control, investigation, and recovery mechanisms appropriate to the registered entity’s obligations.
7. **Verifiable governance.** Identify who can alter product terms, matching logic, circuits, keys, interfaces, fees, oracle rules, or emergency controls; make changes delayed, auditable, and attributable.

**INFERRED:** Zero-knowledge proofs are strongest where they make compliance claims *more testable*. They are weakest when used to argue that no responsible entity can obtain records, supervise a market, or respond to an emergency. A regulator-observable Shielded design can reduce unnecessary disclosure to the public and commercial operators while preserving a specified access contract. Whether a true Dark design can satisfy current requirements without a decryption path is a separate classification and compliance question, not an assumed result.

## IV. Requested policy work

**PROPOSED:** I respectfully recommend five work products:

### 1. A milestone taxonomy

Publish a request for comment or interpretive framework that maps authored policy, signature, funding, binding effect, multi-participant interaction, match, claim creation, oracle finality, and settlement to the instrument, venue, intermediary, and clearing questions. The framework should make clear that fixed maximum loss and proof-carrying settlement are risk controls, not automatic jurisdictional exclusions.

### 2. Privacy-compatible audit-trail criteria

State which data must be public, visible to the regulated operator in real time, recoverable on demand, linked across positions, and retained. Invite architectures that meet those objectives with encryption, threshold disclosure, and zero-knowledge proofs.

### 3. Proof and test objectives

Provide machine-testable examples for collateral sufficiency, conservation, deterministic matching, order priority, cancellation races, duplicate prevention, settlement-source validity, position/access limits, and complete audit reconstruction. A formal proof should complement—not replace—governance, operations, and examination.

### 4. Functional guidance for developers and interfaces

Clarify the significance of upgrade control, frontend operation, solicitation, order receipt, routing discretion, market creation, oracle control, transaction-linked compensation, custody, and continuing maintenance. General-purpose publication, deployed protocol operation, and an interface that actively facilitates transactions should not be collapsed into one undifferentiated category.

### 5. A coordinated innovation path

**SOURCED:** The Commission describes the IAC, written input, industry engagement, and its Innovation Task Force as ways it develops insight for policy, interpretation, and rulemaking.[^innovation] The IAC charter makes the Committee advisory.[^charter]

**INFERRED:** A public comment does not constitute Commission clearance.

**PROPOSED:** A useful path would let technical researchers first present a non-transaction-specific architecture, then work with counsel and the relevant operating divisions on product-, venue-, and clearing-specific questions, and finally seek any registration, exemptive, or no-action process that the actual facts require.

## Conclusion

**INFERRED:** Guarded commitments and candidate-result computation offer a disciplined way to separate policy authorship, bounded authorization, execution, selection, and settlement. Privacy-preserving proof systems offer a disciplined way to separate public transparency, ordinary commercial confidentiality, and regulatory access. None is a jurisdictional shortcut. Their public value is that they can expose the legally and operationally important milestones, prevent unauthorized state transitions, retain provenance, and make compliance properties testable.

**PROPOSED:** The central policy question for a regulator-observable venue is whether a market can minimize unnecessary disclosure while remaining fully reconstructible, surveillable, governable, and accountable to the Commission. A separate question is whether a true Dark system, whose frozen leakage contract has no general decryption path, can satisfy the applicable obligations. Clear functional milestones and privacy-compatible control objectives would allow researchers and registered entities to design toward the relevant standard from the first line of code.

Respectfully submitted,

[FULL NAME]

[AFFILIATION, IF ANY]

[DATE]

---

[^anprm]: **SOURCED:** Commodity Futures Trading Commission, [“Prediction Markets,” Advance Notice of Proposed Rulemaking, 91 Fed. Reg. 12516](https://www.cftc.gov/LawRegulation/FederalRegister/proposedrules/2026-05105.html) (published Mar. 16, 2026; retrieved Aug. 17, 2026), including its explanation that “event contract” is not a defined CEA/CFTC term, that such contracts may be swaps or futures, that a general-public venue for swaps or futures must be a DCM, and that SEFs are limited to eligible contract participants. The ANPRM’s own comment period closed Apr. 30, 2026; it is cited for the Commission’s public analysis, not as the open docket for this statement.

[^definitions]: **SOURCED:** 7 U.S.C. § 1a(15), (16), (50), and (51), [Office of the Law Revision Counsel, current preliminary edition](https://uscode.house.gov/view.xhtml?edition=prelim&num=0&req=granuleid%3AUSC-prelim-title7-section1a) (definitions of derivatives clearing organization, electronic trading facility, swap execution facility, and trading facility; retrieved Aug. 17, 2026). See also 7 U.S.C. § 7, [DCM core principles](https://uscode.house.gov/view.xhtml?edition=prelim&num=0&req=granuleid%3AUSC-prelim-title7-section7); 7 U.S.C. § 7a-1, [DCO registration and core principles](https://uscode.house.gov/view.xhtml?req=%28title%3A7+section%3A7a-1+edition%3Aprelim%29); and 7 U.S.C. § 7b-3, [SEF registration and core principles](https://uscode.house.gov/view.xhtml?edition=prelim&num=0&req=granuleid%3AUSC-prelim-title7-section7b-3).

[^phantom]: **SOURCED:** CFTC Division of Market Oversight, [Staff Letter No. 26-09](https://www.cftc.gov/csl/26-09/download) (Mar. 17, 2026; retrieved Aug. 17, 2026). The letter states a staff no-action position based on the described facts and conditions and says it does not bind the Commission or other divisions.

[^part39]: **SOURCED:** 17 C.F.R. pt. 39, especially §§ 39.2, 39.12–.14, 39.18, and 39.20, [current eCFR](https://www.ecfr.gov/current/title-17/chapter-I/part-39) (current through Aug. 14, 2026; retrieved Aug. 17, 2026).

[^part38]: **SOURCED:** 17 C.F.R. pt. 38, especially §§ 38.156–.157, 38.251, 38.254, 38.256, 38.550–.552, 38.650, 38.950, and 38.1050, [current eCFR](https://www.ecfr.gov/current/title-17/chapter-I/part-38) (current through Aug. 14, 2026; retrieved Aug. 17, 2026).

[^part38data]: **SOURCED:** 17 C.F.R. § 38.7, [current eCFR Part 38](https://www.ecfr.gov/current/title-17/chapter-I/part-38) (current through Aug. 14, 2026; retrieved Aug. 17, 2026).

[^reportingproposal]: **SOURCED:** Commodity Futures Trading Commission, [“Reporting Requirements for Event Contracts,” Notice of Proposed Rulemaking](https://www.cftc.gov/media/14276/EventContractsReportingNPRM062526/download) (approved June 25, 2026; retrieved Aug. 17, 2026), proposed 17 C.F.R. § 16.03(f)–(h). This is a proposal, not a final rule.

[^innovation]: **SOURCED:** Commodity Futures Trading Commission, [“Innovation at the CFTC”](https://www.cftc.gov/About/Innovation) (retrieved Aug. 17, 2026).

[^charter]: **SOURCED:** Commodity Futures Trading Commission, [Innovation Advisory Committee Charter](https://www.cftc.gov/media/13366/IAC_Charter030326/download) (amended Mar. 3, 2026; retrieved Aug. 17, 2026).
