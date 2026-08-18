# Comment on swap and security-based swap data reporting

**Re:** Joint Request for Comment on Swap and Security-Based Swap Data Reporting

**CFTC:** RIN 3038-AF70

**SEC:** File No. S7-2026-22; RIN 3235-AN78

**Submitted by:** [FULL NAME]

**Affiliation:** [INDEPENDENT RESEARCHER / AFFILIATION]

**Contact:** [PUBLIC-CONTACT-SAFE EMAIL OR OTHER REQUIRED FIELD]

> **Pre-filing note — delete this block before submission.** This is non-counsel technical and policy analysis, not a legal opinion or a request for approval of a transaction, product, facility, or deployment. Complete the identity fields, obtain legal review, recheck the docket, and remove anything that should not become public. The joint request states that comments are due **August 24, 2026**. Its CFTC instructions warn that comments will be published without review or removal of personal or confidential business information; its SEC instructions say to submit only information the filer wishes to make public. [Official Federal Register document, June 24, 2026; retrieved Aug. 17, 2026](https://www.govinfo.gov/content/pkg/FR-2026-06-24/pdf/2026-12742.pdf).

> **Research-label note — retain or delete after counsel review.** **VERIFIED** means directly inspected formal or local source; **SOURCED** means supported by a cited authoritative primary source; **INFERRED** is a stated deduction rather than a legal conclusion; and **PROPOSED** is a question or recommendation. A label introducing a paragraph, list, or subsection governs its material claims until the next label.

## Summary and questions answered

**SOURCED:** Question 3 asks whether swap and SBS transactions occurring on a blockchain require changes or guidance for reporting to and public dissemination by SDRs and SBSDRs, and whether blockchain transactions should be treated differently. Question 8 asks whether public dissemination affects liquidity or risks disclosure of participant identity or trading strategies and what mitigations are appropriate. Question 19 asks whether the Commissions should integrate machine-readable rule structures or standardized reporting logic, how they should maintain and interpret those structures, and what benefits, risks, and costs would result. [Joint Request for Comment, 91 Fed. Reg. 37877, 37879–80 (June 24, 2026); retrieved Aug. 17, 2026](https://www.federalregister.gov/documents/2026/06/24/2026-12742/joint-request-for-comment-on-swap-and-security-based-swap-data-reporting).

**INFERRED:** Blockchain records can improve provenance but should not be treated as a substitute for a reporting model. A transaction hash alone may omit beneficial ownership, product semantics, lifecycle state, offchain modifications, rejected or canceled instructions, allocation, and the facts needed to understand a reorganization or protocol upgrade. Conversely, placing every regulatory field in a public transaction can expose strategies and permit re-identification even when names are omitted.

**PROPOSED:** The Commissions should define three separate outputs:

1. a **public transparency record** containing fields justified by price discovery and market transparency;
2. a **confidential regulatory record** containing the full owner-linked lifecycle needed for supervision, examination, and enforcement; and
3. a **machine-readable validation package** containing versioned schemas, deterministic validation rules, test vectors, and provenance commitments.

This separation should be technology-neutral. Onchain systems should report equivalent economic events and satisfy equivalent quality objectives, while adding chain-specific provenance fields that make those events interpretable.

## I. Question 3: reporting for blockchain transactions

### A. Report economic events, not merely ledger writes

**PROPOSED:** The reporting unit should be a normalized lifecycle event. Depending on the applicable framework, a blockchain adapter should be able to distinguish at least:

- policy or order creation;
- signature and authorization;
- funding or collateral lock;
- acceptance or rejection;
- modification and cancellation;
- match or execution;
- allocation;
- candidate, disputed, or pending resolution state;
- finality or resolution certificate;
- settlement, transfer, termination, or correction; and
- reversal or replacement caused by a chain reorganization or application-level dispute rule.

**INFERRED:** These states are not interchangeable. A submitted transaction may fail before state change; a state commitment can remain pending; an execution can precede final settlement; and an apparent ledger observation can be reorganized. Reporting logic that maps every transaction hash directly to “trade” risks both false positives and missing lifecycle events.

### B. Chain-specific provenance fields

**PROPOSED:** In addition to the ordinary economic and counterparty fields, standardized blockchain reporting should capture, where applicable:

| Field | Purpose |
|---|---|
| Network and chain identifier | Disambiguate ledgers and forks. |
| Block/slot height and block hash | Anchor the observation. |
| Transaction identifier and intra-block index | Establish exact ordering. |
| Instruction, log, or event index | Identify the relevant state transition inside a transaction. |
| Program/contract address and immutable code hash | Identify executed logic. |
| Interface/schema version | Interpret encoded fields. |
| Product/rule identifier and terms hash | Bind the event to economic semantics. |
| Batch or accepted-input root | Bind an aggregate execution to its input set. |
| Prior and resulting state commitment | Support lifecycle continuity. |
| Finality status and finality timestamp | Distinguish observed, confirmed, and final. |
| Reorganization/replacement reference | Link superseded records without silent deletion. |
| Fee and payer fields | Separate execution economics from product value. |
| Onchain-account-to-reported-party linkage | Preserve confidential owner continuity without public identification. |

**PROPOSED:** The Commissions should specify how to report timestamps when ledger consensus exposes block time, slot time, local receipt time, execution ordering, and economic execution time that may differ. They should also specify finality and correction semantics for probabilistic or economically finalized ledgers.

### C. Commitments, encryption, and proof-carrying records

**VERIFIED (technical distinction):** This research uses the following modes. In **Clear**, specified state and computation are public. In **Shielded**, a named executor, committee, auditor, or disclosure coalition may learn private inputs. In **Dark**, under a frozen leakage function and corruption model, no actor learns beyond that leakage and its authorized local output. A due-process threshold decryption path is therefore a regulator-observable Shielded system, not Dark.

**PROPOSED:** Guidance should recognize four architectures without conflating them:

1. **Clear:** public ledger fields directly encode the transaction and lifecycle.
2. **Shielded operator-readable:** a responsible entity sees plaintext and reports it while the public sees commitments or selected results.
3. **Due-process selective disclosure (Shielded):** encrypted records can be opened by a specified threshold under a frozen authorization and audit process.
4. **Dark:** no general record-opening actor exists; only a frozen leakage function and authorized local outputs are revealed.

**INFERRED:** The first three can be evaluated against full confidential reporting and regulatory-access objectives by inspecting their data and governance paths. A Dark architecture presents a different question: whether fixed encrypted compliance queries and bounded leakage can satisfy every applicable reporting, correction, examination, and enforcement need without a general disclosure capability. This should be tested, not assumed.

**PROPOSED:** Where commitments or encryption are used, each reportable event should include a binding commitment to the complete underlying record, a versioned interpretation schema, and a defined method for correcting the regulatory record without erasing history. A zero-knowledge proof may establish properties such as valid schema, authorized signer, collateral sufficiency, deterministic matching, position-limit compliance, conservation, or consistency between public and confidential fields. The proof should complement—not replace—the retained evidence unless the Commissions expressly determine that a particular proof is the required datum.

### D. Equivalent requirements, additional guidance

**PROPOSED:** Blockchain transactions should not receive categorically less reporting merely because they are onchain. They do need additional guidance where ledger mechanics create new ambiguity: smart-contract versioning, state commitments, failed transactions, batching, finality, reorganization, cross-chain execution, bridge events, account abstraction, and pseudonymous-address linkage.

## II. Question 8: public dissemination, identity, and strategy leakage

**SOURCED:** The request notes that counterparty identity is not publicly disseminated and asks whether the dissemination frameworks nevertheless raise identity- or strategy-disclosure concerns and how to mitigate them. [91 Fed. Reg. 37879 (June 24, 2026); retrieved Aug. 17, 2026](https://www.govinfo.gov/content/pkg/FR-2026-06-24/pdf/2026-12742.pdf).

**INFERRED:** Removing a name does not necessarily remove identity or strategy leakage. A unique combination of public address, exact timestamp, size, price, product, collateral movement, bridge transfer, settlement destination, and repeated behavior can permit linkage. A low-volume or bespoke product may make a transaction recognizable from public facts alone. Immediate exact dissemination may also reveal entry, hedge, liquidation-avoidance, or inventory policy before the position can be managed.

**PROPOSED:** The Commissions should define public dissemination as a purpose-limited leakage function and empirically test each field against its transparency value and re-identification/strategy cost. The following design questions should be answered per product/liquidity class rather than by a universal assumption:

- Must exact notional be public immediately, or would caps/buckets preserve transparency for unusually large or sparse trades?
- Must precise sub-second timing be public, or would short aggregation windows reduce linkage without damaging price discovery?
- Which chain identifiers, wallet addresses, transaction hashes, and settlement links are useful to the public, and which primarily enable re-identification?
- Can economically related executions be aggregated without concealing manipulation or materially degrading price discovery?
- When a trade is corrected, canceled, or reorganized, what public linkage is necessary to prevent a misleading tape?
- Should dissemination policy account for the number of active participants, trade frequency, product bespoke-ness, and observable settlement graph?

**PROPOSED:** Any cap, bucket, delay, or aggregation should be explicit, deterministic, and subject to periodic empirical review. The full confidential regulatory record should remain timely and exact even where the public record is coarser. The Commissions should publish the policy objective, threat model, test corpus, and measured effects on liquidity, price discovery, manipulation surveillance, and re-identification risk.

### Suggested field separation

**PROPOSED:**

| Data class | Public transparency record | Confidential regulatory record |
|---|---|---|
| Product | Stable product identifier and public terms | Full terms, internal classifications, rule version |
| Execution | Time under stated precision, price, volume under stated cap/bucket | Exact receipt, ordering, execution, allocation, and correction times |
| Participant | No name or direct owner mapping | Verified owner/account/controller linkage and relevant identifiers |
| Ledger | Network and finality information needed to understand execution | Exact accounts, transaction/instruction indices, funding and settlement graph |
| Order lifecycle | Public fields justified by the applicable transparency policy | Receipt, modification, cancellation, match, allocation, rejection, disposition |
| Strategy-sensitive data | Minimized under an explicit policy | Exact data protected for regulatory use |
| Proof | Public consistency or integrity proof where useful | Proof plus underlying committed evidence and opening/validation data |

## III. Question 19: machine-readable rule structures

**PROPOSED:** The Commissions should integrate machine-readable reporting logic, but treat it as a governed specification—not as a single opaque executable or a substitute for legal text. A useful package would contain:

1. **Normative semantic model.** Define reportable economic events, states, field meanings, valid transitions, and correction history independently of any chain or vendor.
2. **Versioned schemas.** Publish machine-readable types, units, integer bounds, enumerations, optionality, identity/reference rules, and cross-field constraints.
3. **Deterministic validation logic.** Provide small, reviewable predicates for syntax and semantic consistency.
4. **Conformance vectors.** Publish valid and invalid examples, boundary values, lifecycle sequences, reorganization cases, batching, ambiguous timestamps, and corrections.
5. **Reference implementation.** Offer non-normative, reproducible code in more than one implementation where feasible; avoid making a single codebase the unstated law.
6. **Property-based and differential testing.** Require implementations to agree across randomized and adversarial corpora, with exact integers for prices, quantities, fees, and conservation-sensitive values.
7. **Semantic versioning and effective dates.** Bind each report to the exact schema and rule version; retain old versions for replay and historical interpretation.
8. **Public change process.** Publish proposed changes, rationale, examples, migration rules, and expected operational impact; provide a process for interpretation and correction.
9. **Failure taxonomy.** Standardize rejected, pending, unsupported, expired, corrected, reorganized, and backend-unavailable states so uncertainty is not silently reported as execution.
10. **Human-readable parity.** Identify the legal text controlling in a conflict and publish a mapping between each rule provision, data element, validation predicate, and test.

### Guarded and partial computations as reporting primitives

**VERIFIED (technical research object):** A weak guarded commitment fixes the authorized transition shape and accepts a later value only when its committed predicate holds. Candidate-result computation preserves multiple justified outcomes until a separate stability or finality condition permits one result.

**PROPOSED:** Machine-readable reporting logic could use these ideas narrowly:

- bind each later update to an eager report identifier, actor, field, guard/rule version, and permitted transition;
- fail closed when a late datum violates its schema or cross-field predicate;
- preserve “pending” or a set of provenance-bearing candidates rather than inventing a final value;
- require an explicit finality/correction certificate before collapsing to one final regulatory state;
- retain rejected and superseded transitions without counting them as trades; and
- prove that public dissemination is a deterministic projection of the confidential regulatory record under the applicable leakage policy.

**INFERRED:** This is most useful as a way to make state transitions and ambiguity explicit. It does not imply that a regulator must adopt the Dregg or Leanuweave formalism, and it does not make a private record complete merely because a proof verifies one predicate.

## IV. Governance, costs, and risks

**PROPOSED:** A machine-readable regime should be evaluated against at least these failure modes:

- code and legal text diverge;
- a schema change is retroactively applied to old records;
- a validator rejects economically valid transactions or accepts incomplete ones;
- vendor implementations interpret units, time, nullability, or corrections differently;
- a public field unexpectedly enables re-identification;
- a private-data key is lost, compromised, or used outside the authorization policy;
- an encrypted compliance predicate is too narrow for a later investigation;
- proof-system assumptions, circuits, or setup parameters change;
- chain finality or reorganization semantics are misunderstood; and
- the cost of maintaining multiple historical rule versions shifts disproportionately to smaller firms.

**PROPOSED:** Before mandating a machine-readable structure, the Commissions should run a public conformance pilot with reporting entities, SDRs/SBSDRs, public-interest privacy researchers, market-data users, cryptographers, and smaller implementers. The pilot should publish acceptance/rejection rates, correction frequency and latency, implementation cost, performance, disagreement cases, and privacy measurements.

## Conclusion

**INFERRED:** Onchain provenance, confidential regulatory data, public transparency, and machine-verifiable rules are compatible only when their boundaries are explicit. “Onchain” should not mean “already reported,” “public” should not mean “publish every strategy-revealing field,” “encrypted” should not mean “unavailable to every lawful process,” and “machine-readable” should not mean “an opaque program silently becomes law.”

**PROPOSED:** The Commissions should standardize normalized lifecycle events and chain provenance, preserve an exact confidential regulatory record, minimize public dissemination through a documented and empirically tested leakage policy, and publish governed machine-readable schemas with conformance vectors. They should study Dark architectures separately from regulator-observable Shielded systems and state which obligations, if any, can be met through fixed privacy-preserving compliance queries rather than a general disclosure path.

Respectfully submitted,

[FULL NAME]

[AFFILIATION, IF ANY]

[DATE]
