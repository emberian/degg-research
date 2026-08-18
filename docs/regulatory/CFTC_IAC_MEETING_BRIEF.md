# CFTC IAC meeting brief: guarded commitments, candidate computation, and private execution

**VERIFIED (local repository state):** Prepared August 17, 2026. This is non-counsel issue-spotting and technical policy analysis. It does not conclude that any instrument, protocol, interface, entity, or person is inside or outside CFTC jurisdiction. Nothing has been submitted, and no regulator contact, deployment, or public announcement is authorized by this document.

Claim-label convention: **VERIFIED** is directly inspected; **SOURCED** is supported by a cited authoritative primary source; **INFERRED** is a deduction rather than a legal conclusion; and **PROPOSED** is a question or recommendation. A label introducing a paragraph, list, table, or subsection governs its material claims until the next label. Current agency and legal sources were retrieved August 17, 2026.

## 1. Process and posture

**SOURCED:** The open process is the CFTC Innovation Advisory Committee (“IAC”) meeting on **August 20, 2026 at 1:00 p.m. Eastern**. The exact published topic is broad: discussion of “topics including crypto assets, artificial intelligence, and prediction markets, along with recent CFTC activity in these markets.” Written statements are due **Thursday, August 27, 2026**, must reference “Innovation Advisory Committee,” and should be filed by one method only. The docket is **CFTC-2026-1717**, document **CFTC-2026-1717-0001**. Sources: [official Federal Register notice, Aug. 11, 2026](https://www.govinfo.gov/content/pkg/FR-2026-08-11/pdf/2026-16328.pdf), [CFTC event page](https://www.cftc.gov/PressRoom/Events/opaeventiac082026), [Regulations.gov docket](https://www.regulations.gov/docket/CFTC-2026-1717), and [comment document](https://www.regulations.gov/document/CFTC-2026-1717-0001).

**VERIFIED (official-page inspection):** As checked on August 17, the event page did not link a narrower agenda.

**PROPOSED:** Do not represent a local or third-party listening guide as an official CFTC agenda.

**SOURCED:** The filing is public by design. The notice warns not to submit personal information or business information that the filer does not want posted; comments are published without Commission review or removal of personal or confidential business information. Inappropriate content may be filtered from public display, but the submission is retained and may remain available under FOIA. [Official notice, Aug. 11, 2026](https://www.govinfo.gov/content/pkg/FR-2026-08-11/pdf/2026-16328.pdf).

**SOURCED:** The IAC is an advisory body. Its charter gives it a broad innovation mandate, but policy decisions remain with the Commission. [IAC Charter, amended Mar. 3, 2026](https://www.cftc.gov/media/13366/IAC_Charter030326/download).

**INFERRED:** A comment or meeting is therefore a policy conversation, not a clearance, registration, no-action letter, or adjudication.

### Recommended posture

**PROPOSED:**

- Present an **open formal-methods research problem**, not a secretly completed venue and not a request to bless a planned launch.
- Ask how to design auditability and institutional responsibility correctly before production implementation.
- Keep four questions separate: **instrument, venue/intermediary, clearing/custody, privacy/compliance**.
- State clearly that bounded loss, formal proofs, encryption, decentralization, open source, and lack of routine operator visibility are control properties—not legal exemptions.
- Do not discuss a token address, holdings, revenue plan, mainnet date, private keys, named counterparties, unreleased cryptography, or any business fact not intended to become permanently public.

## 2. Ninety-second opening

**PROPOSED:**

> I am an independent software and formal-methods researcher studying staged programmable contingent transactions. One primitive, a guarded commitment, fixes authority, affected state, guard conditions, and a bounded value transition before a later witness can fill it. A second retains a provenance-bearing set of possible results and makes selection of one enforceable result a separately authorized finality step. A third research direction studies distinct Clear, Shielded, and Dark modes. A regulator-observable Shielded mode could keep ordinary infrastructure operators from unilaterally viewing every strategy while preserving a specified compliance and Commission-access path; a true Dark mode has no general decryption path and requires separate analysis.
>
> I am not suggesting that these properties avoid the Commodity Exchange Act, venue or clearing registration, surveillance, recordkeeping, or customer protection. I am asking where the legally relevant instrument forms across policy, signature, funding, match, selection, and settlement; which software and governance functions make a person or system the venue or intermediary; when atomic fully collateralized settlement is clearing; and what regulator-readable privacy architecture can meet the Commission’s audit and surveillance objectives. A milestone taxonomy and testable privacy-compatible audit criteria would let researchers build toward compliance instead of discovering late that a technically irreversible architecture made lawful operation impossible.

## 3. What the formal research actually establishes

### Dregg weak guarded hole

**VERIFIED (local formal sources):** At the reviewed Breadstuffs commit `44d0dea45349be20896ed3360a094866a3f62260`, the formal structure fixes `field`, `actor`, `target`, and `guard`; only the value is late. The proven successful-fill property binds the exact underlying write and guard satisfaction. A violating fill returns failure. The surrounding design explicitly rejects a “strong hole” that could leave an undetermined value contribution or retroactive authority grant.

**INFERRED:** Regulatory relevance: this could make **authorization scope and maximum transition shape** machine-checkable. It does not establish whether signing or funding the hole creates an option, swap, future, order, or no instrument; nor whether admitting a fill is execution.

### Leanuweave gluing

**VERIFIED (local formal sources):** At the reviewed Leanuweave commit `f1450667cc87a48706c61f6d5ead71f73ab43fb1`, a guarded hole consists of a fill-space predicate and an invariant over the joined state. Admissibility requires both fit and guard evidence. Under an explicit spanning hypothesis, all independently admissible divergent fills preserve the guard on merge if and only if the guard is invariant-confluent. A separate seam theorem describes systems that are coordination-free within a segment but require coordination across segments; a budget example interprets this as freedom within an allocation and coordination to reallocate.

**INFERRED:** Regulatory relevance: the model can identify **where coordination or a responsible decision point is mathematically unavoidable**. It does not identify which person is legally responsible or turn a distributed merge into a registered market.

### Leanuweave candidate-result computation

**VERIFIED (local formal sources):** The separate `Uwueave/Holes.lean` model represents partial results as grow-only sets. The image of candidate worlds under any deterministic function commutes with union. The model preserves correlations between fields; it expressly warns that independently combining per-variable candidate sets can invent impossible worlds. A multi-candidate result is an honest result. Demanding exactly one candidate is not invariant-confluent; final collapse requires a stability premise about what can still arrive. The general semantic carrier is noncomputable over an unbounded world predicate, while a finite-list representation has a computable counterpart. The current result is a join-homomorphism statement, not a general incremental evaluator or a production market implementation.

**INFERRED:** Regulatory relevance: the model exposes a **pre-final candidate phase** and makes the finality certificate explicit. It does not establish that candidate objects are legally irrelevant. Transferability, funding, binding effect, economic exposure, and interactions among participants remain decisive facts to ask about.

## 4. Milestone classification map

**SOURCED:** The CEA’s “trading facility” definition turns on multiple participants’ ability to execute or trade agreements, contracts, or transactions through open bids/offers or predetermined nondiscretionary matching, and it contains an exclusion for facilities where bids, offers, and acceptances are not binding. The DCO definition separately addresses novation, multilateral settlement/netting, and mutualization or transfer of credit risk, with specified exclusions. [7 U.S.C. § 1a(15), (16), (50), and (51), current preliminary edition](https://uscode.house.gov/view.xhtml?edition=prelim&num=0&req=granuleid%3AUSC-prelim-title7-section1a).

**PROPOSED:** The table deliberately poses questions instead of conclusions.

| Stage | Instrument question | Venue/intermediary question | Clearing/custody question | Privacy/audit record |
|---|---|---|---|---|
| Reusable policy authored | Is this only expressive software, or does it grant any enforceable right? | Who publishes, curates, markets, or recommends it? | None unless assets or obligations attach. | Version, author, terms, circuit/hash. |
| Signed, unfunded policy | Is it binding, revocable, transferable, exercisable by another, or economically priced? | Does a system receive or expose it to multiple participants? | No collateral yet; no conclusion. | Signature, owner linkage, time, terms, cancellation. |
| Funded guarded commitment | Does funding create contingent exposure or an irrevocable right? | Can others interact with or accept it? Who solicits and routes? | Who controls collateral? When and how can it be withdrawn? | Funding provenance, maximum-loss proof, owner, guard, sequence. |
| Admitted guarded fill | Is this execution, exercise, delivery of evidence, or only validation? | Who supplies/selects the witness? Is there discretion? | Does admission lock, move, or allocate collateral? | Witness commitment, guard proof, exact state delta, failure record. |
| Candidate-result set | Are candidates merely internal evaluations, or are any priced, funded, transferable, or redeemable? | Are multiple binding interests interacting through a rule? | Who maintains possible obligations pending finality? | Correlated worlds, provenance, computation/rule version. |
| Match / selection | Which rights become binding and whose interests interact? | Does predetermined matching make this a facility? Who controls rule and emergency action? | Does the system net or arrange multilateral settlement? | Inputs, priority, timing, match proof, rejected alternatives. |
| Oracle/dispute finality | Is finality part of the original contract terms? | Who controls sources, hierarchy, disputes, and corrections? | Which obligations become final? | Source provenance, observations, disputes, finality certificate. |
| Settlement / claim mint | Is the output delivery, a continuing derivative, or a new spot asset? | Who offers any secondary market? | Novation? multilateral settlement? bilateral or spot exclusion? | Payout vector, conservation, recipients, full disposition. |

## 5. Clear, Shielded, Dark, and observable variants are different postures

**SOURCED:** Current DCM regulations require automated surveillance, real-time monitoring, individual-trader and position data, reconstruction of activity, an order audit trail from receipt through fill/allocation/disposition, unalterable sequential source records, retention, and system safeguards. They also contemplate effective rule enforcement and certain authority to adjust prices or cancel trades. [17 C.F.R. pt. 38, especially §§ 38.156–.157, 38.251, 38.254, 38.256, 38.550–.552, 38.650, 38.950, and 38.1050, current through Aug. 14, 2026](https://www.ecfr.gov/current/title-17/chapter-I/part-38).

**INFERRED:** The following table maps the research privacy taxonomy to the regulatory discussion; it does not conclude compliance.

| Mode | Ordinary visibility | What is attractive | What remains hard |
|---|---|---|---|
| **Clear** | The specified state and computation are public. | Simplest independent verification and reconstruction. | Public pseudonyms may still be inadequate for owner/position surveillance; exact public order data can create strategy leakage and MEV. |
| **Shielded** | A named executor, committee, auditor, or other actor set may learn private inputs. | Conventional monitoring and exception handling can be attached to an accountable actor. | Concentrated confidentiality and abuse risk; collusion and security burden; the allowed viewers and failure behavior must be explicit. |
| **Due-process selective disclosure (Shielded)** | A named threshold can open encrypted records under a frozen authorization and receipt process. | Separates routine commercial visibility from a specified regulatory-access path. | The threshold coalition can learn disclosed plaintext; scope, bulk access, due process, key recovery, and governance must be explicit. This is not Dark. |
| **Dark** | No actor learns beyond a frozen public leakage function and its authorized local output, under an explicit corruption model. | Strongest strategy-confidentiality claim if actually proved end-to-end. | No general regulator-readable opening path. Whether fixed encrypted compliance queries can satisfy every applicable monitoring, reconstruction, correction, and enforcement obligation is unresolved and cannot be assumed. |

**SOURCED:** Regulation 38.7 separates regulatory data from marketing use by restricting business/marketing use of proprietary or personal regulatory data without consent and permitting regulatory sharing where necessary. [17 C.F.R. § 38.7, current through Aug. 14, 2026](https://www.ecfr.gov/current/title-17/chapter-I/part-38).

**INFERRED:** For a regulator-observable design, the useful target is no routine unilateral commercial visibility combined with a precise access contract. Under the research taxonomy, that is Shielded. Dark remains a separate architecture and regulatory question.

**SOURCED:** The June 25, 2026 event-contract reporting NPRM is only a proposal. Its proposed § 16.03(f)–(h) would require specified public execution data, enumerated trader-identifying information retained through the contract and at least five years after termination, and access to specified records by regulators. The Commission explains that trader-identifying information supports wash-trading, insider-trading, and cross-market surveillance. [CFTC, “Reporting Requirements for Event Contracts,” NPRM, approved June 25, 2026](https://www.cftc.gov/media/14276/EventContractsReportingNPRM062526/download). It should be treated as a live design signal, not misstated as current final law.

## 6. Minimum regulator-observable Shielded architecture

**PROPOSED:** This is an architecture for discussion, not a compliance conclusion and not a Dark design.

1. **Two data planes.** Public product and market data; confidential owner-linked regulatory data. Do not equate on-chain pseudonyms with verified beneficial ownership.
2. **Complete chronological commitments.** Append-only commitments for receipt, entry, modification, cancellation, match, allocation, rejection, and settlement, with reliable timestamps and sequencing.
3. **Regulator-readable encrypted records.** Every commitment opens to an intelligible record. Define who can compel or authorize opening, the response time, and how access is logged.
4. **Threshold key governance.** No single commercial operator should read the book unilaterally; a legally and operationally defined compliance function must be able to recover required data. Include rotation, backup, loss recovery, member replacement, compromise response, and retention beyond contract expiry.
5. **Identity and position continuity.** Cryptographically bind orders to verified customers/accounts and enable position aggregation, access controls, ownership/control analysis, and cross-market inquiry without publishing the mapping to the world.
6. **Proof-carrying controls.** Prove guard satisfaction, signature authority, access eligibility, collateral sufficiency, conservation, deterministic matching/priority, no duplicate spend, price/position bounds, valid finality source, and payout correctness. Proofs complement the underlying evidence; do not destroy it.
7. **Surveillance outputs.** Confidential execution cannot be a black box. Supply a surveillable event stream, anomaly features, selective openings, investigation queries, and replay against committed inputs.
8. **Emergency and disciplinary control.** Make halt, cancel/correct where legally authorized, withdrawal protection, circuit disabling, participant restriction, and incident containment technically real. Document who can act and how the action is reviewed.
9. **Finality provenance.** Store the source hierarchy, observation, dispute window, competing candidate results, decision rule, and finality certificate. A settlement proof without a defensible input provenance only proves faithful use of a potentially bad input.
10. **Upgrades and reproducibility.** Bind each order to exact product terms, program/circuit version, key set, fee schedule, oracle rule, and governance state. Delay and audit upgrades; preserve old verifiers and records.
11. **Independent failure testing.** Test malformed ciphertexts, proof-system soundness assumptions, committee withholding/collusion, key loss, inconsistent oracle states, censorship, replay, sequencing manipulation, chain reorganization, denial of service, and recovery.

### What zero knowledge can and cannot do

**INFERRED:** Good zero-knowledge statements are precise: “this account passed the applicable eligibility rule,” “maximum loss is fully funded,” “the selected match is the unique result of rule version H over committed inputs S,” “the payout conserves value,” or “this encrypted record opens to the order committed at sequence n.” Those statements can reduce unnecessary disclosure while improving repeatability.

**INFERRED:** Weak claims are institutional: “the market is compliant,” “there is no operator,” “the code cannot manipulate,” or “nobody can be responsible.” A proof only covers its relation, assumptions, inputs, setup, implementation, and verifier. It cannot by itself decide product classification, resolve ambiguous real-world events, perform customer support, investigate misconduct, or supply regulatory authority.

## 7. Instrument, venue, clearing, and developer questions to put on the record

**PROPOSED:** These are narrowly framed questions, not asserted conclusions.

### Instrument

1. At which milestone does a staged policy become an agreement, contract, transaction, or order: signature, funding, later guarded fill, interaction, match, claim mint, or settlement?
2. Which facts distinguish a nonbinding executable policy from a funded contingent instrument?
3. If candidate states are unpriced and nontransferable internal computations, are they distinct from candidate claims that are funded, priced, redeemable, or transferable?
4. Does fixing maximum loss and all authority except a later bounded witness affect classification, or only risk and customer-protection analysis?

### Venue and intermediary

5. Which functions—interface operation, market curation, solicitation, order receipt, message relay, matching, oracle selection, fee collection, upgrades, or emergency power—cause a person or group to maintain or provide a facility?
6. What separates publication of general-purpose source code from active operation or facilitation of a live market?
7. If deterministic matching can be executed by any node, which supervisory powers and responsibilities must remain with the registered entity?
8. How should transaction-linked interface compensation be analyzed when the interface has no custody, recommendation, or routing discretion?

CFTC Staff Letter 26-09 is a useful, limited comparison: Division of Market Oversight staff gave a fact-specific, nonbinding no-action position to an interface that sent orders directly to registered DCM collaborators, did not custody assets or provide buy/sell signals, and did not exercise routing/execution discretion. Marketing, solicitation, and transaction fees were among the reasons the request did not fit older technology-provider letters; conditions addressed disclosures, policies, records, collaborator relationships, and jurisdiction. [CFTC Staff Letter No. 26-09, Mar. 17, 2026](https://www.cftc.gov/csl/26-09/download). It is not an open-source safe harbor.

### Clearing and custody

9. Does a protocol that locks maximum loss and settles atomically without novation, credit extension, or loss mutualization nevertheless “arrange or provide” multilateral settlement under 7 U.S.C. § 1a(15)?
10. How does the analysis change when collateral is locked at commitment rather than match?
11. If output claims continue to trade, when does original clearing end and a new instrument or spot transaction begin?

Current Part 39’s fully-collateralized definition and DCO rules make clear why maximum-loss evidence is valuable, while its participant/product eligibility, risk-management, settlement, safeguards, and recordkeeping provisions show that collateral is not the whole institutional question. [17 C.F.R. pt. 39, current through Aug. 14, 2026](https://www.ecfr.gov/current/title-17/chapter-I/part-39).

### Privacy and auditability

12. Which fields must a DCM see in real time, and which may be encrypted if timely selective disclosure and reconstruction are guaranteed?
13. Can committed encrypted records plus proofs satisfy audit objectives when a regulated threshold process can open them?
14. Which compliance predicates may be proven without routinely disclosing underlying facts, and which underlying facts must nevertheless be retained?
15. What key governance and failure-recovery properties are necessary before encrypted execution can support a registered venue?

## 8. Build-in-public and third-party deployment are not loopholes

**PROPOSED:**

### Public communications

- Publishing formal specifications, threat models, benchmarks, and unresolved questions can improve the public record.
- Label every artifact accurately: concept, model, prototype, simulation, testnet, unaudited, or production. Do not say “CFTC compliant,” “approved,” “cleared,” “legal,” “trustless,” “fully verified,” or “cannot be manipulated” without a basis appropriate to that exact claim.
- Separate research-code publication from a transaction interface. A hosted UI, market list, wallet connection, fee path, oracle control, support channel, upgrade key, or solicitation campaign changes the factual posture.
- A public comment will expose the concepts and questions. Do not attach unreleased cryptographic mechanisms, exploit-relevant details, personal financial interests, or business information unless intentionally made public.
- Build-in-public messaging should invite peer review and describe uncertainty, not announce an evasion theory or imminent U.S. launch before product-specific advice.

### Third parties

**INFERRED:** Do not assume “someone else deployed immutable code” disposes of the analysis. Control, maintenance, solicitation, interface operation, compensation, custody, product creation, oracle governance, and ongoing participation remain factual questions.

**SOURCED:** Official enforcement releases illustrate fact-specific outcomes relevant to decentralization claims:

- In the [Ooki DAO case release (June 9, 2023)](https://www.cftc.gov/PressRoom/PressReleases/8715-23), the CFTC reports that a federal court held the DAO was a “person” under the CEA on the case facts and ordered shutdown measures.
- In the [Uniswap Labs order release (Sept. 4, 2024)](https://www.cftc.gov/PressRoom/PressReleases/8961-24), the CFTC focused on development/deployment of a protocol and web interface through which specified leveraged tokens were traded by non-eligible-contract participants.
- In the [Polymarket order release (Jan. 3, 2022)](https://www.cftc.gov/PressRoom/PressReleases/8478-22), the CFTC described facts including smart contracts and the creation, definition, hosting, and resolution of event markets.

**INFERRED:** Those are fact-specific enforcement outcomes, not a conclusion about publication of code in general. They are enough to reject “open source,” “DAO,” “immutable,” “noncustodial,” or “third-party deployed” as standalone clearance theories.

## 9. Engagement ladder after the IAC comment

**PROPOSED:**

1. **Public IAC statement:** ask general, non-transaction-specific classification and auditability questions. Do not request a project approval in the public docket.
2. **Counsel review:** identify the actual proposed instrument, users, collateral, matching, interface, fees, governance, oracle, settlement, and U.S. touchpoints.
3. **Innovation Task Force discussion:** the CFTC’s innovation page expressly offers industry engagement and a meeting route. A technical architecture meeting can surface which divisions and registrations are implicated. [Innovation at the CFTC, accessed Aug. 17, 2026](https://www.cftc.gov/About/Innovation).
4. **Division-specific analysis:** likely Division of Market Oversight for venue/product questions, Division of Clearing and Risk for clearing, and other Commission functions as appropriate. Do not assume the IAC or Innovation Task Force can bind them.
5. **Formal relief only on complete facts:** CFTC guidance says no-action requests and responses are generally public; a request should set out all material facts and the legal/public-policy basis, and temporary confidential treatment is limited. [CFTC “Requests for Exemptive, No-Action and Interpretative Letters,” current FAQ](https://www.cftc.gov/Transparency/relieffaqs). Do not request relief around a hypothetical architecture whose economically material features are undecided.

## 10. Pre-submission checklist

**PROPOSED:**

- [ ] Counsel has reviewed the public statement and the exact proposed factual posture.
- [ ] Identity and affiliation placeholders are complete and safe for public posting.
- [ ] No private key, home address, confidential repository detail, counterparties, unannounced funding, or unnecessary personal financial information appears.
- [ ] No proposal is misstated as a final rule.
- [ ] The docket, deadline, meeting page, and any late agenda have been rechecked.
- [ ] The comment uses “questions,” “could,” and “may” where classification is unresolved.
- [ ] The submission does not claim approval, non-jurisdiction, compliance, or a deployment right.
- [ ] The comment is filed once, by one method, in English, and the receipt is retained.
- [ ] Any build-in-public post is separately reviewed and accurately labels research maturity.

## Bottom line

**INFERRED:** The strongest proposition for a regulator-observable Shielded design is not “cryptography makes regulation unnecessary.” It is: **formal staging and cryptography can make authority, bounded exposure, order history, matching, finality, and settlement more testable while reducing unnecessary commercial disclosure, provided the architecture preserves its specified accountable access path.** A true Dark design makes a different claim and requires separate analysis. Guarded holes help specify what a later actor is allowed to change. Candidate-result computation helps specify when a result is not yet final. Neither answers the instrument, venue, or clearing question; each makes the question sharper enough for useful Commission guidance.
