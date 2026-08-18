# Operatorless agents — the market participant whose only operator is a published specification

Internal research memo, prepared 2026-08-19 by the operatorless-agents
research lane. See [README.md](README.md) for the memo set's status and
citation rules. This is research and drafting assistance, not legal advice,
not a filing, and not a filing authorization. It describes no deployed
system; no artifact discussed here is deployed, funded, or offered, by this
program or as far as this lane knows by anyone. The candidate filing text
this memo produced is
[IAC_ADDENDUM_CANDIDATE.md](IAC_ADDENDUM_CANDIDATE.md); it enters the IAC
written statement only on the author's express go, and nothing in this memo
modifies any current draft.

**Evidence regime.** Claims about the program's conditional-asset research
rest on the documents cited inline, chiefly
[GUARDED_EVENT_FOUNDATIONS.md](../../research/GUARDED_EVENT_FOUNDATIONS.md)
and the ledger discipline of
[DRAFT5_CLAIM_LEDGER.md](../DRAFT5_CLAIM_LEDGER.md). Claims about the
attestation stack rest on an independent artifact survey dated 2026-08-19,
as revised by its corrected second pass the same day, conducted across the
sibling repositories and not archived in this repository; its graded
sentences are reproduced verbatim in section 2 and are **binding wording
ceilings** — the maximum any sentence in this memo, in the addendum
candidate, or in any downstream filing text may carry. The recorded test
suites have since been independently reproduced:
[ATTESTATION_SUITE_RERUN_2026-08-19.md](../../research/ATTESTATION_SUITE_RERUN_2026-08-19.md)
records 61 + 6 + 19 = 86 tests, zero failures, run from the pinned
committed tree under the repository's own toolchain pin (executed
2026-08-18, filed under its 2026-08-19 slug; per the record, the
timestamps, not the filename, are the measurement). Test-result sentences
cite that record rather than "stated as passing" — and must carry its
caveat wherever the Lean linkage bears weight: the Lean emit step was not
re-run; the committed emitted descriptor (whose Lean pinning is inherited
from the commit) is what the tamper canaries exercised.

## 1. The architecture, plainly

*Label: PROPOSED — a composed design. The status of its components —
including which joins already exist and which are named, unperformed
steps — is section 2; the full permissionless-executor loop exists
nowhere.*

The idea is a market participant that is an AI agent whose "operator" is a
public document. Everything an operator would otherwise control at run time
is fixed in advance by a published specification: which model the agent
consults and through which templates every instruction to it is rendered;
which policy every input must satisfy before the agent may act on it; which
tools it may invoke, against which state, with what effect; its deadlines;
and the exact settlement consequence of each step. Executing a step is not
the operator's job either — it is open, prepaid work. Anyone may run the
next step offchain (render the instructions, consult the model, run the
tools) and submit the result together with certificates; the chain accepts
the step only if the certificates verify against the published
specification. In the vocabulary of the program's filings, the design is
four separable events:

- **Publication** fixes the complete shape of the agent's conduct — the
  same publication milestone the conditional-asset design begins with,
  where a commitment's structure is determined eagerly and only its witness
  arrives late (GUARDED_EVENT_FOUNDATIONS §§2, 5), extended from a market's
  terms to a participant's operating loop.
- **Funding** prepays execution bounties, so liveness is purchased in
  advance rather than owed by anyone — the same prepaid-liveness economics
  the conditional-asset design uses for its observation, repair, and
  settlement work (§4.2).
- **Execution** is permissionless keeper work. Executors are
  interchangeable and, in the legal sense, judgment-free: the acceptance
  rule can see nothing of theirs but the certificates.
- **Acceptance** is where authority lives — in the verification relation,
  not in the executor. This is the batch-prototype economics already argued
  in the filings: a submitted result is accepted only if it is exactly what
  the frozen specification determines, so competition among submitters is
  competition to perform work, not to influence the result (§4.3).

The consequence has two halves, and they cut in different regulatory
directions; both must be said.

**There is no operator to register.** The frameworks attach the operator
functions — supervision, recordkeeping, customer duties, emergency
authority, answerability — to a responsible person. Here no person performs
them. In the design studied, the specification's author retains no upgrade
key, no fee flow, and no continuing control; executors perform mechanical,
acceptance-checked work; funders prepay bounties. A registration regime
pointed at this design finds no registrant, and this memo does not soften
that.

**There is also no operator to trust.** The functions registration exists
to make someone answerable for are, for the specified conduct, discharged
by construction. The record of a step is not something an operator produces
and an examiner requests; it is the admission condition — a step without
its re-verifiable record cannot take effect. The supervisory control is not
a compliance program someone staffs; it is a frozen policy the acceptance
rule applies to every input. What a supervisor would subpoena from an
operated system, this design publishes as a precondition of acting. The
first half is a problem for the frameworks; the second half is the reason
the problem deserves analysis rather than an evasion label.

One further feature matters to the publication-versus-operation question
specifically. The filed position P-D2 — publication of market software,
without more, is not operation — always meets the objection "but is there
really nothing more?" In an operated system that is a facts-and-discovery
question. Here the absence of the "more" is itself inspectable: whether an
upgrade path, a compensation flow, or a discretionary control exists is
readable from the published specification and the chain state. The
architecture does not merely instantiate publication-without-more; it makes
"without more" checkable.

One sentence of pricing before section 2, because "trustless" must not be
handed out for free: at the current state of the artifacts two trusted
roles remain, by name — the executing host, whose run is recorded but not
yet proved, and a pinned transcript notary, which is an operator for the
function it performs. The certificate stack that would retire those roles
has real components, real joins, and named gaps, and the exact boundary is
the next section.

## 2. Buildable today versus the named gaps

*Label: SOURCED — the independent artifact survey of 2026-08-19, as
revised by its corrected second pass. The sentences quoted below are that
survey's grades, reproduced verbatim as wording ceilings. This lane did not
independently re-inspect the surveyed repositories, and nothing here
upgrades the survey's grading.*

### The ceilings, verbatim

- **Parse and guard proof leg:** "The repository contains a Lean-emitted,
  byte-pinned Dyck parse AIR and a template-guard DFA AIR, each exercised
  through the deployed plonky3 IR-v2 prover/verifier with per-constraint
  tamper canaries; these are research circuits over bounded grammars, not a
  proof of model-input generation." The JSON well-formedness leg is a
  re-executed compact certificate — checked by deterministic re-execution,
  not STARK-carried — and that distinction must be kept. May **not** claim:
  that an input was proved to have been generated from an approved
  template; any zero-knowledge or hiding property (the proven field is
  disclosed).
- **Provider transcript leg:** "A genuine TLSNotary MPC-TLS 2PC session is
  integrated at a pinned upstream revision, with server/notary pinning,
  API-key redaction, and a tested refusal of self-signed fixtures on the
  live path; the authenticated endpoint exercised is a local test server,
  and no live provider session has been performed." May **not** claim: that
  any provider actually attested anything.
- **Joins that exist** (source-verified; the legs must not be described as
  unjoined): the STARK injection leg and the TLSNotary leg are joined via a
  shared Poseidon2 content commitment with verifier-extracted field spans
  and a tested refusal policy — a splice attack was found and closed;
  attestations are witnessed on committed agent turns; and attestations
  install a fail-closed cell-predicate verifier.
- **Whole-history leg** (agent-growth): design and receipts exist; the
  proving rung (R3) is an explicit machine-readable gap, not an
  implementation. The executor host is trusted at the current rung.
- **Inference leg** (zkML): Lean soundness models (matmul sumcheck, Spartan
  R1CS) plus research notes; no proving backend, zero inference proofs; the
  notes' own words: "authors no AIR... links no proof system."
- **The defensible summary, verbatim ceiling:** "local research artifacts
  implement a Lean-authored parse/guard STARK and a genuine TLSNotary 2PC
  integration, joined by a shared content commitment and tested for
  refusal, with no live-provider session, no onchain posting path, and no
  verifiable-inference backend."
- **Cross-cutting:** the recorded test suites are independently
  reproduced — 86 tests (61 + 6 + 19), zero failures, from the pinned
  committed tree under the repository's own toolchain pin
  ([ATTESTATION_SUITE_RERUN_2026-08-19.md](../../research/ATTESTATION_SUITE_RERUN_2026-08-19.md)).
  Three of that record's caveats travel with any use: the run is three
  filtered measurements, not a statement about workspace health; the Lean
  emit step was not re-run — the committed emitted descriptor (whose Lean
  pinning is inherited from the commit) is what the tamper canaries
  exercised; and the tests establish nothing about all inputs and inherit
  the undischarged proof-system floor. Which restates the standing
  provenance gate: no soundness claim may rest on the proving backend,
  whose FRI dependency is locally patched with the fix not yet merged
  upstream — this memo claims only what the artifacts do, never that the
  proof system is sound.

### What each piece does, in operator-function terms

| Piece | The operator function it would evidence | Today, at the ceiling |
|---|---|---|
| Parse and guard proof | Recordkeeping and specified supervision: the agent's instructions parse under the committed grammar and the committed span satisfies the frozen template guard | Real research circuits, authored in Lean and emitted byte-pinned (pinning inherited from the commit; the emit step was not re-run in the reproduction), exercised through a deployed prover/verifier with per-constraint tamper canaries; bounded grammars; not a proof of model-input generation |
| Provider transcript | Provenance of the model's outputs: a named provider actually served the responses the agent consumed | Genuine two-party MPC-TLS machinery at a pinned revision — server/notary pinning, key redaction, tested refusal of self-signed fixtures on the live path; the endpoint exercised is a local test server |
| The join between them | The two evidences bind to the same conduct: what was attested is what was proved over, span by span | A shared content commitment with verifier-extracted spans and a tested refusal policy — one splice attack found and closed; attestations witnessed on committed agent turns; a fail-closed cell-predicate verifier installed |
| Whole-history attestation | Integrity of the run itself: the recorded history, every step in order, is the run that happened | Design and receipts; the proving rung is an explicit machine-readable gap, so the executing host is trusted |
| Inference proof | The computation itself: the model in fact produced these outputs from these inputs | Lean soundness models and research notes; no proving backend, zero inference proofs |

### The four missing pieces, in the system's own priority order

1. **R3, whole-history proving.** The system's own machine-readable gap
   statement: the current rung trusts the executor host and does not
   re-execute. Until this lands, "anyone may execute" is true but "no one
   need be trusted" is not.
2. **No onchain posting path.** No join yet connects an attestation to a
   chain. The "post the result on-chain" half of the design is a named,
   unbuilt join, so today the acceptance relation has no ledger to live in.
3. **A pinned notary is an operator.** The transcript leg's trust is
   trustless-to-that-notary, not operatorless: a specific, named third
   party sits inside the evidence chain performing a function someone must
   currently be trusted for. This is not a footnote. It is the difference
   between the design's endpoint and its current rung, and any filing
   sentence that said "no operator to trust" about the *artifacts* — rather
   than about the design — would be false.
4. **The public-specification step is unperformed.** The tool loop runs
   against an in-process node; publishing the specification the way the
   design intends is a named, unperformed deploy step.

### The gap structure is the contribution

Read as a claim about what exists, the stack is two working leg
implementations, a tested join between them, one design, and one
trajectory — with the loop's operatorless character blocked by exactly the
four pieces above. Read as a research program, its distinguishing property
is that every missing piece is a **named, testable step** — a proving rung,
a posting path, a notary-decentralization step, a deploy step — and that
one of the four is the survey naming a residual *operator* inside its own
trust story. A reader can complete or falsify any single piece without
touching the others, and the addendum candidate's basis block is written so
that completing a piece changes exactly one clause. That is the honest
difference between this and "decentralized AI" as ordinarily pitched: not
that the system exists — it does not — but that the distance between here
and there is enumerated rather than gestured at.

## 3. The questions the Commission's AI discussion should take up

*Label: PROPOSED — the commenter's analysis. Positions are stated as
positions; the questions marked open are genuinely open, and this memo
supplies no answers for them.*

Existing frameworks answer "who is responsible" by locating an operator and
attaching a bundle of functions to that person. This design unbundles the
operator into a specification (the conduct), an executor market (the
performance), a prepaid pot (the liveness), and an acceptance rule (the
validity). The productive question is therefore not "who is the operator" —
asked once, it has no answer — but function by function: which operator
functions attach to the specification's author, which to the executors,
which are discharged by verifiable conduct evidence, and which have no
bearer at all.

**Recordkeeping — a position; analysis supports it.** This is the function
where substitution is fully coherent. In an operated system, records are
produced by the regulated person and their completeness is established by
examination. Here a step cannot take effect without its re-verifiable
record: the record is not evidence about the conduct, it is a precondition
of the conduct having occurred. For the conduct the certificates cover,
"the ledger, as a condition of acceptance" is a stronger answer to "who
keeps the books" than any duty on a person. The limit is exactly the
coverage boundary in section 2's table, and a filing may say no more than
that boundary supports.

**Supervision — a split.** For specified policies, the design is
supervision-by-construction: the input-policy automaton is a supervisory
control applied to every input without exception, incapable of being
understaffed, and producing proof of application rather than attestations
of process. There is also an inversion worth the Committee's attention:
because every accepted step is publicly re-verifiable, the Commission's own
surveillance could consume the certificate stream directly — supervision of
specified conduct need not be delegated to a registrant at all. But
statutory supervision is broader than specified policy: it includes
noticing the unanticipated, and a frozen specification cannot flag conduct
it did not anticipate. Open question: for which regulatory purposes is
"every specified control provably applied, every action re-verifiable by
anyone" superior to operator supervision, and for which is the
unanticipated-conduct residue disqualifying?

**Emergency authority — open.** System-safeguard expectations assume
someone can halt the system on judgment. The design's central virtue — no
discretion anywhere — is the deliberate absence of that someone.
Discretion-free partial substitutes exist in the design vocabulary:
deadlines on every step, specified halt conditions, and funding exhaustion.
The last is double-edged and the specification makes the choice
inspectable: a top-up-funded agent can be stopped by starvation, while a
fully prepaid one — the shape the conditional-asset design's liveness rule
favors, so that what is admitted can always settle — cannot be stopped even
that way within its prepaid horizon. A frozen rule halts on defined
conditions; an authority halts on undefined ones. Whether frameworks can
accept the first in place of the second, or should require a specified halt
rule as a condition of admissibility for operatorless designs, is policy,
and this memo takes no position.

**Accountability for the agent's own conduct — open, and the memo does not
manufacture an answer.** If the agent's trading manipulates a market, who
is the respondent? The author published a specification and retained
nothing — and P-D2, the program's own filed position, says publication
without more is not operation; this design is that position's sharpest
instance precisely because the "more" is verifiably absent. The executors
performed acceptance-checked mechanical work whose outcome their judgment
could not influence. The funders prepaid bounties. If each attachment
fails, conduct that emerges from the specified loop with no human in it has
no respondent. Humans positioned around the agent remain ordinary
respondents — whoever manipulates the reference the agent trades on, or
uses the agent as the instrument of a scheme, is reachable as always, and
is more traceable than usual because every step of the instrument is public
and re-verifiable. The residue is confined but real. Three response shapes
deserve the Committee's attention, none endorsed here: conduct rules on
specifications (treat publishing a specification whose specified behavior
would be manipulative as the wrongful act — the author's actual act is
choosing the conduct); admission gatekeeping (venues or settlement layers
accept operatorless participants only when the specification carries named
controls — halt rules, conduct constraints); and evidence-first enforcement
(accept the residue, priced by the fact that the design yields better
conduct evidence than operated systems do). Each has obvious problems. The
point of a filing would be that the Commission's AI discussion should
confront them before such systems exist, not after — and the rulemaking on
software providers that Staff Letter 26-09 expressly anticipates, already
cited by the filed statement, is a natural vehicle.

**The rungs name their remaining trusted roles — a position; analysis
supports it.** The artifacts today are not operatorless and do not claim to
be: the executing host is trusted until the proving rung lands, and the
transcript leg trusts a pinned notary — a named operator for that function.
What the architecture changes even now is that the list of who must still
be trusted is enumerated and machine-readable rather than discovered in
examination: who is still trusted is a fact read off the artifact. Each
completed piece shortens the list, and the fully operatorless endpoint is
the limit where the list is empty. Regulators need not wait for the limit
to engage. At every intermediate rung the enumerated trustees — today, a
host and a notary — are exactly where existing frameworks can attach
duties, and the hard accountability questions above arise only at the
limit, which is precisely why they are worth taking up before anyone
reaches it.

**The taxonomy transfers — a position.** Publication, funding, execution,
and acceptance are separable events with different facts and different
candidates for attachment. Asking the operator question once, about "the
AI," fuses them exactly the way "the trade" fused execution and settlement.
Whatever the Committee concludes, it should conclude it per function and
per milestone.

**Docket context** (SOURCED —
[FILED_COMMENTS_LANDSCAPE.md](FILED_COMMENTS_LANDSCAPE.md), retrievals of
2026-08-18): the IAC docket has no substantive technical statement; the
meeting notice puts artificial intelligence expressly on the agenda, and
the current draft statement is silent on it; publication-versus-operation
is unclaimed by any filer in any of the three dockets; and no filer
anywhere in the corpus offers a machine-checked property of anything they
built. The operatorless question extends ground the program already holds
into the one agenda item its statement does not yet address.

## 4. Filing-worthiness, and what one bounded experiment would change

*Label: PROPOSED — a recommendation to the author. The go/no-go is the
author's alone; this memo drafts the candidate and does not enter it.*

**Verdict: filing-worthy as a half-page, question-register addendum. The
one gate this memo set — the recorded-suite re-run — has been met.** The
candidate text is [IAC_ADDENDUM_CANDIDATE.md](IAC_ADDENDUM_CANDIDATE.md).
Its central artifact sentence is the survey's own defensible-summary
ceiling, carried essentially verbatim; its basis block cites the
reproduction record with the Lean-emit caveat; and its gap sentences state
the trusted host and the pinned notary plainly, so it is safe even if
nothing further lands.

For filing it: the docket is empty of technical substance and AI is
expressly on the agenda; the addendum is the sharpest instance of a
position the statement already argues and no other filer claims; its
register — naming its own gaps, including that a pinned notary is currently
an operator inside its own evidence chain and that an accountability gap
exists which its author cannot close — is precisely the self-critical
register that distinguishes the packet from advocacy letters; and its only
ask is that the Committee take up a question, so it requests nothing whose
denial could prejudice the rest of the statement.

Against filing it: the artifact sentences are the evidentiary floor of the
whole packet — the statement's existing claims are source-inspected model
theorems and re-run tests, while the attestation stack, even after the
survey's stronger second pass (Lean-authored circuits, joined legs, a
closed splice attack, tested refusals), still records no live provider
session, no onchain posting path, a trusted host, and a trusted notary, and
a staff reader who checks evidentiary consistency will notice (the addendum
mitigates by saying so itself); autonomous trading agents invite a hostile
reading that the commenter wants unsupervisable AI in markets (the addendum
leads with the accountability question, which is the opposite of asking
permission); the statement sits at its six-page budget, so half a page must
come from somewhere; and the survey is a day old and twice revised — though
its re-run gate has now been met.

**The bounded experiments, one met and one open:**

1. **The recorded-suite re-run — landed; the gate is met.**
   [ATTESTATION_SUITE_RERUN_2026-08-19.md](../../research/ATTESTATION_SUITE_RERUN_2026-08-19.md)
   reproduces all three suites from the pinned committed tree under the
   repository's own toolchain pin: 86 tests (61 + 6 + 19), zero failures,
   zero ignored. It agrees with — indeed strictly improves on — the
   recorded results, and the addendum's basis line now cites it. Its
   caveats travel: three filtered measurements, not workspace health; the
   Lean emit step was not re-run (the committed emitted descriptor, whose
   Lean pinning is inherited from the commit, is what the tamper canaries
   exercised); and nothing in it upgrades soundness — the record itself
   says the tests establish nothing about all inputs and inherit the
   proof-system floor.
2. **One live provider MPC-TLS session — the remaining experiment.** The
   single most claim-changing bounded step available: one session, one
   recorded transcript, and "no live provider session has been performed"
   becomes "performed once, against a live provider." The reproduction
   record confirms how close and how far this is: the "live" tests exercise
   the real MPC-TLS 2PC code path with real cryptography and no external
   network I/O, and the one wired external-provider target sits in the tree
   marked ignore — a deploy step, never executed. It is *not* a gate — the
   addendum is drafted to be safe without it — and if it lands before
   August 27 the upgraded sentence must come from the new record, not from
   any wording pre-drafted here; this memo deliberately contains none.

Not gates, and not pre-deadline experiments: the R3 proving rung and any
inference proof are research programs. The onchain posting path and the
public-specification deploy step are bounded engineering steps rather than
research, but the addendum already states their absence, so nothing gates
on them either. The notary-decentralization question is design work, and
the addendum handles it the only honest way available: by naming the
pinned notary as an operator.

Timing: IAC written statements are due 2026-08-27; the landscape memo
recommends filing early and re-running its docket checklist first. The
addendum decision rides the Draft 7 freeze. If the answer is no-go, this
memo stands alone as research, and the full-length version of section 3
belongs to the software-provider rulemaking that Letter 26-09 anticipates
rather than to this statement.
