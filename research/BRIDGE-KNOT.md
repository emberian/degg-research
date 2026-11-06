# BRIDGE-KNOT: what "each system has a checked loader of the other" closes

**Date:** 2026-11-06. **Code:** `emberian/DreggNet`, branch `p3-bridge`,
`docs/engine/probes/p3-bridge/` (forward exporter `exporter/`, Lean package `lean/`,
reverse exporter `reverse/`, audit outputs `audit/`). **Survey:** `BRIDGE-SURVEY.md`.
**Codex exchanges:** `BRIDGE-CODEX.md`.

Tags: [MEASURED] I ran it; [READ] I read it; [INFERRED] my reasoning; [CLAIM] someone else's.

## 0. The knot, in five sentences

1. Forward: CakeML's Pancake semantics (panLang/panSem at CakeML `ed31510b3`, the
   revision Sem.lean was transcribed from, built on hbox) is exported mechanically into
   Lean on a stated fragment, and three checkers pass [MEASURED]. On that fragment the
   Lean definitions satisfy the HOL4 kernel's equations: the stored ones, or
   kernel-certified slices and fix_clock forms of them. 73 equations are re-proved in
   Lean. 24 types and 92 constructors match, including record field names. 42 vectors
   agree between HOL and Lean.
2. Lean proves that the hand-written `Sem.lean` agrees with that export on every scoped
   program (`evaluate_agree`). It then re-derives the orb's serve rung about the exported
   panSem `evaluate` itself (`serveExport_serves_405_panSem`). The proof uses no axiom
   beyond propext, Classical.choice and Quot.sound. It also proves the two places where
   Sem.lean is *not* panSem.
3. Reverse: the chokepoint device's Lean definitions are exported mechanically into HOL4.
   These are `step`, `run`, `enc`, and the `Summary`/`proj`/`stepS` that the seL4 PD
   runs. HOL4 re-proves every Lean equation lemma. HOL's TypeBase matches Lean's
   constructors, including their order and count. HOL `EVAL` reproduces four vectors
   that Lean computed and checked in its kernel, one of them `stepS` at counter 2^64−1,
   where num must not wrap. HOL4 then proves commit-before-forward and `stepS_refines`
   itself, and CakeML's proof-producing translator gives certificate theorems for
   `step`, `run` and `stepS`.
4. That closes this gap: on the closed fragment (`liftProg`'s image in one direction,
   the device's definitions in the other), a theorem proved in either kernel now
   concerns, checkably, the same function the other kernel defines. "The proof is about
   a hand copy" is false for the orb rung, and "the HOL theorem is about a different step" is false for the device.
5. TCB: the Lean kernel; the HOL4 kernel; the prelude-row identifications (about 60
   rows, witnessed by vectors, not yet specified by proved contracts); and the exporters'
   printers. That means type interpretation, name and binding translation, and
   declaration selection, to the extent the vectors do not exercise them (§4). For
   anything that reaches machine code, add CakeML's own trusted base (ISA model, FFI/oracle model) or Lean's
   compiler and the C compiler. Proofs never cross the bridge; only definitions do.

## 1. What a "loader" is here

A loader is a **definitional reflection with checkers**, not proof transport (ADR-3-REV,
"the bridge is definitional, not proof-level"). Exporter A walks system A's kernel data
structures and prints declarations for system B. B elaborates them. Three checkers then
compare what B ended up with against what A has:

| checker | forward (HOL→Lean) | reverse (Lean→HOL) | catches |
|---|---|---|---|
| 1 structural | Lean env `ConstructorVal`s, rendered by `Audit.lean`, diffed against HOL TypeBase dump (`audit/diff_skeletons.py`) | HOL TypeBase constructors compared *as HOL types* (parsed, `=`) with Lean's rendering, inside the HOL script | wrong arity, order, parameters, record field order, nested types |
| 2 equations | every HOL stored/certified equation printed as a Lean theorem, proved by unfolding only (`hol_eqn`) | every Lean equation lemma (`getEqnsFor?`) printed as a HOL theorem, proved by `simp[Once f_def]` | clause order, overlap, pattern shape, termination wrappers |
| 3 behaviour | HOL `EVAL` theorems printed as Lean theorems, proved by computing with the reflected defs | Lean values (`Meta.reduce`, kernel-checked by `Eq.refl` `addDecl`), proved in HOL by `EVAL_TAC` | wrong meaning of a prelude row (signedness, endianness, fold order, …) |

**A correction to P2's framing.** It matters for the TCB claim, so it gets its own
paragraph. Checkers 1 and 2 compare two outputs of *the same printer*: the Lean
definition and the Lean statement of the HOL equation. If the printer maps a HOL
constant to the wrong Lean function consistently, both sides carry the bug and checker 2
passes. I measured this. Mapping `word_lt` to `BitVec.ult` instead of `BitVec.slt`
passes checkers 1 and 2. It fails exactly two of the vectors (36 at the time), `vec_cmp_less_signed`
and `vec_cmp_notless` [MEASURED]. So the exporter's term printer is not simply "a
generator whose output is re-checked". It is re-checked structurally, and its *meaning*
is re-checked only on the vectors. §4 gives the upgrade that would make it fully
untrusted.

## 2. Forward: Pancake into Lean

**Built** (`exporter/export2leanScript.sml`, grown from P2's; P2 itself is untouched):

- 24 datatypes / 92 constructors: `mlstring`, the ffi types (`ffi_state` as a record with
  its oracle field), `asm$binop/cmp`, `ast$shift`, panLang `shape/panop/varkind/exp/
  opsize/primop/prog/struct_info`, panSem `word_lab/v/state/result`. Records are now
  Lean `structure`s with accessors and `with` updates (P2 §5-3).
- Definitions: `byte_index`, `get_byte`, `set_byte`, `align`, `byte_align`,
  `read_bytearray`, `word_cmp`, `word_op`, `word_sh`, `call_FFI`, `shape_of`,
  `mem_load_byte`, `mem_store_byte`, `write_bytearray`, `mem_store(s)`, `pan_op`,
  `set_var/global/kvar`, `lookup_kvar`, `is_valid_value`, `res_var`, `empty_locals`,
  `dec_clock`, `fix_clock`. Also **`eval`** (12 of 17 stored clauses) and **`evaluate`**
  (14 of 21 stored clauses: Skip, Dec, Assign, Store, StoreByte, Seq, If, Break, Continue,
  While, Return, Tick, Annot, ExtCall). Both are recursive and carry Lean termination
  proofs (P2 §5-1).
- **Certified slices with an opaque remainder** extend P2's `do_app_arith_slice` to
  recursive definitions. The unexported clauses go to an `opaque f_unspec` catch-all, so
  Lean proves nothing there. The opaque cases are: eval's RField/NStruct/NField/Load32;
  evaluate's Primitive/Store32/ShMemLoad/ShMemStore/Raise/Call/DecCall; and the non-`One`
  shapes of `mem_load`, `is_wf_shape`, `size_of_sh_with_ctxt` and `flatten`. Each
  remaining clause is ordinary engineering (§6).
- **`evaluate` in fix_clock form.** panSem stores `evaluate_def` with `fix_clock`
  rewritten away by `fix_clock_evaluate`. The fix_clock form is the one HOL's own
  termination argument needs, so the exporter puts `fix_clock` back. It wraps exactly the
  let-bound recursive calls whose body recurses again (Seq, While) and proves in the HOL
  kernel that each wrapped clause equals the stored one. The Lean definition then
  terminates on HOL's measure `(clock, prog_size)` using two proved lemmas (`Support.lean`).
- **Monomorphised at `'a := 64`, by kernel instantiation.** Closed `dimindex` /
  `bytes_in_word` / `LOG2` subterms are reduced by `EVAL` inside the exporter, so every
  emitted equation is still a HOL theorem. This is the x64 instance the orb compiles to.
  Polymorphic width is §6.
- String literals are now emitted as explicit byte lists. That removes P2's
  `hstr`/UTF-8 row (P2 §5-6). The sanitizer is checked for injectivity at emission (P2 §5-7).

**New prelude rows and their witness pairs** (all in `audit/hol_eval_vectors.txt` ⇄
`lean/PanBridge/GenVectors.lean`, both kernels) [MEASURED]:

| row | splitting pair |
|---|---|
| `word_lt ↦ BitVec.slt` (SIGNED), `word_lo ↦ BitVec.ult` | `0x8000…0 < 1` is 1 under Less and 0 under Lower; `NotLess` gives 0 |
| byte endianness (via `byte_index`, reflected not mapped) | byte 1 of `0x0807060504030201` is 2 when be=F and 7 when be=T |
| `word_asr ↦ sshiftRight`, `word_lsr ↦ >>>` | `0x8000…0 >> 63`: all-ones vs 1 |
| `word_lsl` with shift ≥ width (through `word_sh`) | `1 << 64` is NONE (word_sh guards), not 0 |
| `word_ror ↦ rotateRight` | `ror 1 4 = 0x1000…0` |
| `FOLDR ↦ List.foldr` | `FOLDR (λx a. x−a) 0 [5;3] = 2` (foldl gives 0) |
| `word_op` fold identity and arity (reflected) | `Op And []` is all-ones; `Op Sub` with 3 arguments is NONE |
| `n2w`, `w2w` | `n2w 300 : word8 = 44`; `w2w 0x1FF : word8 = 0xFF`; `w2w 0xFF : word64 = 0xFF` (zero-extend) |
| `word_slice` (inclusive) vs `word_slice_alt` (exclusive) | `(3 '' 0) 0xFF = 0xF`; `word_slice_alt 3 0 0xFF = 7` |
| `fmap ↦ α → Option β`: FUPDATE, FUPDATE_LIST, DOMSUB | `FEMPTY |++ [(1,2);(1,3)]` gives 3 (last wins); `|+` twice: outer wins; `\\` removes only its key |
| `UPDATE` | `(1 =+ 2) ((1 =+ 3) f) 1 = 2` |
| `ZIP` (HOL4's is total and truncating) | `ZIP ([1;2],[3]) = [(1,3)]` |
| `MOD ↦ %`, `DIV ↦ /` | HOL4 *defines* `k MOD 0 = k` and `k DIV 0 = 0` (arithmetic `MOD_0`, `DIV_0`), which is exactly Lean's Nat. The witnesses are HOL's own theorems at 7 (`mod_zero`, `div_zero`), because `EVAL` leaves these terms unreduced. **An earlier draft read the stuck `EVAL` as "unspecified" and mapped them to opaque branches. Codex round 1 caught it.** That is P2 §4.2's lesson again: a witness has to be a theorem, not the absence of a reduction. |
| `word_or ↦ |||`, `word_xor ↦ ^^^` | `Op Or [3;1] = 3`, `Op Xor [3;1] = 2`. Overlapping bits are needed: the byte-insertion vectors combine disjoint fields, where OR and XOR agree (Codex round 1 found that gap). Also `Cmp Test 2 1 = 1` and `Cmp NotEqual 2 1 = 1`. |

The `fmap` row's soundness argument (in `Prelude.lean`): `FLOOKUP` is an injective
homomorphism into `α → Option β` that commutes with the four operations. That justifies
transfer of **first-order, universally quantified statements about terms built from those
operations**, and nothing wider. Codex's counterexample: `¬ ∀ m : FMap ℕ ℕ, ∃ k, m k = none`
holds for the function carrier and fails for finite maps. So finiteness *is* observable
to a higher-order or existential Lean statement, even without an `FDOM` row. The theorems
in §3 quantify over states universally and mention maps only through lookup and update,
so they are inside the justified fragment [INFERRED].

**Results** [MEASURED, `lake build` green, Lean 4.30.0]: checker 1 reports 24 types, 92
constructors and the record field names, 0 failing. Checker 2 re-proves 59 + 14 HOL
equations in Lean. Checker 3 passes 42 vectors.

**Fragment.** The identification is per clause. A clause that is exported can still
reach the opaque remainder. For example, `Seq Skip (Call …)` passes through exported
`Seq` into unexported `Call`, and `Load` with a non-`One` shape reaches an opaque helper.
So "same function" holds on the **transitively closed** fragment: programs built only from
exported constructors, with `One`-shaped values. `liftProg`'s image is such a fragment,
and `evaluate_agree` needs no more (Codex round 1, finding 1).

## 3. The theorem that makes the objection false

`lean/PanBridge/Agree.lean` compiles `breadstuffs/orb-compiler/Pancake/Sem.lean` from its
own source, as a `lean_lib` with its srcDir in breadstuffs; nothing is copied. Every
theorem below depends only on `propext`, `Classical.choice` and `Quot.sound`
(`#print axioms` in the build log) [MEASURED].

- `liftExp`/`liftProg`: Sem.lean's AST into the reflected panLang. Binary ops become
  `Op b [l; r]`, `.loadWord` becomes `Load One`, names become `mlstring` by their UTF-8
  bytes (`nm`, proved injective).
- `SR ss hs`: the fields Sem.lean models agree. These are locals through `nm` and
  `ValWord`, memory through `Word`, memaddrs and be as Bool vs Prop, clock, ffi and base.
  The fields it omits are unconstrained.
- `oracleOf ev`: Sem.lean's abstract FFI oracle, instantiated by **panSem's own
  `call_FFI`** on `ExtCall (nm name)`. `ev` maps `final_event` to Sem.lean's lossy
  `FinalEvent`, and the theorem holds for every such map.
- **`eval_agree`**: `eval hs (liftExp e) = (Pancake.eval ss e).map ValWord`, for every
  expression.
- **`evaluate_agree`**: for every Sem.lean program `p` with `Scoped Γ p`, and every `ss`,
  `hs` with `SR ss hs` and the locals in `Γ` bound in `ss`, three things hold. The
  results are related by `RR`. The final states are related by `SR`. And a live result
  leaves `Γ` bound. This covers all ten constructors, including `While` (strong induction
  on the clock), `ExtCall` (byte-array read/write and `call_FFI`'s length lemma) and
  `StoreByte` (`set_byte` against Sem.lean's mask form, proved bit by bit).
- **`serveExport_serves_405_panSem`** (`ServeTransfer.lean`): the orb-compiler's
  `serveExport_serves_405`, restated about `P3.Pan.evaluate` on
  `liftProg (serveModel bs200 bs405)`. The run returns `ValWord |bs405|`, and the
  response bytes are in memory at `outBase`, read with panSem's own `mem_load_byte`.
  `serveModel_scoped` discharges the scoping hypothesis. `agree_witness` and
  `serve_405_hypotheses_inhabited` show the hypotheses are satisfiable.

**Scope of what these say** (Codex round 1, findings 7 and 8). `SR` constrains only the
fields Sem.lean models. `RR` lets `ev` collapse final events. `oracleOf` is panSem's
`call_FFI` wrapper around an arbitrary HOL oracle, not the deployed driver. `Scoped`
constrains assignment targets only. The serve theorem covers the request-byte-88 path:
the 405 bytes in memory and the returned length word. It does not cover HTTP
transmission or compiled-binary behaviour. `serve_405_hypotheses_inhabited` now exhibits
a concrete pair of states related by `SR`, with every premise, including `hinj`.

**The two transcription deltas, as theorems.** Neither appeared in Sem.lean's own header.

1. `sem_lean_assign_unscoped_disagrees`: on `assign` to an unbound local, Sem.lean
   continues and panSem returns `Error`. panSem's `Assign` checks `is_valid_value`, which
   requires the variable to be bound. Sem.lean dropped that check as "vacuous (One)". It
   is vacuous for the *shape*, not for the *binding*. This is why `evaluate_agree` takes
   `Scoped`, and the orb's serve program satisfies it.
2. `sem_lean_write_unmapped_disagrees`: when the head byte is unmapped, panSem's
   `write_bytearray` returns the *original* memory (`NONE => m`, where `m` is the
   parameter) and so discards the tail writes. Sem.lean's `writeByteArray` keeps them
   (`none => m'`). The `ExtCall` path cannot reach this, because the array was just read
   successfully, so `evaluate_agree` still covers `ExtCall`. Any other user of
   `writeByteArray` inherits the delta.

**Should the orb use the agreement theorem, or prove against `Generated` directly?**
Both now work. Prefer re-targeting the orb ladder at `P3.Pan.evaluate`: Sem.lean becomes
a convenience layer with a proved bridge, not a second source of truth. `transfer_return`
plus `SR` already turns any return-value or memory property of a scoped Sem.lean run into
the panSem one.

## 4. The composed theorem shapes, and the exact TCB

**Orb ladder** ("datacake down to the bytes"):

```
Lean:  serveExport_serves_405           (property of the Sem.lean run of the lowered serve)
Lean:  serveExport_serves_405_panSem    (same property of P3.Pan.evaluate; evaluate_agree)
  ≡    the same statement about HOL4's panSem$evaluate at word64     ← THE BRIDGE (definitional)
HOL4:  pan_to_target compiler correctness (CakeML)                    ← proved in HOL4
HOL4:  target ISA semantics, then machine code                        ← CakeML trusted base
```

The "≡" is not a proof step in either kernel. It is the claim that `P3.Pan.evaluate`
denotes HOL's `evaluate` under the prelude interpretation. Checker 2 supports it: the Lean
function satisfies HOL's defining equations clause by clause. Those equations, with HOL's
well-founded measure, determine the function uniquely on the exported clauses. So given the
rows, the Lean function *is* HOL's on that domain [INFERRED: a uniqueness argument, not
mechanised]. Checkers 1 and 3 support the rows and the printer. To compose a Lean-proved
statement with HOL's compiler theorem, one of two things must happen. Either the statement
is re-proved in HOL4, or that identification is accepted as the CR-2 ledger row.

**Chokepoint device**:

```
Lean:  stepS, stepS_refines, commit_before_forward (CommitStep/CommitBytes)
  ⇒    Lean2Hol: HOL4 Datatype/Definition (checkers 1-3)                ← THE BRIDGE (definitional)
HOL4:  commit_before_forward, stepS_refines, commit_before_forward_summary  ← RE-PROVED in HOL4
HOL4:  ml_translatorLib certificates step_v, run_v, stepS_v              ← proved in HOL4
HOL4:  CakeML compiler correctness → machine code                        (not yet run: §6)
```

On this route the device property is proved by the HOL4 kernel about the HOL4 definition
that gets compiled. **What the property says is about output labels.** Every `forward`
in the stream is immediately preceded by a `persist` of the matching leaf. That a
`persist` label means a durable, acknowledged write is the host's contract (a named
residual in `CommitStep`'s README), and so is the host loop that executes `runS`'s fold.
Neither is in any theorem here (Codex round 1, finding 9). The bridge's only job is to say that this HOL definition is the Lean
`stepS` the Lean development reasons about. That matters for linking with Lean-only facts,
such as `leaf_collision` against minidregg's BLAKE3. The machine code obeys
commit-before-forward without trusting Lean at all.

**Exact TCB of the composed statements**

| item | orb (Lean proof, HOL compiler) | device via CakeML | device via Lean compiler (today) |
|---|---|---|---|
| Lean kernel | yes | only for the Lean-side facts linked in | yes |
| HOL4 kernel | yes | yes | no |
| prelude rows (forward ~50, reverse ~25), each with a witness pair | forward | reverse | no |
| exporter term printer, beyond the vectors | forward | reverse | no |
| definitional identification (§4 "≡", the CR-2 row) | yes | for Lean-side links only | no |
| CakeML trusted base (ISA model, FFI oracle model, basis/startup) | yes | yes | no |
| Lean compiler + C compiler + runtime | no | no | yes |
| Microkit / seL4 (its own proofs, CapDL) | yes | yes | yes |

**Round trip, and why it is not enough (Codex round 2).** With both directions, HOL → Lean
→ HOL can be checked by `aconv` against the stored equations. That catches printer bugs
which independent code does not undo. It does **not** establish meaning. A forward
`word_lt ↦ ult` paired with a reverse `ult ↦ word_lt` round-trips perfectly while the
Lean middle is wrong. So the round trip is fault detection, not a soundness argument.
What does move rows out of "witnessed" into "specified" is a **row contract**. Each row
names a mathematical specification independent of both printers, for example signed
comparison through `toInt` / `w2i` on `0 ≤ n < 2^w`. It then has a kernel lemma *in each
system* tying the native operation to that specification (Lean `BitVec.slt_iff_toInt_lt`,
HOL `WORD_LT` with `w2i`). The dispatch tables are generated from the proved contracts,
never by inverting one table into the other. Kaliszyk–Krauss's Isabelle importer
("characteristic properties" of mapped concepts, ITP 2013) is the precedent. The open
question this cannot close: whether the two specifications mean the same thing, which is
one small, reviewable, shared definition per row. Then the residual printer TCB is
binding, name and type translation, which round trip plus vectors attack from two sides.

## 5. What stays open by construction

Neither kernel validates the other. Trusting the composite means trusting the union of
both kernels, not their intersection. A mutual loader cannot change that: by Gödel II,
neither theory proves its own consistency. svenvs's Löb reduction shows that a system
which internally trusts its own reflected checker proves anything it can prove that
checker accepts, so self-trust is bought only by Löb-style reductions, never for free
(svenvs CLAIMS.md, [READ]). Nothing in this bridge moves proofs. A theorem proved in one
kernel is *re-proved* in the other (commit_before_forward in HOL4) or *used across the
identification* (the CR-2 row). The identification is checked, extensionally and on
vectors, but not proved. Proving it would need a formal semantics of one logic inside the
other. That is a Lean model in HOL4 (hol-reflection's `LCA` cardinals are the starting
point) or a verified Lean checker in Candle. No one has either (BRIDGE-SURVEY §0, §9).
What is closed is narrower and real. Every cross-system claim we make is now *about the
same named definition*. The places where two hand-maintained copies diverge become
theorems instead of surprises, and two of them already have.

## 6. Plan for what is not done

1. **Remaining `evaluate` clauses.** Call and DecCall need `lookup_code` (FUPDATE_LIST,
   ZIP, LIST_REL) and a `match h :` emission. The wrapped `fix_clock` call binds its state
   in a case pattern, which Lean's termination proof needs as a hypothesis. Raise needs
   `size_of_sh_with_ctxt` in full (`ALOOKUP`). Store32/Load32 need
   `mem_store_32`/`mem_load_32` (word_of_bytes, aligned). ShMem needs
   `word_to_bytes`/`word_of_bytes`. NStruct/NField need UNZIP/ALOOKUP plus auto-attach
   lemmas. The gate is the same for each: the stored clause re-proved in Lean, plus a
   vector.
2. **fix_clock-free equations.** Prove `evaluate_clock` and `fix_clock_evaluate` in Lean.
   The stored `evaluate_def` clauses then follow, as they did in HOL.
3. **Polymorphic word width**: `'a ↦ (w : Nat)`, `dimindex ↦ w`, with `0 < w` as a
   side condition. This needs a witness at w = 8, 32 and 64.
4. **Row contracts** (§4): a shared specification and kernel lemmas on both sides for
   every row, starting with the word rows. Then the round-trip checker as an additional
   fault detector, tested against a coordinated two-table mutation.
5. **Device to machine code**: `cv_compile`/in-logic compile of the translated `stepS`
   with a CakeML `main` that calls the Microkit FFI. Whether a CakeML binary can share
   the seL4 seat is an ember decision (below). The capability theorem over the CapDL
   output (degg-research `b98c704`) is the consumer.
6. **Independent kernel re-checks** (BRIDGE-SURVEY): run nanoda/lean4lean over the
   generated Lean, and the Candle-verified OpenTheory reader over an article export of
   `leanChoke`.

## 7. Decisions only ember can make

- Re-target the orb ladder at `P3.Pan.evaluate` and retire Sem.lean's "faithful
  transcription" header, or keep Sem.lean with `evaluate_agree` as its warrant. Either
  way, the `Scoped` hypothesis is now part of the orb's contract.
- Whether the chokepoint PD ships the Lean-compiled `stepS` (today) or the CakeML-compiled
  one (proved commit-before-forward in HOL4, without Lean's or C's compiler in the TCB).
  Also whether the CakeML route's FFI/ISA trusted base is acceptable inside a Microkit PD.
- Whether the CR-2 ledger accepts the definitional identification as a row, or requires
  re-proof in HOL4 for every composed statement. The second is the device route's shape
  already; for the orb it means porting the serve proof.
- Whether to spend the next lane on row contracts (moving the rows from witnessed to
  specified) before adding more `evaluate` clauses.

## 8. Reproduce

- HOL: `docs/engine/probes/p3-bridge/exporter/run-on-hbox.sh export2leanTheory` and
  `reverse/hol/run-on-hbox.sh chokeProofTheory` / `chokeTransTheory`. These use hbox,
  CakeML `ed31510b3`, `cakeml-heap`, and `swarm-build` with an 8G cap. Each takes 2–30 s
  [MEASURED]. The Mac's `~/dev/HOL` is half-built: no `bossLib.uo`, no `hol.state`, and
  `hol.state0` was rewritten on Jul 27, so Holmake cannot run there today [MEASURED].
- Lean: `cd docs/engine/probes/p3-bridge/lean && lake build` (Lean 4.30.0, core only;
  it builds the orb-compiler modules it imports into its own `.lake`), then
  `python3 ../audit/diff_skeletons.py`.
- Reverse Lean side: build the chokepoint oleans with
  `experiments/chokepoint-step/check.sh` (`OLEANS=…`), then
  `LEAN_PATH=$OLEANS lean reverse/Lean2Hol.lean`.
