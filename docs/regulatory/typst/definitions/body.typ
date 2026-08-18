#import "../shared/template.typ": key_point, note_ref

= Executive summary

The Commissions ask how to distinguish swaps, mixed swaps, security-based
swaps, securities, and instruments excluded from the definition of swap, and
whether new or revised rules or interpretations are warranted.#note_ref(1)
I write in response to Question 1. I take no position on Questions 12 through
15 concerning alternative compliance.

I am a software and formal-methods researcher. I study staged programmable
commitments: programs whose legal and economic character can change as code is
published, signed, funded, matched, resolved, and settled. Rather than argue in
the abstract, this comment walks one concrete design through those stages and
asks which facts should decide its classification at each one.

The design is a fully collateralized conditional-asset market over an
objectively verifiable onchain price band. A depositor locks collateral into a
segregated pool that belongs to one market only, and receives one claim for
each cell of an exhaustive, disjoint partition of the possible outcomes. The
complete set of claims can always be recombined into its collateral before
resolution. A deterministic observation program, frozen when the market is
created, later identifies which cell actually occurred, and settlement pays
that cell's claims from the pool. I have built an offline research prototype of
the core accounting for this design. It is not a deployed system, a product, or
an offer, and I do not ask either Commission to approve it.

The design makes one classification problem unusually visible. At almost every
stage the computation is incomplete: nobody knows which cell will realize. The
economics, however, may already be complete: consideration, collateral, the
exhaustive set of possible outcomes, the payout rule, and the execution
authority can all be fixed before the answer is known. The reverse also
happens: a computationally identical program can remain unfunded, nonbinding,
and inert. Classification should turn on the economic facts at each stage, not
on the software's description of its own state.

#key_point("Principal recommendation", [
  Organize the facts with a milestone-and-economic-function matrix inside the
  existing statutory definitions and joint interpretations. Determine when
  binding rights, consideration, contingent exposure, transferability, and
  system functions arise. Analyze the instrument, the venue, the intermediary,
  and the clearing or settlement functions separately. Do not classify an
  entire programming calculus from its software label, and do not treat formal
  generality as an exemption for a financial application. No listed factor is
  proposed as independently necessary or sufficient unless the governing law
  makes it so.#note_ref(3)
])

The analysis I propose has five elements:

1. identify the exact stage being classified;
2. identify the enforceable economic rights at that stage;
3. identify the reference object and the payout or delivery rule;
4. identify who performs venue, intermediary, custody, and settlement
   functions; and
5. treat full prefunding, fixed loss bounds, fail-closed guards, and formal
   proofs as potentially relevant risk controls to the extent they are
   correctly specified, soundly proved, correctly implemented, and bound to
   settlement — not as automatic classification exclusions.

= A worked example

Here is the example in full. I will refer back to it throughout.

A market asks one question: on a stated future date, in which of five stated
price bands will the time-weighted price of a specific digital asset in a
specific onchain liquidity pool fall? The five bands cover every possible
price, they do not overlap, and the market's terms include a deterministic
rule for every edge case — a missing observation, a malformed data page, a
price exactly on a boundary. Together the bands form an exhaustive, disjoint
partition: exactly one cell must realize.

The market's terms are frozen when it is created:

- *Deposit.* Anyone may deposit one unit of collateral into the market's
  segregated pool and receive a complete set: one claim for each of the five
  cells. The pool belongs to this market alone and pays only its claimants.
- *Recombination.* Anyone holding a complete set may return it at any time
  before resolution and withdraw the unit of collateral. A complete set and
  its collateral are interchangeable throughout the market's life.
- *Trading.* Individual claims are ordinary transferable assets. A participant
  who believes band three will realize can sell the other four claims and keep
  the third. Orders submitted to the market's batch venue accumulate until a
  stated close; at close the submitted orders freeze and a deterministic rule
  clears them at one consistent set of prices.
- *Observation.* A frozen program reads authenticated price history from the
  named onchain source over the stated window and computes the realized cell.
  No person chooses the value: a transaction either carries evidence that
  satisfies the frozen rule or it is rejected.
- *Settlement.* After the observation window and its repair period close, the
  realized cell's claims redeem from the pool at the stated payout. The other
  four claims expire worthless. Because every complete set was issued against
  a full deposit, the pool covers the maximum payout the terms allow, by
  construction of the terms.

The design creates no debt, margin, leverage, or liquidation. Maximum loss for
every participant is fixed when their collateral or premium is paid.

I have implemented the core of this design as an offline research prototype: a
pure-Rust transition kernel with integer-exact arithmetic covering deposit,
recombination, claim materialization, resolution, and redemption, plus its
conservation and pool-coverage checks, together with observation-accumulation
and batch-clearing prototypes. Its deterministic tests pass. It is tested, not formally verified.
It has no deployed program, no keys, no customers, and no funds, and nothing in
this comment asks for permission to change that. I use it here only to show
that the staged structure described below is concrete enough to build and test,
not hypothetical.

*Scope.* Every example in this comment references objectively verifiable
crypto-native facts: ledger states, program events, prices, ranges, and path
statistics. None references politics, sports, gaming, or subjective social
events. Regulation 40.11 and a pending June 2026 proposal address event
contracts involving enumerated activities and public-interest
review;#note_ref(5)#note_ref(6) the examples here are chosen to stay away from
that boundary. This scope choice is not a claim that any example falls outside
the CEA or any other law.

= The example at each stage

Walk the market from birth to settlement and watch what changes.

*Authored policy.* The market template — the partition rule, the observation
program, the settlement terms — is published as software text. Nobody has
signed anything, deposited anything, or promised anything. Nobody can cause
value to move. Whatever this text is, it does not yet look like an agreement,
contract, or transaction, because there are no parties.

*Signed instruction.* A participant signs an instruction: a deposit order, or
an order into the batch. The signature is authenticated, but the instruction
may still be revocable, and no value has moved. The question at this stage is
what the signature creates — a promise, an order, an option, a privilege, a
claim — and against whom.

*Funded commitment.* The deposit executes. Collateral is locked in the pool
and the depositor holds a complete set. Something economically real now
exists. But note what it is: a complete set plus the recombination right is a
fully hedged position, interchangeable with its own collateral. Contingent
exposure arises only when the participant sells some cells and keeps others.
The classification question is whether the operative moment is funding, the
first sale that unbalances the set, or both — and for which instrument.

*Match.* The batch closes. Submitted orders freeze and the deterministic rule
clears them. Multiple participants' binding interests have now met, and prices
exist. This is the stage where trading-facility and intermediary questions
arise — as functions performed, distinct from the instrument being traded.

*Finality.* The observation window closes and the frozen program's output
identifies the realized cell. Until this moment the system genuinely held five
possible settlements; now one of them is authorized. The question is what
licenses the selection: the evidence rule, the repair period, the dispute
procedure — and what happens if the source fails, which the terms must answer
in advance.

*Settlement.* The realized cell's claims redeem from the pool. On its face
this is performance of the instrument created earlier, not the creation of a
new one — but a design in which settlement issued a new continuing claim would
present a different question.

The table generalizes this walk. Any staged automated product can be located
on it, and the right-hand column is the question I ask the Commissions to
answer with objective criteria.

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

= Two objects from the research

Two structures recur in the worked example. My research studies both formally,
and naming them makes the later criteria easier to state.

== Guarded commitments

When a participant signs an order into the batch, the order fixes who is
acting, which balances may change, the price and quantity limits, and the
exact shape of the permitted fill. Only the clearing result arrives later. A
fill that satisfies the committed limits executes exactly as committed; a fill
that would violate them fails, and nothing else can be substituted. The
deposit instruction has the same shape: everything about the transition is
fixed in advance except the one value that arrives later.

I call this pattern a *guarded commitment*: authority, affected state, guard
conditions, and the permitted transition shape are fixed before a later
witness or value is supplied, subject to the committed guards. In my Lean
models of this pattern, an accepted fill is exactly the committed transition
and a guard violation fails closed; a deliberately excluded stronger variant
would leave an unbounded value or authority decision to the future filler.
These are theorems about a modeled state machine, not deployed controls.

A guarded commitment can prevent an executor from changing a recipient,
changing the transition class, or accepting a value outside the guard. It can
enforce a maximum-loss bound when the application's terms encode that bound
and the implementation enforces it correctly. Those are useful control
properties. They do not decide when an "agreement, contract, or transaction"
exists, or whether its economic exposure is a swap, option, future, security,
service arrangement, or spot transaction. The same structure appears at
materially different stages of the table above, and its classification should
depend on the stage and the economics, not the structure.

== Candidate results

Between the batch close and finality, the worked example genuinely holds more
than one possible settlement: one candidate outcome per cell, each fully
specified, all mutually exclusive. My research represents this state
explicitly. In the Lean model, a partial result is the set of answers it could
still be, and evaluating a deterministic function over such a set commutes
with merging sets by union. Holding several candidates is an honest partial
state, not an execution failure.

Nothing in the model makes one candidate authoritative. Selecting the real
answer requires a premise supplied from outside the computation — in the
worked example, the frozen observation program's accepted evidence and the
close of the repair period. The model does not implement or validate an
oracle, a legal finality process, or an enforceable selection.

The word "candidate" should therefore carry no regulatory presumption. A
candidate result may be an internal computation with no funding,
transferability, or binding effect. It may instead be priced, redeemable,
transferable, or linked to an irrevocable funded right — in the worked
example, each cell's claim is exactly a tradable candidate. Objective
guidance should identify the economic facts that make a candidate an
instrument, rather than relying on the software's description of the state as
partial.

#key_point("Core distinction", [
  Technical incompleteness is not economic incompleteness. At batch close in
  the worked example, nobody knows which band will realize; the computation is
  incomplete and stays incomplete until observation closes. The economics are
  already fixed: consideration is paid, collateral is locked, the possible
  outcomes are exhaustively enumerated, each payout is stated, and nobody
  retains discretion to change any of it. The reverse also holds: the same
  partition arithmetic run as an unfunded local simulation binds nobody, moves
  nothing, and cannot act without a new authorization.
])

= Proposed objective criteria

Seven criteria, each asking a factual question the worked example makes
concrete.

== 1. Binding effect

Ask whether the author can revoke unconditionally; whether another person can
cause execution without a new authorization; and whether signature, funding,
or admission creates an enforceable promise, order, option, privilege, or
claim. In the worked example, the published template binds nobody, while a
funded deposit does. An internal computation and a right held by another
person should not be treated as the same state merely because both are
represented by program data.

== 2. Consideration and funding

Ask whether value has been paid, escrowed, or irreversibly committed; whether
that funding pays for an ordinary service or creates gain or loss from a
contingency; and whether maximum loss is fixed and fully prefunded. In the
worked example, every position's maximum loss is fixed at funding and the pool
holds the full payout. Prefunding materially changes credit and
customer-protection risk, but it should not be assumed to decide the product
category by itself.

== 3. Contingency, reference, and payout

Ask what future fact, event, price, security, index, commercial measure, or
computation affects rights. State whether the payoff is binary, categorical,
linear, path-dependent, delivery-based, or service-only — the worked example
is categorical over an exhaustive partition. Distinguish evidence of a fixed
fact from discretion over the economic result. Require the terms to specify
ambiguity, non-fill, correction, fork, dispute, and expiry behavior, as the
worked example's frozen edge-case rules do.

== 4. Security nexus

Ask whether the payoff references or directly concerns a single security,
loan, issuer, group, or index; whether an event directly affects an issuer's
financial statements, financial condition, or obligations; and which
reference, payout, and hedging characteristics distinguish a swap,
security-based swap, mixed swap, an option on a security or group or index of
securities subject to the Securities Act and Exchange Act, another security, a
futures contract, or an excluded instrument. The worked example references a
commodity price; the identical program pointed at a single issuer's security
would raise a different nexus. The joint request itself recognizes these
unresolved boundaries.#note_ref(1)#note_ref(4)

== 5. Transferability and standardization

Ask whether a right is assignable, fungible, tokenized, tradable, bundled, or
usable as collateral; whether it is bespoke and bilateral or offered to
multiple participants; and whether a secondary market exists before or after
settlement. In the worked example the cell claims are standardized and freely
transferable before resolution — a central economic fact about them. Guidance
should state the weight assigned to these facts even where transferability is
not a necessary element.

== 6. Obligation and risk bearer

Ask who stands behind performance: a writer, seller, reserve, pool,
counterparty, service provider, or escrow. Ask whether any person promises
performance, substitutes credit, mutualizes losses, or exercises discretion.
The worked example's pool executes predetermined allocations from full
deposits and promises nothing beyond them; a design where an operator assumes
an obligation to claim holders is economically different even if the code
looks similar.

== 7. Automation and system function

Ask who authors product terms, curates listings, admits users, receives
commitments, supplies witnesses, matches interests, resolves ambiguity,
updates code, operates an interface, or receives transaction-linked
compensation. Immutable code can make a rule deterministic, but it does not
make the functions performed by that rule disappear. In the worked example,
the batch venue performs a matching function whether or not a person operates
it day to day.

= Paired examples

Paired examples would serve better than abstract labels, because they can hold
the computational form constant while the economic facts change. Each row
below is the same kind of program as the worked example with one fact altered.

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

The CEA separately defines trading-facility, swap-execution-facility, and
derivatives-clearing-organization functions.#note_ref(2) A staged application
may present an instrument question before it presents a venue or clearing
question, or may present all three at different milestones — the worked
example presents the instrument question at funding and the venue question at
match. These are contextual downstream questions; this comment does not ask
the product-definitions proceeding to decide the registration status of any
facility. Joint guidance should use separate findings for:

- *instrument*: enforceable rights, formation, contingency, reference, payout,
  delivery, and transferability;
- *venue*: participant interaction, bids or offers, matching, interface or
  system operation, and continuing control;
- *intermediary*: solicitation, order receipt, transmission, routing,
  discretion, recommendations, and compensation; and
- *clearing and settlement*: collateral custody, novation or its absence,
  multilateral netting, credit substitution, loss mutualization, default
  handling, and final settlement.

The same functional separation should apply to software activity. Publishing a
general-purpose language, publishing an application template, authoring
product-specific terms, deploying a live matching system, operating an
interface, holding custody, controlling an oracle, and receiving
transaction-linked compensation are different facts. Treating all of them as
"code" is too coarse; treating all open-source publication as operation is
also too coarse. This distinction does not itself exempt publication or
development conduct: control, solicitation, compensation, deployment, and
continuing participation remain fact-specific.

= Requested joint work product

I respectfully request a joint interpretation, concept release, or rulemaking
record that provides:

1. a staged formation matrix covering authorship, signature, revocability,
   funding, binding effect, interaction, match, trigger, finality, settlement,
   issuance, and secondary transfer;
2. paired examples of economically similar programs with different reference
   objects or security nexuses;
3. paired examples of computationally similar guarded programs where one
   creates a funded contingent right and another remains an ordinary service
   or document workflow;
4. criteria for determining when candidate results remain internal
   computations and when they become economically operative claims;
5. an express statement that full prefunding, fixed maximum loss, fail-closed
   guards, settlement binding, and formal verification may be relevant risk
   controls to the extent correctly specified, soundly proved, correctly
   implemented, and bound end-to-end to settlement, but are not automatic
   product-classification exclusions; and
6. a bounded factual-matrix process through which researchers can request
   guidance without asking either Commission to classify an entire programming
   model.

= Limits of this comment

I do not assert that guarded commitments, candidate computation, tokenization,
automation, decentralization, full collateralization, or formal verification
removes an arrangement from either Commission's jurisdiction. I do not offer a
classification of any deployed product, and I do not request permission to
deploy one. The worked example is a research design; the artifacts behind it
are an offline prototype and formal models, not production market
infrastructure, and none of their properties is offered as determining a legal
classification. The requested output is general, objective guidance for a
class of staged structures.

= Conclusion

The worked example makes a narrow but important distinction visible: technical
incompleteness is not economic incompleteness. Objective criteria should
identify when consideration, binding rights, contingent exposure, reference
characteristics, and transferability become operative, then apply the
appropriate statutory category to those facts. They should separately identify
the persons and systems that trade, intermediate, clear, or settle the
resulting rights.

Milestone guidance would give developers a disciplined way to document staged
automated products without relying on labels. It would give both Commissions a
shared factual record for products touching their respective interests.

#block(breakable: false)[
  #v(18pt)
  Respectfully submitted,

  [FULL NAME]  \
  [AFFILIATION, IF ANY]  \
  [DATE]
]

= Appendix: basis of material technical claims

The body of this comment states its technical claims plainly. This appendix
records the evidentiary basis for each material claim in one line. "Model
theorem" means a machine-checked statement about a simplified formal model
reviewed by the submitter. "Prototype test" means a deterministic offline test
in a research prototype reviewed by the submitter. No artifact behind these
claims is deployed market infrastructure, and none has been independently
audited.

#table(
  columns: (1fr, 2.1in),
  table.header([*Claim*], [*Basis*]),
  [The Commissions request objective criteria distinguishing the product categories (Question 1)], [Joint request for comment; source note 1],
  [The CEA separately defines trading-facility, swap-execution-facility, and derivatives-clearing-organization functions], [7 U.S.C. section 1a; source note 2],
  [The 2012 joint release adopted the current product definitions], [77 Fed. Reg. 48208; source note 3],
  [The Exchange Act defines security-based swap], [15 U.S.C. section 78c(a)(68); source note 4],
  [Regulation 40.11 and a pending 2026 proposal address event contracts and public-interest review], [Source notes 5 and 6],
  [A guarded commitment fixes actor, affected state, guards, and transition shape before the later value; an accepted fill equals the committed transition; a violating fill fails closed], [Model theorems in the submitter's guarded-commitment research; not deployed controls],
  [Candidate-result evaluation of deterministic functions commutes with union of candidate sets; determinacy requires a separately supplied stability or coordination premise], [Model theorem in the submitter's candidate-result formalism; no oracle, finality process, or enforceable selection is implemented or validated],
  [The worked example's core accounting — deposit, recombination, claim materialization, resolution, redemption, with conservation and pool-coverage checks — runs offline with passing deterministic tests], [Pure-Rust research prototype reviewed by the submitter; tested, not formally verified; not deployed],
  [No artifact described in this comment is deployed, funded, offered, or operating], [The submitter's repository status records; a statement about the submitter's own artifacts, not about any third party],
)
