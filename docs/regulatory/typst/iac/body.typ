#import "../shared/template.typ": key_point, note_ref

= Executive summary

*VERIFIED (formal-model scope):* I submit this statement as an independent
software and formal-methods researcher. I study typed programmable transactions
in which an eager declaration fixes a late value's type and codec, turn and
pre-state root, an abstract authority demand, finite state footprint, guard and
effect declarations, deadline, continuation, and replay domain. Late advice
contributes only a value of the preselected type. Separate application logic may
lock collateral sufficient to bound maximum loss; that property is not supplied
by the guarded-advice primitive itself. I also study deterministic computation
over sets of correlated candidate worlds or answers.

*PROPOSED (privacy taxonomy):* I analyze possible implementations as Clear,
Shielded, or Dark according to who may learn private inputs and what leakage or
disclosure path is permitted. These labels describe design targets, not present
implementation claims.

*INFERRED:* These technical properties are not jurisdictional shortcuts. They do not remove
an instrument from the Commodity Exchange Act, make venue or clearing functions
disappear, or replace surveillance, recordkeeping, customer protection,
governance, and Commission access. Their potential policy value is narrower: they can
make authority and economic milestones explicit, make selected
transition-shape substitutions and out-of-footprint writes formally refutable at
the modeled semantic boundary, and turn selected control objectives into testable
propositions. Deployment claims require additional cryptographic, persistence,
authorization, collateral, and system evidence.

*SOURCED:* The Commission explains that "event contract" is not itself a defined
term in the CEA or CFTC regulations. It states that a prediction market offering
event contracts in swap or futures form to the general public must register as a
DCM, while a SEF may make swaps available for trading only to eligible contract
participants.#note_ref(1)#note_ref(11) The analysis should therefore begin with
the instrument, participants, and functions at each stage, not labels such as
"prediction," "policy," "token," "smart contract," "Dark," or
"decentralized."

*SOURCED:* Current Regulation 40.11 addresses registered-entity listing or
clearing of contracts involving specified enumerated activities. A June 2026
proposal would revise that framework, define additional terms, and establish a
structured public-interest review; it remains proposed, not current law.#note_ref(10)#note_ref(11)
*PROPOSED (initial research scope):* The examples discussed here are limited to
objectively verifiable crypto-native onchain reference events, such as specified
ledger states, program events, prices, ranges, or path statistics. They do not
target an enumerated activity under CEA section 5c(c)(5)(C) or Regulation 40.11,
while making no claim that any example falls outside the CEA or other law.

#key_point("Five requested work products", [
  *PROPOSED:*  \
  1. A milestone-based functional taxonomy for staged programmable transactions.  \
  2. Privacy-compatible audit-trail criteria separating public transparency from exact confidential regulatory records.  \
  3. Independently reviewable proof and control objectives for matching and settlement.  \
  4. Functional guidance distinguishing general-purpose software publication from operation, solicitation, order handling, intermediation, and control.  \
  5. A structured path for researchers to present bounded factual matrices before a live product or deployment exists.
])

= 1. A technical classification problem

== Guarded commitments

*VERIFIED (frozen prototype):* "Guarded hole" is local research vocabulary, not
a proposed legal category. In the inspected frozen Breadstuffs prototype, a weak
guarded hole fixes a field, actor identifier, target, and list of guard
predicates; only an integer arrives later. Lean theorems state that successful
fill equals the model's exact state transition and that every declared caveat
admits the value, while a guard-violating value fails closed. This is a formal
state-transition result, not evidence of a deployed proof circuit.

*VERIFIED (current formal model):* Current Minidregg research generalizes the
eager declaration to include a typed value codec, pre-state root, abstract
authority demand, finite footprint, guard and effect commitments, deadline,
continuation, and replay domain. Its modeled controller produces a semantic
commit intent only after modeled checks accept; accepted effects use a declared
finite write list and frame keys outside that footprint. Replay refusal is
modeled through a logical CAS and nullifier state.

*VERIFIED (scope ceiling):* These are theorems about Lean models. They do not
establish deployed cryptographic commitment binding, signature or key-custody
correctness, physical database or chain atomicity, network liveness, collateral
sufficiency, matching correctness, oracle validity, or legal compliance.

*INFERRED:* The formal discipline can make late substitution of the declared
transition shape unrepresentable at the modeled boundary, refuse values that
fail the guard, and frame modeled writes outside the declared footprint. Whether
this prevents unauthorized execution in deployment depends on independently
supplied signature, commitment, persistence, and collateral mechanisms. It does
not decide when a binding agreement exists,
whether contingent exposure is a swap, option, future, security, service, or
spot arrangement, or whether interaction among participants constitutes a
trading facility.

*PROPOSED:* The relevant stages may include:

#table(
  columns: (1.25in, 1fr, 1.3in),
  table.header([*Milestone*], [*State*], [*Question*]),
  [Policy authored], [Reusable software or terms exist], [Is anyone bound or funded?],
  [Instruction signed], [A participant authenticates terms], [Is it revocable, binding, or executable by another?],
  [Commitment funded], [Collateral or consideration is locked], [Has contingent exposure arisen?],
  [Early exit, compression, or unwind], [A participant closes, nets, transfers, or cancels a right before maturity], [Is an existing instrument terminated, another created, or discretion reintroduced?],
  [Witness admitted], [A guard accepts later evidence], [Is this evidence or economic discretion?],
  [Interests interact], [Several participants' binding states meet], [Which venue or intermediary functions arise?],
  [Candidate selected], [A finality rule licenses one result], [What evidence and authority make the selection final?],
  [Settlement], [Balances or claims become final], [What is delivered and by whom?],
  [Claim transfers], [A resulting right trades], [What is the continuing instrument and venue?],
)

== Candidate-result computation

*VERIFIED (local research object):* A separate formal model treats a partial result as a grow-only set of correlated
possible worlds or answers. Deterministic computation can proceed inside every
world without falsely selecting one result. More than one result is an honest
partial state. For deterministic functions over candidate-world sets merged by
union, evaluation commutes with that union. Determinacy requires a separately
supplied stability or coordination premise; the model does not implement or
validate an oracle, legal finality process, or enforceable selection.

*INFERRED:* The word "candidate" should carry no presumption. An unfunded, nonbinding,
nontransferable local simulation may remain only computation. A priced,
transferable, redeemable, or irrevocably funded candidate state may create
economic rights before final selection. Technical incompleteness is therefore
not necessarily economic incompleteness.

== Four axes should remain separate

*SOURCED:* The CEA defines trading-facility and derivatives-clearing-organization functions
separately from product definitions.#note_ref(2) A functional analysis should keep
four questions distinct. *PROPOSED:* The factual matrix should include:

1. *Instrument.* What enforceable rights exist, when do they arise, what
   contingency or reference affects them, and what is delivered?
2. *Venue and intermediary.* Who solicits users, receives or routes instructions,
   permits multiple interests to interact, applies a matching rule, operates an
   interface, receives transaction-linked compensation, or retains control?
3. *Clearing and custody.* Who holds collateral, substitutes credit, nets or
   settles obligations, manages defaults, and controls withdrawal or finality?
4. *Privacy and compliance.* Which data are public, which are available to a
   responsible regulated function, which are cryptographically protected, and
   what exact activity can be reconstructed?

= 2. Seven questions for Commission guidance

*PROPOSED:* I respectfully offer seven questions for general guidance:

1. At which milestone - publication, signature, funding, irrevocability, witness
   admission, participant interaction, match, claim creation, or settlement -
   does a staged program create an agreement, contract, transaction, or
   contingent exposure?
2. Which combinations of upgrade control, interface operation, solicitation,
   order receipt or routing, matching, oracle control, custody, emergency control,
   and transaction-linked compensation constitute operation of a venue or
   intermediary rather than publication of general-purpose code? Staff Letter
   No. 26-09 illustrates that frontend facts can matter in a narrow, conditional,
   nonbinding staff analysis.#note_ref(3)
3. If separate application logic locks collateral sufficient for a stated
   maximum loss, while a guarded declaration fixes the transition envelope but
   leaves a value or external fact for later proof, which milestone creates the
   contingent exposure?
4. If that application settles atomically without novation, credit extension, or
   loss mutualization, what remaining facts determine whether it arranges
   multilateral settlement as a DCO? Part 39 illustrates that full
   collateralization can coexist with substantial institutional requirements.#note_ref(4)
5. Which order, identity, ownership, funding, position, lifecycle, and settlement
   fields must a DCM read in real time, and which may remain encrypted if complete
   linked records are timely recoverable and intelligible to the DCM and
   Commission?
6. May due-process threshold disclosure satisfy those access objectives without
   routine plaintext visibility to one commercial operator? For longer-horizon
   research, which obligations structurally preclude a Dark architecture with no
   general disclosure path beyond a frozen leakage function and authorized local
   outputs?
7. Which facts may be established through independently verifiable proofs, what
   underlying records must remain available, and what registered-entity pilot,
   exemptive, interpretive, or no-action pathway should a developer discuss with
   the relevant operating division before deployment?

= 3. Privacy-compatible auditability

== Exact modes, not marketing adjectives

*PROPOSED (local terminology):* This research uses the following taxonomy:

- *Clear*: the specified state and computation are public.
- *Shielded*: a named executor, committee, or auditor may learn private inputs.
- *Dark*: no actor learns anything beyond the frozen leakage function and its own
  authorized local output, within an explicit corruption model.

*INFERRED:* An end-to-end claim under this taxonomy must also state the topology,
assumptions, ingress, intermediate-state, proof-production, settlement, timing,
and availability leakage. Threshold encryption is Shielded when a qualifying coalition can decrypt.
Plaintext proof production is Shielded when one producer sees the full witness.
Public accounts, amounts, timing, ingress, or settlement can prevent an otherwise
private computation from being end-to-end Dark. These terms describe information
flows; they do not claim that the local research has produced a deployed Dark
venue.

*SOURCED:* Current DCM rules include surveillance, real-time monitoring, trader and position
data, order-lifecycle tracking, reconstruction, source records, and retention
requirements.#note_ref(5) A public transaction hash does not itself satisfy those
functions. Regulation 38.7 also illustrates that regulatory and public commercial
data need not be identical.#note_ref(6)

== A regulator-observable Shielded reference architecture

*PROPOSED:* The following is a discussion target, not a present compliance claim:

1. *Public market layer.* Publish terms, source hierarchy, rule and verifier
   identifiers, settlement commitments, and required market data.
2. *Confidential regulatory layer.* Retain owner-linked orders, identity and
   authorization, timestamps, accepted-set provenance, modifications,
   cancellations, matches, allocations, funding, positions, and settlement.
3. *Governed disclosure.* Separate ordinary commercial access from authorized
   regulatory disclosure. Define key custody, threshold, due process, logging,
   rotation, recovery, retention, and compromise response.
4. *Proof-carrying execution objective.* Require independently reviewable proof
   artifacts designed to establish relation-specific claims about admitted inputs,
   guards, application-specific collateral, matching, nullifiers, conservation,
   and settlement against named rules and state roots.
5. *Explicit finality.* Preserve pending and candidate results until a named
   oracle, dispute, causal, or governance rule licenses selection.
6. *Operational controls.* Preserve effective halt, correction, cancellation,
   position control, investigation, and recovery consistent with the applicable
   obligations.
7. *Verifiable governance.* Identify and bind changes to terms, matching logic,
   circuits, keys, interfaces, fees, sources, and emergency controls.

*INFERRED:* Proofs should make controls more testable. They should not be treated as proof
that all eligible events were included, real-world evidence was true, collateral
was unencumbered, data remain available, governance is adequate, or legal duties
were satisfied unless those exact propositions and external facts are part of
the verified statement.

== The no-general-opening research boundary

*INFERRED:* A Dark system has no general decryption path. It may expose only a frozen
set of compliance-query outputs, proofs, and local disclosures. Whether that can
satisfy every applicable reporting, examination, correction, surveillance, and
enforcement objective is not answered by the existence of encryption or zero
knowledge. The Commission should separate this long-horizon research question
from regulator-observable Shielded pilots, which have materially different access
and failure properties.

*PROPOSED:* This statement treats regulator-observable Shielded as the practical
pilot posture. Dark is retained only as a narrowly defined research boundary,
not a current compliance posture, venue label, or deployment recommendation.

= 4. Proof and test objectives

*PROPOSED:* Useful proof targets include accepted-input binding, authorization, collateral
sufficiency, conservation, deterministic matching, priority, cancellation races,
duplicate prevention, payout validity, settlement-source provenance, access or
position limits, and consistency between public and confidential records.

Every proof claim should name:

- the exact relation and rule version;
- public inputs, committed evidence, state root, and ledger snapshot;
- proof system, parameters, verifier, and verification result;
- assumptions and corruption model;
- evidence-availability and retention policy; and
- what the proof does *not* establish.

*PROPOSED:* The Commission could publish machine-testable positive and negative examples for
these controls. Conformance vectors should cover maximum integers, rounding,
failed signatures, duplicate nullifiers, omitted inputs, reorganization,
ambiguous timestamps, stale sources, circuit or key rotation, correction, and
unavailable evidence. Formal proof should complement governance, operations,
surveillance, and examination rather than replace them.

= 5. Functional guidance for developers and interfaces

*INFERRED:* Researchers need clearer factual markers for the transition from general-purpose
work to product or facility operation. Relevant facts may include:

- creation of product-specific terms or listings;
- control of upgrades, sources, keys, or emergency actions;
- custody or control of participant value;
- solicitation, recommendations, or personalized signals;
- receipt, transmission, routing, matching, or disposition of orders;
- operation of a branded or transaction-oriented frontend;
- transaction-linked compensation; and
- continuing maintenance or discretionary intervention.

*PROPOSED:* No single factor is proposed as dispositive. The requested guidance should avoid
two errors: treating all software publication as market operation, and treating
an actively operated financial interface as mere publication because its backend
uses open-source or immutable code.

= 6. Requested policy work

*PROPOSED:* I respectfully recommend that the Commission develop:

1. *Milestone taxonomy.* Map authorship, signature, revocability, funding,
   interaction, match, claim creation, early exit, compression, unwind, finality,
   settlement, and transfer to the instrument, venue, intermediary, and clearing
   questions.
2. *Privacy-compatible audit criteria.* State which records must be public,
   available in real time to a responsible function, recoverable on demand,
   linked across positions, and retained.
3. *Proof and test criteria.* Publish bounded control objectives, negative cases,
   and reconstruction tests rather than accepting generic claims of verification.
4. *Developer and interface guidance.* Explain the significance of product
   creation, upgrade and source control, frontend operation, order handling,
   custody, fees, and continuing control.
5. *A coordinated innovation path.* Permit a researcher to present a
   non-transaction-specific architecture, then work with counsel and the relevant
   divisions on product-, venue-, and clearing-specific facts before seeking any
   registration, exemptive, or no-action process the actual design requires.

*SOURCED:* The Commission identifies public input, advisory work, meetings, and the
Innovation Task Force as channels for developing policy insight.#note_ref(7) The IAC
is advisory.#note_ref(8) A comment or meeting does not constitute Commission approval.

= Limits and current research status

*VERIFIED (local research status):* The formal guarded-commitment and
candidate-result artifacts are research objects. Independently provenanced
repositories contain separately scoped prototype clearing, proof, and privacy
components. They do not presently compose into a production, permissionless,
end-to-end Dark market system. This statement does not describe a deployed product,
accepted customer funds, live orders, or a request to approve deployment. It does
not claim that formal verification proves legal compliance.

= Conclusion

*INFERRED:* Programmable commitments can separate policy authorship, bounded authorization,
execution, finality, and settlement. Privacy systems can separate public
transparency, commercial confidentiality, and regulatory access. Proof systems
can make some controls independently testable. None eliminates the institutional
questions of responsibility, surveillance, governance, and access.

*PROPOSED:* The immediate opportunity is to make those distinctions explicit. A
milestone-based taxonomy, privacy-compatible audit criteria, precise proof
objectives, and functional developer guidance would allow technical systems to
be designed toward identifiable standards from their first line of code.

#block(breakable: false)[
  #v(18pt)
  Respectfully submitted,

  [FULL NAME]  \
  [AFFILIATION, IF ANY]  \
  [DATE]
]
