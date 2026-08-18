# Comment on further definition of “swap” and “security-based swap”

**Re:** Joint Request for Comment on Further Definition of “Swap” and “Security-Based Swap” and on Alternative Compliance

**CFTC:** RIN 3038-AF71

**SEC:** File No. S7-2026-21; RIN 3235-AN79

**Submitted by:** [FULL NAME]

**Affiliation:** [INDEPENDENT RESEARCHER / AFFILIATION]

**Contact:** [PUBLIC-CONTACT-SAFE EMAIL OR OTHER REQUIRED FIELD]

> **Pre-filing note — delete this block before submission.** This is non-counsel technical and policy analysis, not a legal opinion or a request for approval of a transaction, product, facility, or deployment. Complete the identity fields, obtain legal review, recheck the docket, and remove anything that should not become public. The joint request states that comments are due **August 24, 2026**. Its CFTC instructions warn that comments will be published without review or removal of personal or confidential business information; its SEC instructions say to submit only information the filer wishes to make public. [Official Federal Register document, June 24, 2026; retrieved Aug. 17, 2026](https://www.govinfo.gov/content/pkg/FR-2026-06-24/pdf/2026-12743.pdf).

> **Research-label note — retain or delete after counsel review.** **VERIFIED** means directly inspected formal or local source; **SOURCED** means supported by a cited authoritative primary source; **INFERRED** is a stated deduction rather than a legal conclusion; and **PROPOSED** is a question or recommendation. A label introducing a paragraph, list, or subsection governs its material claims until the next label.

## Summary and Question 1

**SOURCED:** The CFTC and SEC request principled, objective criteria for innovative products implicating both agencies’ interests. Question 1 specifically asks what event contracts or innovative product structures create interpretive questions; whether additional criteria should distinguish swaps, mixed swaps, security-based swaps (“SBS”), securities, and instruments excluded from “swap”; and whether new or revised rules or interpretations are warranted. The request identifies onchain automated structures and uncertainty about some event contracts as part of the problem. [Joint Request for Comment, 91 Fed. Reg. 37873, 37874–76 (June 24, 2026); retrieved Aug. 17, 2026](https://www.federalregister.gov/documents/2026/06/24/2026-12743/joint-request-for-comment-on-further-definition-of-swap-and-security-based-swap-and-on-alternative).

**VERIFIED (technical research object):** I study a general-purpose formal-programming primitive called a **weak guarded hole**. It fixes the actor, target, affected state, guard predicates, and transition shape before a later value or witness arrives. A successful late fill must be the committed state transition and satisfy the guard; a violating fill fails closed. The design deliberately excludes a “strong” hole that would leave an unbounded economic delta or authority decision to the future filler. A separate candidate-result model evaluates deterministic programs over a correlated set of possible worlds and preserves all justified candidate results until a separately evidenced stability or finality step selects one result.

**INFERRED:** These primitives sharpen—but do not resolve—the joint classification problem. A program can remain computationally incomplete while the parties have already fixed a complete economic arrangement: consideration, collateral, eligible future states, payout map, expiry, transferability, and the person allowed to submit the later witness. Conversely, the same general-purpose primitive can express a delayed document attachment, service callback, proof of completed work, or revocable simulation with no funded or tradable economic right. Classification should attach to the application’s legal and economic facts at the stage when rights and obligations arise, not to the base calculus’s software name.

**PROPOSED:** The Commissions should adopt a **milestone-and-economic-function test** for staged programmable arrangements. Guidance should identify which facts matter at each of these stages:

1. reusable policy text is authored;
2. a participant signs but retains an unconditional right to revoke;
3. consideration is paid or collateral is locked;
4. another person receives an irrevocable right or ability to act;
5. a later guarded witness is admitted;
6. multiple participants’ binding interests interact or match;
7. a finality or dispute rule selects one candidate result;
8. settlement occurs or a transferable claim is issued; and
9. any resulting claim continues to trade.

Guidance should separately analyze the **instrument**, **venue/intermediary**, and **clearing/settlement** questions. A product may present one without presenting all three at the same instant.

## Principled criteria for staged and automated products

**PROPOSED:** For Question 1, a joint interpretation or rule should ask the following objective questions about the exact application and stage:

### 1. Binding effect

- Can the author revoke the policy unconditionally before another participant acts?
- Does signature create an enforceable promise, option, privilege, claim, order, or right?
- Can another person cause execution without a new authorization from the author?
- Is the “candidate” merely an internal computation, or does a person hold a right tied to it?

### 2. Consideration and funding

- Has value been paid, escrowed, or irreversibly committed?
- Does funding cover an ordinary service obligation or create gain or loss from a contingency?
- Is maximum loss fixed and fully prefunded? If so, that is a risk fact; the Commissions should clarify whether and how it affects classification rather than treating it as an exclusion by itself.

### 3. Contingency and payout

- What future fact, event, price, security, index, commercial measure, or computation changes the parties’ rights?
- Is the payout binary, categorical, linear, path-dependent, delivery-based, or service-only?
- Is the later witness merely evidence of a previously fixed fact, or may the filler choose among economic outcomes?
- What occurs on ambiguity, non-fill, late fill, correction, fork, or dispute?

### 4. Security nexus

- Does the payoff reference, derive from, or directly concern a single security, loan, issuer, group, or index?
- Does the event directly affect an issuer’s financial statements, financial condition, or financial obligations, or does it concern a broader external event?
- Which reference, payout, and hedging characteristics should distinguish a swap, SBS, mixed swap, securities option, other security, future, or an excluded instrument?

**SOURCED:** The joint request itself identifies uncertainty concerning the SBS single-security, narrow-based-index, and event-contract prongs and asks for additional objective criteria. It also asks in Question 8 about event contracts settling by reference to a security or securities index. [91 Fed. Reg. 37875–76 (June 24, 2026); retrieved Aug. 17, 2026](https://www.govinfo.gov/content/pkg/FR-2026-06-24/pdf/2026-12743.pdf).

### 5. Transferability and standardization

- Can a right be assigned, traded, bundled, tokenized, or used as collateral?
- Is the arrangement bespoke and bilateral, or fungible and offered to multiple participants?
- Does a secondary market exist before or after final settlement?
- If transferability is not required for classification, what weight should it receive?

### 6. Who bears the obligation

- Is there a writer, seller, reserve, pool, counterparty, service provider, or protocol-defined escrow?
- Does any person promise performance, substitute credit, mutualize losses, or exercise discretion?
- Does a fully funded pool merely settle predetermined allocations, or does it assume obligations to claim holders?

### 7. Automation and system function

- Who authors product terms, curates listings, admits users, accepts commitments, supplies later witnesses, matches interests, resolves ambiguity, updates code, operates an interface, or receives transaction-linked fees?
- Which functions occur in immutable code and which remain discretionary?
- Does a general-purpose calculus merely permit an application, or does an operator actively structure and facilitate it?

**INFERRED:** These questions prevent two symmetric errors. Formal incompleteness should not hide a funded contingent claim whose economics are already fixed; formal generality should not turn every mundane delayed computation into a financial instrument.

## Candidate results require a separate formation rule

**VERIFIED (local formal sources):** In the candidate-result model, evaluation is deterministic inside each possible world. Multiple results arise because the current information admits multiple correlated worlds. Merging candidate-world sets and then computing produces the same candidate set as computing and then merging. Requiring exactly one answer is not coordination-free; a separate stability premise is needed before collapse.

**INFERRED:** For classification, the word “candidate” should carry no presumption. The relevant distinction is whether a candidate result is only an internal epistemic state or whether it has been made economically operative.

**PROPOSED:** Joint guidance should give contrasting examples:

| Configuration | Proposed question for classification |
|---|---|
| A local simulation displays several possible payouts, is unfunded, nonbinding, nontransferable, and requires a new signature to act. | Is this only software analysis until a later authorization? |
| A participant signs and fully funds a bounded policy; another participant can trigger a payout by supplying evidence of a future contingency. | At signing, funding, or trigger, when does the contingent instrument arise? |
| A service escrow pays a contractor only after proof of specified work, with no trading, fungibility, or speculative return. | Which service and delivery facts distinguish this from an event contract? |
| Candidate-result tokens are transferable or redeemable before finality. | Are the candidate claims themselves instruments even though the computation has not collapsed? |
| Candidate states remain internal; only the final settled claim exists and trades. | Which facts determine the status of the post-settlement claim and the original arrangement? |

## The underlying calculus should not be classified wholesale

**INFERRED:** A general-purpose document or proof calculus can express both mundane and financial applications. Classifying the entire calculus as one product type would be overinclusive; treating the calculus’s generality as immunity would be underinclusive. The application-level analysis should consider its frozen term set, rights, economics, reference object, and operation.

**PROPOSED:** The Commissions should distinguish:

- publication of a general-purpose language or verifier;
- publication of a reusable application template;
- creation of product-specific terms;
- operation of a live interface or matching system;
- custody or control of value;
- oracle, dispute, or upgrade control; and
- transaction-linked compensation.

This functional separation would let open technical research continue while preserving application-specific analysis when a person creates or operates a financial product or market.

## Requested joint work product

**PROPOSED:** I respectfully request a joint interpretation, concept release, or rulemaking record with:

1. a staged formation matrix covering authorship, signature, revocability, funding, binding effect, match, trigger, finality, settlement, and secondary transfer;
2. paired examples of economically similar programs that differ in the reference object or security nexus;
3. paired examples of computationally similar guarded programs that differ because one creates a funded contingent right and the other is an ordinary service or document workflow;
4. criteria for determining when candidate results remain internal computations and when they become claims;
5. a clear statement that full prefunding, fixed maximum loss, fail-closed guards, exact settlement, and formal verification are relevant risk and customer-protection facts but not automatic product-classification exclusions; and
6. a process through which researchers can present a bounded factual matrix without asking either Commission to classify an entire programming model.

## Conclusion

**INFERRED:** Guarded holes and candidate-result computation make a useful regulatory distinction visible: **technical incompleteness is not necessarily economic incompleteness**. The best objective criteria should identify when consideration, binding rights, contingent exposure, reference characteristics, and transferability become economically operative, then apply the appropriate statutory category to those facts. They should also distinguish the instrument from the persons and systems that trade, intermediate, clear, or settle it.

**PROPOSED:** Joint milestone guidance would give developers a disciplined way to design and document staged automated products without relying on labels, and would give both Commissions a common factual record for products that touch their respective interests.

Respectfully submitted,

[FULL NAME]

[AFFILIATION, IF ANY]

[DATE]
