# Privacy Modes and Observable Variants

Status: PROPOSED taxonomy.

## 1. Clear

Inputs, state, intermediate computation, output, and settlement are public except
for ordinary local secrets such as signing keys.

A Clear protocol can still be formally verified, noncustodial, permissionless,
and resistant to operator substitution. Clear does not mean naive.

Minimum claim:

- anyone can reconstruct the accepted input set;
- anyone can recompute the result;
- settlement is bound to the result;
- no privileged actor is required for correctness.

## 2. Shielded

A named actor set may learn private inputs or intermediate state. Examples:

- a single trusted execution service;
- an MPC committee that can reconstruct under its threshold;
- a regulator or auditor with a decryption capability;
- a source viewer that constructs a proof over the whole witness;
- an enclave cluster.

A Shielded claim must name:

- who may learn;
- the collusion threshold;
- whether learning is technical, procedural, or merely prohibited by policy;
- what happens after compromise;
- whether past ciphertexts become readable;
- who can refuse service or censor;
- how key rotation and recovery work.

Threshold FHE is usually Shielded unless the protocol constrains decryption and
the security definition proves that permitted parties cannot reconstruct more
than authorized outputs.

## 3. Dark

No actor learns more than:

- the frozen public leakage function;
- its own submitted input;
- its own authorized local output;
- information logically implied by those values.

The claim is relative to a corruption model, topology, cryptographic assumptions,
and availability model. A Dark design needs a simulator or comparable
noninterference statement.

Darkness is end-to-end. A dark computation followed by public account-and-amount
settlement is pre-trade dark, not fully dark. A dark book with unpadded ingress
may reveal participation and timing. A dark evaluator with a plaintext proof
producer is Shielded.

## 4. Regulatory observability is a separate axis

Do not redefine Dark to mean “dark except for a master key.” Instead describe an
observable variant explicitly.

### Due-process selective disclosure

Encrypted records are available to a threshold of independently governed
custodians under a frozen authorization process. Every request and disclosure is
committed to an append-only public receipt.

Questions:

- Who can authorize?
- Can the subject contest or learn of access?
- Can access be retroactive and bulk?
- Is the query bounded?
- Does a threshold coalition obtain the raw book?
- What happens if law or governance changes?

This is Shielded with a narrow disclosure policy unless a stronger cryptographic
definition proves otherwise.

### Encrypted compliance queries

The system evaluates frozen surveillance predicates and reveals only bounded
flags or aggregates. This may reduce routine exposure but cannot be assumed to
satisfy every investigation, recordkeeping, or emergency obligation.

### Privacy-preserving eligibility

Credentials can prove membership, jurisdictional eligibility, limits, or
non-revocation without publishing identity. An issuer or regulated intermediary
may still know the holder mapping. This separates public privacy from full
anonymity.

## 5. Leakage declaration template

Every experiment declares:

| Category | Declaration |
|---|---|
| Market identity | public / hidden / bucketed |
| Batch cadence | public / hidden |
| Participant count | exact / padded capacity / hidden |
| Arrival timing | exact / epoch-only / hidden |
| Order side | public / hidden |
| Limit | public / hidden |
| Quantity | public / hidden / bucketed |
| Cancellation | public / hidden |
| Clearing price | public / recipient-only / hidden |
| Aggregate volume | public / bucketed / hidden |
| Individual fill | public / recipient-only |
| Account identity | public / credentialed / anonymous-set |
| Position | public / recipient-only / hidden |
| Settlement graph | public / shielded / hidden |
| Failure | public type / opaque / hidden |
| Regulatory access | none / query / threshold disclosure / full |

## 6. Forbidden claims

Do not say:

- “FHE means nobody can see” without discussing key holders and outputs.
- “ZK means private” when the prover sees the witness.
- “MPC means decentralized” without enrollment and corruption assumptions.
- “Onchain means available” when ciphertext inputs can be withheld.
- “Operatorless” when a frontend, committee, oracle, resolver, or upgrader is
  indispensable.
- “Auditable” when the audit plane can silently decrypt everything.
- “Anonymous” when wallets, timing, gas, relayers, or settlement graph identify
  participants.

