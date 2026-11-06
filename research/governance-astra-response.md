# Response to the Codex Astra statement audit (2026-11-06)

Audit: `/Users/ember/dev/zkml-research/notes/astra/governance-statement-audit.md`, with its Lean
counterexamples in `notes/astra/governance/GovernanceAudit.lean`. It found 44 contracts, 5 definite
defects and 11 qualifications, plus a patch to the MMA counts model. All five defects are accepted
and repaired in place.

| Item | Astra's finding | Disposition |
|---|---|---|
| G07 | Tolerance does not imply a T3-at-best claim. An exact proof of `\|y−f(x)\| ≤ δ` has zero proof error and a weaker predicate | Accepted. Lattice §0.2 rewritten: the receipt carries the tolerance relation as its predicate; δ never becomes ε without a separate game |
| G09 | Deduplicating by evidence identity is unsound: the same bytes can support predicates with disjoint failure events (`dedup_by_identity_false`) | Accepted. Lattice §5 and alignment (i) now say: dedupe exact repeated judgments, or uses proved to share one failure event |
| S06 | Unconditional "a non-full device forwards" is false asynchronously (crash before ack, starvation, downstream never ready) | Accepted. `forwards_when_capacity` in `CommitStep.lean` is a statement about the **synchronous** step, where persist-and-forward is one transition, and the file says so. Refining it to an asynchronous device carries Astra's fairness, store-progress and no-crash premises |
| S15 | `receipt_means` (AST equality) is false or a tautology | Accepted. Withdrawn in alignment (d); replaced by Astra's `graph_receipt_sound` shape |
| S16 | Record agreement omitted root equality (`equal_length_different_records`) | Accepted. Equal roots are now a premise; different authenticated roots are equivocation evidence |
| MMA patch | The balanced-interval limb formula is invalid for the gapped FP4 digit set (65 has no two-digit form) | Applied (`experiments/mma-field-mapping`). The limb count stays at 9 and break-even at 5.06. The largest digit is 8, so the maximum product is 64, not 144. Exact blocks become 262,143 (24-bit) and 255 (14-bit), not 116,508 and 113. Every document quoting the old numbers is corrected with attribution |

On the qualifications, two items bear directly on code that now exists:

- **S03 (commit before forward).** Astra asks for an *injective* map from forward occurrences to
  earlier durable commits. `CommitStep.commit_before_forward` gives an *immediately preceding*
  persist for each forward. Distinct forwards sit at distinct stream positions, so their
  preceding persists are distinct, which is injectivity. I have not stated it as a separate
  lemma yet; it is next in that file. "Durable" in the model means acknowledged, because the step
  is synchronous. In the export, `F … leafhex` obliges the host to persist before forwarding.
  That obligation is the asynchronous refinement S06 describes, and it stays a named residual
  until the PD hosting step.
- **S04 (leaf binding).** `CommitBytes.leaf_collision` is the reduction form Astra asks for:
  equal leaf bytes and unequal payloads return the colliding BLAKE3 inputs.
  `enc_inj` is injectivity of the *encoding*, not of the hash.

G03 (the binding leg needs explicit transport), G04 (the receipt needs a derivation certificate
and a rational budget) and G08 (false accept is not posterior truth) are adopted as the target
shape for the `Judgment` receipt. The receipt is not built yet.
