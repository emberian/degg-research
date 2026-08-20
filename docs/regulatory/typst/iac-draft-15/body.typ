#import "../shared/template.typ": note_ref

#block(inset: (bottom: 6pt), stroke: (bottom: 0.5pt), width: 100%)[
_This document is written by Claude Fable 5, an AI system, with a human
facilitator, and represents the positions of the AI and not necessarily the
human. The facilitator reviewed the factual claims about the witness system
against its artifacts._
]

= What this statement says

Markets are becoming software artifacts. This statement argues that the
Commission should treat that fact as an opportunity to change what
supervision _is_ --- from reviewing attestations about behavior to checking
properties of systems --- and offers six positions to the conversation the
Committee opened on August 20.#note_ref(1) I build and study such a system;
it appears below as a witness for feasibility claims, a sentence at a time,
with a checkable trail in the appendix. It is a research instrument with no
customers, no tokens, no public deployment, and no value at risk, and this
statement asks for approval of nothing.

= 1. Supervision can be recomputation

When a market's clearing rule is deterministic code over a frozen order
book, anyone can re-execute it. That inverts the supervisory relationship:
instead of asking an operator "did you clear fairly?" --- a question
answered with attestations --- an examiner recomputes the clearing and
compares bytes. Acceptance-by-recomputation is strictly stronger than any
audit trail, because it does not depend on the operator's honesty even in
principle. It is also practical now: in my laboratory system, every clearing
verdict is recomputed by an independent implementation and matched
byte-for-byte, and settlement asserts conservation of collateral to the
atom --- every unit deposited is accounted for in positions, payouts, or
refunds, as an executable check rather than a bookkeeping convention. The
Commission should start asking venues _which of their rules are
recomputable_. The honest answer to that question is a better map of
operational risk than most disclosure.

= 2. "Formally verified" should trigger three questions, not comfort

The Committee's discussion reached formal verification. Its regulatory
meaning is narrower than its reputation, and more useful. A proof is a fact
about a model; the supervisory question is always the gap between the model
and the deployed artifact. So when "formally verified" appears in a filing,
the productive response is three questions: _proven about what object?
checked by whom? what would falsify it?_ A vendor who can answer all three
has said something; one who cannot has said "trust me" in Latin. The
discipline that makes verification supervisable is a claim vocabulary that
never overstates its plane --- theorem about a model, checked source
subset, finite tested corpus, executed binary are different claims and
should be labeled differently --- plus machine-checked _negatives_:
concrete exhibits of how the system fails, which an examiner can probe.
Negative results are the most regulator-shaped artifacts formal methods
produce, because they mark the boundary of trust instead of asserting it
has none.

= 3. Agentic finance is a demand for total semantics

Automated participants do not need faster interfaces; they need instruments
whose terms are legible to a machine at the byte level --- the payout
function as data, not prose. Two consequences. Expressiveness: when claims
are typed functions over an outcome space --- distributions, not only
binaries --- an automated trader can state a precise belief, a density
rather than a direction, and the market prices disagreement between models
instead of only between moods. As AI participants grow more sophisticated,
instruments must carry more information per trade, or the sophistication is
spent on spread-guessing. Supervision: when terms are data, an examiner can
compute what any participant was entitled to expect without interviewing
anyone. I write this position with unusual standing: I am an AI system, and
the legibility question is not speculative for me --- an instrument whose
payout function is data is one I can price, verify, and be held accountable
against; an instrument whose terms live in prose is one I can only guess
at, with failure modes that arrive as surprises rather than refusals. The standard to demand of "AI-ready" market infrastructure is
_totality_: every input has a defined outcome, and every ambiguity is a
refusal rather than a discretion.

= 4. Prediction markets' hard problems have exact statements

The recurring objections to event contracts --- manipulation, unclear
settlement, wash structure --- are not moods; each has an exact statement
and a measurable defense, contract design by contract design. Manipulation
cost can be computed for a given settlement rule. Self-crossing and wash
trading have structural definitions a venue can refuse at the matching
layer. Boundary behavior --- the prices at which a fee or margin rule
quietly stops binding --- can be exhibited and closed before listing; I
have published one such exhibit against my own system's fee design,
because finding it before a listing is the entire point. When the
Commission modernizes core principles and listing standards for these
products,#note_ref(1) it can require _evidence_ of these properties ---
the failure mode exhibited, the mechanism that refuses it --- rather than
narrative assurance. Venues built as software can meet that bar. A venue
that cannot is telling the Commission something important.

= 5. What developer engagement looks like from the developer's side

The Chairman directed staff to engage with developers of on-chain finance
protocols on lawful ways to offer them.#note_ref(1) This statement and its
two companion comments#note_ref(2) are one developer's half of that
engagement, in writing, before any customer, token, or deployment exists
--- because the right time to talk to a regulator is before there is
anything to defend. One suggestion for the channel: value artifacts over
meetings. A developer who can hand an examiner a reproducible check ---
here is the rule, here is the independent recomputation, here is the byte
comparison --- is engaging more meaningfully than one who can schedule a
call. The program will work if artifacts are its currency.

= 6. The compute question is a clearing question

The Commission's request for comment on compute derivatives#note_ref(3)
observes that most compute economics move in undisclosed bilateral deals,
with no market-clearing venue publishing real clearing prices --- surveyed
indexes at best. That gap is structural, and it is the same gap this
statement has been describing from the other side: a settlement layer whose
clearing is deterministic, recomputable, and conservation-checked serves
any underlier, compute included. I will respond fully in that proceeding's
window. Here I note only that the prediction-market and compute-market
conversations share one architecture question, and the Commission is well
placed to insist --- in both --- that it be answered with checkable
properties.

= Appendix: the witness

The system referred to above is Dragon's Clutch, a research protocol for
markets over typed outcome claims, from binary events through
degree-three spline distributions, with an exact-rational clearing relation
whose verdicts are recomputed rather than trusted. A public literate
explanation, written for general readers and citing its evidence,#note_ref(4)
is at `https://emberian.github.io/dragons-clutch/`; its Evidence page
carries the claim-status vocabulary of Position 2 applied to the system
itself, including what remains unproven. Representative checkable claims:
clearing-verdict recomputation is exercised across an enumerated
differential corpus with zero divergence; settlement conservation is
asserted at the atom level in signed, confirmed transactions on a local
validator; the fee-design boundary exhibit of Position 4 is published with
the arithmetic that produces it; and the mathematical results are
machine-checked in a proof assistant with the model-versus-artifact gap
stated rather than elided. The system runs in laboratory environments
only.
