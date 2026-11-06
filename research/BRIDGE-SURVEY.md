# BRIDGE-SURVEY — the HOL4 ⇄ Lean 4 transport landscape, late 2026

**Date:** 2026-11-06. **Scope:** what exists for moving definitions, theorems or
proofs between HOL4/CakeML and Lean 4, and whether any of it should replace or sit
beside our *definitional bridge*. The bridge (ADR-3-REV, `DreggNet/docs/engine/10-DECISIONS.md`;
evidence `DreggNet/docs/engine/probes/P2-REPORT.md`) has two directions:

- **forward:** an SML exporter walks HOL4's TypeBase/DB and emits Lean `inductive`/`def`s.
  Three checkers re-check it: a structural skeleton diff, the stored HOL defining
  equations re-proved in Lean by `rfl`/`simp only`, and behavioural vectors evaluated in
  both kernels.
- **reverse:** a Lean metaprogram emits HOL4 `Datatype`/`Define` from the elaborated Lean
  environment, re-checked from the HOL side.

**Evidence tags.** [READ]: I read the source, README, page or code myself in this pass.
[CLAIM]: someone asserts it and I did not verify it. [INFERRED]: my reasoning from what
I read. A "last observed" date is the GitHub `pushed_at` value I queried on
2026-11-05/06. It counts a push to any branch, not a release.

**Local checkouts used:** HOL4 `~/dev/HOL` at `2faefbd` (2026-05-15); CakeML
`~/dev/CakeML` at `ac654a0` (2026-05-11); `~/dev/hol-reflection` (last commit
2017-01-30); `~/dev/svenvs`. Upstream heads are newer: CakeML was pushed 2026-11-06
and HOL4 2026-11-04 [READ, GitHub API].

---

## 0. One-paragraph answer

No published tool does what our bridge does: a definitional HOL4→Lean reflection that
emits *native* Lean datatypes and functions, guarded by kernel-checked equation
re-proofs. No tool does the reverse either, a checked Lean→HOL4 definition export.
The nearest neighbour in the forward direction is the Deducteam HOL Light→Lambdapi→Lean
pipeline. It is proof-level: it translates proofs through hol2dk, then "aligns" the
ε-defined images with Mathlib by Lean proofs (`theorem add_def : Nat.add = ADD`). The
alignment step is the same idea as our checker 2, and the pipeline is the closest prior
art to cite. The nearest neighbour in the reverse direction is a 2026 hobby project
(`hol-init-e`). It imports a Lean AST into HOL4 by an *unverified* Python text importer
and says explicitly that this is "not a proof of agreement between Lean and HOL".
Verified checkers exist on the HOL side: CakeML's OpenTheory reader carries a
machine-code soundness theorem. On the Lean side, Lean4Lean is partial, and its
consistency proof is still `sorry`. No Lean/CIC type checker is verified in
HOL4, CakeML or Candle. None of these replaces the bridge. Two are worth standing up
beside it as extra, independent checkers: Candle-checked OpenTheory articles for the
HOL side of the reverse direction, and nanoda/lean4lean re-checks of our generated Lean.

---

## 1. OpenTheory

### 1.1 Format and HOL4 support

- **What:** a stack-machine article format for HOL-family kernel proofs (Hurd). The
  reference tool is `gilith/opentheory` (SML); `gilith/hol` is a Haskell kernel that
  reads articles. https://github.com/gilith/opentheory
- **Last observed:** `gilith/opentheory` 2026-10-11, but the recent commits are PHP-8
  website fixes ("PHP 8 changes"). The last substantive code commit I saw is older.
  `gilith/hol` was pushed 2026-09-12. [READ]
- **HOL4 writer:** `src/opentheory/postbool/Logging.{sig,sml}` provides
  `start_logging`, `export_thm` and `stop_logging`, plus name handlers. It logs the
  kernel inferences behind each exported theorem into an article. The kernel's proof
  constructors were restructured on 2026-03-18 ("Restructure Def_*_prf proof
  constructors"), so the writer is maintained. [READ]
- **HOL4 reader:** `src/opentheory/reader/OpenTheoryReader.{sig,sml}` provides a
  `reader` record with `define_tyop`, `define_const`, `axiom`, `const_name` and
  `tyop_name`. Its default `axiom_in_db` resolves article assumptions against the
  HOL4 DB. Articles are replayed through HOL4's own LCF kernel, so the reader is
  trusted only as far as the HOL4 kernel is. [READ]
- **Scale evidence (2026):** `flamingoponderado/hol-init-e` exports a 4.4 GB
  OpenTheory article from a HOL4 `cv_compute` certificate. Its README says it was
  "independently replayed". [CLAIM]
- **Verified vs trusted:** article *checking* can be verified (§1.2). Article
  *production* (HOL4 `Logging`) is unverified, but it does not need to be trusted: a
  wrong article fails to check.
- **Relevance:** OpenTheory works at the granularity of kernel theorems over simple
  type theory. It carries no datatype package, no `Define` and no recursion compiler,
  only the primitive `defineConst`/`defineTypeOp` steps. That is why P2 §5 put it out
  of scope as the *definition* bridge, and that remains correct. Its real use for us
  is the **reverse direction's HOL side**: once Lean-emitted `Datatype`/`Define`
  scripts have run in HOL4, the resulting theorems can be logged as articles and
  re-checked by the verified reader (§1.2). That would make the HOL-side check
  independent of HOL4's own kernel. [INFERRED]

### 1.2 The verified OpenTheory checker in CakeML

- **What:** an article checker built on the Candle kernel (Abrahamsson; JLAMP line,
  merged in CakeML PR #515 in 2018). It moved to `examples/opentheory/` in 2020 and
  was improved in 2021–22. In 2026 it gets maintenance-only commits, the latest being
  "Fuse strlit and implode" on 2026-04-30. [READ: `git log`]
- **Where:** `~/dev/CakeML/examples/opentheory/` (`readerScript`, `readerProofScript`,
  `readerSoundnessScript`, `compilation/proofs/readerProgProofScript`). [READ]
- **What is verified (statement read):**
  - `readerSoundnessScript.sml : reader_sound` says that if `reader_main` succeeds,
    `the_context extends init_ctxt` and every output `Sequent asl c` satisfies
    `is_set_theory mem ⇒ (thyof ctxt, asl) |= c`. This is semantic validity in
    Candle's set-theoretic model. [READ]
  - `compilation/proofs/readerProgProofScript.sml : machine_code_sound` lifts the
    same statement to the **x64 machine code** (`installed_x64 reader_code mc ms ⇒
    machine_sem … ⊆ extend_with_resource_limit {Terminate Success …}`). There is also
    an ag32 variant. [READ]
- **Trusted residue:** the x64/ag32 ISA model, the FFI/basis model,
  `is_set_theory` (a set-theoretic universe exists), and that `init_ctxt` is the
  intended theory. A user also trusts that the article's *conclusions* say what they
  mean, so name mapping matters. [READ for the first three; INFERRED for the last]
- **Relevance:** this is the only proof checker in our landscape with an end-to-end
  verified binary. It can sit beside the reverse bridge as a HOL-side checker that
  does not trust HOL4's kernel. It cannot replace either direction, because it checks
  HOL theorems and knows nothing about Lean. [INFERRED]

### 1.3 Lean and OpenTheory

- **No Lean reader of OpenTheory articles exists.** A GitHub code search for
  `defineTypeOp language:lean` returned 0 hits. The unrestricted search found the
  command name only in gilith/opentheory, gilith/hol, HOL4, CakeML and
  `c-cube/trustee` (OCaml). [READ: search results]
- **Logipedia/STTfa** (Thiré, LFMTP'19) exported Matita arithmetic through Dedukti
  simple type theory to several systems, including OpenTheory and Lean (3).
  `Deducteam/Logipedia` was last pushed 2024-11-11. [READ date; CLAIM for content, from
  the Dedukti references cited in the dk2isa page]
- **Holide** (Assaf & Burel 2015) is an OpenTheory→Dedukti translator. It translated
  the OpenTheory standard library (arXiv 1507.08720). Last pushed 2019-11-05, so it
  is dormant. [READ date and abstract]

---

## 2. Lean's export format and independent Lean checkers

### 2.1 lean4export

- https://github.com/leanprover/lean4export, last observed 2026-10-14. It is "plain-text
  declaration export" in an **NDJSON** format (`format_ndjson.md`), with options
  `--export-unsafe` and `--export-mdata`. It dumps a module's kernel declarations
  together with their transitive dependencies. [READ README]
- The Lean reference manual ("Validating a Lean Proof") describes the
  `lake comparator` flow: build in a sandbox, export, then replay with Lean's kernel
  and/or nanoda. The manual names the **Lean Kernel Arena** as the place to find more
  checkers. It also says `Lean.trustCompiler` / native evaluation "was removed from
  Lean in version 4.35.0". [READ]
- **Relevance:** this is the right *input* for the reverse direction. We could read
  lean4export NDJSON instead of walking the live `Environment` in a metaprogram. That
  would make our reverse exporter a standalone program with a stable,
  checker-community-reviewed input format. It would also mean the same bytes we
  translate could be re-checked by nanoda or lean4lean. [INFERRED]

### 2.2 Independent checkers

- **nanoda_lib** (Rust, ammkrn), https://github.com/ammkrn/nanoda_lib, last observed
  2026-10-29. It is an external type checker that reads lean4export output. It has a
  config-file interface with `permitted_axioms` and opt-in Nat/String kernel
  extensions. It is unverified, but it is an independent implementation, and it is
  the checker comparator supports. [READ README]
- **Lean4Lean** (Carneiro), https://github.com/digama0/lean4lean, last observed
  2026-10-10, toolchain `v4.33.0-rc2`. It is a Lean kernel written in Lean, "derived
  directly from the C++ kernel … likely shares some implementation bugs". It contains
  `Theory/` (a formal typing relation `VEnv`/`IsDefEq`/`HasType`) and `Verify/`
  (kernel implementation ⇒ theory). The paper is *Lean4Lean: Verifying a Typechecker
  for Lean, in Lean*, TYPES 2025, LIPIcs 384, published 2026-07-30. [READ]
  - **Status measured on a shallow clone (master, 2026-11-05):** 131 `sorry`
    tokens. Most are in `Experimental/`, but sorries remain in core files:
    `Theory/Typing/ChurchRosser.lean` (2), `Theory/Typing/Injectivity.lean` (3),
    `Theory/Inductive.lean` (2), `Theory/Typing/UniqueTyping.lean`,
    `Verify/Typing/Lemmas.lean` (7), and `Verify/TypeChecker/{IsDefEq,WHNF,Reduce,InferType}`.
    The 2026 commits close substitution sorries and verify primitives. [READ]
  - **Consistency:** `digama0/lean4lean-model` (last observed 2026-09-23) states
    consistency of `Lean4Lean.Theory` "assuming ω inaccessible cardinals" in
    `Consistency.lean`, and that statement is "`sorry`ed pending the model
    construction". [READ README]
  - **Verdict:** Lean4Lean is partially verified with respect to a formalised typing
    relation, and the relation's consistency is unproved. Even when finished, it would
    be verified *in Lean*, so its foundation is the system it checks. [READ + INFERRED]
- **Lean Kernel Arena**, https://arena.lean-lang.org/, has a round in progress. I
  counted 33 checker entries, among them `official v4.34.1`, `lean4lean`, `nanoda`,
  `nano-lean`, `tenet`, `tenet-hs`, `kefir`, `rpylean`, `lean4cobol`, `evmlean`,
  `ind-models`, `metalean-*` and `canonical-min`. The front page gives no
  implementation languages and no verification claims. [READ]
  The separate SAIR **Lean Kernel Challenge** (Stage 1, announced 2026-09) is about
  *performance* of verified computation, not checker verification. [CLAIM, from the
  Kagi snippet and Tao's post title]

### 2.3 Lean type theory formalised elsewhere

- **In HOL4, CakeML or Candle: nothing found** (absence list, §8). The CakeML and
  HOL4 trees contain no mention of Lean (`git grep -i "lean4|leanprover|lean 4"` was
  empty). [READ]
- **`FawadHa1der/lean4lean-to-rocq-hol`** was created 2026-10-26 and has 6 commits,
  0 stars and 0 forks. It claims "independent, machine-verified ports of
  `lean4lean`" to Rocq 9.0 (104 modules, "zero axioms, zero Admitted") and to
  **HOL Light** (131 modules, "zero added axioms"). It also claims a proved
  Church–Rosser property and a kernel-to-theory soundness bridge. [CLAIM]
  *Caution:* upstream lean4lean still has `sorry` in exactly those places
  (ChurchRosser, UniqueTyping), and a day-old repo with this profile is likely
  machine-generated and may prove weaker statements under the same names. Treat it as
  unaudited. If it is ever cited, the gate is to read the theorem *statements*. Even
  at face value its "soundness" is checker ⇒ declarative typing, not a model
  theorem. [INFERRED]
- **Rocq's MetaCoq/MetaRocq** is the closest mature analogue for CIC: a verified
  safe checker and erasure, verified relative to a formalised CIC whose strong
  normalisation is assumed. It covers Rocq's theory, not Lean's (no definitional
  proof irrelevance, no K-like reduction, different universe handling). [CLAIM,
  background knowledge, not re-read in this pass]

### 2.4 Rocq/Coq importers of Lean

- **rocq-lean-import** (Gaëtan Gilbert), https://github.com/rocq-community/rocq-lean-import,
  last observed 2026-10-01. It reads lean4export files and is described as "an
  experimental alpha … useful to compare how Lean and Rocq work but probably not much
  beyond that". [READ from the Kagi snippet of the README]
  According to the lean2agda README, its Mathlib run was made "with conversion
  checking off". [CLAIM, second hand]
- **Lean4Less → lean2dk → Dedukti → Rocq:** see §3.
- **Babel-formal** (Stoskopf, Cohen, Tabareau; NeurIPS 2025 MATH-AI workshop) does
  LLM proof-term-pivot translation between Lean and Rocq on a benchmark of 117
  lemmas. It is not a checked translator. [READ snippet]
- **lean2agda** (`formalverification/lean2agda`, status 2026-10) reads `.olean` and
  intends to emit Agda; "no translated module checks whole yet". [READ via WebFetch]

---

## 3. Dedukti / Lambdapi / EuroProofNet

- **Lambdapi**, https://github.com/Deducteam/lambdapi, last observed 2026-11-05, is
  actively maintained. Its `src/export/` has `coq.ml`, `dk.ml`, `hrs.ml`, `lean.ml`,
  `stt.ml` and `xtc.ml`. `lean.ml` had commits on 2026-10-27 ("Lean export: prefix
  namespace by root_path …"). [READ]
- **HOL Light → Dedukti/Lambdapi/Rocq: `hol2dk`** (Blanqui, LPAR 2024),
  https://github.com/Deducteam/hol2dk, last observed 2026-10-27. It translates HOL
  Light's Multivariate library ("more than 20,000 theorems") and the Logic library to
  Rocq (`coq-hol-light`, `rocq-hollight-logic`), with HOL Light types and functions
  mapped to the Rocq stdlib. [READ README]
  Per the lean2agda README, the theorems ship "as axioms" with a regeneration script
  that re-checks the proofs in about a day. [CLAIM]
  **Alignment** is the step that replaces translated ε-definitions by idiomatic
  targets with proved equalities: Blanqui & Gontard, *Aligning HOL-Light and Rocq
  libraries formally*, arXiv 2609.24598 (LPAR-26), with Rocq tactics that automate
  the equivalence proofs. [READ abstract]
- **HOL Light → Lean 4** (Iván Martínez Comas, supervised by Blanqui and Massot;
  slides at navimath.github.io/files/slides.pdf): HOL Light → hol2dk → Lambdapi →
  Lean export. HOL `new_recursive_definition` arrives as a `noncomputable def ADD :=
  Classical.epsilon (…)`, which is then aligned by `theorem add_def : Nat.add = ADD`
  using a custom `epsilon_tac`. [READ: slide text extracted with pdftotext]
  The slides list alignments proved so far: connectives, quotients, unit, product,
  num with arithmetic, even/odd and min/max, option, sum, list ("most of its
  functions") and reals. A repo `Deducteam/lean-hollight` was created 2026-10-23 and
  holds only `mapping.lp` and `renaming.lp` so far. [READ]
  - **What is verified:** each HOL Light proof is re-checked as a Lambdapi term and
    then by Lean's kernel. Each alignment is a Lean theorem. The trusted parts are the
    encoding files (`encoding.lp`, `mappings.lp`) and the claim that the HOL Light
    proof dump is the proof HOL Light checked. [INFERRED from the pipeline slide]
  - **Relevance:** this is the closest prior art to our forward direction, and it is
    the *opposite design point*. They carry proofs and get ε-terms they must align
    afterwards. We carry definitions as native Lean and check the stored equations.
    Their `add_def : Nat.add = ADD` alignment is our checker 2 pointed the other way
    (target-native = source-image, proved in the target). Cite it. It targets HOL
    Light, not HOL4. A HOL4 route would go through OpenTheory→Holide (dormant) or a
    new HOL4→Dedukti exporter. Nothing on HOL4 exists. [INFERRED]
- **Lean → Dedukti:**
  - **Lean4Less** (Vaishnav; ICTAC 2025; HAL hal-05310102),
    https://github.com/Deducteam/Lean4Less, last observed 2026-06-17. It eliminates
    proof irrelevance and K-like reduction by inserting casts, producing "Lean⁻".
    [READ README]
    It has handled `Mathlib.Data.Real.Basic` but "does not yet scale to higher Mathlib
    modules". [CLAIM, per the lean2agda README]
  - **lean2dk**, https://github.com/Deducteam/lean2dk, last observed 2026-06-15, "WIP".
    Its most complete branch, `claude-stable` (pinned to Lean v4.18.0-rc1),
    "translates and type-checks `Init.Data.Nat.Lemmas` end-to-end against a patched
    Dedukti kernel" with constant stubbing. [READ README]
  - **Theory paper:** Blanqui & Vaishnav, *Encoding Lean's Type Theory in Dedukti*,
    arXiv 2609.24604, 2026-09-21. It presents a Dedukti theory encoding Lean⁻ and "a
    typability-preserving translation from some large subset of Lean". [READ abstract]
- **Isabelle → Dedukti:** `Deducteam/isabelle_dedukti`, last observed 2026-07-01. It
  targets Isabelle2025 and needs a patched HOL/HOL-Library ("so that … no oracle are
  used"); it is checked by kocheck, dkcheck or lambdapi. [READ README]
- **Coq → Dedukti:** `Deducteam/CoqInE`, last observed 2025-09-04. [READ date]
- **Dependent → simple type theory (Lean/Rocq → Isabelle/HOL Light):** Blanqui's
  dk2isa project page (updated 2026-11-03) is a student-project proposal and states
  "there is currently no translator in the other way around". [READ]
  This is direct absence evidence for any checked Lean→HOL translator.
- **What is verified across Dedukti:** none of the translators is itself verified.
  Soundness comes from re-checking the output in Dedukti/Lambdapi (or kocheck) and
  then in the target kernel. The encodings (`encoding.lp`, the Lean⁻ theory) are
  trusted, though some have paper proofs of conservativity or typability
  preservation. [INFERRED]

---

## 4. Isabelle ⇄ HOL Light/HOL4, Isabelle/HOL Light ⇄ Lean, Coq ⇄ Lean

- **Isabelle HOL-Import:** the session exists in Isabelle2025 (`Import_Setup`,
  `import_data.ML`, `import_rule.ML`, `HOL_Light_Maps`). [READ: session index page, via
  search result]
  It comes from Obua & Skalberg (2006) and Kaliszyk & Krauss (*Scalable LCF-Style
  Proof Translation*, ITP'13). A newer effort is Bergeron, Glondu & Tourret, *HOL
  Light to Isabelle/HOL Translation, Rebooted* (Isabelle Workshop 2024). [READ
  references]
  It replays proofs through Isabelle's kernel, so no translator is trusted.
  `xrchz/isabelle-opentheory` was last pushed 2019 and
  `isabelle-prover/opentheory-component` in 2018. [READ dates]
- **HOL4 → Isabelle:** the 2006 importer covered HOL4 too. I found no maintained HOL4
  path in 2026. [READ (2006 abstract); absence otherwise]
- **Isabelle → Lean / Lean → Isabelle:** "Lean2Isabelle: Factorized Cross-Assistant
  Proof Translation" (ICML 2026) is LLM-based and verifier-grounded per target
  theorem. It is not a definitional translator. [READ snippet]
  `lean-dojo/ITPEval` (arXiv 2607.19407) benchmarks LLM statement and proof
  translation across Lean, Rocq, Isabelle and HOL Light. [READ snippet]
- **HOL Light → Lean by LLM:** `zengyuzhi/HOL-Light-to-Lean4` (last pushed
  2025-05-29) is an LLM port of the top-100 theorems; only `Quartic.lean` is fully
  proved. [READ README]
- **Coq ⇄ Lean:** see §2.4. Nothing in either direction is checked at library scale.

---

## 5. Candle as a host for a Lean checker; a Lean kernel in HOL4 or CakeML

- **What Candle is:** a HOL Light-style kernel implemented in CakeML and proved sound
  against a set-theoretic semantics (`candle/standard/{syntax,semantics,ml_kernel,monadic}`,
  `candle/prover`). There is also an ad-hoc-overloading variant
  (`candle/overloading/`) with consistency proofs. [READ README]
  svenvs uses `holSoundnessTheory.proves_sound` and the `holSyntax` checker stack,
  `cv_eval`-translated (svenvs CLAIMS §4–§5). [READ]
- **Could a Lean (CIC with universes) checker be stated in it?** [INFERRED throughout]
  1. **Stated in HOL4, then compiled by CakeML:** yes, in principle, by the same
     recipe CakeML used for Candle. The pieces would be: a deep embedding of Lean's
     expressions and levels (Lean4Lean's `VExpr`/`VLevel` are small; FawadHa1der
     claims a HOL Light port of them), a typing relation (`VEnv`, `IsDefEq` with
     proof irrelevance, eta for structures, K-like reduction, quotients, nested and
     mutual inductives), a checker, and `checker ⇒ typing`. This is MetaCoq-sized
     work, measured in person-years. Lean4Lean's still-open `Verify/` sorries show
     how far even the author-in-Lean version is from done.
  2. **Soundness down to semantics** needs a model with ω inaccessibles, which is
     exactly what `lean4lean-model` wants. **HOL4 already has the large-cardinal
     scaffolding:** `hol-reflection/lca/lcaScript.sml` defines `LCA n P` (n nested
     strongly inaccessible cardinals), with `LCA_holds` giving `∀n. ∃P. LCA n P` from
     a hypothesis, and svenvs's `loebReduction` already runs on it. [READ]
     A HOL4 model of Lean's universe hierarchy would have that as its natural base.
     This is the most interesting long-run "verified Lean checker in HOL" foothold,
     and it is unbuilt.
  3. **Inside Candle as a theory** (an object-logic definition of Lean typing in
     Candle's HOL, checked by the live kernel): possible, but strictly more work than
     (1) for no extra assurance, because Candle's soundness is already a HOL4 theorem.
- **Any work verifying a Lean kernel in HOL4:** none found (§8). The only
  "Lean kernel in a HOL" is the unaudited HOL Light claim in §2.3.

---

## 6. Proof-producing translation into and out of HOL4

- **`ml_translatorLib`** (`~/dev/CakeML/translator/`) is "a proof-producing translator
  from HOL functions to CakeML". For each HOL function it emits CakeML code plus a
  certificate theorem relating them in CakeML semantics. It has monadic and PMATCH
  variants (Myreen & Owens; Abrahamsson for the monadic translator). [READ README]
- **`cv_transLib`** (HOL4 `src/num/theories/cv_compute/`, last touched 2026-04-13
  locally) translates HOL functions into `:cv` equations that the kernel `compute`
  primitive evaluates, each with an equivalence theorem. CakeML's `cv_translator/`
  uses it to evaluate the whole compiler in-logic. [READ]
  `hol-init-e` uses it to translate all of Pancake and the RISC-V backend.
- **These are the in-HOL analogues of our checker 2**, translation plus a
  kernel-checked equivalence theorem, but both stay inside one logic. Our
  cross-kernel version cannot produce one theorem spanning both logics. That is why
  the bridge is a CR-2 ledger row, re-checked independently on each side. [INFERRED]
- **Lean → HOL4 definitions, proof-producing:** **does not exist anywhere I could
  find.** The closest things are:
  - **`flamingoponderado/hol-init-e`** (created 2026-10-07, last observed
    2026-10-26, 0 stars). `tools/import_ast.py` converts a Lean
    `Guest/SoftwareAst.lean` into a HOL4 `panLang$decl list`. "The importer checks
    the entire input hash and rejects unknown syntax and constructors. Its output is
    checked as HOL definitions, but the importer is **not a proof of agreement between
    Lean and HOL**." The README repeats several times that agreement with the Lean
    decoder and step "remains to be proved". [READ README]
    This is a live, concrete instance of the exact gap our reverse direction closes.
  - **`Lee-Juntong/CakeML_data_extraction`** (created 2025-09-24, last observed
    2026-11-04) is HOL4→Lean by **Gemini**: it covers CakeML's `ast`, `location`,
    `namespace` and `namespaceProps` (180 declarations), with LLM-based "validation".
    It has no kernel cross-check. [READ README]
    It is the same target slice as P2, with none of P2's three checkers. Worth
    knowing as the alternative our checkers beat.
- **Lem:** maintained (pushed 2026-06-10; 2026-05 HOL4 Trindemossen-2 fixes). It has
  no Lean backend on master. A `lean-experiment` branch by Peter Sewell (5 commits,
  2026-03-07..09) copies the Coq backend to get started, producing "only partly
  broken Lean" on a trivial example, and has stalled since. [READ: branch compare]
  CakeML's semantics are no longer Lem-sourced (P2 §0), so Lem is not a path for us.
- **Sail** has an active Lean backend (`src/sail_lean_backend`, commit 2026-11-03).
  [READ]
  It is relevant only as a precedent for a multi-target spec language reaching Lean.
- **Other HOL4 → Lean export efforts besides ours:** only the LLM one above. A GitHub
  repo search for "hol4 to lean", "hol2lean", "lean4 export hol" and "lean to hol"
  returned nothing. [READ]

---

## 7. Interoperability and proof transfer, 2025–2026

- **Trocq** (Cohen, Crance, Mahboubi et al., ESOP 2024),
  https://github.com/rocq-community/trocq, last observed 2026-11-04. It is a modular
  parametricity plugin for proof transfer inside Rocq, with a hierarchy of relation
  structures up to type equivalence. [READ README]
  It works within one system. I did not search for a Lean port.
- **CompPoly / ArkLib** (`Verified-zkEVM`, last observed 2026-11-05/06): computable
  polynomial representations with `RingEquiv`s to Mathlib's `Polynomial R` and
  `MvPolynomial (Fin n) R` (and multilinear and bivariate equivalents). [READ README]
  This is the within-Lean analogue of P2's open item §5-2, the **computability twin**:
  ship the reflected (noncomputable, `Prop`-valued) semantics *and* a computable twin
  joined by a proved equivalence, rather than making the reflection itself
  computable. [INFERRED]
- **Lean-side re-checking practice (2026):** comparator plus nanoda, lean4checker, and
  the Arena (§2). This is the community's working answer to "trust the kernel less".
- **2026 papers found:** Blanqui & Vaishnav (arXiv 2609.24604), Blanqui & Gontard
  (arXiv 2609.24598, LPAR-26), Carneiro (TYPES 2025, LIPIcs 2026), ITPEval (arXiv
  2607.19407), Lean2Isabelle (ICML 2026). I did not get a CPP 2026 or ITP 2026 paper
  on HOL4⇄Lean from search; the queries mostly returned conference home pages.
  [READ: search results]
- **MMT/OMDoc:** library alignment and knowledge management (Kohlhase, Rabe). It does
  not do checked transport in our sense. [CLAIM, background knowledge, not searched in
  this pass]

---

## 8. Verdict

**Keep the definitional bridge as the primary transport, in both directions.** Nothing
in the 2026 landscape does checked *definition* transport between HOL4 and Lean in
either direction. The Deducteam line is the only library-scale cross-logic transport
into Lean, and it is proof-level, HOL Light-only and ε-heavy. The one third party that
needed Lean→HOL4 (`hol-init-e`) used an unverified text importer and left the
agreement proof open. The bridge is not redundant with any existing tool, and we
should still not call it a "first" in print. Say "we found no prior checked HOL4⇄Lean
definitional translator", cite Deducteam HOL Light→Lean as the closest prior art and
hol-init-e as the gap, and keep the absence list below as the evidence.

| Candidate | Role | Replace or beside? | Cost |
|---|---|---|---|
| **Candle-checked OpenTheory articles** (CakeML `examples/opentheory`, `machine_code_sound`) | HOL-side re-check of the *reverse* direction's output, and of any HOL4 theorem we cite as the backend anchor | **Beside.** It adds a checker that does not trust HOL4's kernel and whose binary is verified down to x64. | Low to medium. Turn on `Logging.export_thm` for the generated theories and map names through `OpenTheoryMap`. The risks are article size (hol-init-e reports 2.8–4.4 GB for heavy `cv_compute` proofs, so keep the exported cone to the generated defs plus their equations, not the backend correctness theorem) and the fact that articles carry no datatype package, only its primitive consequences. |
| **nanoda + lean4lean re-check** of our generated Lean (via lean4export NDJSON) | A second and third Lean kernel on checkers 1–3 | **Beside.** Nearly free. | Low. One CI job: `lean4export` → nanoda config with `permitted_axioms = [propext, Classical.choice, Quot.sound]`, then `lake env lean4lean` on the Generated module. Also switch the reverse exporter to read NDJSON instead of the live environment, so translator and checkers see the same bytes. |
| **A verified Lean checker in HOL4/CakeML** | Would let the HOL side *verify* the Lean reflection's typing, removing trust in Lean's kernel | **Not now; research beside it.** | Very high, MetaCoq-scale. The foothold exists: `hol-reflection/lca` gives nested inaccessibles in HOL4, and Lean4Lean's `Theory/` is a ready spec to port. Even when built, it does not check *semantic agreement*, which is checker 2's job, so it would not replace the bridge. |
| **Dedukti/Lambdapi as pivot** (HOL4→OpenTheory→Holide→Lambdapi→Lean; Lean→Lean4Less→lean2dk→…→HOL) | Proof-level transport | **No.** | High and brittle. Holide has been dormant since 2019. lean2dk works on Lean v4.18-rc1 at `Init.Data.Nat.Lemmas` scale with a patched kernel. No Dedukti→HOL4 path exists. The output would be ε-terms needing alignment, which is the work our bridge avoids by emitting native defs. |
| **Isabelle HOL-Import / HOL Light routes** | n/a for HOL4⇄Lean | **No.** | Only relevant if ADR-3's Isabelle arm (CryptHOL) ever needs HOL4 theorems, and then HOL4 support is the missing piece. |
| **LLM translators** (CakeML_data_extraction, Lean2Isabelle, Babel-formal) | n/a | **No.** | They are the unchecked baseline our three checkers exist to beat. |

**Design changes this survey suggests** (none of them overturn ADR-3-REV):
1. **Reverse direction:** read **lean4export NDJSON** rather than the live environment,
   and re-check the HOL4 result with the **CakeML OpenTheory reader** as well as HOL4's
   own kernel.
2. **Forward direction:** add **nanoda and lean4lean** to the checker set (Generated and
   the `*_hol_eqn_*` theorems), in addition to Lean's C++ kernel.
3. **Computability twin (P2 §5-2):** follow the CompPoly pattern (computable twin plus
   proved equivalence) rather than making the reflection computable.
4. **Prior-art framing:** cite the Deducteam `add_def : Nat.add = ADD` alignment as the
   same move as our checker 2, applied after a proof-level translation.
5. **Watch list:** `Deducteam/lean-hollight` (2026-10, nearly empty), lean4lean-model
   consistency, Lambdapi `lean.ml`, and any audit of `FawadHa1der/lean4lean-to-rocq-hol`.
   A real HOL-family Lean kernel would reopen §5.

---

## 9. Absence evidence (searches that returned nothing relevant)

GitHub (`gh search repos` / `gh api search/*`, 2026-11-05):
- repos `"hol2lean"`, `"hollight lean4"`, `"HOL-Light Lean4 translation"`, `"hol4 to lean"`,
  `"lean to hol"`, `"lean4 export hol"`, `"opentheory lean"`, `"lean4 opentheory"`:
  **0 results each**. Sanity control: `lean4lean` and `hol2dk` returned hits, so the
  search worked.
- repos `"hol light lean"`: only the LLM port, ITPEval, the unaudited lean4lean port
  and an unrelated repo. Repos `org:Deducteam lean`: lean2dk, Lean4Less, lean-hollight
  only.
- code `defineTypeOp language:lean`: **0** (no Lean OpenTheory reader). Code
  `opentheory language:lean`: 83 hits, and the first page is all name collisions
  (PolyFun "OpenTheory" interaction modules, CatCrypt).
- repos `lem lean backend`: 0. Lem master `src/` has no `lean_backend.ml`; it exists
  only on the stalled `lean-experiment` branch.

Local trees:
- `git grep -i "lean4|leanprover|lean 4"` in `~/dev/CakeML` and `~/dev/HOL`: **empty**.
- `lcrio search nanoda` (the local cargo registry, not the panamax mirror, which was
  absent): no nanoda crate. nanoda was read from GitHub instead.

Kagi (17 queries in total; these returned nothing on point):
- "Lean 4 kernel type theory formalized soundness HOL4 OR Isabelle OR Coq verified type
  checker": only Lean4Lean and generic Isabelle pages.
- "verified Lean type checker CakeML OR Candle OR HOL4": only Lean4Lean.
- "Lean 4 HOL4 bridge export definitions CakeML": only the Gemini translation repo and
  hol-init-e (unverified importer).
- "translate Lean 4 definitions to HOL4 OR Isabelle exporter": only LLM work
  (Lean2Isabelle) and Blanqui's dk2isa page, which asserts no such translator exists.
- "\"Lean\" \"HOL4\" translator definitions proof-producing": nothing on point.
- "proof assistant interoperability translation ITP 2026 OR CPP 2026 OR CICM 2026":
  conference home pages only. **This is a weak negative**: I did not read the
  ITP/CPP 2026 accepted-paper lists.
- "Logipedia STTfa export OpenTheory Lean Matita 2025": junk results (Logopedia
  fandom). Logipedia status comes from the GitHub date only.

Not searched (gaps, not absences): a Lean port of Trocq or a parametricity plugin; the
full ITP 2026, CPP 2026 and CICM 2026 programmes; the Holide/Assaf HOL4-specific
article coverage; the Isabelle2025 NEWS file text for HOL-Import changes.

---

## Addendum (2026-11-06, after Codex synthesis, see BRIDGE-CODEX.md)

- MM0 (`digama0/mm0`) has `to-hol`, `to-othy` and `to-lean` exporters: one specification
  exported toward several systems. This is adjacent prior art for "the same definition
  in two kernels". It is not HOL4⇄Lean transport. [CLAIM: Codex round 1; not read here]
- Kaliszyk–Krauss, *Scalable LCF-style proof translation* (ITP 2013), the Isabelle
  importer of HOL Light, maps concepts using proved "characteristic properties". This is
  the precedent for the row contracts that BRIDGE-KNOT §4 now proposes. [CLAIM]
- Gordon–Hunt–Kaufmann–Reynolds, *An Integration of HOL and ACL2* (2006): a round trip
  used as confidence, not proof. [CLAIM]
