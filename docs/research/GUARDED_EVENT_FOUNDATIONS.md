# Guarded event foundations

Staged commitment, candidate truth, and prefunded exposure in the Dark Egg /
Dragon's Clutch research program.

Status: technical research exposition, written 2026-08-18. This document
explains the foundational calculus behind the program's regulatory research so
that a technical adviser, counsel, or cryptographer can read it straight
through and disagree with it precisely. It is not legal advice, not a filing,
and not a description of any deployed system. Every artifact discussed is an
offline research model or prototype. Every material claim in the body carries a
bracketed identifier, [C-01] through [C-25], resolved in Appendix A to its
evidence class, exact file path, and repository commit; the body itself is
written plainly, without that apparatus.

---

## 1. The question a staged commitment raises

Programmable settlement systems can now issue commitments in stages. The
complete terms of a future state change — who may act, on which state, under
what condition, by what deadline, at most how many times — can be fixed,
published, and made mechanically binding at one moment, while the fact that
completes the change arrives at another moment, possibly much later, possibly
from a party who did not exist when the terms were fixed.

Familiar market categories tend to fuse into a single event — "the trade,"
"execution," "settlement" — several things that such systems hold apart. When a
classification analysis asks its usual questions, the fused vocabulary gives
fused answers. The questions themselves are separable:

1. **When do the operative terms become fixed, and what can still change
   afterward?**
2. **When does economic exposure come into existence, and what bounds it?**
3. **When is the outcome determined, by what evidence, and with what
   discretion remaining?**

The research program described here studies a small calculus in which each of
those questions attaches to a distinct, mechanically defined event, and the
separation is not a narrative choice but a property of the mathematical
objects. The foundational object is what this document calls a **guarded
event**: a state transition whose complete shape is committed before the value
that completes it exists, whose completion either matches that shape exactly or
does not happen at all, and which can happen at most once.

Three layers own the story, and they are deliberately owned by different
artifacts in different repositories:

- **Layer 1 — shape is fixed early.** A commitment's structure is determined
  eagerly; only its witness arrives lazily. Owned by Lean models in the
  Breadstuffs and Minidregg repositories.
- **Layer 2 — truth is honestly partial until a licensed cut.** Before
  coordination, a distributed computation's result is a set of candidates, and
  collapsing that set to one enforceable answer is itself a coordination act
  with a stated precondition. Owned by Lean models in the Leanuweave
  repository.
- **Layer 3 — economic exposure is bounded by prefunding, not by the
  calculus.** The calculus deliberately carries no value inside its
  commitments; a separate economic design makes every claim fully funded
  before it exists. Owned by the Dragon's Clutch repository's offline Rust
  prototypes.

The layers are research artifacts. They do not presently compose into one
system, and this document will say so again, precisely, in Section 6 [C-23].
What they already provide is a vocabulary in which the three classification
questions above have distinct, checkable answers — and in which a reader who
disagrees can point at the specific theorem, definition, or boundary they
disagree with.

## 2. Layer 1 — the shape is fixed before the value arrives

### 2.1 The guarded hole, minimally

The simplest modeled object is the **weak guarded hole**, a small Lean
prototype in the Breadstuffs repository. A hole is created with four things
fixed: the field it will write, the actor permitted to fill it, the target
state cell, and a list of guard predicates the eventual value must satisfy.
Nothing else about the hole is negotiable later; the only thing that arrives
late is a value [C-01].

Two theorems in the model give the object its teeth:

- **A successful fill is exactly the committed transition.** If a fill
  succeeds, the resulting state is precisely the pre-committed write — no
  hidden or additional mutation — and every guard predicate the hole promised
  was actually discharged against the arriving value [C-02].
- **A violating value fails closed.** A value that does not satisfy the guard
  produces no state change at all. There is no partial effect, no penalty
  branch, no discretionary fallback; the fill simply does not occur. The model
  includes a concrete two-valued instance — a guard that admits the value 50
  and rejects the value 55, with the rejection exercised — so the fail-closed
  property is demonstrated on a case where the guard actually discriminates,
  not satisfied vacuously [C-03].

The design documents accompanying the model state the governing principle in
one sentence: **determination is eager, witness is lazy**. A contribution's
shape — which field it writes, whose authority it demands, its exact effect —
is fixed when the hole is created; only the value, or the proof that a
condition now holds, arrives later [C-04].

The same documents draw a second line that matters for everything downstream.
A hole in a *value* or *authority* position — "I will contribute an amount to
be determined," or "this action will be authorized by something supplied
later" — is called a **strong hole**, and the calculus deliberately provides
no primitive for it. Conservation of value in the modeled calculus is a
per-transition property; a hole that reserved an undetermined value
contribution would break it, so the type simply does not exist. The failure
mode is inexpressibility at construction time, not a runtime check that could
be forgotten [C-04].

### 2.2 The stronger re-expression

The Minidregg repository re-expresses the same idea with the shape made
maximally explicit and the machinery reduced to first-order data. There, a
hole's specification fixes eleven things at creation: the hole's identity, the
logical type of the value that may fill it, the transaction it belongs to, the
exact pre-state root it applies to, the authority it demands, the finite set
of state locations it may touch, commitments to its guard and to its effect, a
deadline, a continuation, and a replay domain. The late-arriving contribution
— called advice, and treated as untrusted — contains exactly one thing: a
value of the type the specification selected. The advice type is
mathematically indexed by the full specification, so "advice for a different
shape" is not a request that gets rejected; it is a phrase that does not
type-check. Acceptance provably cannot substitute any eager field [C-05].

Around that core, the model proves a cluster of properties that together
describe what this document means by a guarded event:

- **Verification is total and its refusals are distinguished.** The verifier
  is a function that always returns exactly one of: accepted; rejected for a
  named semantic reason (authority, guard, or effect); pending; expired;
  unsupported; or backend-unavailable. Operational inability and temporal
  states are kept apart from policy rejection, so "the system was down" can
  never be confused with "the condition failed" [C-06].
- **Refusal changes nothing.** A rejected presentation leaves durable state
  exactly as it was [C-06].
- **Fills are one-shot.** The commit step consumes a replay key derived
  entirely from the hole's eager shape. In the model, the first presentation
  of an accepted fill commits, and a second presentation of the very same
  accepted fill is refused as already consumed [C-07].
- **The forbidden thing is exhibited, not just avoided.** The model contains a
  deliberately unsafe anti-model in which late data is allowed to carry a
  replacement specification, and a theorem showing that admitting it
  substitutes a different shape whenever one is offered. The safe advice type
  has no corresponding construction. This is the strong hole, refuted rather
  than merely omitted [C-08].
- **The controller adds framing, not discretion.** A separate controller
  model shows that a commit-ready decision binds every eager field of the
  hole plus the encoded advice, and that every state location outside the
  declared write footprint is unchanged by the commit. What the host may do is
  observe and physically apply; it supplies no semantic verdict of its own
  [C-09].

All of this is source-inspected Lean at a stated commit; the evidentiary
status, including what was and was not rebuilt, is recorded per-claim in
Appendix A [C-10].

### 2.3 A miniature: the resolution-gated redemption

A concrete way to hold Layer 1 in mind. Imagine a redemption ticket printed at
the moment a market is created, months before anyone knows the outcome. The
ticket names: the account to be debited, the account to be credited, the
condition ("the recorded observation program reports outcome B"), the deadline
after which it is void, and a serial number that can be punched exactly once.

When someone later presents the ticket together with the recorded observation,
exactly one of two things happens. Either the observation satisfies the
printed condition, in which case the transfer that occurs is exactly the
printed transfer — not an approximation of it, not a renegotiation — or the
observation does not satisfy the condition, in which case nothing happens and
the ticket remains unpunched. Presenting the ticket twice fails the second
time. Presenting a ticket with different printing is not a forged fill that
the verifier must catch; in the modeled type system it is not a ticket for
this slot at all.

The point of the miniature is where the commitment lives in time: everything a
counterparty, an auditor, or an adversary could care about — who, what, under
which condition, by when, how many times — was fixed and inspectable at
printing time. The only information the future contributes is the witness.

## 3. Layer 2 — truth is a set of candidates until a licensed cut

Layer 1 is about one ledger. Layer 2 asks what happens before there is one
ledger — while independent replicas, gateways, or parties each hold part of
the picture — and it is owned by the Leanuweave repository's Lean models.

### 3.1 Partial results, and the one equation

The model represents an in-progress distributed computation's result as a
**grow-only set of candidate worlds**: complete possible valuations of the
inputs, of which several may be live at once because different replicas have
seen different things. "I know nothing yet" is the empty candidate set; "we
disagree" is a set with several elements; both are ordinary points of one
lattice, not error states.

The headline theorem is one equation: for any deterministic evaluation
function, **computing then merging equals merging then computing**. The image
of a merged set of candidate worlds is the merge of the images. A replica may
gossip its inputs or its results; a peer may combine answers or combine worlds
and recompute; any batching, any order, arrives at the same value [C-11].
Within the model this is the exact sense in which *computation over honest
partial truth is coordination-free*: no ordering service, sequencer, or leader
is needed to keep replicas' derived answers consistent, because consistency is
algebra, not protocol.

Two refinements sharpen what the equation does and does not license:

- **Candidates must be tracked as whole worlds, not per variable.** The model
  proves that tracking candidate values independently per component is a sound
  over-approximation but strictly lossy: it manufactures phantom results that
  no actual candidate world justifies, by forgetting the correlation between
  components. A system that stores candidates per register has already lost
  information that no downstream cleverness recovers [C-12].
- **Wanting one answer is not free.** "This computation has at most one
  candidate" is provably not a property that survives coordination-free
  merging: two replicas can each hold a perfectly determinate answer whose
  merge is indeterminate. Demanding a single enforceable result from a
  replicated computation is, in the model, a coordination requirement — not a
  quality-of-implementation issue [C-13].

### 3.2 Collapse is explicit, and it has a precondition

Because a determinate answer is not free, the model makes collapsing to one —
**sealing** — a separate, explicit operation with a stated precondition. A
seal is the claim "the answer is a." What licenses it is a **stability**
premise: nothing that may still arrive can move the result. Under that
premise, the sealed claim provably survives everything that can still arrive.
Without it, the model exhibits the failure concretely: a replica seals its
answer, a peer arrives holding a different candidate, and the sealed claim is
false of the merged state — not stale, false, asserted with confidence the
state never justified. A companion theorem gives the practical mechanism:
stability of the *inputs* transports to stability of every *derived result*,
so finalizing what is being computed over seals the answers with no
per-computation argument [C-14].

Within the model, then, **finality is a licensed act, not an observation**.
The license is a premise about the future — what can still arrive — and the
model is deliberately agnostic about what real-world evidence discharges it
(an arbiter's announcement, a causal cut, a closed membership), because those
differ in what their evidence is worth. The theorem transfers exactly the
strength of the license, never more.

### 3.3 When do guarded events tolerate divergence?

The two layers meet in a gluing question: two replicas independently fill the
same guarded hole; can their results be merged without coordination, with the
guard still holding? The model's answer is exact. For a hole whose fill space
is *spanning* (a stated non-degeneracy hypothesis: every state is reachable as
an admissible fill), gluing across all divergent admissible fills holds **if
and only if** the guard is order-insensitive — insensitive to how independently
accumulated states merge. The spanning hypothesis is not decoration: the model
exhibits a hole whose guard is provably order-sensitive and which nonetheless
glues because its fill space is restricted, so the two notions genuinely
differ and the hypothesis is exactly what identifies them. And when a guard is
not order-insensitive, the model does not stop at "no": it constructs the
four-state counterexample — two replicas, two locally legal fills, a merge the
guard rejects — as a concrete scenario to replay [C-15].

There is a refinement with practical consequences. A guard that fails to glue
globally can glue **within a seam**: a partition of states such that merges
inside one fiber are safe and only crossing the seam requires coordination.
The model's worked instance is a budget: "total spending within quota" is
provably not order-insensitive globally — two independently legal spending
states can merge into an over-budget one — yet within a fixed allocation, all
spend merges are safe. Spends never coordinate; changing the allocation is the
only coordination point [C-16].

That instance is the bridge to Layer 3, and it is worth saying plainly:
**balance-type conditions are exactly the ones the gluing theorem predicts
will not merge coordination-free.** Anything that behaves like a balance, a
quota, or a conservation constraint sits on the coordination side of the line.
The mathematics does not suggest that settlement can be made commutative; it
predicts the opposite, and the program's economic design treats that
prediction as load-bearing rather than as an obstacle.

### 3.4 A miniature: the clearing set at auction close

During a batch auction's window, several gateways each hold a view of the
order flow. Model each gateway's knowledge as candidate worlds — complete
possible books — and the clearing computation as a deterministic function of a
book. The headline equation says the gateways may share inputs or share
computed candidate clearings, in any order, with any batching, and end at the
same candidate set; nobody needs to sequence the gossip [C-11].

"The clearing price," though, is one answer, and the model prices that
demand: while orders may still arrive, two gateways can each hold a
determinate candidate clearing whose merge is not determinate [C-13]. The
close is the licensed cut: a frozen rule saying no further orders will be
admitted is precisely a stability premise on the inputs, and input stability
transports to the derived clearing [C-14]. Sealing before the close is the
concrete failure the model exhibits — a clearing price asserted while a book
merge could still falsify it. After the close, computing the result is again
mere algebra: anyone can recompute it, and all recomputations agree.

The miniature also shows what the model does *not* say. The theorems are about
sets, merges, and functions; they do not establish that any particular
real-world announcement of a close is trustworthy, that an oracle's value is
correct, or that anyone is legally bound by the sealed result. A collapse is
only as good as the evidence that licensed it, and that evidence lives outside
the model.

## 4. Layer 3 — exposure is bounded by prefunding, not by the calculus

### 4.1 Why the calculus carries no value

A reader may notice what Layers 1 and 2 never mention: money. That is by
construction. The commitment calculus deliberately excludes value from the
hole: the promise's ledger effect is the minting of the hole itself (evidence
that a commitment exists), and value moves only inside fully determined
transitions, each of which conserves it exactly. A promise that carried an
undetermined value contribution — "I will owe an amount fixed later" — is the
strong hole of Section 2, the thing the modeled calculus provides no primitive
for and the Minidregg model refutes as an explicit anti-model [C-04, C-08].

The consequence is a clean division of labor. The calculus answers *what is
committed and when*; it is silent on *how much anyone can lose*. Economic
exposure therefore has to be bounded by a mechanism outside the calculus, and
the mathematics of Layer 2 constrains what that mechanism can look like:
because balance-type guards do not glue [C-16], settlement is designed as a
serialized, one-shot, fail-closed step — a guarded fill against a single
ledger — rather than as a merge of concurrent claims.

### 4.2 The Dragon's Clutch design

The economic layer is owned by the Dragon's Clutch repository. Its design
documents commit to a specific shape [C-21]:

- **Full collateralization.** A participant deposits collateral and receives
  one claim for each cell of an **exhaustive, disjoint partition** of a future
  objective observable. The complete set of claims can always be recombined
  into its collateral before resolution. There is no margin, no leverage
  account, no liquidation, and no socialized loss to have.
- **A maximum-liability invariant.** The design's central promise is that for
  every reachable state, the market-local collateral vault (the Hoard) covers
  the maximum payout the market's immutable terms allow.
- **Frozen observation programs.** Resolution consumes a versioned, frozen
  observation specification. No reporter chooses the value: a transaction
  either carries uniquely qualifying evidence or is rejected, and every
  failure mode follows a frozen rule rather than granting discretion.
- **Prepaid liveness.** Every admitted market can observe, repair, finalize,
  and settle from resources prepaid at creation, even if later volume is
  zero.
- **Permissionless candidate work at batch close.** Clearing a batch is a
  frozen deterministic relation over a frozen book; submitting the clearing
  result is open work, and validity is checked by recomputation rather than by
  trusting the submitter.

### 4.3 What the prototypes actually do

The design is partially instantiated in offline Rust prototypes, and their
status should be stated exactly: they are offline research prototypes —
fifty-two tests pass across the repository's six Rust crates, re-run for this
document on 2026-08-18 — and the repository records no deployed program, no
key, no market, no financial authority, and no formal verification of the Rust
code [C-22]. Within that
boundary, three properties are present in the code rather than only in prose:

- **The solvency check is structural.** The kernel prototype computes required
  collateral as the maximum, over the market's immutable set of payout
  vectors, of the liability that vector implies at current claim supply,
  rounding against the protocol — and refuses, as an invariant violation, any
  state whose collateral is below that maximum. The check runs at market
  construction and around every state transition: split, merge,
  materialization, transfer, resolution, redemption [C-18].
- **Resolution has no discretionary input.** The kernel accepts only an index
  into the immutable payout set; the observation prototype is an
  interval-summary algebra that combines authenticated observations
  associatively and *refuses* to answer questions its retained information
  cannot support, rather than approximating them [C-18, C-20].
- **The batch relation is recomputable by anyone.** The batch prototype
  freezes its policy — price grid, tie rule, remainder rule — when the book is
  constructed. It derives a single canonical clearing candidate, and its
  verifier accepts a submitted candidate only if it matches what the frozen
  book itself determines, recomputed from scratch and never trusting the
  submitter's claimed quantities. Competition among submitters is competition
  to perform work, not to influence the result [C-19].

### 4.4 A miniature: a fully funded claim set

Deposit 100 units of collateral against a market whose partition has four
cells — say four disjoint ranges of an observable that will be known in
March. You receive four claims, one per cell, and the vault holds 100. At
every moment between now and March, the vault's balance is at least the
largest payout any resolution could require; the software refuses to enter any
state where that stops being true. If you change your mind before resolution,
the complete set of four claims recombines into the deposit. In March, the
frozen observation program identifies the realized cell; the claim on that
cell redeems by a guarded fill — one-shot, fail-closed, exactly the printed
transfer — and the other three redeem nothing.

At no point in this story does anyone owe an amount fixed later, and at no
point does the design make solvency depend on a future price, future volume,
or anyone's continued participation. That is the sense in which exposure is
bounded by prefunding rather than by the calculus: the calculus fixes the
*shape* of every event; the funding rule fixes the *ceiling* on every
liability, at the moment the liability comes into existence.

## 5. The milestone taxonomy that falls out

Putting the three layers side by side yields a five-milestone timeline. Each
milestone is a distinct mechanical event, owned by a distinct artifact, and
each answers a different one of Section 1's questions.

| Milestone | What becomes fixed | What it is, mechanically | Owning layer |
|---|---|---|---|
| **Publication** | The complete shape: parties, state, condition, effect, deadline, replay domain | Creation of a guarded hole / market template | Layer 1 [C-01, C-05] |
| **Funding** | The exposure ceiling | Collateral deposit; the maximum-liability invariant starts holding and is never allowed to lapse | Layer 3 [C-18, C-21] |
| **Close** | The candidate set stops growing | A frozen rule ends admission of new inputs; the book/observation window is sealed | Layers 2 and 3 [C-14, C-19] |
| **Finality** | One result out of the candidates | A licensed collapse: the stability premise is discharged and the seal survives everything that can still arrive, in the model | Layer 2 [C-13, C-14] |
| **Settlement** | The final ledger effect | A serialized guarded fill: one-shot by replay key, fail-closed, exactly the pre-committed transition | Layers 1 and 3 [C-02, C-07, C-16] |

Two features of this timeline are the ones a classification discussion should
engage with, because they are where it differs from the fused vocabulary:

**Shape and exposure are created at different moments, by different acts.** At
publication there is a complete, inspectable commitment structure and no
exposure: no collateral has moved, and there is nothing to perform. At funding
there is exposure, and its ceiling is fixed by the same act that creates it.
An analysis that needs to locate "when the commitment binds" and "when risk
exists" can point at two different mechanical events with different evidence
attached to each.

**Determination is separated from both.** The outcome is determined neither at
publication (the value does not exist) nor at funding (the vault covers every
cell symmetrically), but at finality — and finality, in this framework, is not
a moment when truth spontaneously arrives but an explicit collapse whose
legitimacy is exactly the strength of its stability license. Between close and
finality, the honest description of the system's state is a candidate set,
and the model treats that as a value, not a failure.

This taxonomy is offered as an interpretive framework: a description of the
time-structure the mathematical objects actually have, intended to make
questions about staged programmable commitments precise. It is not a legal
category, not a claim about how any instrument should be classified, and not a
property of any deployed system [C-24].

## 6. What this document does not establish

Each layer's evidence has a definite ceiling, and stating the ceilings is part
of the exposition, not a disclaimer appended to it.

**Model theorems are theorems about models.** Every Layer 1 and Layer 2 result
is a Lean statement about mathematical objects, holding under its stated
premises. Such a theorem does not establish: that any deployed cryptography
realizes the modeled commitments; that signatures, key custody, or consensus
behave as the model's abstract authorities do; that any physical system
executes a fill atomically; that a network delivers or a prover is sound; or
that any of it has legal effect. Where the model consumes a premise — most
importantly the stability license behind a seal — the conclusion is worth
exactly what the real-world evidence for that premise is worth, and the model
is silent on that worth [C-10, C-14, C-17].

**The prototypes are tested, not verified, and not deployed.** The Dragon's
Clutch code is an offline research prototype whose fifty-two tests pass; its
repository records no deployed program, no live market, and no formal
verification of the Rust code, and its own documents state that all parameters
remain hypotheses until proofs, benchmarks, simulations, and adversarial tests
pass [C-22].

**The layers do not presently compose.** The Lean models and the Rust
prototypes are separate research artifacts, in separate repositories, with
deliberately separate provenance. The prototypes are not mechanically derived
from the models, and no end-to-end system connecting staged commitment,
candidate collapse, and prefunded settlement exists in these repositories.
This is a bounded statement about the local state of the research record, not
a claim about what exists elsewhere or about what could be built [C-23].

**Value-bearing commitments are outside the calculus by design.** Nothing in
Layers 1 and 2 asserts anything about economic performance, solvency, or
recovery; the calculus was shaped so that it could not. Conversely, nothing in
Layer 3's funding rule is proven by the commitment theorems; it is enforced by
prototype code and stated as design. Neither layer borrows the other's
evidence [C-04, C-21].

**No legal conclusion is drawn anywhere in this document.** Whether any
mechanism described here, if ever built and operated, would fall within any
statutory category, and what obligations would attach, are questions for
counsel and for regulators, informed by facts about an actual deployment that
do not exist for these research artifacts. This document's ambition is
narrower: that when those questions are asked, the events they are asked
about have exact names.

---

## Appendix A — claims and sources

Labels follow the repository's claim discipline (`AGENTS.md`): VERIFIED
(directly inspected source, proof, or reproduced measurement), SOURCED (cited
source, not independently reproduced), INFERRED (stated deduction from
identified premises), PROPOSED (a design or framing, not a result).

Inspection record for this document, 2026-08-18. All Lean claims are
source-inspection claims: the theorem statements and definitions were read at
the commits below; no Lean build was executed for this document. Wording
ceilings from `docs/regulatory/DRAFT3_CLAIM_AUDIT.md` (V-03 through V-08)
apply to every row and were treated as binding.

| Repository | Commit at inspection | Working tree |
|---|---|---|
| /Users/ember/dev/breadstuffs | `44d0dea45349be20896ed3360a094866a3f62260` | dirty (unrelated files); all cited files unmodified |
| /Users/ember/dev/minidregg | `9db15e7da48c0ee6e5685b3c707fa1362895b802` | dirty (unrelated files); all cited files unmodified. Prior audit snapshot `bf45a611ec8f2bf401012376ea14b45827910b6f` recorded in DRAFT3_CLAIM_AUDIT.md V-04 |
| /Users/ember/dev/leanuweave | `f1450667cc87a48706c61f6d5ead71f73ab43fb1` | clean; commit matches DRAFT3_CLAIM_AUDIT.md V-06/V-07 |
| /Users/ember/dev/dragons-clutch | `fa4efb4e5a5a3ef14c6b8b33a949525928ae5a70` | clean |

| ID | Claim (as used in the body) | Label | Exact artifact |
|---|---|---|---|
| C-01 | The weak guarded-hole prototype fixes field, actor, target, and guard predicates at creation; only a value arrives later. | VERIFIED (source inspection; Lean model) | `/Users/ember/dev/breadstuffs/metatheory/Dregg2/Exec/GuardedHole.lean:37-56` (`GuardedHole`, `fillGuarded`) |
| C-02 | In the model, a successful fill is exactly the pre-committed transition with every promised guard discharged. | VERIFIED (source inspection; Lean model) | `/Users/ember/dev/breadstuffs/metatheory/Dregg2/Exec/GuardedHole.lean:59` (`holeFill_binds_in_circuit`) |
| C-03 | In the model, a guard-violating value produces no state change; the demonstration guard is two-valued (admits 50, rejects 55) and the rejection is exercised. | VERIFIED (source inspection; Lean model) | `/Users/ember/dev/breadstuffs/metatheory/Dregg2/Exec/GuardedHole.lean:67` (`holeFill_rejects_guard_violation`), `:77-89` (demo `#guard`s) |
| C-04 | "Determination is eager, witness is lazy" is the governing design verdict; value-bearing and authority-bearing holes are deliberately given no primitive; per-turn conservation is stated to hold because the hole moves no value. | SOURCED (design documents; the conservation theorem they cite was not independently inspected) | `/Users/ember/dev/breadstuffs/docs/DESIGN-partial-turn-promises.md` §§1, 2, 6 (esp. lines 40-45, 228-252); `/Users/ember/dev/breadstuffs/metatheory/docs/DREGG-CALCULUS.md` §§0, 6 |
| C-05 | In the Minidregg model, the full transition shape (eleven fields incl. type code, pre-root, authority demand, footprint, guard/effect commitments, deadline, replay domain) is fixed in `HoleSpec`; advice is dependently typed by that exact spec, and acceptance cannot substitute any eager field. | VERIFIED (source inspection; Lean model) | `/Users/ember/dev/minidregg/Theory/GuardedAdvice.lean:38-66` (`HoleSpec`, `Advice`), `:176` (`verify_accepted_binds_eager_shape`) |
| C-06 | In the Minidregg model, verification is total with distinguished non-accepting outcomes, and a rejection leaves durable state unchanged. | VERIFIED (source inspection; Lean model) | `/Users/ember/dev/minidregg/Theory/GuardedAdvice.lean:118-160` (`VerifyOutcome`, `verifyFill`), `:286` (`verify_rejected_no_mutation`) |
| C-07 | In the Minidregg model, a first commit consumes a replay key derived from the eager shape and a replay of the same verified fill is refused. | VERIFIED (source inspection; Lean model) | `/Users/ember/dev/minidregg/Theory/GuardedAdvice.lean:302` (`commit_then_replay_refused`) |
| C-08 | The strong hole is refuted as an explicit anti-model: admitting shape-carrying late data definitionally substitutes a different shape; the safe advice type has no such construction. | VERIFIED (source inspection; Lean model) | `/Users/ember/dev/minidregg/Theory/GuardedAdvice.lean:322-347` (`UnsafeStrongAdvice`, `strong_hole_substitution_refuter` at `:339`) |
| C-09 | In the Minidregg controller model, a commit intent binds every eager hole field plus the encoded advice, and keys outside the declared footprint are unchanged. | VERIFIED (source inspection; Lean model) | `/Users/ember/dev/minidregg/Theory/ReactiveController.lean:371` (`CommitIntent.request_binds_hole`), `:415` (`CommitIntent.frame`) |
| C-10 | Minidregg evidence is source inspection at a stated commit; no umbrella build was run for this document. | VERIFIED (repository inspection) | Commit table above; `/Users/ember/dev/degg-research/docs/regulatory/DRAFT3_CLAIM_AUDIT.md` V-04 |
| C-11 | In the Leanuweave model, a partial result is a grow-only set of candidate worlds, and deterministic evaluation commutes with merge under any order and batching. | VERIFIED (source inspection; Lean model) | `/Users/ember/dev/leanuweave/Uwueave/Holes.lean:555` (`evalSet_hom`), `:590-601` (`evalSet_fold`) |
| C-12 | Per-component candidate tracking is sound but strictly lossy: it manufactures results no candidate world justifies, on a concrete witness. | VERIFIED (source inspection; Lean model) | `/Users/ember/dev/leanuweave/Uwueave/Holes.lean:803` (`monadic_has_phantoms`) |
| C-13 | Determinacy is not preserved by coordination-free merging, on results or pulled back to inputs; demanding one answer is modeled as a coordination requirement. | VERIFIED (source inspection; Lean model) | `/Users/ember/dev/leanuweave/Uwueave/Holes.lean:856` (`determinacy_not_iconfluent`), `:899` (`determinate_result_not_iconfluent`) |
| C-14 | In the model, sealing is licensed exactly by a stability premise; an unlicensed seal can be false of the merged state; input stability transports to result stability. | VERIFIED (source inspection; Lean model) | `/Users/ember/dev/leanuweave/Uwueave/Holes.lean:937` (`SealsTo`), `:949` (`Stable`), `:956` (`seal_survives_stable`), `:968` (`unstable_seal_clash`), `:993` (`stable_inputs_seal_the_result`) |
| C-15 | In the model, a *spanning* guarded hole glues across divergent admissible fills iff its guard is I-confluent; Spanning is necessary (witnessed); non-confluent guards yield a constructed four-state clash. | VERIFIED (source inspection; Lean model; the Spanning and divergence qualifications are part of the claim) | `/Users/ember/dev/leanuweave/Uwueave/Gluing.lean:250` (`guardGluing_iff_iconfluent`), `:269` (`clashFill_of_not_iconfluent`), §5 witnesses (`stampedHole`) |
| C-16 | In the model, seam-segmented guards glue within fibers while failing globally; the budget/quota guard is a concrete instance (globally non-confluent, fiber-wise gluing). | VERIFIED (source inspection; Lean model) | `/Users/ember/dev/leanuweave/Uwueave/Gluing.lean:606` (`guardGluingSeam_iff_segmented`), `:627-643` (`budgetHole`, `budgetHole_partially_glues`); `/Users/ember/dev/leanuweave/Uwueave/Segmented.lean:76` (`budget_not_iconfluent`) |
| C-17 | Leanuweave evidence is source inspection at the audited commit, clean tree. | VERIFIED (repository inspection) | Commit table above; `/Users/ember/dev/degg-research/docs/regulatory/DRAFT3_CLAIM_AUDIT.md` V-06/V-07 |
| C-18 | The kernel prototype computes required collateral as the maximum liability over the immutable payout set (rounded up) and refuses any state below it, checking at construction and around every transition; resolution accepts only an index into the immutable payout set. | VERIFIED (source inspection + tests re-run 2026-08-18) | `/Users/ember/dev/dragons-clutch/crates/clutch-kernel/src/lib.rs:211-234` (`required_collateral`), `:253-264` (`check_invariants`), `:395` (`resolve`), crate doc header |
| C-19 | The batch prototype freezes policy at book construction, derives one canonical clearing candidate, and verifies submitted candidates by full recomputation, never trusting submitted quantities. | VERIFIED (source inspection + tests re-run 2026-08-18) | `/Users/ember/dev/dragons-clutch/crates/clutch-batch/src/lib.rs:1-40` (header, frozen policy), `:168-237` (`propose`, `verify`) |
| C-20 | The accumulator prototype is a bounded interval-summary monoid that returns explicit refusals for questions its retained information cannot answer. | VERIFIED (source inspection + tests re-run 2026-08-18) | `/Users/ember/dev/dragons-clutch/crates/clutch-accumulator/src/lib.rs:1-35` (header incl. information-theoretic boundary) |
| C-21 | The Dragon's Clutch design commits to full collateralization, exhaustive disjoint partitions with complete-set recombination, the Hoard maximum-liability promise, prepaid liveness, frozen observation programs without discretionary resolvers, and permissionless recomputable batch work; these are design documents, partially instantiated in the prototypes. | SOURCED (design documents) | `/Users/ember/dev/dragons-clutch/PROJECT.md` §§1, 4, 6, 7, 9; `/Users/ember/dev/dragons-clutch/README.md` |
| C-22 | The Dragon's Clutch repository is an offline research prototype: 52 tests pass across six Rust manifests (7 clutch-kernel, 10 clutch-accumulator, 9 clutch-batch, 7 research/vertical-model, 9 programs/solana-layout, 10 programs/solana-reference; re-run 2026-08-18); it records no deployed program, key, market, or financial authority, and its Rust code is tested but not formally verified (Verus not pinned, Rocq unavailable per its README). | VERIFIED (reproduced measurement + repository inspection) | `cargo test` per manifest under `/Users/ember/dev/dragons-clutch/{crates,research/vertical-model,programs}/`; `/Users/ember/dev/dragons-clutch/README.md` "Status" |
| C-23 | The three layers are separate research artifacts that do not presently compose into a production system; the Rust prototypes are not mechanically derived from or checked against the Lean models. Bounded local-state statement, not a universal negative. | INFERRED (from the repository status records above; wording per DRAFT3_CLAIM_AUDIT.md V-08) | `/Users/ember/dev/degg-research/docs/regulatory/DRAFT3_CLAIM_AUDIT.md` V-08; `/Users/ember/dev/dragons-clutch/PROJECT.md` §9 (non-entanglement); commit table above |
| C-24 | The five-milestone taxonomy (publication / funding / close / finality / settlement) as an interpretive framework for staged programmable commitments. | PROPOSED (framing offered by this document; not a legal category, compliance conclusion, or property of a deployed system) | This document, Section 5 |
| C-25 | Breadstuffs evidence is source inspection at the stated commit; cited files unmodified in a working tree that is dirty elsewhere. | VERIFIED (repository inspection) | Commit table above; `/Users/ember/dev/degg-research/docs/regulatory/DRAFT3_CLAIM_AUDIT.md` V-03 |

Provenance note: this document was written fresh in this repository. No source
code, theorem text, or fixture was copied from the Breadstuffs, Minidregg,
Leanuweave, or Dragon's Clutch repositories; theorem and definition names are
cited as identifiers with exact paths, and all descriptions are restatements
with attribution, per the repository-boundary rule in `AGENTS.md`.
