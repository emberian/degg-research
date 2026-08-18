# Dark Relation IR

Status: PROPOSED semantic interface. No parser, compiler, or backend exists.

## 1. Design constraint

The IR is not an encrypted programming language. It is a typed vocabulary for a
small family of relations whose:

- domains are statically bounded;
- arithmetic is exact;
- control flow is data-oblivious at the declared leakage level;
- memory shape is fixed before private inputs arrive;
- output visibility is explicit;
- proof and settlement obligations are derivable;
- cost can be estimated per backend.

If a mechanism cannot be expressed without generic loops, secret-dependent
allocation, unbounded recursion, or dynamic memory, the first response is to
refuse or redesign the mechanism—not silently add a VM.

## 2. Module identity

Every module freezes:

- relation namespace and semantic version;
- canonical specification digest;
- type table;
- parameter bounds;
- public parameter fields;
- input ports and ownership;
- output ports and visibility;
- operator graph;
- refusal codes;
- leakage declaration;
- proof obligations;
- settlement schema;
- canonical encoder version.

Changing any item produces a different relation identity.

## 3. Types

Primitive types:

- Bool.
- UInt(w), with 1 <= w <= backend cap.
- SInt(w), two's-complement meaning but no implicit modular arithmetic.
- NatBound(max).
- Tick(K).
- Owner(N).
- Slot(N).
- Digest256.
- Nullifier256.
- TimePoint(domain, precision).

Containers:

- Vec(T, n), fixed length.
- Matrix(T, rows, cols), fixed shape.
- PaddedSet(T, capacity), canonical empties.
- PublicCommitment(T), content-addressed external value.
- Secret(owner, T).
- SharedSecret(threshold_profile, T).
- LocalOutput(owner, T).

Refinement types:

- NonZero(T).
- InRange(T, lower, upper).
- Unique(Vec(T, n)).
- Sorted(Vec(T, n), order).
- SumBounded(Vec(UInt(w), n), max).
- OneHot(Vec(Bool, n)).
- PermutationOf(root, Vec(T, n)).

No implicit coercion changes width, sign, unit, visibility, or owner.

## 4. Arithmetic semantics

Operations:

- checked_add, checked_sub, checked_mul;
- exact_div returning quotient and remainder;
- min, max;
- eq, lt, le;
- bool_and, bool_or, bool_not;
- select;
- widening_sum;
- dot product with static bound proof.

Every operation has one of:

- proved nonoverflow from static bounds;
- explicit checked refusal;
- widening into a named output width.

Modular arithmetic exists only in cryptographic submodules with a field/modulus
type and cannot be confused with economic integers.

## 5. Structural market operators

### Bounded admission

Validate each padded slot and emit:

- admitted bit;
- canonical contribution;
- refusal code commitment;
- input commitment leaf.

### Segmented reduction

Sum or combine values by public segment identifier or fixed private one-hot
segment. The backend must state whether segment membership leaks.

### Prefix and suffix scan

Associative scan over fixed vectors. Used for cumulative supply/demand, path
accumulators, and bounded policy state.

### Monotone crossing

Given nondecreasing A and nonincreasing B, choose a canonical tick satisfying the
relation's crossing rule, or a named no-cross/refusal output. The module carries
monotonicity obligations; a backend cannot merely return an index.

### Canonical argmax

Return maximum value plus first/last/lexicographic tie index as frozen. Proof
obligation covers both maximality and tie policy.

### Exact pro-rata

Return base quotients, remainders, rank, and residual assignments whose sum is
exactly the target. The rank source and manipulation assumptions are part of the
module identity.

### Conservation transition

Apply a fixed sparse incidence matrix or typed balance deltas and prove:

- allowed asset/unit pairs;
- no debit below zero or reservation;
- global conservation plus named fees/burns/mints;
- nullifier uniqueness;
- exact terminal state commitment.

### Guarded late fill

Freeze:

- value type;
- authority;
- pre-root;
- footprint;
- guard commitment;
- effect commitment;
- deadline;
- continuation;
- nullifier domain.

Accept only a late value/proof. The IR forbids changing the frozen economic
shape during fill.

### Candidate-set map

Apply a deterministic function over correlated candidate worlds while
preserving provenance and correlations. Collapsing a candidate set to one
enforceable value is a distinct Finalize operation with an explicit stability
or authority proof.

## 6. Visibility

Every value is labeled:

- Public.
- Secret(owner).
- SharedSecret(profile).
- LocalOutput(owner).
- Committed.
- Erased.

Visibility changes require explicit operators:

- Commit.
- RevealPublic.
- RevealLocal.
- ThresholdDisclose.
- Prove(predicate).
- EraseAfter(commitment, retention_receipt).

A backend that cannot realize a visibility transition must refuse compilation.
It may not widen visibility silently.

## 7. Leakage

The compiler derives a candidate leakage manifest from:

- module shape and parameters;
- public values;
- ciphertext and proof sizes;
- number and timing of phases;
- failure codes;
- public outputs;
- settlement adapter.

Human review adds topology leakage:

- network addresses;
- relayers;
- key enrollment;
- retry patterns;
- accelerator/service logs;
- chain accounts and fees.

Dark eligibility requires a frozen combined manifest and security statement.

## 8. Refusals and partiality

Named refusals include:

- malformed encoding;
- unsupported relation/version;
- invalid range;
- duplicate nullifier;
- missing inclusion;
- cutoff violation;
- unavailable data;
- overflow bound;
- invalid reservation;
- no crossing;
- ambiguous finality;
- proof invalid;
- key epoch mismatch;
- backend unavailable;
- threshold abort;
- settlement unavailable;
- privacy mode unsupported.

Pending, retryable-unavailable, semantically-invalid, expired, and terminal abort
are different states. A transport error cannot be reinterpreted as a no-trade
result.

## 9. Backend capability descriptor

A backend declares:

- supported operators and dimensions;
- exact arithmetic domains;
- setup/key profile;
- corruption and leakage model;
- proof/attestation mode;
- local-output mechanism;
- expected cost formula;
- maximum artifact sizes;
- abort and recovery;
- provenance and license.

Compilation succeeds only if all semantic, visibility, proof, and settlement
requirements are supported.

## 10. Receipts

Receipt domains are distinct:

- source/admission receipt;
- computation receipt;
- proof verification receipt;
- output-delivery receipt;
- settlement receipt;
- disclosure receipt;
- retention/destruction receipt.

Do not compare digests from different preimages. A joined receipt embeds the
exact child receipts and the mapping between their semantic objects.

## 11. Refinement test

For canonical input x:

    clear_relation(x) = y

Every backend must produce an admitted artifact a such that:

    verify_backend(relation_id, input_root(x), a) = accepted(y_commitment)

and the settlement adapter must prove:

    settle(pre_state, y, settlement_inputs) = post_state

The proof system may vary. The y semantics may not.

## 12. First compiler refusal policy

The first compiler refuses:

- secret-dependent loops;
- dynamic vectors;
- arbitrary user bytecode;
- external calls during computation;
- floating point;
- unbounded prices or quantities;
- unconstrained division;
- hidden relation identity;
- unstated output recipient;
- unspecified remainder;
- unspecified failure payout;
- unverifiable settlement mapping;
- Dark mode without a local-output and proof story.

That refusal surface is part of the product.

