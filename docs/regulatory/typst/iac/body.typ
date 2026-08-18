#import "../shared/template.typ": key_point, note_ref

= Executive summary

I submit this statement as an independent software and formal-methods
researcher, in connection with the Committee's August 20, 2026 public meeting;
the meeting notice provides for written statements received by August 27,
2026.#note_ref(9)

I study staged programmable commitments: systems in which the complete shape
of a future state change --- who may act, on which state, under what condition,
by what deadline, how many times --- is fixed and published at one moment, while
the fact that completes the change arrives at a later moment. I also study
deterministic computation over sets of candidate answers, and I classify
privacy designs by exactly who may learn which private inputs. This statement
presents the time-structure that research gives to programmable markets, and
asks the Committee to recommend policy work that attaches analysis to that
structure.

The Commission explains that "event contract" is not itself a defined term in
the CEA or CFTC regulations. It states that a prediction market offering event
contracts in swap or futures form to the general public must register as a
designated contract market, while a swap execution facility may make swaps
available for trading only to eligible contract
participants.#note_ref(1)#note_ref(11) Innovation will press on those lines,
and it will press hardest where software distributes over time the events that
traditional vocabulary fuses into "the trade." The stages are not a blur: they
can be made mechanically exact, and analysis, audit, and policy can attach to
them. The analysis should begin with the instrument, the participants, and the
functions performed at each stage --- not with labels such as "prediction,"
"policy," "token," "smart contract," "Dark," or "decentralized."

None of the technical properties described below is a jurisdictional shortcut.
They do not remove an instrument from the Commodity Exchange Act, make venue
or clearing functions disappear, or replace surveillance, recordkeeping,
customer protection, governance, or Commission access. Their policy value is
narrower and real: they can make authority and economic milestones explicit,
make certain substitutions and out-of-bounds writes refutable at a modeled
semantic boundary, and turn selected control objectives into testable
propositions. Any deployment claim would require additional cryptographic,
persistence, authorization, collateral, and system evidence that my research
does not supply.

#key_point("Five requested work products", [
  1. A milestone taxonomy for staged programmable transactions --- publication,
     funding, close, finality, settlement, and the events between them.
     Classification, audit, and enforcement all need a shared factual clock,
     and today every product description invents its own.
  2. Privacy-compatible audit-trail criteria separating public transparency
     from exact confidential regulatory records. Developers currently choose
     between total transparency and unauditable opacity because no
     intermediate target has been stated.
  3. Independently reviewable proof and control objectives for matching and
     settlement. Generic claims of "verification" cannot be checked; named
     relations with published negative cases can be checked by anyone.
  4. Functional guidance distinguishing general-purpose software publication
     from operation, solicitation, order handling, intermediation, and
     control. Researchers cannot currently tell which facts move them across
     that line.
  5. A structured path for researchers to present bounded factual matrices
     before a live product or deployment exists. Predeployment dialogue is
     cheapest exactly when nothing is yet at stake.
])

*Scope.* Current Regulation 40.11 addresses registered-entity listing or
clearing of contracts involving specified enumerated activities. A June 2026
proposal would revise that framework, define additional terms, and establish a
structured public-interest review; it remains proposed, not current
law.#note_ref(10)#note_ref(11) Every example in this statement references
objectively verifiable crypto-native facts --- ledger states, program events,
prices, ranges, path statistics --- and none targets an enumerated activity
under CEA section 5c(c)(5)(C) or Regulation 40.11. This scope choice is not a
claim that any example falls outside the CEA or any other law.

= Five milestones in one worked market

I define the milestones by walking one concrete design through its life.

The design is a fully collateralized conditional-asset market over an
objectively verifiable onchain price band. It asks one question: on a stated
future date, in which of five stated price bands will the time-weighted price
of a specific digital asset in a specific onchain liquidity pool fall? The
bands cover every possible price and do not overlap, and the terms include a
deterministic rule for every edge case --- a missing observation, a malformed
data page, a price exactly on a boundary. Exactly one band must realize. A
depositor locks one unit of collateral into a pool that belongs to this market
alone and receives one claim for each band; a complete set of five claims can
be returned for its collateral at any time before resolution; individual
claims are ordinary transferable assets; orders accumulate in a batch that
closes at a stated time and clears by a frozen deterministic rule; a frozen
observation program later identifies the realized band; and settlement pays
that band's claims from the pool. There is no debt, margin, leverage, or
liquidation anywhere in the design, and every participant's maximum loss is
fixed when their collateral or premium is paid.

I have built the core accounting of this design as an offline research
prototype --- pure Rust, integer-exact, with passing deterministic tests. It is
tested, not formally verified. It is not a deployed system, a product, or an
offer, and I do not ask the Commission to approve it. I use it, with Lean
models of the commitment and coordination structure, to make each milestone
concrete.

== Publication fixes the shape

The market template is published. The partition, the observation program, the
batch rule, the payout terms, and the edge-case rules are all inspectable.
Nobody has signed, deposited, or promised anything, and no value can move.

The research treats this earliness as a first-class property. I call the
underlying pattern a guarded commitment: the actor, the affected state, the
guard conditions, and the exact permitted transition shape are fixed before
the value that completes the transition exists. In my earliest Lean prototype
of the pattern, a weak guarded hole fixes a field, an actor, a target cell,
and a list of guard predicates, and only an integer arrives later; theorems
state that an accepted fill is exactly the committed transition and that a
guard-violating value fails closed, changing nothing. In a current, more
general model, the committed shape also includes the late value's type and
codec, the pre-state root, an abstract authority demand, a finite write
footprint, guard and effect commitments, a deadline, and a replay domain --- and
the late contribution is typed by that exact shape, so a contribution for a
different shape is not rejected at run time; it cannot be expressed at all. A
deliberately excluded stronger variant --- a hole in a value or authority
position, "I will owe an amount fixed later" --- is given no primitive and is
refuted in the model as an explicit anti-model. These are theorems about
modeled state machines, not deployed controls.

For classification, the milestone offers one clean fact: at publication there
is a complete, inspectable commitment structure and no exposure. Nothing has
been funded, and there is nothing to perform.

== Funding creates the exposure and fixes its ceiling

A depositor locks collateral and receives the complete set of five claims.
Exposure now exists --- and the same act that creates it fixes its ceiling,
because the pool holds the full deposit and covers every payout the frozen
terms allow, by construction of the terms. The commitment calculus
deliberately carries no value inside its commitments; bounding loss is the job
of this separate economic design, which makes every claim fully funded before
it exists. In my prototype I made the check structural: required collateral is
computed as the largest liability any payout vector in the market's immutable
set implies at the current claim supply, rounding against the protocol, and
I built every transition --- market construction included --- to refuse, as an
invariant violation, any state whose collateral falls below that maximum.

One refinement matters for classification. A complete set plus the
recombination right is a fully hedged position, interchangeable with its own
collateral; contingent exposure in the economic sense arises when a
participant sells some claims and keeps others. Funding and the first
unbalancing sale are distinct facts, and guidance should say which one matters
for which instrument question.

== The close freezes the candidates

Orders accumulate until the stated close. While the batch is open, the honest
description of "the clearing price" is a set of candidates: different gateways
or replicas may have seen different orders, and several complete possible
clearings are live at once. My research models this state explicitly. In the
model, a partial result is a grow-only set of candidate worlds, and
deterministic evaluation commutes with merging such sets by union ---
participants may share inputs or share computed results, in any order and any
batching, and arrive at the same candidate set. A single answer, though, is
not free: in the model, two replicas can each hold a perfectly determinate
result whose merge is indeterminate, so demanding one enforceable answer is a
coordination requirement, not an implementation detail.

The close is that coordination act, made cheap: a frozen rule stating that no
further orders will be admitted. In the model, stability of the inputs
transports to stability of every derived result, so freezing the book seals
the clearing with no per-computation argument. After the close, computing the
result is arithmetic anyone can redo. I built my batch prototype to do exactly
this: freeze its price grid, tie rule, and remainder rule when the book is
constructed, derive one canonical clearing, and accept a submitted clearing
candidate only if it matches what the frozen book itself determines,
recomputed from scratch --- never trusting the submitter's claimed quantities.

== Finality is a licensed collapse

The observation window and its repair period close, and the frozen observation
program's accepted evidence identifies the realized band. Until this moment
the system held five possible settlements; now one is authorized. My models
make the authorization itself explicit. In the model, sealing an answer is
licensed by a stability premise --- nothing that can still arrive can move the
result --- and under that premise the sealed claim survives everything that can
still arrive. Without the premise, the model exhibits the failure concretely:
a sealed answer that a later merge falsifies. Not stale --- false.

The model is deliberately agnostic about what real-world evidence discharges
the license, and it does not implement or validate an oracle, a legal finality
process, or an enforceable selection. In the worked market the license is the
frozen evidence rule plus the closed repair period; in another design it might
be an arbiter's announcement or a closed membership. The collapse is worth
exactly what the evidence licensing it is worth, and guidance should ask for
that evidence by name: what source, dispute rule, or stability premise
licenses selection, and what happens when the source fails.

== Settlement is a serialized guarded fill

The realized band's claims redeem from the pool; the other four expire
worthless. Mechanically, a redemption in my models is a guarded fill whose
shape was committed at publication: one-shot, because a commit consumes a
replay key derived from the committed shape and a second presentation of the
same accepted fill is refused; fail-closed, because a presentation that does
not satisfy the guard changes nothing; and exact, because the transfer that
occurs is precisely the pre-committed one, with writes framed inside the
declared footprint.

Settlement is also deliberately serialized, and the mathematics says why it
must be. In the model, balance-type conditions --- anything that behaves like a
balance, a quota, or a conservation constraint --- are exactly the guards that
provably do not merge coordination-free: two independently legal spending
states can merge into an over-budget one. The design accordingly treats
settlement as one guarded fill against one ledger, never as a merge of
concurrent claims. The theory does not suggest that settlement can be made
commutative; it predicts the opposite, and the design treats that prediction
as load-bearing.

= The taxonomy in one table

#table(
  columns: (1in, 1.85in, 1fr),
  table.header([*Milestone*], [*What becomes fixed*], [*Question for analysis*]),
  [Publication], [The complete shape: parties, state, condition, effect, deadline, replay domain], [Is anyone bound, funded, or able to cause execution?],
  [Funding], [The exposure and its ceiling], [Has contingent exposure become economically operative, and what bounds it?],
  [Close], [The candidate set stops growing], [What frozen rule ends admission, and who can verify the result afterward?],
  [Finality], [One result out of the candidates], [What evidence licenses the selection, and what happens if the source fails?],
  [Settlement], [The final ledger effect], [Is this performance of an earlier instrument or creation of another one?],
)

Between and around the milestones sit events that deserve their own factual
questions: signature (is the instruction revocable, binding, or executable by
another person?); early exit, compression, or unwind (is an existing
instrument terminated, another created, or discretion reintroduced?); witness
admission (is the accepted evidence proof of a fixed fact or a choice among
economic outcomes?); interaction and match (which venue or intermediary
functions arise, and for whom?); and secondary transfer (what is the
continuing instrument, and on what venue does it trade?). Each is a
mechanically distinct event with its own evidence, so a factual record can say
exactly which one it is describing.

#key_point("What the timeline separates", [
  Shape and exposure are created at different moments by different acts: at
  publication there is a complete, inspectable commitment and nothing at
  risk; at funding there is risk, with its ceiling fixed by the same act that
  creates it. Determination is separate from both: the outcome is fixed
  neither at publication nor at funding but at finality, and finality is an
  explicit collapse whose legitimacy is exactly the strength of its license.
  Between close and finality, the honest description of the system is a
  candidate set --- a value, not a failure.
])

= Four questions to keep separate

The CEA defines trading-facility and derivatives-clearing-organization
functions separately from product definitions.#note_ref(2) A functional
analysis of any staged design should keep four questions distinct:

1. *Instrument.* What enforceable rights exist, when do they arise, what
   contingency or reference affects them, and what is delivered?
2. *Venue and intermediary.* Who solicits users, receives or routes
   instructions, permits multiple interests to interact, applies a matching
   rule, operates an interface, receives transaction-linked compensation, or
   retains control?
3. *Clearing and custody.* Who holds collateral, substitutes credit, nets or
   settles obligations, manages defaults, and controls withdrawal or finality?
4. *Privacy and compliance.* Which data are public, which are available to a
   responsible regulated function, which are cryptographically protected, and
   what exact activity can be reconstructed?

= Seven questions for Commission guidance

I respectfully offer seven questions on which general guidance would help:

1. At which milestone --- publication, signature, funding, irrevocability,
   witness admission, participant interaction, match, claim creation, or
   settlement --- does a staged program create an agreement, contract,
   transaction, or contingent exposure?
2. Which combinations of upgrade control, interface operation, solicitation,
   order receipt or routing, matching, oracle control, custody, emergency
   control, and transaction-linked compensation constitute operation of a
   venue or intermediary rather than publication of general-purpose code?
   Staff Letter No. 26-09 shows that frontend facts can matter, in a narrow,
   conditional, nonbinding staff analysis.#note_ref(3)
3. If separate application logic locks collateral sufficient for a stated
   maximum loss, while a guarded declaration fixes the transition envelope but
   leaves a value or external fact for later proof, which milestone creates
   the contingent exposure?
4. If that application settles atomically without novation, credit extension,
   or loss mutualization, what remaining facts determine whether it arranges
   multilateral settlement as a derivatives clearing organization? Part 39
   shows that full collateralization can coexist with substantial
   institutional requirements.#note_ref(4)
5. Which order, identity, ownership, funding, position, lifecycle, and
   settlement fields must a DCM read in real time, and which may remain
   encrypted if complete linked records are timely recoverable and
   intelligible to the DCM and the Commission?
6. Can due-process threshold disclosure satisfy those access objectives
   without routine plaintext visibility to one commercial operator? And for
   longer-horizon research, which obligations structurally preclude a Dark
   architecture with no general disclosure path beyond a frozen leakage
   function and authorized local outputs?
7. Which facts may be established through independently verifiable proofs,
   what underlying records must remain available, and what registered-entity
   pilot, exemptive, interpretive, or no-action pathway should a developer
   discuss with the relevant operating division before deployment?

= Privacy-compatible auditability

== Exact modes, not marketing adjectives

My research uses three words with exact meanings:

- *Clear*: the specified state and computation are public.
- *Shielded*: a named executor, committee, or auditor may learn private
  inputs.
- *Dark*: no actor learns anything beyond a frozen leakage function and its
  own authorized local output, within an explicit corruption model.

These labels describe information flows and design targets, not present
implementation claims. An end-to-end claim under this taxonomy must also state
the topology, the assumptions, and the ingress, intermediate-state,
proof-production, settlement, timing, and availability leakage. Threshold
encryption is Shielded when a qualifying coalition can decrypt. Plaintext
proof production is Shielded when one producer sees the full witness. Public
accounts, amounts, timing, ingress, or settlement can keep an otherwise
private computation from being end-to-end Dark. Nothing in my research has
produced a deployed Dark venue, and I make no claim that a Dark architecture
satisfies current rules.

Current DCM rules include surveillance, real-time monitoring, trader and
position data, order-lifecycle tracking, reconstruction, source records, and
retention requirements.#note_ref(5) A public transaction hash does not by
itself perform any of those functions. Regulation 38.7 also shows that
regulatory data and public commercial data need not be
identical#note_ref(6) --- which is precisely the separation a privacy-compatible
audit trail would formalize.

== A regulator-observable Shielded reference architecture

The following is a discussion target, not a compliance claim about any system,
mine included:

1. *Public market layer.* Publish terms, source hierarchy, rule and verifier
   identifiers, settlement commitments, and required market data.
2. *Confidential regulatory layer.* Retain owner-linked orders, identity and
   authorization, timestamps, accepted-set provenance, modifications,
   cancellations, matches, allocations, funding, positions, and settlement.
3. *Governed disclosure.* Separate ordinary commercial access from authorized
   regulatory disclosure. Define key custody, threshold, due process, logging,
   rotation, recovery, retention, and compromise response.
4. *Proof-carrying execution objective.* Require independently reviewable
   proof artifacts designed to establish relation-specific claims about
   admitted inputs, guards, application-specific collateral, matching,
   nullifiers, conservation, and settlement against named rules and state
   roots.
5. *Explicit finality.* Preserve pending and candidate results until a named
   oracle, dispute, causal, or governance rule licenses selection.
6. *Operational controls.* Preserve effective halt, correction, cancellation,
   position control, investigation, and recovery consistent with the
   applicable obligations.
7. *Verifiable governance.* Identify and bind changes to terms, matching
   logic, circuits, keys, interfaces, fees, sources, and emergency controls.

Proofs should make controls more testable. They should not be treated as proof
that all eligible events were included, that real-world evidence was true,
that collateral was unencumbered, that data remain available, that governance
is adequate, or that legal duties were satisfied, unless those exact
propositions and external facts are part of the verified statement.

== The no-general-opening research boundary

A Dark system has no general decryption path. It may expose only a frozen set
of compliance-query outputs, proofs, and local disclosures. Whether that can
satisfy every applicable reporting, examination, correction, surveillance, and
enforcement objective is not answered by the existence of encryption or zero
knowledge, and I do not claim that it can. This statement treats
regulator-observable Shielded as the practical reference architecture. Dark is
retained solely as a long-horizon research boundary --- not a venue label, a
compliance conclusion, or a deployment recommendation --- and the Commission
should keep that research question separate from regulator-observable Shielded
pilots, which have materially different access and failure properties.

= Proof and test objectives

Useful proof targets include accepted-input binding, authorization, collateral
sufficiency, conservation, deterministic matching, priority, cancellation
races, duplicate prevention, payout validity, settlement-source provenance,
access and position limits, and consistency between public and confidential
records. Every proof claim should name:

- the exact relation and rule version;
- public inputs, committed evidence, state root, and ledger snapshot;
- proof system, parameters, verifier, and verification result;
- assumptions and corruption model;
- evidence-availability and retention policy; and
- what the proof does *not* establish.

The Commission could publish machine-testable positive and negative examples
for these controls. Conformance vectors should cover maximum integers,
rounding, failed signatures, duplicate nullifiers, omitted inputs,
reorganization, ambiguous timestamps, stale sources, circuit or key rotation,
correction, and unavailable evidence. Formal proof should complement
governance, operations, surveillance, and examination, not replace them.

= From software publication to operation

Researchers need clearer factual markers for the transition from
general-purpose work to product or facility operation. The facts that
plausibly matter include:

- creation of product-specific terms or listings;
- control of upgrades, sources, keys, or emergency actions;
- custody or control of participant value;
- solicitation, recommendations, or personalized signals;
- receipt, transmission, routing, matching, or disposition of orders;
- operation of a branded or transaction-oriented frontend;
- transaction-linked compensation; and
- continuing maintenance or discretionary intervention.

No single factor should be dispositive. The requested guidance should avoid
two symmetric errors: treating all software publication as market operation,
and treating an actively operated financial interface as mere publication
because its backend uses open-source or immutable code.

= Limits and current research status

The formal guarded-commitment and candidate-result artifacts are research
models. The market prototype is offline research code. Independently
provenanced repositories contain separately scoped prototype clearing, proof,
and privacy components; they do not presently compose into a production,
permissionless, end-to-end Dark market system. This statement does not
describe a deployed product, accepted customer funds, or live orders, and it
does not request approval of any deployment, product, or transaction. Formal
verification does not prove legal compliance, and I do not claim that it does.

The Commission identifies public input, advisory work, meetings, and the
Innovation Task Force as channels for developing policy insight.#note_ref(7)
The IAC is advisory.#note_ref(8) A written statement or a meeting appearance
is not Commission approval of anything, and I do not treat it as one.

= Conclusion

Programmable commitments can separate policy authorship, bounded
authorization, execution, finality, and settlement. Privacy systems can
separate public transparency, commercial confidentiality, and regulatory
access. Proof systems can make selected controls independently testable. None
of this eliminates the institutional questions of responsibility,
surveillance, governance, and access --- but it can make them precise. A
milestone taxonomy, privacy-compatible audit criteria, exact proof objectives,
functional developer guidance, and a structured predeployment path would let
technical systems be designed toward identifiable standards from their first
line of code. I ask the Committee to recommend that work.

#block(breakable: false)[
  #v(18pt)
  Respectfully submitted,

  [FULL NAME]  \
  [AFFILIATION, IF ANY]  \
  [DATE]
]

= Appendix: basis of material technical claims

This appendix records the evidentiary basis for each material technical claim
in one line. "Model theorem" means a machine-checked statement about a
simplified formal model reviewed by the submitter. "Prototype" means a
deterministic offline research prototype reviewed by the submitter. No
artifact behind these claims is deployed market infrastructure, and none has
been independently audited.

#table(
  columns: (1fr, 2.1in),
  table.header([*Claim*], [*Basis*]),
  [The Commission's description of event-contract terminology, general-public DCM registration, and the SEF/ECP limitation], [Commission releases; source notes 1 and 11],
  [Regulation 40.11's current scope, and the June 2026 proposal's status as proposed, not current law], [17 C.F.R. section 40.11; 91 Fed. Reg. 35806; source notes 10 and 11],
  [The CEA separately defines trading-facility, swap-execution-facility, and derivatives-clearing-organization functions], [7 U.S.C. section 1a; source note 2],
  [Staff Letter No. 26-09 is a narrow, conditional, nonbinding staff analysis in which frontend facts mattered], [Source note 3],
  [Part 39 accommodates fully collateralized positions alongside substantial institutional requirements], [17 C.F.R. part 39; source note 4],
  [Current DCM rules include surveillance, monitoring, audit-trail, reconstruction, and retention functions; regulatory and public data need not be identical], [17 C.F.R. part 38 and section 38.7; source notes 5 and 6],
  [The August 20, 2026 meeting and the August 27, 2026 written-statement deadline], [91 Fed. Reg. 51697; source note 9],
  [Public input, advisory work, meetings, and the Innovation Task Force are Commission channels; the IAC is advisory], [Source notes 7 and 8],
  [Guarded-commitment properties: eager shape fixing; an accepted fill equals the committed transition; guard violations fail closed; the typed late contribution cannot substitute any committed field; commits are one-shot by replay key; writes are framed to the declared footprint; value- and authority-bearing holes are given no primitive and are refuted as an anti-model], [Model theorems in the submitter's guarded-commitment research (a weak prototype and a current generalization), source-inspected at stated commits; not deployed controls],
  [Candidate-result properties: deterministic evaluation commutes with union of candidate sets; determinacy does not survive coordination-free merging; sealing is licensed by a stability premise and an unlicensed seal can be false of a merged state; input stability transports to result stability; balance-type guards do not merge coordination-free], [Model theorems in the submitter's candidate-result formalism; no oracle, legal finality process, or enforceable selection is implemented or validated],
  [The worked market's accounting: structural refusal of undercollateralized states (required collateral as the maximum liability over the immutable payout set); batch clearing verified by full recomputation of a frozen book; an observation accumulator built to refuse questions its retained information cannot support], [Offline pure-Rust research prototype reviewed by the submitter; deterministic tests pass; tested, not formally verified; not deployed],
  [No artifact described in this statement is deployed, funded, offered, or operating, and the research artifacts do not presently compose into an end-to-end system], [The submitter's repository status records; a statement about the submitter's own artifacts, not about any third party],
)
