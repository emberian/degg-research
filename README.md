# Dark Egg Research

Dark Egg Research is the public notebook for a family of specialized private and
verifiable market relations. The central bet is that a useful market does not
need an arbitrary encrypted virtual machine. It needs a small collection of
precise relations—aggregation, monotone crossing, allocation, conservation,
policy activation, net settlement, and selective release—that can be compiled
into several execution and privacy backends without changing their meaning.

This repository keeps questions, designs, evidence, experiments, literature, and
regulatory research. Executable protocol code belongs in Dragon's Clutch or a
future implementation repository. Formal systems continue to accrete in
Minidregg and Leanuweave. Breadstuffs is treated as prior art and experimental
lineage, not as a source tree to copy.

## The program

The working compiler picture is:

    market semantics
        ↓
    specialized Dark Relation IR
        ↓
    clear / shielded / dark backends
        ↓
    proof, receipt, availability, and settlement adapters

Initial relation families:

1. A padded frequent batch auction over a fixed price grid.
2. Private net settlement over already-authorized fills.
3. A fixed-topology multi-token ring clear.
4. Receipt-compiled OCO and bracket policies.
5. Guarded late fills and partial computations with eager economic shape.
6. Public solvency and deterministic settlement proofs over private state.

The desired end state is not merely hidden inputs. It is a system where the
declared leakage is small, the result is independently checkable, the inputs
cannot be silently omitted or substituted, aborts have explicit consequences,
and settlement follows the proved relation exactly.

## Read first

- docs/VERDICTS.md — the only current status page.
- docs/RESEARCH_STANCE.md — purpose, non-goals, and anti-oppression constraints.
- docs/ARCHITECTURE.md — the relation-oriented system decomposition.
- docs/DARK_RELATION_IR.md — the proposed typed compiler boundary.
- docs/PRIVACY_MODES.md — Clear, Shielded, Dark, and observable variants.
- relations/CLEARING_V0.md — the first exact market relation.
- docs/EXPERIMENT_PROGRAM.md — what must be measured before architectural claims.
- docs/research/DARK_FBA_RELATION.md — the first executable fixed-grid relation,
  its leakage contract, and the explicit refusal of an unproved Dark backend.
- experiments/dark-fba/ — deterministic Clear/Shielded oracle and golden vectors.
- docs/research/DARK_RELATION_THREAT_MODEL.md — role-specific transcript leakage,
  inference limits, abort semantics, and a concrete disclosure budget.
- experiments/leakage-lab/ — deterministic transcript and role-view vectors.
- docs/ROADMAP.md — staged research, proof, systems, and regulatory gates.
- docs/LOCAL_LINEAGE.md — what exists in sibling repositories and what may move.
- docs/regulatory/README.md — three current public-comment drafts, meeting
  brief, and submission-week plan.
- docs/regulatory/DRAFT3_CLAIM_AUDIT.md — artifact-by-artifact support ceilings
  and human filing gates for the current Draft 3 packet.
- paper/CLAIM_LEDGER.md — claims, evidence, and falsifiers.
- swarm/OPEN_QUEUE.md — bounded research tasks.

## Current truth in one paragraph

Prior local work contains serious pieces: a measured FHE uniform-price clearing
kernel, fast BFV aggregation with an MPC boundary, proof experiments, a broad
cleartext verify-not-find solver family, and substantial Lean semantics. It does
not contain a composed permissionless no-viewer venue. Minidregg proves valuable
private-computation and guarded-fill semantics and a narrow BFV input relation,
but has no deployed FHE backend or vFHE proof. Therefore the first honest target
is a tiny relation with explicit modes and cross-backend differential tests—not
a claim that the dark exchange already exists.

## Public research posture

We intend the work to be legible enough for cryptographers, protocol engineers,
market designers, civil-liberties researchers, and regulators to disagree with
it precisely. Research can challenge a regulatory model without laundering a
technical prototype into an unexamined legal conclusion. The repository will
publish questions, assumptions, negative results, and safety boundaries as
carefully as positive results.

No repository content authorizes a live venue or represents CFTC approval.

## License

Original material is AGPL-3.0-or-later. See LICENSE, LICENSING.md, and NOTICE.
Sibling repositories and cited third-party work retain their own rights and
provenance.
