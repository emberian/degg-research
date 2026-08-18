# Research Stance

## What we are trying to learn

The project asks whether market mechanisms can be decomposed into small
relations that are:

- economically meaningful;
- exact and deterministic;
- cheap in clear execution;
- naturally batchable;
- compatible with confidentiality;
- independently verifiable;
- composable with onchain custody and settlement;
- honest about availability, censorship, and failure.

The strongest version is a dark relation: nobody learns private inputs or
intermediate state beyond a declared leakage function, yet anyone can verify
that the specified result and settlement followed from the committed inputs.

## Why the relation comes first

Generic encrypted computation is seductive because it postpones semantic
choices. It also makes performance, proof, leakage, and auditability harder to
reason about. A market already supplies algebra:

- flows add;
- limits induce monotone step functions;
- uniform clearing is a crossing or argmax problem;
- pro-rata allocation is bounded exact arithmetic;
- settlement is a conservation transition;
- policies can often be compiled into a small mutually exclusive family.

We will try to make those operators first-class. A backend must implement the
same relation, not merely produce plausible outputs.

## Anti-oppression constraint

Privacy is not a novelty layer. It protects people from unnecessary extraction
of strategies, intent, identity, and behavioral traces. A system that hides the
book from traders while granting a permanent omniscient operator or government
key has not achieved the Dark objective.

At the same time, research should not pretend away real coercion, law, or
deployment consequences. We will study multiple observable designs explicitly:

- public proofs with no decryption capability;
- narrow threshold disclosure with due-process and immutable access receipts;
- privacy-preserving eligibility credentials;
- encrypted surveillance queries that reveal bounded flags;
- systems where legally required observability makes the mode Shielded rather
  than Dark.

The taxonomy is descriptive. It must not be bent for marketing.

## No trust by adjective

These words do not prove a system:

- decentralized;
- onchain;
- FHE;
- ZK;
- threshold;
- open source;
- immutable;
- operatorless.

For each design, name:

- custody and asset authority;
- input admission and identity assumptions;
- inclusion and censorship rules;
- data availability;
- computation correctness;
- confidentiality and collusion threshold;
- decryption and release authority;
- settlement authority;
- liveness and abort;
- upgrade and governance power;
- external dependencies;
- public leakage;
- regulatory observability.

## Evidence hierarchy

From weakest to strongest:

1. Idea.
2. Written relation.
3. Executable clear oracle.
4. Property tests and adversarial fixtures.
5. Formal functional theorem.
6. Backend implementation.
7. Cross-backend differential equivalence.
8. Reproducible benchmark.
9. Cryptographic security argument under explicit assumptions.
10. Independent proof or audit.
11. Multi-process fault and censorship experiments.
12. Public testnet artifact.
13. Production claim, only after separate security, legal, and operational gates.

A high rung in one dimension does not upgrade another. A fast cipher is not a
privacy proof. A Lean theorem is not an SBF artifact proof. A public chain is
not an availability proof for withheld encrypted orders.

## Research outputs

Good outputs include:

- a falsified mechanism;
- a leakage lower bound;
- an impossibility or cost floor;
- a tiny verified relation;
- a cross-backend test corpus;
- a reproducible negative benchmark;
- a precise regulator question;
- a design that is honestly Shielded;
- a refusal to claim Dark.

The research succeeds when it makes the feasible space clearer, even if it
prevents a launch.

