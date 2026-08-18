# Experiment Program

Status: PROPOSED.

## Objective

Identify the smallest market relations for which confidentiality, correctness,
availability, and settlement can be made jointly credible at useful cost.

The program does not select a favorite cryptosystem in advance. It compares
backends against identical relations, leakage contracts, vector corpora, and
receipts.

## E0 — Clear semantic oracle

Deliver:

- one exact written relation;
- bounded integer domains;
- total refusal semantics;
- exhaustive enumeration at the smallest dimensions;
- property-based tests at larger dimensions;
- stable canonical vectors;
- independent implementation cross-check.

Properties:

- deterministic price and allocation;
- no overflow or wrap;
- every fill respects its limit;
- side totals match;
- conservation;
- fee conservation;
- no replay;
- no unassigned rounding value;
- permutation behavior matches the specified priority policy;
- malformed or incomplete input refuses.

Stop if the clear relation is ambiguous.

## E1 — Cost decomposition

Measure each operator separately:

- admission/range checks;
- ciphertext construction;
- demand/supply aggregation;
- prefix/suffix scan;
- comparison/crossing;
- argmax/tie-break;
- allocation;
- proof production;
- proof verification;
- threshold release;
- local-output delivery;
- transcript construction;
- settlement verification.

Report wall time, throughput, memory, communication, serialized bytes, setup,
amortization, failure rate, and energy when observable.

## E2 — Backend bakeoff

Candidates:

- clear exact Rust;
- proof-carrying clear solver;
- BFV/BGV aggregation plus MPC boundary;
- discrete CKKS exact-integer candidates;
- TFHE boolean/integer baseline;
- relation-specific functional bootstrapping;
- Shielded MPC;
- hybrid encrypted aggregation plus ZK result proof.

Matrix:

- owners N = 4, 8, 32, 128;
- ticks K = 4, 16, 64, 256;
- quantities q bits = 4, 8, 16, 32;
- padding occupancy = 25%, 50%, 100%;
- one and multiple markets;
- one-shot and amortized key/setup runs;
- honest, malformed, boundary, overflow, and abort cases.

Every backend consumes the same canonical input vectors and must match exact
output bytes or a named permitted representation mapping.

## E3 — Leakage and topology

For every backend, run a red-team worksheet:

- What can the submitter infer from receipts?
- What can an evaluator infer from ciphertext shape and traffic?
- What can key holders infer alone and in coalition?
- What can a proof producer infer?
- What can a chain observer infer?
- What can a settlement recipient infer?
- What do timing, retries, failures, padding, and memory reveal?
- Does chosen-ciphertext or adaptive-query behavior create an oracle?
- Can a user probe another position through carefully chosen orders?

No performance result is accepted without this worksheet.

## E4 — Inclusion and censorship

Build synthetic multi-process tests:

- accepted order omitted from proposed root;
- two users receive inconsistent batch roots;
- root published but ciphertext unavailable;
- batch builder withholds a losing/winning order;
- committee aborts after learning partial output;
- decryption share withheld;
- proof producer stalls;
- invalid result proposed repeatedly;
- inclusion receipt arrives near cutoff;
- delayed network delivery crosses epoch boundary.

Metrics:

- detection;
- recoverability;
- user refund;
- bounded loss;
- time to final outcome;
- party able to censor;
- evidence available to third parties.

## E5 — Guarded holes and partial computation

Create factual mechanism cases:

1. Late value fills a non-economic document field.
2. Late proof releases an already-fixed service action.
3. Late price observation determines a claim payout.
4. Late event determines whether escrow transfers.
5. Late advice chooses among multiple economically different actions.
6. A partial computation is tradable before completion.
7. A hole can be assigned, transferred, or bundled.
8. The protocol charges for creating, filling, or trading the hole.

For each, record:

- who gives consideration;
- who bears risk;
- what future contingency matters;
- whether payoff is binary, bounded, linear, or path-dependent;
- whether the holder can transfer the right;
- who defines terms;
- who supplies or verifies the fill;
- who operates admission and matching;
- what happens on non-resolution.

This corpus serves both mechanism design and regulatory discussion.

## E6 — Verifiability

Compare:

- direct recomputation;
- succinct proof;
- proof of a certificate;
- committee attestation;
- optimistic dispute;
- replicated independent evaluation;
- vFHE candidate.

Never equate signatures with a proof of computation. Record exactly which actor
could cause an invalid result to settle.

## E7 — Accelerator research

An accelerator experiment must include:

- relation and operator being accelerated;
- arithmetic representation;
- transfer and serialization overhead;
- setup/key residency;
- batch reuse;
- resource utilization;
- exact-output differential check;
- side-channel and multi-tenant assumptions.

The first question is not “is GPU/TPU faster?” It is “which relation operator
becomes the bottleneck after aggregation and specialization?”

## E8 — Deployment-independent economics

Model:

- fixed setup and ceremony cost;
- per-batch evaluator cost;
- proof cost;
- committee compensation;
- data availability;
- chain verification;
- failed/aborted batches;
- padding overhead;
- liveness reserve;
- user fee elasticity;
- maker incentives;
- adversarial wash and Sybil behavior.

Safety must not depend on expected future trading volume. Guaranteed service
work is prepaid or the protocol explicitly refuses admission.

## Promotion rules

A candidate moves forward only if:

- exact outputs match the clear oracle;
- failure modes are typed and exercised;
- leakage is frozen;
- the cost includes setup and communication;
- no indispensable role is hidden;
- the settlement join is specified;
- its provenance is clean;
- its strongest claim survives an adversarial review.

The winner is not the fastest cryptosystem. It is the lowest-total-trust design
that meets a useful relation's cost and liveness envelope.

