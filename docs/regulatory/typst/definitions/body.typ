#import "../shared/template.typ": key_point, note_ref

= Executive summary

*SOURCED:* The Commissions ask for principled, objective criteria for event contracts and
other innovative product structures that may implicate both agencies' interests.
Question 1 asks how to distinguish swaps, mixed swaps, security-based swaps,
securities, and instruments excluded from the definition of swap, and whether
new or revised rules or interpretations are warranted.#note_ref(1) This comment
responds principally to Question 1 and does not take a position on Questions 12
through 15 concerning alternative compliance.

*VERIFIED (local research object):* This comment responds from the perspective of software and formal-methods
research on staged programmable commitments. The research includes two useful
technical objects:

- a *guarded commitment*, in which authority, affected state, guard conditions,
  and the permitted transition shape are fixed before a later witness or value
  is supplied, subject to the committed guards; and
- a *candidate-result computation*, in which several provenance-bearing possible
  results remain explicit. Determinacy requires a separately supplied stability
  or coordination premise; the model does not implement or validate an oracle,
  legal finality process, or enforceable selection.

*INFERRED:* Neither object is offered as a legal category. Both expose a classification
problem: a program can remain computationally incomplete after the parties have
fixed a complete economic arrangement, while the same programming primitive can
also express an ordinary service callback, document workflow, or unfunded local
simulation.

#key_point("Principal recommendation", [
  *PROPOSED:* Use a milestone-and-economic-function matrix to organize facts
  within the existing statutory definitions and joint interpretations. Determine when binding
  rights, consideration, contingent exposure, transferability, and system
  functions arise. Analyze the instrument, venue, intermediary, and clearing or
  settlement functions separately. Do not classify an entire programming
  calculus from its software label, and do not treat formal generality as an
  exemption for a financial application. No listed factor is proposed as
  independently necessary or sufficient unless the governing law makes it so.#note_ref(3)
])

*PROPOSED:* The proposed analysis has five elements:

1. identify the exact stage being classified;
2. identify the enforceable economic rights at that stage;
3. identify the reference object and payout or delivery rule;
4. identify who performs venue, intermediary, custody, and settlement functions;
5. treat full prefunding, fixed loss bounds, fail-closed guards, and formal proofs
   as potentially relevant risk controls to the extent correctly specified,
   soundly proved, correctly implemented, and bound to settlement, not as
   automatic classification exclusions.

= The staged formation problem

== Guarded commitments

*VERIFIED (local research object):* In the inspected research design, a weak guarded commitment fixes the actor,
target, affected field, guard predicates, and permitted state transition at
creation. Only the value or witness arrives later, subject to the precommitted
guard. A successful fill must
be exactly the committed transition and satisfy every guard. A violating fill
fails closed. A deliberately excluded stronger construction would leave an
unbounded value delta or authority decision to the future filler.

*INFERRED:* This structure can prevent an executor from changing a recipient,
changing the transition class, or accepting a value outside the guard. It can
enforce a value or maximum-loss bound only when an application's eager terms and
guard encode that bound and the implementation correctly enforces it. Those can
be useful control properties. They do not decide when an
"agreement, contract, or transaction" exists or whether its economic exposure
is a swap, option, future, security, service arrangement, or spot transaction.

For classification, the same control structure can appear at materially
different stages:

#table(
  columns: (1.15in, 1.55in, 1fr),
  table.header([*Stage*], [*Possible legal or economic fact*], [*Question requiring guidance*]),
  [Authored policy], [Reusable software text], [Is any participant bound, funded, or able to cause execution?],
  [Signed instruction], [Authenticated but possibly revocable direction], [Does signature create a promise, order, option, privilege, or claim?],
  [Funded commitment], [Collateral or consideration is locked], [Has contingent exposure become economically operative?],
  [Early exit, compression, or unwind], [A right is canceled, netted, transferred, or closed before maturity], [Does the transition terminate an existing instrument, create another one, or reintroduce discretion?],
  [Admitted witness], [A guard accepts later evidence], [Is the witness evidence of a fixed fact or a choice among economic outcomes?],
  [Interaction or match], [Several participants' binding interests meet], [Does a trading-facility or intermediary function arise here?],
  [Finality], [One candidate result becomes authorized], [What source, dispute rule, or stability premise licenses selection?],
  [Settlement or issuance], [Balances move or claims are delivered], [Is this performance of an earlier instrument or creation of another one?],
  [Secondary transfer], [A resulting claim continues to trade], [What is the status of the continuing claim and its venue?],
)

== Candidate results

*VERIFIED (local research object):* Candidate-result computation retains a set
of correlated possible worlds or answers. For deterministic functions over
candidate-world sets merged by union, evaluation commutes with that union. More
than one result is an honest partial state, not an execution failure. A
determinate or authoritative result requires a separately supplied stability or
coordination premise. The model does not itself provide or validate that premise.

*INFERRED:* The term "candidate" should carry no regulatory presumption. A candidate result
may be an internal epistemic object with no funding, transferability, or binding
effect. It may instead be priced, redeemable, transferable, or linked to an
irrevocable funded right. Objective guidance should identify the economic facts
that cause the latter state to become an instrument, rather than relying on the
software's description of the state as partial.

#key_point("Core distinction", [
  Technical incompleteness is not necessarily economic incompleteness. The
  application may already fix consideration, collateral, eligible future states,
  payout, expiry, and execution authority even though a later witness remains
  unknown. Conversely, an incomplete computation can remain nonbinding,
  unfunded, nontransferable, and incapable of action without a new authorization.
])

= Proposed objective criteria

== 1. Binding effect

*PROPOSED:* The analysis should identify whether the author can revoke unconditionally;
whether another person can cause execution without a new authorization; and
whether signature, funding, or admission creates an enforceable promise, order,
option, privilege, claim, or other right. An internal computation and a right
held by another person should not be treated as the same state merely because
both are represented by program data.

== 2. Consideration and funding

*PROPOSED:* The analysis should identify whether value has been paid, escrowed, or
irreversibly committed; whether that funding pays for an ordinary service or
creates gain or loss from a contingency; and whether maximum loss is fixed and
fully prefunded. Prefunding can materially change credit and customer-protection
risk, but should not be assumed to decide the product category by itself.

== 3. Contingency, reference, and payout

*PROPOSED:* The analysis should identify the future fact, event, price, security, index,
commercial measure, or computation affecting rights. It should state whether
the payoff is binary, categorical, linear, path-dependent, delivery-based, or
service-only; distinguish evidence of a fixed fact from discretion over the
economic result; and specify ambiguity, non-fill, correction, fork, dispute,
and expiry behavior.

== 4. Security nexus

*PROPOSED:* The analysis should identify whether a payoff references or directly concerns a
single security, loan, issuer, group, or index; whether an event directly affects
an issuer's financial statements, financial condition, or obligations; and
which reference, payout, and hedging characteristics distinguish a swap,
security-based swap, mixed swap, an option on a security or group or index of
securities subject to the Securities Act and Exchange Act, other security,
futures contract, or excluded instrument. The joint request itself recognizes
these unresolved boundaries.#note_ref(1)#note_ref(4)

== 5. Transferability and standardization

*PROPOSED:* The analysis should identify whether a right is assignable, fungible, tokenized,
tradable, bundled, or usable as collateral; whether it is bespoke and bilateral
or offered to multiple participants; and whether a secondary market exists
before or after settlement. Guidance should state the weight assigned to these
facts even where transferability is not a necessary element.

== 6. Obligation and risk bearer

*PROPOSED:* The analysis should identify the writer, seller, reserve, pool, counterparty,
service provider, or escrow; whether any person promises performance,
substitutes credit, mutualizes losses, or exercises discretion; and whether a
fully funded pool merely executes predetermined allocations or assumes an
obligation to claim holders.

== 7. Automation and system function

*PROPOSED:* The analysis should identify who authors product terms, curates listings, admits
users, receives commitments, supplies witnesses, matches interests, resolves
ambiguity, updates code, operates an interface, or receives transaction-linked
compensation. Immutable code can make a rule deterministic, but it does not make
the functions performed by that rule disappear.

= Paired examples

*PROPOSED:* Paired examples would be more useful than abstract labels because they can hold
the computational form constant while changing the economic facts.

#table(
  columns: (1.2in, 1.7in, 1fr),
  table.header([*Configuration*], [*Facts*], [*Question for joint guidance*]),
  [Local simulation], [Unfunded, nonbinding, nontransferable, revocable; a new signature is required to act], [Does an instrument exist before the later authorization?],
  [Funded guarded policy], [Maximum loss, payout class, guard, and filler are fixed; later evidence can trigger value movement], [At signature, funding, or trigger, when does contingent exposure arise?],
  [Service escrow], [Payment follows proof of specified work; no fungibility, trading, or speculative return], [Which service and delivery facts distinguish this from an event contract?],
  [Transferable candidate claim], [A candidate state is independently priced or redeemable before finality], [Is the candidate claim itself an instrument?],
  [Internal candidates], [Candidate states remain internal; only a final settled claim can be held], [What is the status of the original arrangement and the later claim?],
  [Same program, different reference], [One application refers to a commodity price; another to a single issuer or security], [Which objective reference and nexus facts allocate jurisdiction?],
)

= Separate the instrument from the system

*SOURCED:* The CEA separately defines trading-facility, swap-execution-facility, and
derivatives-clearing-organization functions.#note_ref(2) A staged application may
present an instrument question before it presents a venue or clearing question,
or may present all three at different milestones. These are contextual downstream
questions; this comment does not ask the product-definitions proceeding to decide
the registration status of any facility. *INFERRED:* Joint guidance should therefore
use separate findings for:

- *instrument*: enforceable rights, formation, contingency, reference, payout,
  delivery, and transferability;
- *venue*: participant interaction, bids or offers, matching, interface or system
  operation, and continuing control;
- *intermediary*: solicitation, order receipt, transmission, routing, discretion,
  recommendations, and compensation; and
- *clearing and settlement*: collateral custody, novation or absence of novation,
  multilateral netting, credit substitution, loss mutualization, default
  handling, and final settlement.

The same functional separation should apply to software activity. Publication
of a general-purpose language, publication of an application template, creation
of product-specific terms, deployment of a live matching system, interface
operation, custody, oracle control, and transaction-linked compensation are
different facts. Treating all of them as "code" is too coarse; treating all
open-source publication as operation is also too coarse. This distinction does
not itself exempt publication or development conduct; control, solicitation,
compensation, deployment, and continuing participation remain fact-specific.

= Requested joint work product

*PROPOSED:* I respectfully request a joint interpretation, concept release, or rulemaking
record that provides:

1. a staged formation matrix covering authorship, signature, revocability,
   funding, binding effect, interaction, match, trigger, finality, settlement,
   issuance, and secondary transfer;
2. paired examples of economically similar programs with different reference
   objects or security nexuses;
3. paired examples of computationally similar guarded programs where one creates
   a funded contingent right and another remains an ordinary service or document
   workflow;
4. criteria for determining when candidate results remain internal computations
   and when they become economically operative claims;
5. an express statement that full prefunding, fixed maximum loss, fail-closed
   guards, settlement binding, and formal verification may be relevant risk
   controls to the extent correctly specified, soundly proved, correctly
   implemented, and bound end-to-end to settlement, but are not automatic
   product-classification exclusions; and
6. a bounded factual-matrix process through which researchers can request
   guidance without asking either Commission to classify an entire programming
   model.

= Limits of this comment

*VERIFIED (document scope):* This comment does not assert that guarded commitments, candidate computation,
tokenization, automation, decentralization, full collateralization, or formal
verification removes an arrangement from either Commission's jurisdiction. It
does not offer a classification of any deployed product or request permission to
deploy one. The formal artifacts are research prototypes, not production market
infrastructure. The requested output is general, objective guidance for a class
of staged structures.

= Conclusion

*INFERRED:* Guarded commitments and candidate-result computation make a narrow but important
distinction visible: technical incompleteness is not necessarily economic
incompleteness. Objective criteria should identify when consideration, binding
rights, contingent exposure, reference characteristics, and transferability
become operative, then apply the appropriate statutory category to those facts.
They should separately identify the persons and systems that trade, intermediate,
clear, or settle the resulting rights.

*PROPOSED:* Milestone guidance would give developers a disciplined way to document staged
automated products without relying on labels. It would also give both Commissions
a shared factual record for products touching their respective interests.

#block(breakable: false)[
  #v(18pt)
  Respectfully submitted,

  [FULL NAME]  \
  [AFFILIATION, IF ANY]  \
  [DATE]
]
