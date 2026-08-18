# FHE and Verifiable Private Computation Frontier

Status: SOURCED research map, reviewed 2026-08-17. Paper results have not been
independently reproduced here.

## The question

The project does not ask which library can execute arbitrary encrypted Rust. It
asks which recent constructions make a small family of exact market relations
materially cheaper, more verifiable, more distributed, or less dependent on a
standing trusted operator.

## 1. Exact approximate-arithmetic computers

Discrete CKKS work is relevant because it uses the packing and arithmetic shape
of CKKS while recovering exact discrete semantics through specialized
bootstrapping and encoding. The attractive possibility is a wide-SIMD market
backend where histogram construction, prefix/suffix scans, and fixed-point
relations operate over packed exact integers rather than bitwise TFHE gates.

Research targets:

- exact comparison cost;
- exact range and overflow behavior;
- cyclic versus saturated semantics;
- bootstrap frequency;
- packed crossing and select;
- proof compatibility;
- parameter/noise audit.

Primary leads:

- IACR ePrint 2025/066.
- IACR ePrint 2025/1440.
- SIMD homomorphic ALU, IACR ePrint 2026/233, with public implementation work
  from tsinghua-ideal/fhe-simd-alu.

Verdict: promising backend family, not yet a selected architecture.

## 2. Relation and representation changes

Several papers attack the cost model by changing representation rather than
merely tuning the same TFHE circuit:

- REFHE, IACR ePrint 2025/1449.
- Nested RNS constructions, IACR ePrint 2025/346.
- Matrix-oriented FHE, IACR ePrint 2025/1935.
- Low-rank functional bootstrapping, IACR ePrint 2026/1264.
- Multivariate functional bootstrapping, IACR ePrint 2026/1401.
- Sparse Hermite techniques, IACR ePrint 2026/1026.

Research question: can a market operator such as bounded monotone crossing or
piecewise allocation be encoded as a low-rank, multivariate, sparse, or
functional-bootstrap map more efficiently than a generic encrypted comparator
network?

Verdict: high-value paper studies; no local implementation claim.

## 3. Multi-key and threshold evolution

Dark Eggs needs more than one evaluator with one immortal secret key. Relevant
directions:

- CRS-less multi-key FHE, IACR ePrint 2026/322.
- HERDS, IACR ePrint 2025/1804.
- Dynamic multiparty FHE, IACR ePrint 2025/581.
- Ajax threshold work, IACR ePrint 2025/1834.

Questions:

- Can parties join and leave without exposing historic books?
- Can keys be refreshed without replaying all ciphertext state?
- What threshold assumptions survive adaptive corruption?
- Can aborting parties be replaced?
- Is decryption constrained to relation-authorized local outputs?
- What transcript proves correct resharing and release?

Verdict: essential to the Dark claim; currently a research gap.

## 4. Bootstrapping and composition

Leads:

- HasteBoots, IACR ePrint 2025/261.
- Laminate, IACR ePrint 2025/2285.
- Composable FHE, IACR ePrint 2024/1545.
- Lattice-isomorphism FHE, IACR ePrint 2026/465.
- Carousel, IACR ePrint 2024/2032.

The experiment question is relation-specific: do these constructions reduce the
depth/refresh bottleneck of packed aggregation, crossing, and select when
measured end to end with serialization and key residency?

## 5. Verifiability

FHE evaluation does not tell a relying chain whether the evaluator followed the
program. Candidate bridges:

- direct SNARK/STARK proof of the FHE evaluation relation;
- proof of a smaller algebraic certificate derived from encrypted computation;
- independently replicated computation with an optimistic dispute relation;
- relation-specific vFHE;
- proof-carrying threshold release.

Current practical vFHE work remains expensive enough that the project should
start with tiny fixed relations and prove only what settlement needs.

Minidregg's zero proof-suite/controller pins are a useful design discipline:
until proof semantics, codec, verifier, and history join exist, the system should
make the vFHE deployment uninhabitable rather than substitute an attestation.

## 6. Broken or weakened directions matter

Research records must include attacks and retractions. GRAFHEN-related claims
have received multiple follow-up attacks or corrections, including IACR ePrint
2026/700, 2026/1460, and 2026/1518. A paper's novelty or benchmark does not make
its security assumption safe for a custody-bearing protocol.

## 7. TPU and accelerator hypothesis

No local TPU implementation exists. The user's recollection concerns external
work. The accelerator program should search and reproduce external systems only
after the relation is frozen.

The highest-value hypothesis is not “TPUs run FHE.” It is:

    a packed specialized market relation may map unusually well to a dense,
    resident, batched accelerator pipeline.

The benchmark must include host/device transfer, key and table residency,
multi-batch amortization, memory pressure, exactness, and side-channel model.

## 8. Baseline from local prior work

Historical Breadstuffs evidence records a TFHE uniform-price kernel in the tens
to hundreds of seconds across small-to-medium N/K grids, while BFV aggregation
plus an MPC boundary was measured in the millisecond regime under a materially
weaker trust model.

That gap strongly motivates hybrids:

- linear encrypted aggregation;
- a tiny nonlinear boundary;
- proof or MPC for comparison;
- encrypted local allocation;
- explicit settlement proof.

It does not prove the hybrid is Dark. Preprocessing, collusion, proof production,
release, and liveness remain separate gates.

## 9. Selection criteria

A construction is interesting only if it improves at least one of:

- exact end-to-end latency;
- throughput;
- communication;
- setup/ceremony;
- dynamic participation;
- malicious security;
- proof cost;
- local-output privacy;
- permissionless verification;
- failure recovery;
- implementation/provenance freedom.

It must not quietly worsen custody, patent, deployability, or surveillance
centralization beyond the declared target.

