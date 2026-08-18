# Relation-Oriented Architecture

Status: PROPOSED.

## 1. Semantic layer

The semantic layer defines the economic state transition without cryptography,
chain accounts, serialization, committees, or accelerators.

For a batch auction it fixes:

- asset and collateral units;
- admissible order fields and bounds;
- batch identity and cutoff;
- price grid;
- eligibility and reserved balances;
- clearing objective and deterministic tie-break;
- marginal allocation and rounding;
- fees;
- cancellations and non-reveals;
- resulting balances, positions, and public outputs.

The semantic relation is total: malformed input, unavailable data, overflow,
ambiguous crossing, and insufficient collateral produce named refusal states.

## 2. Dark Relation IR

The IR represents the operators that matter to the relation:

- bounded vector admission;
- boolean and range constraints;
- exact integer add and compare;
- segmented or SIMD reduction;
- prefix and suffix scans;
- monotone crossing;
- argmax with canonical tie-break;
- bounded multiply and divide with named remainder;
- select and conditional move;
- permutation and inclusion constraints;
- Merkle or polynomial commitment openings;
- conservation and balance update;
- encrypted local-output projection;
- public-output projection;
- receipt and domain separation.

Every operation carries:

- exact bit width and signedness;
- overflow behavior;
- visibility class;
- owner or recipient of a local output;
- whether shape, value, or both are secret;
- proof obligation;
- backend cost estimate;
- deterministic serialization.

The IR has no generic recursion, heap, dynamic allocation, arbitrary pointer
access, or unconstrained user program in the first version.

## 3. Backend layer

### Clear backend

An exact safe-Rust oracle is the reference implementation. It is deterministic,
bounded, no-std compatible where practical, and produces exhaustive receipts.
This backend supplies test vectors and permits Verus-oriented verification.

### Proof backend

An untrusted solver may propose a result and a certificate. The verifier checks
input commitment, feasibility, conservation, objective/tie conditions, fees,
allocation, and output root. This is the preferred path whenever verify-not-find
is materially cheaper than encrypted search.

### Shielded MPC backend

A named committee executes the relation over shares. The mode states whether
the committee learns inputs, intermediate values, outputs, or only shares.
Preprocessing, robustness, transport, enrollment, and abort are part of the
backend contract.

### FHE backend

Candidate families include discrete CKKS integer computation, BFV/BGV-style
linear aggregation with a nonlinear boundary, TFHE as a comparison baseline,
and relation-specific functional bootstrapping. Parameters and noise bounds are
part of every artifact.

FHE correctness alone is not verifiable computation. A backend must identify
how a chain or relying party distinguishes correct evaluation from garbage:

- succinct vFHE proof;
- proof of a committed execution trace;
- multiple independent evaluators plus a sound dispute system;
- threshold attestation, explicitly weaker;
- or an honest statement that correctness is not yet trustless.

### Accelerator backend

GPU, FPGA, TPU, and SIMD CPU implementations are backends, not semantics. An
accelerator result is accepted only through the same vector corpus and receipt
contract as the clear oracle.

## 4. Transcript layer

Every run has a content-addressed transcript:

- relation identifier and version;
- semantic-spec digest;
- backend identifier and version;
- parameter digest;
- input commitment and availability commitment;
- accepted-input count and padded capacity;
- cutoff and time domain;
- public leakage declaration;
- output commitment and public output;
- per-recipient local-output commitment;
- proof, attestation, or explicit absence;
- refusal or abort status;
- resource measurements;
- source and build commits.

The transcript never claims more privacy than the backend and topology deliver.

## 5. Settlement layer

Computation and settlement are separate relations joined by exact identifiers.
Settlement verifies:

- the accepted input root;
- the relation/program identifier;
- the exact output root;
- authorization and nullifiers;
- asset conservation;
- limit satisfaction;
- fee schedule;
- replay resistance;
- phase transition;
- custody availability.

Public SPL settlement reveals account and amount information. A private
computation followed by ordinary public transfers is pre-trade dark at most.
Post-trade darkness requires shielded notes, confidential token support, or
another explicitly modeled settlement layer.

## 6. Availability and inclusion layer

Darkness cannot excuse hidden omission. Before a batch closes, users need:

- an inclusion commitment or receipt;
- a canonical batch root;
- a non-equivocation rule;
- a data-availability or recoverability contract;
- a timeout/refund path;
- a censorship-evidence story.

A correct proof over an operator-selected subset is not a fair market.

## 7. Formal refinement chain

The intended chain is:

    economic relation
      refines
    Dark Relation IR denotation
      refines
    backend computation and receipt
      joins
    settlement transition

Proof responsibilities:

- functional correctness;
- exact arithmetic and no wrap;
- collateral and position conservation;
- deterministic tie-break;
- inclusion completeness;
- public leakage noninterference;
- local-output correctness;
- replay and phase safety;
- backend-specific cryptographic assumptions.

No single tool is expected to prove the whole chain. Cross-tool boundary
artifacts must be content-addressed and independently checked.

## 8. First vertical slice

The smallest serious slice is a fixed four-owner, four-price, one-outcome-pair
frequent batch:

- quantities 0 through 15;
- fixed padded order slots;
- one uniform clearing price;
- deterministic ties-low rule;
- exact pro-rata fill with explicit remainder rule;
- public batch root, clearing price, and volume;
- private per-owner fills;
- public conservation proof;
- synthetic collateral ledger;
- no chain deployment.

It is large enough to exercise aggregation, crossing, allocation, leakage,
proof, and settlement, yet small enough for exhaustive enumeration and direct
comparison with prior local formal work.

