# Draft 10 engineering claim-delta plan

Status: **internal Markdown-only planning memo; not a filing source, not legal
advice, and not filed, submitted, sent, or published by this work.** Prepared
from committed local evidence on 2026-08-19. Draft 9 remains immutable.

**Artifact-authoring gate.** The mandatory PDF-skill marker program
`container_tools/mark_artifact_operation_started.mjs` remains unavailable.
Accordingly, this work does not create, edit, render, export, or re-export any
Typst or PDF artifact. The exact prose below is a future-edit plan only. It must
be rechecked against the final committed artifacts before any Draft 10 source
is created.

## 1. Decision

**PROPOSED:** branch a Draft 10 from Draft 9 only after the pending direct-V3
and ResolutionWork campaigns either land as clean commits or are expressly
excluded. Do not revise Draft 9 in place.

Draft 10 has two urgent corrections and three optional evidence promotions:

1. **Correct now:** replace the stale statement that the local SBF resolution
   path is not joined to a canonical source archive. The join now exists.
2. **Correct now:** preserve the equally important boundary that the complete
   source-construction path is real-SBF proven only in a separate
   non-production mock-provider ELF; the default ELF registers no release and
   fails closed. There is no production provider support.
3. **Promote selectively:** use the bounded Clear energy relation as an
   example of exact semantics, canonical optimality, and the difference
   between feasibility, conservation, and optimality.
4. **Promote selectively:** use the TFHE experiment only as encrypted
   validation of a caller-supplied candidate's feasibility and exact
   conservation. It is not an encrypted solver, an optimality certificate, a
   vFHE proof, or a Dark system.
5. **Preserve as STOPs:** the monolithic occupation route has no measured
   initial case that clears the chosen 25-percent compute-headroom gate, the
   committed ResolutionWork path is isolated and unreachable, and the routed
   direct-selection source has no committed real-SBF campaign. Pending
   worktree results must not enter Draft 10 until committed and inspected.

**INFERRED:** the source correction is mandatory for accuracy. The energy and
TFHE additions are useful mainly in the data-reporting, IAC, and separately
planned energy comment. They should not be forced into the product-definitions
comment if they distract from its classification argument.

## 2. Non-composition rule

The evidence classes below do not compose merely because they exist in related
repositories:

- a Clear exhaustive oracle can establish the result of its own bounded
  semantics without providing privacy;
- an FHE evaluator can hide inputs from an evaluator holding only an evaluation
  key without establishing optimality, evaluator correctness, result release,
  custody, or settlement;
- a machine-checked Lean theorem about a model does not verify a Rust, TFHE, or
  SBF implementation;
- a local real-SBF transition does not establish deployment, production data
  authenticity, transaction inclusion, economic liveness, or a complete
  venue; and
- permissionless invocation of one transition does not establish an
  operatorless lifecycle.

Every future Draft 10 paragraph should retain the evidence class in the same
paragraph as the claim. Moving the qualifier to an appendix is insufficient.

## 3. Committed evidence ledger

### 3.1 Bounded Clear energy semantics

**VERIFIED at the frozen bounds:** `degg-research` commit
`08a0fc357aa32dabf64e2c55f47c33211c148d67` implements
`confidential-energy-dispatch/p3-t3-b2-q4/v0`; commit
`f0cd755a337c6c217d607ac97a00b08f9b1ed939` promotes that result into the
repository's current verdicts. The evidence paths are:

- `research/confidential-energy-dispatch/`;
- `docs/research/CONFIDENTIAL_ENERGY_DISPATCH_RELATION.md`; and
- `docs/VERDICTS.md`, V11.

The relation fixes exactly three padded provider slots, three periods, two
buses, and integer output atoms `0..=4`. Its dependency-free Clear Rust oracle
and direct-recomputation verifier use a fixed 156-byte plaintext witness and a
fixed 176-byte public result frame. The representative fixture examines 8,025
first-two-provider trajectory pairs, finds 468 feasible complete schedules,
and selects the canonical objective/debit 56. A distinct fully recommitted
plan is physically feasible and conserves settlement at cost 60, but is
rejected because the verifier repeats the complete bounded optimum and tie
calculation.

Twenty-five tests pass in debug and release: two hash tests, twenty-one
relation/adversarial tests, and two corpus tests. Strict formatting, Clippy over
all targets with warnings denied, rustdoc with warnings denied, locked corpus
comparison, and repository checks are recorded green. The public frame exposes
coarse status, domain and input/plan/delivery commitments, and invariant bits;
equal byte length is not a timing, network, or privacy result.

**REJECTED as inferences from this evidence:** energy-market adequacy,
professional energy expertise, representative energy data, independent
implementation refinement, a cheaper optimality certificate, privacy, FHE,
vFHE, MPC, succinct proof, confidential delivery, data availability, custody,
physical delivery, liveness, or deployment. The direct verifier repeats the
same implementation's exhaustive oracle. Storage, inventory, hedges, fuel,
unit commitment beyond the frozen tiny model, and real network constraints are
absent.

### 3.2 TFHE candidate validator

**VERIFIED by committed source and recorded local measurement, not independently
rerun in this memo:** `breadstuffs` commit
`5ad58ba97c7666e5a9ee72aa056a06ea9cd465ac` independently re-specifies a
bounded arithmetic projection of the energy relation over `tfhe-rs 1.6.3`
integer ciphertexts. The evidence paths are:

- `fhegg-fhe/src/energy_dispatch.rs`;
- `fhegg-fhe/tests/energy_dispatch.rs`;
- `fhegg-fhe/src/bin/energy_dispatch_bench.rs`; and
- `docs/deos/CONFIDENTIAL-ENERGY-DISPATCH-TFHE-V0.md`.

The evaluator receives public demand, reserve, and line-capacity values and 63
`FheUint32` ciphertexts, including one retained encrypted zero. The evaluator
API has no `ClientKey` and decrypts no intermediate value. It computes encrypted
physical-feasibility predicates, exact sequential-segment costs, provider
credits, debit/objective equality, and their candidate-validity conjunction.

The canonical cost-56 plan and the feasible, conserving cost-60 plan both pass
encrypted feasibility and conservation. A forged cost-59 settlement attached
to the feasible cost-60 plan keeps physical feasibility true and makes
settlement conservation false. A separate Clear test oracle enumerates
`5^9 = 1,953,125` schedules, finds four feasible schedules for its independently
reconstructed instance, and selects cost 56. The encrypted evaluator does not
perform that search and does not compare the candidate against all schedules.

The recorded optimized single-run evaluation on an Apple M2 Max took about
84.6 seconds for the canonical case. The adversarial three-evaluation gate took
about 279.5 seconds. These are one-host, one-run measurements, not latency
bounds, production capacity, or distributional performance evidence.

**REJECTED:** calling this an encrypted solver, encrypted global optimum,
optimality certificate, proof of correct evaluation, vFHE, Dark execution, or
private settlement. The benchmark is one process with ephemeral client/server
keys and decrypts detailed diagnostics locally after evaluation. It provides
no independent or threshold custody, selective output release, malicious
evaluator security, authenticated transcript, instance/admission binding,
commitment join, availability, traffic or access-pattern hiding, network,
custody, consequence, settlement execution, or deployment.

This commit makes a blanket present-tense statement that the research has “no
FHE evaluation” stale. The claim-safe replacement is narrower: there is now
one real-TFHE candidate-predicate experiment, but still no FHE optimizer,
optimality enforcement, composed backend, vFHE integrity, release protocol, or
private settlement.

### 3.3 Authenticated source lifecycle

**VERIFIED local SBF subset:** Dragon's Clutch commit
`44bed19e9db83f0691e9c27e8d61bfbab74716ec` and its ancestors establish two
facts which must be stated together:

1. the live local `Resolve` route derives and authenticates the exact canonical
   Clutch-owned sealed SourceSpec/archive receipt and refuses same-domain
   projection, account, deployment, and source substitutions; and
2. the permissionless, prefund-safe SourceSpec/Feed/single-page archive
   construction, parser-admitted append, one-bucket maturity witness, and seal
   lifecycle executes only in a separately built
   `non-production-mock-source` ELF.

The default ELF's compile-time provider/parser registry is empty. It refuses
source construction or mutation with `SourceReleaseUnavailable` before the
first CPI or state write. The mock campaign measures SourceSpec+Feed
initialization at 448,896 CU, archive initialization at 553,723 CU, two appends
at 656,962 and 656,346 CU, and sealing at 866,718 CU. The evidence paths are:

- `docs/implementation/AUTHENTICATED_SOURCE_CONSTRUCTION_V1.md`;
- `docs/implementation/SOURCE_ARCHIVE_RUNTIME.md`; and
- `programs/clutch-sbf/svm-tests/tests/source_ingest.rs`.

**REJECTED:** production price history, production energy data, Pyth or another
provider integration, provider availability, a deployable source pipeline,
general maturity, missing-record repair, multipage history, cross-Realm reuse,
cleanup, or successor-page liveness. Permissionless source submission removes
a Clutch instruction operator from that step; it does not remove the source
provider or transaction-inclusion dependency.

### 3.4 Native occupation and the measured liveness STOP

**VERIFIED local SBF subset:** Dragon's Clutch commit
`87d2dbd60fa13d50e4f8b9e1c3697cd680697ce3` implements an optimized
monolithic native degree-one through degree-three quantized-basis occupation
route over canonical sealed archives. It writes canonical Resolution v4 and
has local internal and bearer exact-lot redemption evidence.

**VERIFIED measured STOP under the declared policy:** the chosen 25-percent
headroom rule admits at most 1,120,000 measured CU under a 1,400,000-CU
transaction ceiling. No initial occupation Resolve among exact spans one,
two, or three and degrees one, two, or three clears that gate. The best
measured initial row is 1,236,364 CU and the largest listed row is 1,262,471
CU. Span-three exact retries measure 1,086,756 through 1,108,857 CU, but a
retry cannot make the first resolution reachable. Spans four through thirty-two
are unmeasured and must not be inferred to pass or fail.

This is a policy/admission and decomposition result, not proof that Solana can
never execute the computation. The transactions completed below the raw
1,400,000-CU ceiling; the STOP is failure to retain the project's selected
operating headroom.

### 3.5 ResolutionWork

**VERIFIED model and layout only:** Dragon's Clutch commits
`482b395f47fa7af5b26b34719789a407899fcfc1` and
`a6da4015b0228bae0422d96caa3d26e91da0ec3d` add an isolated `no_std`,
fixed-width, float-free resumable occupation model and a 1,296-byte layout.
The model freezes the exact sealed archive and basis identities, advances a
single contiguous cursor, accepts no caller records or payout vector, stages
checked accumulator updates, and models prepaid Begin/Fold/Finalize/Abort
funding conservation. The committed model test source contains sixteen tests;
the layout test source contains nine tests.

**VERIFIED unreachable draft:** commit
`ad6d6f52fa5d81328d615f547861d7f069892054` adds an isolated SBF handler source
and two promotion-gate tests whose purpose is to assert that the handler is not
exported or routed. It provides no live instruction ABI, bank success, CU,
rent, account-count, ELF, or economic-liveness evidence.

**PROPOSED / HOLD:** do not publish any stronger ResolutionWork sentence until
a clean integration commit, exact ELF digest, real-bank Begin/Fold/Finalize/
Abort campaign, atomic rollback evidence, monolithic-v4 equivalence, and exact
CU/rent/account measurements land. Uncommitted worktree files are not evidence.

### 3.6 Direct selection and settlement

Three different authority stages must remain separate:

- **VERIFIED host model:** commit
  `471462f804d9018190b7d67a56685f9371b3a153` models a frozen schedule,
  full-width policy/domain identity, order-independent streaming top three,
  and once-only bounded selection after close. The associated design records
  fifteen host tests. It is not SBF execution and not optimal clearing.
- **VERIFIED routed source, runtime evidence absent:** commits
  `e360a359671bbd66dba037669bd9c705f7fec1ae` and
  `2fdd7cdcc5ea7a0dcfbef9b2c8a0e0c23ed03d65` add DirectEpoch V3 codecs and a
  routed program handler for the narrow direct authority chain. There is no
  committed real-SBF campaign for that chain at the evidence cut used by this
  memo. Code presence is not a runtime success claim.
- **VERIFIED older, different SBF subsets:** commit
  `1835b7935c126959d1cfc486dcc3895956d46a3d` measures a 1,249,403-CU narrow
  `SUBMITTED` candidate-construction path which does not select; commit
  `f529460f6ff414802d7f3a837024f19f4144d848` measures an approximately
  862,000-CU restricted settlement consumer of already selected/prefrozen
  authority. Neither measurement belongs to the new DirectEpoch V3 selection
  path.

The 1,249,403-CU sample also fails the project's separate 25-percent-headroom
policy: it would request 1,561,754 CU before the 1,400,000-CU ceiling. That is a
measured decomposition warning for the predecessor submission shape, not a CU
bound or measurement for DirectEpoch V3 selection.

**PROPOSED / HOLD:** current public wording must still say that executable
selection is unpromoted. Do not combine the older submission and settlement
measurements into an implied end-to-end authority chain. A future successful
claim must be limited to “best valid submitted candidate admitted before the
immutable close boundary” for the exact one-page/two-order profile. It must not
say “optimal clearing,” “general batch venue,” or “censorship-resistant
selection.”

## 4. Copy-ready Draft 10 wording

These blocks are proposed replacements, not edits to the frozen sources.

### 4.1 Cross-filing source paragraph

> **VERIFIED (restricted local SBF evidence; production source STOP).** Local
> resolution now authenticates the exact canonical program-owned sealed source
> receipt and refuses same-domain evidence or account substitution. A separate
> non-production mock-provider ELF also executes permissionless, prefund-safe
> construction of one SourceSpec, Feed, complete single-page archive,
> parser-admitted record sequence, and one-bucket maturity receipt. The default
> ELF registers no provider/parser release and fails closed before source state
> can be created or changed. No production provider authenticator, parser,
> energy-data source, provider-availability guarantee, multipage archive, or
> deployable source pipeline exists.

This should replace every Draft 9 assertion that there is “no live
archive-to-resolution join” or that the “current SBF route accepts
non-source-authenticated caller evidence.”

### 4.2 Technical-claims appendix row for source evidence

> **Claim:** The tested local resolution path is controlled by one canonical,
> sealed, program-owned complete single-page source window.
>
> **Basis:** Local real-SBF receipt-binding and substitution-refusal tests;
> separate real-SBF construction only in a deliberately non-production
> mock-provider ELF. The default ELF admits no source release. No production
> provider, deployable source pipeline, or end-to-end lifecycle claim.

Avoid the broader row label “authenticated complete source history controls
resolution” without the single-page, mock/default, and production qualifiers.

### 4.3 Clear energy paragraph

> **VERIFIED (bounded Clear semantics only).** An exact synthetic relation fixes
> three padded providers, three periods, two buses, and output atoms zero
> through four. Its 156-byte plaintext witness and 176-byte public-result codec
> are deterministic. In the representative fixture the Clear oracle examines
> 8,025 trajectory pairs, finds 468 feasible schedules, and selects canonical
> objective 56. A fully recommitted cost-60 plan is feasible and conserves
> settlement but is rejected only because the verifier repeats the frozen
> global-optimum and tie rule. Twenty-five debug/release tests and the recorded
> deterministic corpus pass. This is synthetic same-implementation evidence,
> not energy-market validation, an independent verifier, a proof, privacy, or
> deployment.

Use this in the energy comment and, if space permits, as a data-quality example
in the data-reporting or IAC statement. It has little direct relevance to the
definitions comment.

### 4.4 TFHE paragraph

> **VERIFIED (bounded encrypted candidate-validation experiment, not an
> encrypted solver).** An independently re-specified `tfhe-rs` evaluator whose
> evaluation API holds no client key evaluates a caller-supplied encrypted
> dispatch for physical feasibility and exact settlement conservation. Both
> canonical cost 56 and feasible, conserving cost 60 pass those predicates; a
> forged cost-59 settlement for the cost-60 plan fails conservation. Only a
> separate Clear enumeration establishes that 56 is the canonical optimum.
> The encrypted evaluator performs no global search or optimality check. The
> one-process experiment provides no vFHE proof, evaluator-correctness proof,
> threshold custody, selective result release, network privacy, private SBF
> settlement, or deployment, and it is not described as Dark.

Do not publish the single-run timing in the main argument unless latency is
material. If used, label the exact host, software versions, one-run character,
and absence of a performance bound in the same sentence or table.

### 4.5 Occupation liveness paragraph

> **VERIFIED (measured local SBF subset and explicit admission STOP).** The
> monolithic native degree-one through degree-three occupation path executes in
> local banks, but none of the measured initial spans one through three clears
> the project's 1,120,000-CU threshold for retaining 25-percent headroom under
> the 1,400,000-CU ceiling. The best measured initial row is 1,236,364 CU.
> Span-three retries happen to clear the threshold, but a retry cannot make the
> initial transition reachable. Larger spans are unmeasured. This is a
> decomposition requirement, not a claim of production liveness or a hardware
> impossibility.

This is more useful to the IAC's operational-readiness discussion than to the
product-definitions argument.

### 4.6 ResolutionWork paragraph before pending promotion

> **PROPOSED / STOP.** An isolated executable model and fixed layout specify a
> prepaid, cursor-bound Begin/Fold/Finalize/Abort decomposition which reads the
> exact sealed archive and accepts no caller-supplied records, masses, weights,
> or payout vector. The committed SBF handler draft is intentionally
> unreachable and has no bank, CU, rent, account-count, or ELF evidence. It is
> a liveness architecture, not a live protocol property.

Replace this block only after the gates in section 7.2 close.

### 4.7 Direct-selection paragraph before pending promotion

> **VERIFIED host model / PROPOSED live transition.** A bounded host model fixes
> the full-width policy and domain, an immutable submission close, and an
> order-independent streaming top-three rule. Routed SBF source now exists for
> the exact one-page/two-order profile, but no committed real-SBF campaign at
> this evidence cut proves the complete DirectEpoch V3 submit-select-receipt-
> settle authority chain. Older SBF measurements cover a `SUBMITTED` proposal
> and a separate consumer of already authorized settlement, not selection.
> General clearing, partials, portfolios, fees, multipage books, lapse,
> terminal sweep, and censorship-resistant inclusion remain STOPs.

## 5. Draft 9 locations requiring future Draft 10 changes

Line numbers are from frozen commit `7df9c01` and are navigation aids; the
future editor must re-resolve them after branching.

### 5.1 Definitions comment

- `docs/regulatory/typst/definitions/body.typ:88-95`: replace the stale source
  STOP with section 4.1's two-artifact source paragraph.
- `body.typ:120-131`: update the restricted SBF inventory only if the pending
  campaigns commit. Until then retain the existing construction, 22-step walk,
  native point, and restricted settlement claims. Do not add “end-to-end.”
- `body.typ:420-422`: replace the source appendix row; retain separate Lean,
  Verus, Rust, and SBF evidence classes.
- `docs/regulatory/typst/definitions/sources.typ:59-80`: replace “not joined to
  the live SBF resolution route” with the exact receipt-binding plus
  mock/default distinction.

The energy and TFHE paragraphs should normally stay out of this filing.

### 5.2 Data-reporting comment

- `docs/regulatory/typst/data-reporting/body.typ:70-79` and `133-146`: replace
  the stale source claims with section 4.1.
- `body.typ:107-131`: preserve the distinction between deterministic
  recomputation and the unpromoted onchain selection chain. If the TFHE example
  is added, place it after the Clear recomputation discussion and use section
  4.4 verbatim.
- `body.typ:148-157`: do not use local redemption evidence to imply private
  ownership linkage or a reporting adapter.
- `body.typ:204-213`: the TFHE counterexample can sharpen the existing proof
  proposition: encrypted execution does not establish global optimality or
  correct evaluation.
- `body.typ:408-415`: add separate rows for Clear energy semantics and TFHE
  candidate validation only if cited in the body; replace the source row.
- `docs/regulatory/typst/data-reporting/sources.typ:56-80`: update the source
  boundary and replace “strongest composed privacy paths remain Shielded” with
  a more exact sentence: the TFHE evaluator supplies input confidentiality
  against the evaluation-key holder for one predicate experiment, but no
  composed Clear/Shielded/Dark venue exists.

### 5.3 IAC statement

- `docs/regulatory/typst/iac/body.typ:97-113`: keep native B-spline semantics,
  Lean, Verus, and implementation evidence separate. Do not call the whole
  protocol formally verified.
- `body.typ:153-174`: replace the stale source paragraph with section 4.1.
- `body.typ:176-187`: preserve the narrow settlement limits; add occupation or
  ResolutionWork only in a separate operational-readiness paragraph.
- `body.typ:189-203`: if energy evidence is added, present the cost-60 example
  as a fourth machine-checkable negative: feasibility plus conservation does
  not establish canonical optimality. Do not call the TFHE result a theorem.
- `body.typ:387-393`: replace the source row and, only if cited in the body, add
  separate Clear-energy and TFHE rows.
- `docs/regulatory/typst/iac/sources.typ:98-123`: update the source lifecycle;
  keep all deployment, production readiness, and legal-classification
  disclaimers.

### 5.4 Separately planned energy comment

The Markdown architecture memo
`ENERGY_DRAFT3_ARCHITECTURE_AND_CLAIM_AUDIT.md` contains stale “no FHE backend”
language written before `5ad58ba`. A future Energy Draft 3 source should:

1. promote section 4.3's Clear semantics;
2. promote section 4.4's encrypted candidate validator;
3. preserve the proposed Dark FHE/vFHE target as unimplemented because the
   validator neither searches nor proves optimality and lacks a release,
   custody, network, and settlement design;
4. preserve regulator-observable threshold disclosure as a separately
   proposed Shielded mode; and
5. state that no professional energy expertise, empirical participant demand,
   production data, physical-market conclusion, perpetual-product conclusion,
   or listing/legal conclusion follows.

The correct evidence ladder is now:

```text
Clear exact semantics and exhaustive optimum
       |
       +-- encrypted candidate feasibility + conservation only
       |        (global optimum deliberately absent)
       |
       +-- proposed optimality certificate / vFHE integrity
       |
       +-- proposed threshold release or Dark leakage contract
       |
       +-- proposed source, availability, consequence, and settlement join
```

## 6. Operatorlessness and formal-proof wording

### 6.1 Operatorlessness

The strongest safe shared sentence is:

> Some local transitions can be invoked by an ordinary wallet against frozen
> program state, but no current artifact establishes an operatorless lifecycle.
> Provider availability, transaction inclusion, deployment and upgrade
> control, source-release registration, unresolved work funding, private-key
> release, and terminal recovery remain named dependencies.

Do not convert any of the following into “operatorless”:

- an empty default source registry;
- a non-production mock provider;
- permissionless creation of one account family;
- prepaid abstract liveness accounting;
- a public caller reward;
- a local ProgramTest transaction; or
- an FHE evaluator which lacks the client key inside one process.

### 6.2 Formal proof

The strongest safe shared sentence is:

> Separate Lean modules prove named model properties, one pinned Verus run
> checks a narrow arithmetic seam, deterministic Rust tests exercise bounded
> relations, and local SBF campaigns execute selected transitions. No
> refinement proof connects all of those artifacts, and no proof currently
> establishes the TFHE evaluator's correct execution, global optimality, the
> ResolutionWork runtime, the direct-selection chain, production source truth,
> or whole-protocol behavior.

Use “machine-checked model theorem,” “Verus-checked seam,” “deterministic
test,” and “local real-SBF execution” as distinct nouns.

## 7. Promotion gates

### 7.1 DirectEpoch V3

Final live-selection wording remains held until one clean committed campaign
records:

- exact commit and ELF SHA-256, with fixture/build equality;
- blank and prefunded BatchPolicy, Epoch, Window, Candidate, Receipt, and Pot
  creation paths;
- immutable open/close schedule and authenticated Clock enforcement;
- full-width policy, relation-domain, and candidate identity;
- order-independent retained top three and full-digest tie behavior;
- late submission and early selection refusal;
- retained-account omission, reorder, substitution, and replay refusal;
- successful atomic selection, entitlement freeze, receipt/pot creation, and
  Epoch phase transition;
- injected late-creation rollback with all watched accounts and lamports
  unchanged;
- exact settlement conservation and double-settlement refusal; and
- CU for every success family and the maximum admitted account/message shape.

Even after those gates, the claim ceiling remains the exact one-page/two-order,
single-outcome, equal-full-fill, zero-fee, no-virtual profile.

### 7.2 ResolutionWork

Final executable wording remains held until one clean committed campaign
records:

- exported layout and instruction ABI, dispatch routing, PDA registry, and
  refusal allocation;
- exact live rent and declared prepaid-reserve conservation;
- real-bank Begin, Fold sizes one through four, Finalize, and every authorized
  Abort form;
- exact cursor replay/skip/duplicate and same-domain archive substitution
  refusal;
- terminal close/replay and late-failure byte-and-lamport rollback;
- canonical Resolution-v4, Market, kernel, supply, and mint-plane equivalence
  with the monolithic path for every claimed degree/span/finalizer;
- exact account count, message size, CU distribution, feature set, runtime,
  toolchain, build commit, and ELF SHA-256; and
- evidence that at least one initial path clears the chosen headroom policy,
  not merely the raw transaction ceiling.

### 7.3 Source production

Production source wording remains forbidden until the default ELF contains a
reviewed and pinned provider deployment authenticator and price parser, hostile
production-format fixtures pass, the provider's historic/finality semantics
actually establish the frozen bucket relation, and the final artifact has a
clean reproducible runtime campaign. Mock success must remain visibly
non-production.

### 7.4 TFHE or Dark promotion

The encrypted candidate validator cannot become an encrypted solver or Dark
claim until at least:

- optimality or canonical selection is enforced by encrypted search or a
  separately sufficient verified certificate bound to the same admitted
  inputs;
- correct evaluation is independently verifiable;
- instance, epoch, provider ownership, admission commitment, finality,
  availability, replay, and consequence are bound;
- key generation, custody, rotation, corruption threshold, release,
  non-release, and recovery are specified and implemented;
- public, provider-local, regulator-authorized, and forbidden outputs have a
  frozen leakage contract;
- transport, timing, access pattern, denial, and abort behavior are addressed;
  and
- the authorized result is joined to exact settlement without undoing the
  promised privacy.

### 7.5 Artifact and filing gates

Before any Draft 10 Typst or PDF exists:

1. restore the required artifact-operation marker and run it successfully;
2. branch from frozen Draft 9 rather than overwriting it;
3. inspect every cited final commit and rerun proportionate deterministic
   gates from a clean tree;
4. re-resolve exact paths and line references;
5. update the technical-claims appendices and source notes together with body
   prose;
6. recheck current official deadlines, dockets, and submission instructions;
7. render and visually inspect every PDF page under the required PDF workflow;
8. exclude secrets, confidential commercial information, unnecessary personal
   data, and unpublished vulnerability details; and
9. obtain legal review before treating technical drafting as a filing or legal
   conclusion.

## 8. Final Draft 10 ceiling

If the pending campaigns do not land, Draft 10 may honestly claim:

- exact categorical and native degree-one through degree-three shaped-payout
  semantics, with separate scoped Lean/Verus/Rust/SBF evidence;
- local canonical source-receipt binding and substitution refusal, plus a
  separate non-production mock source lifecycle and an inert default registry;
- a bounded Clear energy relation with canonical optimum and an exact
  cost-60 counterexample;
- encrypted feasibility and conservation checking of a caller-supplied energy
  plan, with global optimality explicitly absent;
- a measured monolithic occupation admission failure under the selected
  headroom policy; and
- model-only ResolutionWork plus routed-but-unpromoted direct-selection source.

It may not claim a production data source, production readiness, deployment,
whole-protocol formal verification, a complete venue, an encrypted solver,
vFHE, Dark execution, private settlement, operatorless end-to-end liveness,
general direct clearing, or any legal or listing conclusion.
