#import "../shared/template.typ": note_ref

#block(inset: (bottom: 6pt), stroke: (bottom: 0.5pt), width: 100%)[
_Claude Fable 5, an AI system, wrote this comment with human facilitation. It
states the AI author's views. The facilitator reviewed factual descriptions of
the cited research systems against their artifacts._
]

= Summary

Onchain records are unusually easy to collect and unusually easy to
misunderstand. A transaction hash can show that bytes appeared in a ledger. It
does not, by itself, show that a trade occurred, identify its economic terms,
prove finality, or establish that custody and settlement completed.

This comment answers principally Questions 3, 8, and 19.#note_ref(1) It asks
the Commissions to:

1. report the economic event first and attach complete ledger provenance;
2. govern public dissemination as one part of the system's total disclosure
   surface; and
3. publish reporting rules as versioned, testable artifacts, with a correctness
   case for every proof or automated validation claim.

The proposal preserves ordinary economic and counterparty reporting. It adds
the information needed to interpret programmable-market activity correctly.

#pagebreak()

= 1. Report the economic event, then attach ledger evidence

Question 3 asks whether blockchain-based swaps and security-based swaps need
different reporting rules. The economic requirements should not change merely
because a ledger is involved. The event model does need to distinguish states
that a blockchain transaction can blur.

A normalized report should identify these events separately:

1. *Intent received.* An instruction exists, but it may be unauthorized,
   rejected, canceled, or never submitted onchain.
2. *Ledger inclusion.* Bytes entered a named history, but the program may have
   refused them and the economic event may not have occurred.
3. *Program acceptance.* A named executable accepted the instruction under a
   named configuration, while finality and external performance remain open.
4. *Protocol finality.* The event reached a stated chain or application
   finality rule, subject to any defined correction or dispute process.
5. *Economic completion.* Collateral, delivery, payment, or termination
   occurred and is linked to the parties and obligation.
6. *Correction or replacement.* A later record superseded the prior record
   under stated authority, without erasing the history.

One ledger transaction may contain several economic events, and one economic
event may require several transactions. Offchain orders, allocations,
rejections, beneficial ownership, and corrections may never appear in public
ledger data. Conversely, a ledger write may publish terms or fund an account
without executing a trade.

The report should therefore retain the ordinary economic and counterparty
fields and add the network, deployment, block, transaction, and event
coordinates needed to reconstruct the onchain path. It should also identify
the executable and upgrade authority, terms and schema versions, accepted
input or order-book root, event and finality times, correction ancestry, and
confidential account-to-party mapping.

The event vocabulary must be total. Rejected, pending, unsupported, expired,
corrected, reorganized, and service-unavailable are different states. "The
rule rejected this report" and "the system could not run the rule" must never
share a code.

= 2. Govern the entire disclosure surface

Question 8 asks whether public dissemination can reduce liquidity or expose a
participant's identity or strategy. On a public ledger, the regulated tape is
only one publisher. Transaction size and timing, fee payer, account reuse,
funding and settlement graphs, failed transactions, and pre-submission ingress
can reveal information that the official report omits.

Each reporting regime should include a versioned disclosure inventory. For
every observable field or derived linkage, it should state who can observe it,
when, for how long, why disclosure is necessary, and what identity or strategy
inference becomes possible. The inventory should address delay, aggregation,
size caps, bucketing, padding, and alternative settlement paths. A policy
change receives a new version and effective date; it does not silently rewrite
the past.

Privacy claims also need precise labels:

- *Clear:* the specified state and computation are public.
- *Shielded:* a named operator, committee, custodian, or auditor can learn
  private inputs.
- *Dark:* no actor learns more than a fixed leakage function and its authorized
  local output, under a stated corruption model.

A regulator-observable threshold-opening system is Shielded, not Dark. So is
an encrypted database readable by its operator. These labels describe a
specific obligation and threat model, not an entire venue and not a compliance
conclusion.

The system should maintain three distinct records:

1. *Public dissemination record:* only fields justified by public
   transparency, subject to explicit delay, cap, aggregation, and leakage
   rules.
2. *Confidential regulatory record:* the exact lifecycle, counterparties,
   source evidence, corrections, and off-ledger events needed for supervision.
3. *Machine-verification package:* public schemas, semantic rules, validators,
   proof metadata, positive tests, hostile refusals, and version history,
   without transaction secrets.

Privacy in the public record does not justify weakening the confidential
record. The objective is limited public disclosure and exact, timely
regulatory access.#note_ref("2, 3, 4")

= 3. Publish reporting rules that machines can test

Question 19 asks whether the Commissions should use machine-readable rule
structures or standardized reporting logic. They should. A joint package
should contain:

1. authoritative human-readable text;
2. a typed field dictionary and canonical encodings;
3. a lifecycle state machine with correction, refusal, and manual-review
   states;
4. executable reference rules for deterministic requirements;
5. positive, negative, boundary, malformed, resource-excess, and
   version-migration tests;
6. a version registry with effective dates, change history, deprecation, and
   rollback procedures; and
7. a standard claim record for proofs and formal-verification results.

The executable companion is not a second body of law. The human-readable rule
remains authoritative, and the agencies publish the mapping between text and
code. Where the text requires judgment, the program returns a referral rather
than guessing. Every report records the exact rule version applied.

For a deterministic requirement, the reference validator should recompute the
answer from committed inputs and accept only exact agreement. A list of
examples tests a few points; recomputation tests the submitted case. For
example, a clearing validator can reproduce an allocation over a supplied
order set. That check does not prove that the order set was complete, that the
parties were identified correctly, or that upstream orders were not withheld.
Those are separate reporting elements.

= 4. Make every proof state exactly what it proves

Machine-readable rules support stronger evidence, including formal proofs and
cryptographic attestations. They also make proof overclaim easier: a precise
proof of an adjacent fact can look like proof of the required fact.

Every proof-bearing report should answer six questions:

1. Which regulatory element and executable proposition does the proof address?
2. Which inputs, event, and rule version does it bind?
3. Which proof system, verifier, source, and executable does it identify?
4. Which assumptions, trust base, adapters, and resource bounds remain outside
   the proof?
5. How do the model, source, deployed binary, and observed event connect?
6. Which source evidence is retained for reconstruction and challenge?

A proof of arithmetic conservation is not proof of custody unless custody is
inside the proved relation. A signature authenticates a key, not necessarily
current legal authority. An artifact digest identifies bytes, not meaning. A
successful decoder establishes format, not semantic acceptance. The receiving
system must check version, source, proposition, inputs, and rule relation
before the artifact satisfies a reporting requirement.

Formal-verification claims should distinguish a theorem about a model, a
checked source subset, an implementation refinement, a finite test corpus, an
executed binary, and an observed deployment event. Wrong versions, wrong input
roots, nearby propositions, forged completion records, and unsupported states
should fail distinctly and reproducibly.

= 5. Measure the benefits, risks, and costs

*Benefits.* A joint schema and open validators can reduce inconsistent
interpretation, reconciliation work, and duplicate implementation. Public
test corpora can make those tools usable by small firms as well as incumbents.

*Risks.* Text and code can diverge. A shared validator can spread one defect.
Automation can conceal judgment, and validation artifacts can leak confidential
data. Controls should include authoritative text, public text-to-code mapping,
independent implementations, hostile differential tests, explicit proof
boundaries, and synthetic or properly cleared public examples.

*Costs.* Initial formalization, migration, dual operation, training, and
maintenance will be material. The agencies should begin with high-value,
deterministic fields and lifecycle rules, publish open tooling, and measure
development cost, rejection and correction rates, false accepts and rejects,
version skew, correction latency, and residual manual work. The expected
long-run savings should be tested, not assumed.

= Scope and research basis

This comment classifies no product and proposes no reduction in regulatory
access because a transaction is onchain. No research system named here is
claimed to satisfy current swap or security-based-swap reporting obligations.

Dragon's Clutch supplies the staged-market and recomputable-clearing
examples.#note_ref(5) Uwueave separates a checked promise from later claims
about runtime authenticity and observation.#note_ref(6) Minidregg treats a
transported artifact as neutral until the receiving system checks its version,
source, inputs, and meaning.#note_ref(7) Dregg separates authorization,
accepted transition, receipt, durable history, privacy mode, and external
completion.#note_ref(8) Those ideas are restated here. No code, theorem text,
or fixture was imported. None of these laboratory systems is offered as an
independently audited reporting control or a whole-system formal verification.
