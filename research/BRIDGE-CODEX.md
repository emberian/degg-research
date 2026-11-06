# BRIDGE-CODEX: cross-model synthesis for the P3 bridge (2026-11-06)

Two `codex exec -c model="gpt-6-astra" --sandbox read-only` rounds, run from
`~/dev/DreggNet`. Each was given file paths and no conclusions. Below: what each round
found, how I adjudicated it, and what changed. The full outputs are in the session task
logs. The Astra deputy's watch note (`zkml-research/notes/astra/watch/chokepoint-followup-audit.md`,
§bridge) predates this work. It audited the empty `p3-bridge/` scaffold, and its point
holds: "reflection plus agreement only removes the hand-copy gap relative to the
reflected interpretation". BRIDGE-KNOT §4 now states this as the definitional
identification row.

## Round 1: adversarial audit of the knot, checkers, statements and rows

| # | finding (Codex severity) | adjudication | action |
|---|---|---|---|
| 1 | "exactly the stored equations / same function" overclaims. The opaque remainder is reachable from exported constructors (`Seq Skip (Call …)`), and fix_clock forms are certified, not stored (High) | **Accepted.** | KNOT §0/§2 now say "on the transitively closed fragment" and define it; `liftProg`'s image is one such fragment. |
| 2 | Round trip does not remove printer trust: a coordinated `word_lt↦ult` / `ult↦word_lt` round-trips (High) | **Accepted.** My §4 overclaimed. | Round 2 was asked how to fix it; KNOT §4 rewritten (row contracts). |
| 3 | Concrete bugs survive all three checkers: `word_or ↦ ^^^` (no overlapping-bit vector); a reverse `+` mod 2^64 (no large-counter vector); the reverse direction never exercises word comparison (High) | **Accepted**, with one refinement: no reflected Lean definition uses the reverse word-comparison row, so it is unexercised rather than wrong. It is now marked as such. | Added vectors `op_or_overlap`, `op_xor_overlap`, `cmp_test`, `cmp_notequal` (forward) and `stepS_counter_no_wrap` at 2^64−1 (reverse). All green in both kernels. |
| 4 | Checker 1 does not compare record field names (swap `base_addr`/`top_addr` escapes); the reverse checker does not check constructor order or count (Medium) | **Accepted.** | Both sides now emit record field names in constructor order and `diff_skeletons.py` compares them. The reverse HOL script now checks each type's constructor list for equality, order and count included. |
| 5 | `MOD 0`/`DIV 0` are *defined* in HOL4 (`MOD_0`, `DIV_0`), not unspecified; my justification inferred underspecification from stuck `EVAL` (High) | **Accepted. I verified it on hbox's HOL** (`arithmeticScript.sml:2570–2585`). This was my error, and the same kind as P2 §4.2's first-draft witnesses. | The rows are now `MOD ↦ %` and `DIV ↦ /`, which Lean's Nat matches exactly. The witnesses are the HOL theorems `7 MOD 0 = 7` and `7 DIV 0 = 0`, re-proved in Lean. `HMOD`/`HDIV`/`MOD0`/`DIV0` are deleted from the prelude. |
| 6 | The fmap function-carrier argument justifies only first-order universal statements; finiteness is observable to `¬ ∀ m, ∃ k, m k = none` (Medium) | **Accepted.** | Prelude/KNOT soundness argument restricted accordingly. §3's theorems are inside that fragment. |
| 7 | Agreement is observational and oracle-specific; `ev` may collapse events; `Scoped` restricts only assignments; the deltas are not shown to be the only ones (Medium) | **Accepted as scope, not defect** (Codex: "legitimate restrictions, not vacuity"). | KNOT §3 "Scope of what these say". |
| 8 | The serve theorem name suggests more; the inhabitance witness omitted `SR` and `hinj` (Medium) | **Accepted.** | Witness strengthened: concrete `wS`/`wH` with `SR`, every premise, `hinj` (kernel-checked). Scope sentence added. |
| 9 | Commit-before-forward is about output labels; durable acknowledgment and the host loop are not covered (High) | **Accepted.** The host contract was already a named residual in CommitStep's README; KNOT now repeats it at the HOL theorem. | KNOT §4. |
| 10 | Prior art: MM0's `to-hol`/`to-othy`/`to-lean` exporters (Low) | **Accepted** as adjacent prior art (a shared specification exported to both, not HOL4⇄Lean transport). | Noted here and in the survey addendum. No "first" is claimed anywhere. |

## Round 2: is a round-trip checker sound, and how should rows be specified?

- **(a)** `R(F(t)) ≡α t` shows the translation can be recovered. It does not show the
  translation preserves meaning. Any type-correct misencoding that can be reversed
  survives it: signedness swaps, argument or constructor permutations, coordinated
  literal encodings. It becomes a soundness argument only if `R` itself is
  semantics-preserving: trusted-small, verified, or certificate-producing.
  **Accepted.**
- **(b)** Specify rows as **semantic contracts**. Each row gets a specification
  independent of both printers (for example signed comparison via
  `signed_w(n) = if n < 2^(w−1) then n else n − 2^w`). Each row gets a kernel lemma
  in each system tying the native operation to that specification. Dispatch tables
  are generated from proved contracts, never by inverting one table. Keep the vectors,
  and add deliberate two-table mutations. **Accepted.** This is now the next item
  (KNOT §6.4, ember decision §7).
- **(c)** Prior art: Gordon–Hunt–Kaufmann–Reynolds, *An Integration of HOL and ACL2*
  (2006), has a round trip used as confidence, not proof. Kaliszyk–Krauss (ITP 2013)
  has "characteristic properties" of mapped constants, which is the precedent for row
  contracts. Logipedia and OpenTheory have no round-trip semantic guarantee that Codex
  could verify. **[CLAIM]**: I have not read these papers in this pass. They are cited
  as leads.

## What I did not take

- Codex's round-1 remark that the HOL-side compiler theorem must still be instantiated
  for the chosen backend and calling convention is right. It is unchanged here: no
  `pan_to_target` theorem was instantiated in this lane, and KNOT §4/§6 say so.
- Nothing Codex found refuted `evaluate_agree` or the two delta theorems. Both rounds
  read them as correctly scoped.

## Round 3 (2026-11-06, the next lane): the restated ladder and the row contracts

Codex was given the ladder file, the agreement file, the contracts, the exporter's table
and contract section, and the mutation harness, and asked about vacuity, uniqueness,
printer residue and uncontracted rows. Findings and adjudication:

| # | finding | adjudication | action |
|---|---|---|---|
| 1 | `RefinesPan` keeps `Refines`' whole abstract state but only on word-local, `Γ`-bound states, for the oracle `oracleOf ev`; `SR` leaves globals, structs, code, eshapes, sh_memaddrs, top_addr unconstrained | **Accepted as scope** | KNOT §3 states it |
| 2 | ProofProducing's `redirectStatusStage`/`httpStamp`/`crlfStamp` certificates rest on `WF` hypotheses no state family satisfies (`∀ s, s.locals "code" = some …`); `translateCertPan` inherits that | **Accepted. A real defect in rung [2] as the orb states it, pre-existing.** My restated instance (`closedDemo`) has closed leaves and is not affected | `wf_reading_local_uninhabited` (breadstuffs `e5081531c`); KNOT §7 asks ember whether those demos move to the precondition form |
| 3 | `serve_405_panSem_witness` proved only the return value | **Accepted** | it now also proves the byte lands |
| 4 | List/option contracts are not unique at `Empty` element types | **Accepted, narrowed**: unique at every inhabited type, which is every image of a HOL type | KNOT §4.1 item 3 |
| 5 | Transporting all word rows and the literal printer through a permutation keeps every word contract true | **Accepted**: the carrier and literal rows are base | KNOT §4.1 item 2 |
| 6 | A numeral printer bug (8 printed as 9) changes `byte_index` and passes every contract | **Accepted. The decisive finding**: contracts move trust from per-constant rows to the printer's generic core, they do not remove it | KNOT §0 sentence 5, §4 TCB table, §4.1 item 1 |
| 7 | `word_slice`, `word_slice_alt`, `OPT_MMAP` (on the fragment's path), `word_asr`, `word_ror`, `ARB` uncontracted | **Accepted for the first three** | contracts added (40 forward); mutants caught. asr/ror are outside `liftProg`'s image; `ARB` is underspecified by design |
| 8 | `mutate-row.sh` reporting no failure is not evidence of equivalence | **Accepted** | the harness says "row NOT caught" in that case |

## Round 4 (2026-11-06): reverse contracts, the ledger proposal, overclaims

| # | finding | adjudication | action |
|---|---|---|---|
| 1 | The oracle check ignores axioms: `new_axiom` then contradiction would pass (spoofing `DISK_THM` itself is rejected by the kernel) | **Accepted** | both runs now also require no axioms and no hypotheses; the reverse script checks its theory has no axiom and exactly the expected theorem count; the exporter refuses a theory with axioms (DreggNet `6d3eee7`) |
| 2 | `nat_add` alone does not pin `+` (`A m n := m` satisfies it, since `n + 1` uses the same row) | **Accepted** as a statement about standalone contracts; the mutant is caught by others | KNOT §4.1 item 4 says uniqueness is relative |
| 3 | `bool`: swapping `true`/`false` passes | **Accepted, kept as base**: the swap is an automorphism over Bool alone and no reverse row uses a Bool as a HOL truth value | KNOT §4.1 row table |
| 4 | `ite` at pair/state types (used by `step`, `stepS`) and `UInt8.ofBitVec` were uncontracted | **Accepted** | `ite` contracted at every type, UInt8 carrier contract added (24 reverse); mutants caught |
| 5 | `mutate-rev-row.sh` counted any Holmake failure as caught | **Accepted** | only a named contract failure counts |
| 6 | §4.2: A's "zero HOL work" ignores the seam (body `evaluate` vs `semantics_decls`, entry, compiler premises); B need not port 20k lines (symbolic execution of the concrete serve); the middle form is under A's trust and needs real printer work | **Accepted, all three** | §4.2 rewritten; recommendation now orders the seam lemma first |
| 7 | "Sem.lean appears only inside proofs" overclaims: `RefinesPan` is phrased through `ofPan`/`SR` | **Accepted** | LadderPanSem header (breadstuffs `27ad92ee3`) and KNOT §3 corrected |
| 8 | "a wrong row makes the statement false" should be conditional on coverage | **Accepted** | KNOT §4.1 wording |
