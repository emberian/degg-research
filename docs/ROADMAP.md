# Research Roadmap

Status: PROPOSED. This roadmap authorizes no deployment or filing.

## Phase R0 — Memory and truth

Complete when:

- single current verdict exists;
- local lineage and licensing are audited;
- claim ledger is live;
- source bibliography is primary-source-first;
- Dragon's Clutch and Minidregg roles are explicit;
- old implementation cannot silently enter the greenfield codebase.

Current status: largely scaffolded. Minidregg AGPL selection is recorded;
contributor and third-party notice audits remain.

## Phase R1 — One exact relation

Complete when:

- Clearing V0 semantics are reviewed;
- allocation/tie/remainder choices are frozen;
- two clear implementations agree;
- exhaustive N4/K4 vectors exist;
- formal properties and falsifiers are named;
- no fee, privacy, or chain concerns are mixed into the base oracle.

## Phase R2 — Relation IR

Complete when:

- types and operators have denotations;
- visibility and leakage are first-class;
- compiler refusal surface is frozen;
- clear backend compiles;
- receipts and canonical bytes are stable;
- settlement interface is typed.

## Phase R3 — Shielded baseline

Complete when:

- fixed-size multi-process execution works;
- input validity is maliciously checked;
- committee and preprocessing trust are explicit;
- abort/recovery tests pass;
- per-owner outputs are delivered;
- no Dark claim appears.

## Phase R4 — Backend bakeoff

Complete when:

- TFHE, BFV/hybrid, discrete CKKS, and proof-carrying clear candidates have
  comparable artifacts;
- identical vectors produce identical semantics;
- costs include setup, communication, proof, and failure;
- leakage and patents/provenance are reviewed;
- one or more candidates are rejected with reasons.

## Phase R5 — Verifiable private result

Complete when:

- exact input root and relation identity are bound;
- result correctness is independently verifiable or disputable;
- proof producer need not learn more than declared;
- private local outputs are bound to the public result;
- invalid evaluator output cannot settle.

## Phase R6 — Availability and permissionless operation

Complete when:

- inclusion and non-equivocation receipts exist;
- encrypted input data is recoverable under the model;
- no privileged builder can choose the input subset silently;
- key rotation, party loss, and abort have bounded behavior;
- any required crank can be performed by independent parties;
- incentives do not fund guarantees from hoped-for volume.

## Phase R7 — Synthetic settlement

Complete when:

- a proof-carrying result drives a synthetic asset ledger;
- custody and computation are separate;
- conservation, fees, nullifiers, and retries are tested;
- public versus private settlement leakage is measured;
- no real funds or chain deployment occur.

## Phase R8 — Formal closure

Complete when:

- semantic relation is stated in Lean or Rocq;
- executable clear kernel has a Verus or comparable proof;
- compiler/backends carry refinement evidence;
- cryptographic assumptions and local-output noninterference are explicit;
- proof gaps are machine-uninhabitable rather than represented by zero-value
  assurances.

## Phase R9 — Public test artifact

Requires separate authorization and gates:

- independent security review;
- provenance/reproducible build;
- legal/counsel and regulator posture;
- non-production network or simulator;
- no customer funds;
- public limitations and incident process.

## Phase R10 — Deployment question

Not implied by any earlier phase. A technically excellent research artifact may
remain deliberately undeployed while still succeeding as public infrastructure,
formal science, and a regulatory contribution.

## First-class confidential energy coordination track

This track is not an energy-themed instance of the frequent-batch auction. Its
goal is a specialized relation in which providers can coordinate one efficient,
physically admissible operating plan without disclosing operational and
commercial inputs beyond a frozen leakage function and authorized local
outputs.

Energy E0 — bounded semantic oracle: VERIFIED only at
`confidential-energy-dispatch/p3-t3-b2-q4/v0`. The dependency-free Clear model
freezes integer dispatch, costs, minimum/capacity, ramps, forced outages,
two-bus balance, line limits, reserve, optimality/tie, local outputs, and exact
pay-as-cost conservation. Its verifier repeats exhaustive search; no privacy
backend exists.

Energy E1 — semantic enlargement and independent reference:

- add storage only with charge/discharge, exact efficiency, capacity, initial
  and terminal inventory, degradation, reserve, and settlement frozen together;
- separately version startup/no-load cost, minimum up/down time, network loss,
  security constraints, and load-shedding policy rather than silently widening
  v0;
- build an independently authored oracle and formalize feasibility,
  conservation, canonical tie, and refinement at reduced bounds; and
- freeze physical-source authentication, missing-input refusal, timeout, and
  refund behavior.

Energy E2 — confidential backend bakeoff:

- decompose additive aggregation from comparisons, projections, commitment
  choices, and global argmin;
- compare exact FHE, hybrid additive-encryption plus nonlinear MPC,
  malicious-secure MPC, and proof-carrying Clear candidates against identical
  vectors;
- freeze corruption, key custody, traffic, timing, ciphertext/proof shape, and
  local-output leakage; and
- call a named or threshold-capable viewer Shielded, never Dark by implication.

Energy E3 — verifiable private result:

- bind relation version, physical policy, finalized available encrypted inputs,
  exact feasibility, canonical global optimum or declared bound, local-output
  commitment, and settlement conservation;
- demonstrate that a feasible but suboptimal result cannot settle;
- distinguish a primal/dual or branch-and-bound certificate from an assertion,
  counter, signature, or proof of the wrong circuit; and
- measure direct recomputation before adding succinct-proof overhead.

Energy E4 — composed settlement and liveness:

- deliver only authorized provider and load outputs;
- fund evaluation, proof, availability, retry, and terminal refund work without
  relying on future volume;
- join private computation to custody without reconstructing the operational
  graph publicly; and
- keep real grid operation, customer funds, deployment, and regulatory claims
  outside the research artifact unless separately authorized and evidenced.

## Parallel regulatory track

R-Reg0:

- technical factual matrices;
- current docket map;
- public/private redaction boundary;
- non-counsel draft comments;
- meeting questions.

R-Reg1:

- counsel review;
- user completion of public identity;
- current-deadline revalidation;
- deliberate decision whether to file each nonduplicative comment.

R-Reg2:

- Innovation Task Force contact;
- division-specific questions;
- complete material facts;
- no-action, interpretive, exemptive, registered-pilot, or other pathway only if
  appropriate and separately authorized.
