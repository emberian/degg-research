# Confidential Energy Dispatch Relation

Status: PROPOSED specialized confidential-computation relation with a VERIFIED
offline Clear oracle and direct-recomputation verifier at the exact bounds
below. No Dark, FHE, vFHE, MPC, threshold, proof, network, custody, private
delivery, regulator-query, or deployment system exists here.

Relation identifier:
`confidential-energy-dispatch/p3-t3-b2-q4/v0`.

Executable evidence:
[`research/confidential-energy-dispatch`](../../research/confidential-energy-dispatch/).

## 1. The thesis and its honest boundary

PROPOSED: multiple energy providers should be able to coordinate an efficient
physically admissible plan without revealing plant-level commercial and
operational facts beyond a frozen leakage function and their own authorized
local outputs. Candidate private facts include cost curves, capacity and
minimum-output constraints, ramp limits, forced outages, local dispatch,
settlement amounts, inventory, hedge positions, and other operational state.

VERIFIED only for the executable Clear slice: the crate models private *fields*
for two-segment costs, capacity/minimum, ramp, forced availability, provider
bus, output-recipient binding, dispatch, and credit. One ordinary Rust process
receives every field and computes every output. “Private” describes the target
relation visibility, not a property of this process. Storage, fuel inventory,
minimum up/down time, startup cost, network losses, reactive power, contingency
analysis, hedge books, and actual market/custody settlement are absent.

REJECTED: calling a named or threshold-decrypting executor Dark. Such a system
is Shielded because at least one named actor or coalition can learn the inputs.
REJECTED: treating a SHA-256 commitment, fixed padding, typed ciphertext, or
signed result as a confidentiality or computation-correctness proof.

## 2. Frozen v0 domain

Changing any row creates a new relation version.

| Parameter | v0 value |
|---|---:|
| Provider slots | exactly 3, canonically padded |
| Planning periods | exactly 3 |
| Buses | 2, joined by one lossless line |
| Provider output | integer atoms in `0..=4` |
| Cost curve | 2 sequential integer-width segments |
| Marginal cost | `0..=1,000,000` quote atoms per energy atom |
| Commitment choice | implicit: zero output is off, positive output respects minimum |
| Ramp rule | directional bounds between consecutive online states |
| Forced outage | output zero; each outage boundary bypasses ordinary ramping |
| Reserve | system-wide upward capability under section 5 |
| Settlement | pay exact modeled production cost; zero fees |
| Optimization | minimum total modeled production cost |
| Equal-cost tie | lexicographically greatest provider-major generation vector |
| Arithmetic | checked exact integers; no floating point |

The tie sends otherwise interchangeable output to the earliest provider slot
first. This is a deterministic canonicalization rule, not an economic claim
that slot priority is fair. A future production relation should either justify
priority rights or replace the rule under a new identifier.

### 2.1 Public request

The public request contains:

- nonzero instance binding and public coarse epoch;
- exact demand for each period and bus;
- system upward-reserve requirement per period;
- absolute lossless-line capacity per period;
- one domain-bound SHA-256 commitment to all three provider slots;
- one statement that external admission finalized that commitment; and
- one statement that all accepted fixed-size payloads are available.

The two statements are booleans in the Clear model, not proofs. The model has no
admission log, non-equivocation mechanism, consensus fact, data-availability
network, timeout, recovery, or refund path. A composed system must supply those
facts without allowing an evaluator to omit an inconvenient provider.

### 2.2 Private witness

Each occupied provider supplies:

- a nonzero local-output recipient binding, unique in the batch;
- bus `0` or `1`;
- minimum and maximum output;
- ramp-up and ramp-down bounds;
- output immediately before the horizon;
- three forced-availability bits; and
- two nondecreasing marginal-cost segments whose widths sum to maximum output.

An unoccupied slot is byte-for-byte canonical zero padding. Latent data in
padding refuses rather than being ignored. The accepted-input commitment is
bound to the full public domain, so an old witness or result cannot be replayed
under another epoch, demand, reserve, line policy, or instance.

VERIFIED: the canonical plaintext witness is exactly 156 bytes whether zero,
one, two, or three slots are occupied. This does not specify a ciphertext or
proof size; each future backend must freeze and test its own padded wire shape.

SHA-256 commitments provide binding under the hash assumption, not hiding. The
input domain is small enough for dictionary attacks when low-entropy fields are
guessed. A real confidential submission needs encryption and commitment
blinding or another hiding construction.

## 3. Provider trajectory semantics

For provider `i`, period `t`, and output `q[i,t]`:

```text
if padding or unavailable: q[i,t] = 0
if available:              q[i,t] = 0 or min[i] <= q[i,t] <= max[i]
```

From the public pre-horizon state to period zero, and between two consecutive
available periods, increases are at most `ramp_up` and decreases are at most
`ramp_down`. A boundary touching a forced-unavailable period bypasses the ramp
test. This gives an emergency outage exact semantics rather than making an
abrupt forced shutdown an invalid witness. It does not model the physical
transient, restart time, damage, or replacement power.

Production cost fills segment zero, then segment one:

```text
cost_i(q) = mc[i,0] * min(q, width[i,0])
          + mc[i,1] * max(0, q - width[i,0])
```

Segment marginal costs must be nondecreasing. Nevertheless the relation is not
a convex LP: `q = 0 or q >= min` is an integer commitment decision, and forced
availability and ramp coupling restrict the trajectory set.

## 4. Nodal balance and line flow

Positive line flow is bus 0 to bus 1. For every period:

```text
flow[t] = generation_at_bus_0[t] - demand[t,0]
sum_i q[i,t] = demand[t,0] + demand[t,1]
abs(flow[t]) <= line_limit[t]
```

Those equations imply exact balance at bus 1 for a lossless two-bus system.
There is no load shedding, partial result, or “best effort” schedule. Failure to
serve all demand under all other constraints produces `Infeasible`.

## 5. Reserve rule

This v0 reserve is intentionally narrow. For an available provider already
producing, upward capability is:

```text
min(max_output - q, ramp_up)
```

For an available but off provider it is zero when `min_output > ramp_up`, and
otherwise the same formula. Unavailable and padded slots contribute zero. The
sum must meet the public system reserve requirement.

This is not deliverability-aware nodal reserve, an N-1 criterion, a response
time model, or a second feasible recourse schedule. Those stronger notions
need new public policy and witness semantics.

## 6. Objective, optimality, and the counterexample boundary

The objective is the checked sum of all provider production costs across all
three periods. At equal objective, the provider-major generation vector is
compared lexicographically and the greater vector wins, giving earlier slots
priority.

VERIFIED: the Clear oracle enumerates every valid three-period trajectory for
each provider. It iterates the first two provider trajectories and derives the
only third trajectory that could exactly meet total demand. It then checks
nodal balance, line capacity, reserve, cost, and the tie. The representative
fixture visits 8,025 first-two-provider pairs and finds 468 feasible schedules.
Its unique canonical result is:

```text
generation = [[2,2,2], [1,3,2], [2,0,2]]
line flow  = [0,-1,0]
reserve    = [6,3,6]
credits    = [18,30,8]
objective  = load debit = 56
```

VERIFIED adversarial counterexample: `[[1,2,1], [2,3,3], [2,0,2]]` is also
physically feasible and exactly conserving, but costs 60. The untrusted-plan
assembler recomputes every commitment around that schedule. The verifier still
rejects it as `NotCanonicalOptimum` only after recomputing the exhaustive
optimum.

Two smaller counterexamples explain why a per-period marginal-cost sort is not
the relation. First, a one-atom demand cannot select a cost-1 provider whose
minimum output is two; the exact optimum selects a cost-9 provider that can
produce one. Second, under demand `[1,3,1]`, a cost-1 provider with ramp bounds
one cannot follow the independent merit-order schedule `[1,3,1]`; the optimum
is its `[1,2,1]` plus one middle-period atom from a cost-10 provider, total cost
14. Both are executable tests. Feasibility and optimality couple decisions
across output levels and periods even before richer unit commitment is added.

This cleanly separates three claims:

1. Feasibility checks prove that one proposed schedule satisfies constraints.
2. Settlement checks prove that its exact credits and debit conserve.
3. Global optimality requires comparison with every admissible alternative or
   a sound replacement certificate/proof.

The checked `OptimalityCertificate` stores bindings, claimed objective, and
search counters. It is not a cheap certificate: the verifier reruns the oracle
and compares the exact plan and counters. An asserted objective, transcript
count, committee signature, or FHE ciphertext does not close optimality.

INFERRED: for this tiny relation, direct recomputation is lower-complexity
engineering than introducing a succinct proof system. At useful horizons,
provider counts, network size, or unit-commitment detail, exhaustive
recomputation will not scale. A later relation could study branch-and-bound
proof logs, exact mixed-integer certificates, a declared approximation bound,
or a convex dispatch subset with primal/dual certificates. None is implemented.

## 7. Exact settlement and local outputs

For each occupied provider, the private local output contains its recipient
binding, three-period dispatch, and exact pay-as-cost credit. Padding receives a
canonical zero output. The load-side local output contains total served energy
and one debit. The invariant is:

```text
sum(provider credits) = load debit = objective cost
```

The public result commits to the plan and to all padded local outputs. A test
changes the load debit by one and recomputes the surrounding commitments; the
verifier rejects exact nonconservation. Another test changes line flow or
reserve while recommitting and receives the named physical defect.

PROPOSED: an encrypted backend should deliver only each provider's authorized
local output and the load-side output. The crate merely returns a Rust struct to
the one Clear process. It does not demonstrate selective decryption, output
authorization, fair delivery, custody, payment finality, dispute resolution,
or a no-stranded-value abort.

## 8. Frozen public leakage surface

Every result serializes to exactly 176 bytes:

- relation commitment;
- public-domain commitment;
- accepted-input commitment;
- plan commitment or zero;
- padded local-output-set commitment or zero;
- one coarse status;
- four settled-only invariant bits: demand, line, reserve, conservation; and
- fixed reserved bytes.

Statuses are `MalformedPublic`, `InputUnavailable`, `WitnessRefused`,
`Infeasible`, `ArithmeticRefused`, and `Settled`. Detailed provider defect,
slot, cost, capacity, bus, outage, dispatch, credit, iteration count, and exact
wall time are excluded from the frame. Exact demand, reserve, line limits,
instance, and epoch remain public because the request itself is public.

VERIFIED: success, unavailable-input, and infeasible executions render the same
176-byte shape. Search counters exist only in the Clear executor's candidate
object. This proves a serialization property only.

REJECTED: claiming the Clear implementation meets the proposed timing surface.
Its enumeration, branches, memory access, and wall time depend on private
provider constraints. A Dark backend must freeze message count and size, pad or
otherwise protect evaluation and local-delivery timing, address traffic and
endpoint leakage, and define a corruption model. Public `Infeasible` itself is
an intentional one-bit-or-more disclosure about the joint private state.

## 9. Refusal order and liveness boundary

The Clear executor checks in this order:

1. public domain;
2. admission finality;
3. payload availability;
4. accepted-input commitment opening;
5. private witness semantics;
6. exhaustive feasibility and optimality;
7. derived delivery and settlement invariants.

Only the coarse public status is projected. A future backend must not leak a
secret slot or rule through diagnostic strings, ciphertext/proof size, retry
count, or response timing.

The model terminates offline because every domain is finite. It says nothing
about who pays for encrypted evaluation, proof production, data availability,
or local delivery; who may retry; what deadline applies; or how reserved funds
are refunded after unavailability or exhaustion. Those are required before the
relation can be joined to custody.

## 10. Operation decomposition for future backends

The relation is not “run an arbitrary optimizer under FHE.” Its operators split
as follows.

Mostly additive or linear once a candidate trajectory is selected:

- aggregate generation by period and bus;
- total demand and balance residuals;
- provider and system cost accumulation;
- reserve-capability accumulation;
- provider-credit and load-debit conservation; and
- fixed-shape commitment preimage construction outside an arithmetic circuit.

Comparison, selection, or combinatorial boundary:

- canonical padding and range checks;
- off versus minimum-output disjunction;
- outage masking;
- directional ramp comparisons;
- sequential segment `min` and residual selection;
- absolute line-limit comparison;
- reserve-capability `min` and off-state condition;
- feasible-trajectory membership;
- global minimum-cost comparison; and
- equal-cost lexicographic argmin tie.

The last group, especially feasible-set search and argmin, is the likely cost
center. A packed additive scheme may make the first group cheap while leaving a
small nonlinear MPC, TFHE, or proof boundary. A generic all-TFHE translation
would pay for many comparisons and selections. Approximate arithmetic cannot be
substituted silently: feasibility, ties, settlement, and conservation are exact
integer semantics.

SOURCED local-lineage context, not a measurement of this relation: read-only
inspection on 2026-08-19 of Breadstuffs
`fhegg-fhe/MEASURED-ENVELOPE.md` at its last modifying commit
`c5913c4713613812f63e9e5d58b05e04d331e93a` and file SHA-256
`264a19c9fa895925e84a8a1804c73640ecf24861000afc3298982d48db91a454`
records exact-integer CPU TFHE uniform-price clears of 116.5 seconds at
N=32/K=64 and 297.9 seconds at N=128/K=64. Its separate six-node/eight-edge
PDHG flow-LP experiment records 6.57 seconds per FHE iteration and about eleven
minutes for 100 iterations. It also reports a materially faster additive BFV
fold plus nonlinear MPC boundary under a weaker trust/composition model. Those
are historical, different-relation measurements on named hardware and software;
they neither benchmark nor implement this energy relation. The broader
provenance boundary is in [`LOCAL_LINEAGE.md`](../LOCAL_LINEAGE.md), and the
backend research map is in [`FHE_FRONTIER.md`](../FHE_FRONTIER.md).

## 11. Clear, future FHE/vFHE, and MPC architectures

### Clear oracle: implemented

One process sees public and private inputs, performs exact search, emits all
local outputs, and can be independently rerun. This is strong semantic evidence
and no confidentiality evidence.

### Named or threshold MPC: proposed Shielded architecture

A committee could secret-share provider inputs and evaluate the frozen
relation. Unless the corruption threshold ensures that no allowed actor or
coalition learns more than the frozen leakage, this is Shielded. Malicious input
validity, abort, share withholding, local-output release, dynamic membership,
and custody joins remain separate requirements. Committee signatures attest to
the committee; they do not prove computation.

### FHE: proposed Dark candidate only

Providers could encrypt fixed-shape inputs under a key whose custody and release
policy is separately defined. Evaluation would need exact semantics for the
operator list above, fixed traffic, selective local-output release, and a
binding to the accepted input set. FHE confidentiality alone does not stop an
evaluator from returning a feasible but suboptimal schedule or omitting an
input. No FHE parameters, ciphertext codec, evaluator, key ceremony, threshold
release, or benchmark exists for this relation.

### vFHE or proof-carrying result: proposed

A sound public verifier ultimately needs to bind:

```text
relation version
+ public domain
+ finalized/available accepted-input commitment
+ exact private-input validity
+ exact optimal plan under the frozen tie
+ padded local-output commitment
+ settlement conservation
```

The proof must not publish private fields and must be joined to authenticated
source, availability, local output delivery, and settlement. A proof of
feasibility without optimality accepts the tested 60-cost forgery. A proof of
FHE evaluation is useful only if the circuit implements this exact relation and
the encrypted inputs open the finalized commitment. No such proof exists here.

### Hybrid hypothesis: proposed

The most plausible experiment is encrypted or shared additive aggregation plus
a tightly bounded nonlinear feasibility/argmin boundary, followed by a proof
binding the result and private delivery commitments. Whether this is actually
faster or has an acceptable trust model is an experiment, not a conclusion.

## 12. Storage and richer unit commitment: explicit non-claims

Storage is excluded from v0. Adding it requires freezing at least charge and
discharge power, energy capacity, initial and terminal inventory, exact
efficiency arithmetic, degradation cost, simultaneous charge/discharge policy,
reserve provision, and settlement. Adding a signed “storage flow” without those
rules would create free energy or ambiguous settlement.

Likewise, v0 has no startup/no-load costs, minimum up/down duration, warm/cold
start, fuel constraints, emissions, bilateral obligations, hedges, or security
constrained recourse. Those features make global optimality harder, not merely
the witness larger. A later relation should add one coherent feature family at
a time and retain the Clear oracle as a differential reference at reduced
bounds.

## 13. Adversarial evidence and reproduction

VERIFIED tests cover:

- omitted/substituted provider input;
- infeasible demand without partial dispatch;
- fixed public shape and omitted internal timing/search fields;
- fixed 156-byte occupied/padded plaintext witness shape;
- feasible but forged optimum;
- exact settlement nonconservation after recommitment;
- overflow-shaped marginal cost rejected before multiplication;
- untrusted maximum-width settlement integers rejected without wrap or panic;
- deterministic equal-cost tie;
- epoch/domain replay;
- forged objective and search counters;
- minimum-output and interperiod-ramp merit-order counterexamples;
- forced-outage versus elective-ramp semantics;
- invalid pre-horizon minimum state and refusal priority;
- noncanonical padding and duplicate recipient binding;
- line and reserve field forgeries;
- byte-deterministic repeated execution;
- a separate full `5^6` Cartesian search matching the optimized
  derive-the-third-provider oracle on the two-provider fixture;
- published SHA-256 known-answer vectors; and
- byte-identical checked-in corpus reproduction.

From `research/confidential-energy-dispatch`:

```sh
cargo test --offline --locked
cargo clippy --offline --locked --all-targets -- -D warnings
cargo fmt --check
RUSTDOCFLAGS="-D warnings" cargo doc --offline --locked --no-deps
cargo run --quiet --offline --locked --bin degg-energy-dispatch-vectors \
  | cmp - vectors/v1.txt
```

Passing those gates proves the bounded Clear semantics and exercised
counterexamples. It does not prove the absence of bugs, independent oracle
agreement, scalability, privacy, cryptographic security, optimal power flow,
economic incentive compatibility, regulatory compliance, or production
readiness.
