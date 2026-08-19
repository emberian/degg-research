# Engineering claim audit for the Draft 8 regulatory packet

Date: 2026-08-19

Scope: local technical-claim audit only; not legal advice, filing authority, or
an audit of current law. No filing, regulator contact, publication, deployment,
public RPC, key use, or funds movement occurred.

## 1. Verdict

Draft 8 is still conservative enough that its core negative statements remain
true: nothing described is deployed to a public cluster, no customer funds or
live orders exist, the end-to-end system is not formally verified, the source
and archive plane is not yet authenticated in the live resolution path, and no
operatorless venue exists.

The packet now has two different kinds of drift:

1. **Material underclaim.** It describes Dragon's Clutch chiefly as an offline
   pure-Rust prototype. The landed record now includes native degree-zero
   through degree-three B-spline semantics, substantial Lean proofs of the
   separate mathematical model, a narrow checked Rust arithmetic seam, real
   local-SBF execution, permissionless market/owner construction, typed
   artifact transport, funded order reservation, exact bearer redemption,
   pooled-cash withdrawal, market-global resolution replay, and a clean signed
   22-transaction categorical custody walk.
2. **One material overclaim.** The worked-example prose says a frozen program
   reads *authenticated price history* and computes settlement. The committed
   runtime does not yet authenticate source lineage, and `Resolve` can still
   consume a caller-supplied buffer not joined to the feed history. That
   sentence is supportable only as a **PROPOSED qualifying design**, not as a
   description of current implementation.

Draft 9 should therefore update the engineering account now, in parallel with
continued work. It should not wait for a finished venue. It must, however,
state the exact narrow boundary of newly landed native-SBF, source/archive,
market-construction, and coupled-settlement slices, and keep their unjoined
edges behind explicit reproduction gates.

## 2. Audited snapshots and status words

### Draft snapshot

The current Typst Draft 8 source is degg-research commit `18be77e`.

| Review artifact | Pages | SHA-256 |
|---|---:|---|
| `joint-definitions-comment-draft-8.pdf` | 8 | `4d0f7e1baa393b309390bedacad40084ae54a3364499217f27b694c742892933` |
| `joint-data-reporting-comment-draft-8.pdf` | 9 | `c925c401ae9785ebf1ed364384fe5bb507b830c7786181a44e700e129a9d1a1e` |
| `cftc-iac-written-statement-draft-8.pdf` | 8 | `024f050c16cfad0249951693560122ff572c75a06c8c7adbaccc08330367a621` |
| `cftc-iac-cover-statement-draft-8.pdf` | 1 | `f42d43234718e6cd3e1664fd776b2bc10c426fb6582ed7587085faa578d3b6ce` |

The legal recitals, deadlines, docket identifiers, and third-party source
characterizations are outside this engineering-only audit. Their existing
live-source and counsel gates remain mandatory.

### Engineering snapshot

The initial committed Dragon's Clutch boundary inspected here ended at
`61e1e81`, with special attention to the newly landed `8c929a9`, `7c0f169`,
and `e7d975b`. During the audit three formerly in-flight narrow slices landed:
provider-neutral source/archive codecs and verification (`2b2ee3c`), restricted
coupled settlement (`f529460`), prefund-safe degree-selected market construction
(`ceac012`), and native smooth resolution/internal redemption in SBF
(`6a826b6`). They are included below only at their narrow committed claim
boundaries. The shared worktree still contained an uncommitted blank-bank test
and truth-document work; those bytes are not promoted here.

The labels below mean:

- **SUPPORTED**: the Draft 8 wording stays within committed evidence, although
  a more exact basis may still be desirable.
- **STALE**: it was supportable when written but no longer accurately describes
  the strongest local evidence.
- **OVERCLAIM**: it can reasonably be read as asserting an integration or
  property the committed artifacts do not establish.
- **UNDERCLAIM**: it remains true but omits a material result that Draft 9
  should disclose if the project is being described.
- **WAIT**: do not use the stronger wording until the named in-flight change is
  committed and its exact evidence is rerun.

## 3. Committed engineering evidence index

### Native B-spline semantics

| Evidence ID | Status and exact basis |
|---|---|
| BS-1 | **VERIFIED / HOST-TESTED.** Commit `48b669b`, `crates/clutch-bspline/src/lib.rs`: safe `no_std`, allocation-free, float-free evaluator for degree-zero categorical cells, degree-one hats, and degree-two/three open-clamped B-splines, with canonical largest-remainder quantization. Re-run 2026-08-19: 12 Rust tests pass. |
| BS-2 | **VERIFIED / DIFFERENTIAL-TESTED.** `crates/clutch-bspline/oracle/check.py`: independent Python `Fraction`/Cox-de-Boor oracle. Re-run 2026-08-19: 31,814 exact cases, fixed seed `880230`, six mutants killed. This is finite differential evidence, not proof. |
| BS-3 | **VERIFIED / REFERENCE-HOST-TESTED.** Commit `ead7106`, `programs/solana-reference/src/lib.rs`: native degree-one through degree-three payout-vector derivation; degree-two/three and the persisted first cut require point evidence; non-point smooth evidence refuses. Re-run on the committed reference source: 49 tests pass. |
| BS-4 | **VERIFIED / MODEL-PROVED.** Commits `f70cf09` and `8c929a9`, `lean/DragonsClutch/BSpline.lean`: Lean checks the separate exact low-degree basis model, uniform open-clamped knot expansion/linkage theorem, deterministic largest-remainder selection and uniqueness, quantized admissibility, local support, resolution bound, and complete-set exactness. `lake build DragonsClutch.BSpline` passed on 2026-08-19; the file contains no `sorry`, `admit`, or project `axiom` declaration. The file expressly does **not** prove that the Rust parser, pane-selection control flow, or reduced-fraction implementation refines these definitions. |
| BS-5 | **VERIFIED / HOST-TESTED, NOT INTEGRATED.** Commit `830a8a1`, `crates/clutch-bspline-accumulator/src/lib.rs` and `docs/implementation/BSPLINE_OCCUPATION_ACCUMULATOR.md`: exact fixed-width occupation summaries over the native quantized basis, explicit gaps, associative adjacent combination, exact-only and separately named largest-remainder finalizers. Re-run: 19 tests pass. It authenticates no source, window identity, clock, or archive. |
| BS-6 | **VERIFIED / RESEARCH-PROTOTYPE.** Commit `1d80a1f`, `research/bspline-shape-compiler/src/lib.rs`: exact-in-span and certified-approximation compilation for named payout shapes over the native basis; 14 source tests. It is a research compiler, not a production ABI or pricing algorithm. |
| BS-7 | **VERIFIED / VERSIONED CODEC.** Commit `ef8a3d4`, `programs/solana-layout/src/native_resolution.rs` and `docs/implementation/NATIVE_RESOLUTION_PERSISTENCE.md`: a tested 319-byte version-three record owns one derived point/vector. Its initially isolated status is superseded only for the exact SBF subset in BS-8; the codec alone remains no runtime claim. |
| BS-8 | **VERIFIED / RESTRICTED SBF-EXECUTED NATIVE SUBSET.** Commit `6a826b6`, `programs/clutch-sbf/program/src/instructions/observe_resolve.rs`, `programs/clutch-sbf/svm-tests/tests/native_resolution.rs`, and `docs/implementation/NATIVE_RESOLUTION_SBF.md`: degree-one through degree-three *point* resolution and exact internal redemption execute in real local SBF with v3 Resolution as sole persisted vector owner. Four focused cases pass; host program 131/131, codec 6/6, legacy v2 1/1; built/fixture ELF `07b759e09867a13a89b6f0c27fdfb3f65b03fb4a2e186b94ea5ac87a21ac80a3`. Resolve/retry/redeem CU by degree: d1 802,909/648,089/707,029; d2 845,517/690,697/709,029; d3 880,340/725,520/708,704. The test supplies a pre-created v3 account; native bearer exit and source/archive-to-Resolve authentication remain STOP. |

### Solana execution, custody, and lifecycle

| Evidence ID | Status and exact basis |
|---|---|
| SOL-1 | **VERIFIED / SBF-EXECUTED LOCALLY.** Commits `c05fe84` and `7edaa11`, `docs/implementation/COMMITTED_SBF_WALK.md`: 22 signed, confirmed transactions against one loopback-validator market, including two byte-preserving expected refusals, bearer redemption, two cash withdrawals, and terminal Hoard balance zero. The exact ELF digest recorded there is `70c33c1cd44b475745b0562a79d9107f1d2101cbf698ebd6c233ca167ebab2e6`. The walk is genesis-assisted by 11 program-owned prerequisites and contains no order-book settlement. |
| SOL-2 | **VERIFIED / SBF-EXECUTED LOCALLY.** Commit `d67f5af`, `programs/clutch-sbf/program/src/instructions/construction.rs` and `docs/implementation/PERMISSIONLESS_ACCOUNT_LIFECYCLE.md`: `CreateMarket` created seven absent state PDAs plus outcome mints and Hoard token account in a real local bank (888,587 CU); a second wallet's first backed `Endow` created its Position and Replay (248,131 CU). This is blank-market core construction, not a permissionless end-to-end lifecycle. |
| SOL-3 | **VERIFIED / SBF-EXECUTED LOCALLY.** Commits `ce8c55c` and `e7d975b`, `programs/clutch-sbf/program/src/instructions/artifact.rs`, `programs/clutch-sbf/svm-tests/tests/artifact_transport.rs`, and `docs/implementation/ARTIFACT_TRANSPORT.md`: typed, resumable Policy/Grid/Terms staging and sealing, including restart, expiry/reap, exact refund, hostile bytes, native SHA, idempotent final, and prefunded predictable-PDA cases. The 2026-08-19 focused run recorded a 28,751-CU largest first write and 18,045-CU new Terms seal. Source/archive artifacts are not included. |
| SOL-4 | **VERIFIED / SBF-EXECUTED LOCALLY.** Commits `33a1d41`, `1758ae2`, and `4ea7c16`, `programs/clutch-sbf/program/src/instructions/cash_exit.rs` and `docs/implementation/CASH_WITHDRAWAL_V1.md`: exact unreserved pooled-cash withdrawal with Token-2022 CPI, replay and rollback; 16 collateral-plane and six bearer-plane focused cases; 229,773 CU for exact maximum-free withdrawal in the recorded focused build. |
| SOL-5 | **VERIFIED / SBF-EXECUTED LOCALLY.** Commits `e67c315` and `0e84918`, `programs/clutch-sbf/program/src/instructions/external_exit.rs`: actual Token-2022 mint/account supply is bearer truth and categorical external redemption is executed in the clean 22-step walk. This does not yet establish native fractional bearer redemption. |
| SOL-6 | **VERIFIED / SBF-EXECUTED LOCALLY, ADMISSION ONLY.** Commit `86b72f8`, `programs/clutch-sbf/program/src/instructions/orders_batch.rs`, `orders_batch/reservation.rs`, and the focused SVM reservation test: canonical funded cash/Egg reservation and exact cancellation/release execute. The red-team record reports two real-bank scenarios, with placement around 594k CU and cancellation around 470k CU. `SettlePage` remains outside this committed claim. |
| SOL-7 | **VERIFIED / SBF-EXECUTED LOCALLY, SOURCE-UNAUTHENTICATED.** Commit `15c29a1`, `programs/clutch-sbf/program/src/instructions/observe_resolve.rs` and `docs/implementation/RESOLUTION_REPLAY_DOMAIN.md`: resolution replay is market-global, exact retry is idempotent, conflict refuses, and owner replay is not consumed. The same document expressly says this is not source authentication. |
| SOL-8 | **VERIFIED / RESTRICTED SBF SETTLEMENT SLICE.** Commit `f529460`, `programs/clutch-sbf/program/src/instructions/orders_batch/settlement.rs`, `programs/clutch-sbf/svm-tests/tests/coupled_settlement.rs`, and `docs/implementation/COUPLED_SETTLEMENT_V1.md`: one same-page, full-fill, direct single-Egg, zero-fee slice consumes two exact reservations and one prefrozen receipt atomically. The recorded real-SBF campaign passed two cases; ELF `07b759e09867a13a89b6f0c27fdfb3f65b03fb4a2e186b94ea5ac87a21ac80a3`; successful transaction 862,084 CU. Candidate selection, entitlement construction, complete-set closure, partial/portfolio/virtual/fee/cross-page semantics, lapse, and terminal sweep remain absent. |
| SOL-9 | **VERIFIED / HOST-TESTED MARKET-CONSTRUCTION CUT.** Commit `ceac012`, `construction.rs`, `genesis.rs`, and `market_init.rs`: `CreateMarket` consumes canonical sealed Policy/Terms artifacts, selects the exact 165-byte categorical or 319-byte smooth Resolution shape from immutable degree, and uses rent-shortfall plus PDA-signed allocate/assign so SOL-prefunded program-state and token PDAs do not cause an initialization refusal. The producing lane reports 131/131 host library tests. A new end-to-end real-bank sealed-artifact/native construction proof had not passed at audit time; the older categorical real-bank evidence remains SOL-2. |

### Source, liveness, fractional redemption, and proof scope

| Evidence ID | Status and exact basis |
|---|---|
| SRC-1 | **VERIFIED / HOST-TESTED RELATION ONLY.** Commit `f0a247a`, `programs/clutch-sbf/program/src/source.rs` and `docs/implementation/SOURCE_ADMISSION_V1.md`: a typed source-admission relation names identity, deployment generation, parser release, sequence, clock, freshness, confidence, grid, and canonical selection checks. The live `FeedAdvance` and `Resolve` instructions do not call the qualifying join. |
| SRC-2 | **VERIFIED / RESEARCH PROFILE ONLY.** Commit `a8aa0d6`, `research/source-profile-v1`: a conditional point-at-time parser/profile implements a unique time-boundary crossing relation and outward interval normalization. Re-run: six tests pass. The profile remains non-production because provider deployment/configuration provenance and the live adapter join are open. |
| SRC-3 | **VERIFIED / HOST-EXECUTED PROVIDER-NEUTRAL ARCHIVE SEAM.** Commit `2b2ee3c`, `programs/clutch-sbf/program/src/source_archive.rs`, `programs/clutch-sbf/svm-tests/tests/source_archive.rs`, and `docs/implementation/SOURCE_ARCHIVE_RUNTIME.md`: exact 292-byte SourceSpec and 2,560-byte one-window archive codecs bind runtime key/owner, provider/deployment/parser release, predecessor lineage, exact window, commitment, append/seal, and sealed receipt. Four focused workspace tests pass. No production provider authenticator/parser exists, no routed instruction owns the ABI, and live `FeedAdvance`/`Resolve` do not join it; this is not bank/CU evidence or source-authenticated settlement. |
| LIVE-1 | **VERIFIED / HOST-TESTED, NOT RUNTIME-INTEGRATED.** Commit `7c0f169`, `crates/clutch-liveness/src/lib.rs` and `docs/implementation/LIVENESS_ADMISSION_KERNEL.md`: prepaid market work/storage/resolution and per-order clear/settle compartments, canonical shared-source/archive cost sharing, and owner-bound fee carry. Re-run: 13 tests pass. All maxima are unmeasured policy inputs; this does not guarantee inclusion or move lamports. |
| FRAC-1 | **VERIFIED / MODEL-ONLY.** Commit `61e1e81`, `research/fractional-redemption/src/lib.rs`: exact-lot and persistent-credit policies, claimant/domain-bound carry, transfer/merge, donation and terminal sub-atom accounting. Re-run: 13 tests pass. It is not an onchain redemption implementation. |
| FORM-1 | **VERIFIED / NARROW CHECKED RUST SUBSET.** Commit `3591141`, `verus/kernel/TRANSFER_REFINEMENT.json`: pinned Verus result `2 verified, 0 errors` for `prepare_internal_transfer` arithmetic and exact overflow refusal, with two expected-red mutations. Accounts, phase, codecs, CPI, SBF compilation, runtime, and every other transition are outside it. |
| GH-1 | **VERIFIED / MODEL-PROVED AT FROZEN SOURCE SNAPSHOTS.** Breadstuffs commit `35e4f078c7ac6762161c7770a981c1b035f4779f`, `metatheory/Dregg2/Exec/GuardedHole.lean`, owns `holeFill_binds_in_circuit` and `holeFill_rejects_guard_violation`. Minidregg's relevant files last changed at `a8730762f20e826ea78e80a82fd03e4e78ffef92`: `Theory/GuardedAdvice.lean` (`verify_accepted_binds_eager_shape`, `verify_rejected_no_mutation`, `commit_then_replay_refused`), `Theory/ReactiveController.lean`, and `Theory/ReactiveCellTransition.lean`. Those files were clean on inspection, but the wider Minidregg worktree was dirty at `9db15e7`; no current umbrella build was claimed or rerun here. |
| CR-1 | **VERIFIED / MODEL-PROVED.** Leanuweave commit `f1450667...`, `Uwueave/Holes.lean` and `Uwueave/Gluing.lean`: candidate evaluation commutes with union; determinacy requires a stated stability premise; balance-type and one-shot constraints do not generally glue without the named conditions/seams. No source authenticity or legal finality follows. |

## 4. Claim inventory: Definitions Draft 8

References below are to `docs/regulatory/typst/definitions/body.typ` unless
otherwise stated.

| ID | Lines / claim family | Verdict | Draft 9 action |
|---|---|---|---|
| DEF-1 | 65-71: five exhaustive price bands, frozen deterministic edge behavior | **SUPPORTED as stipulated design**, not an implementation claim | Mark the paragraph **PROPOSED worked example**. Do not let later prototype prose retroactively turn every term into a current feature. |
| DEF-2 | 75-80 and 207-227: deposit, complete-set issuance, recombination, constant aggregate payout | **SUPPORTED** by the kernel/model and the categorical local SBF walk; the general economic qualification is sound | Add the categorical/native distinction. Complete-set exactness is also proved in the separate spline model, but Rust/SBF refinement is not. |
| DEF-3 | 81-87: separately transferable claims, frozen batch, exact committed order or no state change | **SUPPORTED but evidence is conflated** | Attribute the formal eager-shape/fail-closed statement to the guarded-hole model and the frozen-book recomputation/reservation statement to Dragon's Clutch host/SBF evidence. Do not imply the full settled venue is live. |
| DEF-4 | 88-93: “reads authenticated price history from the named onchain source” | **OVERCLAIM** for current implementation | Replace with conditional wording: a qualifying design *would* read a canonical authenticated archive. State that current SBF resolution still uses non-qualifying caller-supplied evidence and cannot support an oracle-secure/trustless claim. See SRC-1/SRC-2. |
| DEF-5 | 94-98: resolution then payout from fully funded pool | **SUPPORTED as terms and core accounting; UNDERCLAIMS local SBF evidence** | State that categorical internal/bearer redemption and terminal withdrawal, plus native smooth point resolution/internal redemption, have executed locally. Native bearer exit, source authentication, and full venue settlement remain open. |
| DEF-6 | 100-107: no borrowing/liquidation and bounded contractual loss under the stipulated fully paid terms | **SUPPORTED as a stipulation**, not as a universal operational-risk claim | Keep the existing custody/implementation/collateral-value disclaimer. |
| DEF-7 | 109-118: “offline research prototype,” deterministic tests, “tested, not formally verified,” no deployed program | **STALE / UNDERCLAIM**, with the final no-deployment clause **SUPPORTED** | Replace “offline” with a layered account: pure Rust host models plus local SBF execution. Replace the broad proof sentence with “the end-to-end Rust/SBF system is not formally verified,” then name the separate Lean spline/model proofs and narrow Verus seam. Say “no public-cluster deployment,” not wording that could be read to deny the existence of a compiled SBF program. |
| DEF-8 | 277-291: formal verification is scoped evidence; “prototype described here is not formally verified” | Principle **SUPPORTED**; artifact description **STALE but safely conservative** | Keep the principle. Clarify proof granularity using BS-4 and FORM-1. |
| DEF-9 | 356-367: offline/formal research only; no deployment, funding, offer, operation | Negative status **SUPPORTED**; “offline” **STALE** | Use “local research implementation, including local-bank SBF execution; no public-cluster deployment or operation.” |
| DEF-10 | 378-397 appendix: guarded order binding | **SUPPORTED at frozen formal snapshots** | Pin the exact producer commit/path. Replace “obligation” language with the formal object's actual fields: actor, target, admitted value type, guard/effect shape, deadline, and replay domain. |
| DEF-11 | 396 appendix: core accounting implemented offline and tested | **SUPPORTED but materially UNDERCLAIMS** | Split into host-kernel evidence and local-SBF categorical lifecycle evidence. Add separate native-spline rows instead of folding splines into “core accounting.” |
| DEF-12 | `definitions/sources.typ` 57-69: local materials described without immutable pins | **STALE provenance** | Pin Dragon's Clutch to the filing freeze, retain the older frozen formal snapshots unless they are re-audited, and give exact paths. Do not call a dirty sibling HEAD “current” evidence. |

## 5. Claim inventory: Data-reporting Draft 8

References are to `docs/regulatory/typst/data-reporting/body.typ` unless stated.

| ID | Lines / claim family | Verdict | Draft 9 action |
|---|---|---|---|
| DAT-1 | 66-75: conditional-asset design and offline core prototype | Design **SUPPORTED**; prototype description **STALE / UNDERCLAIM** | Use the layered host/local-SBF wording from DEF-7. |
| DAT-2 | 79-84: market template “is created on the ledger”; all template facts public | **OVERCLAIM if read as current end-to-end construction** | Mark as the proposed record walk. Typed Policy/Grid/Terms transport and blank-market construction exist, but the full source/feed/epoch plane does not. |
| DAT-3 | 86-103: funded complete set, gross issuance, recombination, later unbalancing exposure | **SUPPORTED** in the pure kernel and portions of local SBF | Keep as a worked economic record. Avoid saying an offchain owner-linked regulatory lifecycle has been implemented. |
| DAT-4 | 105-122: transparent frozen book and full recomputation | **SUPPORTED / UNDERCLAIM** | Host batch relation re-run has 61 tests. Funded SBF order admission/cancel and one restricted preauthorized settlement slice now execute locally. Candidate selection and complete venue settlement do not. Say “host clearing verifier plus local funded reservation and restricted consumption seams,” not “onchain clearing is implemented.” |
| DAT-5 | 124-135: no authoritative early outcome; admitted evidence removes adjudicator discretion | Model/maturity part **SUPPORTED**; authenticated-authority implication **OVERCLAIM** | Separate three claims: early/malformed windows refuse; replay is market-global; source provenance is not yet authenticated. “Admitted” must not hide who authenticated the evidence. |
| DAT-6 | 137-145: redemption outflows and three-record reconstruction | Redemption behavior **SUPPORTED** for the categorical local walk; regulatory-record linkage remains **PROPOSED** | Add that the local program does not implement the confidential owner-to-party record described here. |
| DAT-7 | 167-196: provenance fields and proof metadata | **PROPOSED reporting design**, not current program state | Preserve as a proposal; some fields now have prototype owners, but no reporting adapter exists. |
| DAT-8 | 187-196: proofs establish only their statements | **SUPPORTED analytical principle** | Add concrete examples: BS-4 proves a separate spline model; FORM-1 checks one Rust arithmetic seam; neither proves SBF/runtime/source/custody compliance. |
| DAT-9 | 206-210: transparent batch does not conceal orders | **SUPPORTED** | Keep. Native spline work does not change the privacy status. |
| DAT-10 | 215-235 and appendix 388: Solana ingress/public-ledger facts | **Not re-audited here**; sourced platform claims, not Dragon's Clutch engineering claims | Preserve the official-source gate. Do not use local SBF execution as evidence for network-wide ingress statements. |
| DAT-11 | 299-316: recomputation validator and formally fixed correction authority/fields/version | Recompute claim **SUPPORTED**; guarded correction claim **SUPPORTED only as a model theorem** | Keep the latter explicitly model-only and pin GH-1. It is not a reporting adapter or signature/custody result. |
| DAT-12 | 318-329: accumulator refuses unsupported questions | **SUPPORTED / UNDERCLAIM** | The original accumulator has 24 unit plus two doc tests in the current committed source; the native occupation accumulator adds 19 tests but remains source-neutral. Name both and retain “no source authentication.” |
| DAT-13 | 353-360: no reporting system/deployment/current compliance claim | **SUPPORTED** | Change “offline research prototypes” to “local research prototypes, including local-bank SBF execution.” Keep every compliance disclaimer. |
| DAT-14 | 389-392 appendix: core accounting, batch verifier, accumulator, guarded update | **SUPPORTED but STALE evidence register** | Split by `HOST-TESTED`, `SBF-EXECUTED LOCALLY`, and `MODEL-PROVED`; pin commits and exact paths. |
| DAT-15 | `data-reporting/sources.typ` 54-70: source summary | **STALE / UNDERCLAIM** | Add the native-spline and local-SBF layers; keep the explicit absence of a source-authenticated runtime and confidential venue. |

## 6. Claim inventory: IAC statement Draft 8

References are to `docs/regulatory/typst/iac/body.typ` unless stated.

| ID | Lines / claim family | Verdict | Draft 9 action |
|---|---|---|---|
| IAC-1 | 84-100: five-band worked market and offline pure-Rust prototype | Design **SUPPORTED**; status **STALE / UNDERCLAIM** | Use the layered status block. |
| IAC-2 | 102-107: finite matrices can represent discretized shapes, but no pricing or continuous numerics established | Pricing caveat **SUPPORTED**; continuous-numerics negative is **STALE / UNDERCLAIM** | Replace with the native basis account in section 8 below. The finite-dimensional spline basis provides exact piecewise-smooth settlement semantics over a continuous integer outcome coordinate, and its point-resolution/internal-redemption subset now executes locally in SBF. It does not supply a pricing model, an actually uncountable onchain state space, native bearer exit, or source authentication. |
| IAC-3 | 109-118: publication freezes inspectable rules and nobody can substitute another transition | **SUPPORTED as intended semantics; OVERCLAIM as current full runtime** | State it conditionally. Typed immutable artifacts exist, but the source/feed/epoch and full loader/upgrade provenance are not closed. |
| IAC-4 | 120-134: structural maximum-liability invariant around every prototype transition | **SUPPORTED for the pure kernel/model; UNDERCLAIMS proof detail** | Bound “every transition” to the named pure kernel. Cite the Lean solvency/complete-set model separately and do not imply a proved Rust refinement or every SBF adapter. |
| IAC-5 | 136-144: frozen-book full recomputation | **SUPPORTED at host level** | Add the funded-reservation and restricted direct-settlement SBF seams, while preserving the absent candidate-selection, entitlement-construction, and full-settlement qualifiers. |
| IAC-6 | 146-155: resolution licensed by admitted evidence and no final adjudicator choice | Maturity/refusal **SUPPORTED**; authenticated authority **OVERCLAIM** | Make the missing source/archive join explicit in the body, not only in an appendix. |
| IAC-7 | 157-163: exact, one-shot, serialized settlement | **SUPPORTED in formal models and categorical local-SBF redemption; OVERBROAD for the whole venue** | State the exact evidence: owner replay, market resolution replay, bearer burn/payout, and transaction atomicity executed locally. Full order clearing/receipt settlement and source-authenticated resolution remain open. |
| IAC-8 | 165-179: three machine-checked negatives | **SUPPORTED with attribution repairs** | Row 1 belongs to the guarded-hole model; row 2 to candidate-result stability; row 3 to gluing/coupled-ledger models. “No primitive allows an obligation…” is broader than the formal vocabulary and should be narrowed. |
| IAC-9 | 232-242: useful proof targets and scoped proof evidence | **SUPPORTED as proposal** | Add that some targets now have separate partial evidence, but no composed proof covers implementation/runtime/legal compliance. |
| IAC-10 | 273-283: an operatorless agent design has prepaid permissionless execution and no operator | **OVERCLAIM if present tense; PROPOSED if conditional** | Rewrite “A proposed architecture would…” The committed prepaid liveness kernel is host-only and uses unmeasured maxima. Partial permissionless construction does not establish an operatorless venue or agent. |
| IAC-11 | 285-293: separate certificate-stack artifact and explicit trusted roles/no operatorless agent | **Not changed by Dragon's Clutch; existing boundary is appropriately conservative** | Retain only if its separate producer commit and 86-test reproduction record are frozen again for Draft 9. Do not use Dragon's Clutch commits to refresh that unrelated claim. |
| IAC-12 | 323-333: offline code, non-composition, no deployed/funded product | No-deployment/non-composition **SUPPORTED**; “offline” **STALE** | Say “local research code, including local-bank SBF execution.” |
| IAC-13 | 355 appendix: guarded order, redemption once, and no late amount/obligor | Mixed: guarded part **SUPPORTED**; redemption-once now has direct Dragon runtime evidence; “no primitive” **OVERCLAIM** | Split the row. Use formal field names for guarded holes and SOL-1/SOL-5/SOL-7 for redemption/replay. |
| IAC-14 | 356 appendix: early outcome and balance constraints; “no oracle process implemented” | Formal counterexamples **SUPPORTED**; oracle wording **STALE** | Say “no source-authenticated live onchain oracle/finality path”; a typed admission relation and conditional parser profile now exist. |
| IAC-15 | 357 appendix: finite payout-matrix generality “does not establish … continuous numerics” | **STALE / UNDERCLAIM** | Add separate native B-spline host/proof rows and retain only the no-pricing/no-end-to-end-refinement limitations. |
| IAC-16 | 359-360 appendix: separate agent evidence and boundaries | **Not revalidated in this audit** | Keep separate from the Dragon evidence register; re-pin or omit. |
| IAC-17 | 361 appendix: no deployment/funding/operation/end-to-end system | **SUPPORTED** | Keep. |
| IAC-18 | `iac/sources.typ` 98-114: frozen guarded/candidate materials and offline Dragon accounting | **STALE provenance and engineering summary** | Pin exact commits/paths and use the Draft 9 evidence levels. |

## 7. Claim inventory: IAC cover Draft 8

References are to `docs/regulatory/typst/iac-cover/body.typ`.

| ID | Lines / claim family | Verdict | Draft 9 action |
|---|---|---|---|
| COV-1 | 3-8: no deployed venue, funds, or live orders | **SUPPORTED** | Keep; “no public-cluster deployment” is the most precise engineering wording. |
| COV-2 | 18-25: funding fixes maximum liability; prefunding does not decide clearing status | Technical worked-example component **SUPPORTED** | No engineering correction required. |
| COV-3 | 34-37: proofs count only for encoded statements | **SUPPORTED proposal** | No change beyond aligning the attachment's evidence vocabulary. |
| COV-4 | 38-42: deterministic settlement relocates manipulation risk | **SUPPORTED analytical position**, not proof of source integrity | Ensure the attachment discloses that current source authentication is open. |
| COV-5 | 51-54: regulator-observable Shielded target; Dark research boundary | **SUPPORTED as proposed terminology** | Keep separate from current implementation claims. |

## 8. Recommended Draft 9 wording

These are drafting replacements, not instructions to edit Draft 8 silently.

### 8.1 Shared prototype-status paragraph

Use substantially this wording in the three long filings:

> **VERIFIED (local research artifacts).** I have implemented separate layers
> of this design: exact pure-Rust accounting, observation, spline-settlement,
> and batch relations tested on the host, and a Solana SBF adapter exercised
> only in local Agave banks. A clean categorical custody walk committed 22
> signed transactions against one genesis-assisted loopback market and ended
> with all owned collateral withdrawn or redeemed. This is not a deployed
> venue, a public-cluster result, or a complete order-settlement or
> source-authenticated lifecycle.

Then use a distinct proof paragraph:

> **VERIFIED (scoped proof artifacts).** Separate Lean models prove named
> solvency, complete-set, and degree-zero-through-three B-spline construction
> and quantization properties, and a pinned Verus check covers one internal-
> transfer arithmetic function. No checked refinement connects the complete
> Rust/SBF implementation to those models; accounts, parsers, CPI, SBF code
> generation, runtime behavior, and legal compliance remain outside those
> results. The end-to-end implementation is not formally verified.

This is more accurate than both “offline only” and an undifferentiated
“formally verified.”

### 8.2 Native B-spline paragraph

Replace the IAC finite-matrix/continuous-numerics paragraph with:

> **VERIFIED (host semantics and separate formal model).** Five categorical
> bands are one degree-zero instance, not the payout primitive's limit. The
> research implementation also defines native degree-one hat and degree-two
> and degree-three open-clamped B-spline bases on a frozen integer grid. At a
> resolved point the exact rational basis is quantized by one canonical rule
> into nonnegative integer weights summing to the frozen denominator; exact
> coefficient portfolios over that native basis express shaped exposure
> without replacing the basis with one-hot bins. Host tests and an independent
> exact-rational differential oracle exercise the evaluator, while Lean proves
> corresponding uniform-grid mathematical properties in a separate model.
> A restricted local SBF path now executes degree-one through degree-three
> point resolution and exact internal redemption with the resolved vector
> persisted in one versioned Resolution record. This establishes neither a
> pricing algorithm nor a refinement proof for the Solana adapter; native
> bearer redemption and authenticated source-to-resolution lineage remain
> open.

Add, if the occupation statistic matters to the filing:

> **VERIFIED (host-only statistic).** A separate fixed-width accumulator can
> combine adjacent equal-duration buckets of native quantized basis mass,
> records gaps explicitly, and refuses incomplete windows. It does not
> authenticate the source or archive and is not yet a live settlement path.

### 8.3 Source-authentication paragraph

Replace every present-tense claim that the program reads authenticated history
with:

> **PROPOSED qualifying design; current STOP.** A qualifying observation path
> would authenticate a frozen source specification and deployment/parser
> release, admit one canonical record per bucket under explicit freshness and
> confidence rules, archive the exact accepted lineage, and resolve only from
> that archive. The current committed SBF path does not yet do this: it folds
> caller-supplied evidence and does not prove that the evidence came from the
> source named in the terms. The repository contains a tested admission
> relation and conditional parser profile, not a source-authenticated onchain
> resolution path.

This disclosure should appear in the body, because it limits the worked
example's central authority claim.

### 8.4 Settlement and Solana paragraph

Use:

> **VERIFIED (local SBF subset).** In a clean genesis-assisted loopback run,
> categorical resolution, exact idempotent retry, internal redemption, actual
> Token-2022 bearer burn and payout, and owner cash withdrawal executed as
> signed committed transactions, including byte-preserving refusal and rollback
> cases. This demonstrates a local execution subset, not a deployed venue:
> source history was injected rather than authenticated, and the walk did not
> execute order-book candidate selection or receipt settlement.

One additional committed focused test now supports this narrow supplement:

> **VERIFIED (restricted local SBF settlement seam).** A separate focused bank
> test executes one preauthorized same-page, full-fill, direct single-Egg,
> zero-fee settlement slice against two funded reservations and one prefrozen
> receipt, with exact one-shot consumption and byte-preserving hostile cases.
> Candidate selection, receipt construction, complete reservation-set closure,
> partial and portfolio fills, fees, cross-page settlement, lapse, and terminal
> sweep remain outside this result.

Do not summarize either result as “Solana settlement is implemented” without
those qualifiers.

### 8.5 Operatorlessness and liveness paragraph

Use:

> **PROPOSED, with partial local components.** A host-tested accounting kernel
> defines prepaid work/storage/resolution and per-order clear/settle reserves,
> and local SBF tests exercise permissionless creation of the market-local
> core and typed immutable artifacts. The liveness maxima remain unmeasured,
> the funding/account adapter is incomplete, and source, feed, epoch,
> candidate, and settlement construction do not yet form a blank-bank
> lifecycle. These components do not establish an operatorless venue or
> guarantee transaction inclusion.

### 8.6 Guarded-hole and Minidregg paragraph

Use the formal vocabulary rather than economic universals:

> **VERIFIED (formal models at frozen commits).** In the reviewed weak guarded-
> hole models, the admitted value type, actor/authority demand, target and
> finite footprint, guard/effect commitments, deadline, continuation, and
> replay domain are fixed before later typed advice arrives. Accepted advice
> satisfies the modeled guard/effect checks; rejected or replayed advice leaves
> the modeled state unchanged. These are theorems about hand-written models,
> not cryptographic commitment soundness, physical atomicity, a deployed
> control, or a legal characterization.

Delete “no primitive allows an obligation whose amount or obligor is fixed
later.” It translates a bounded formal constructor into an unbounded claim
about every primitive and uses legal/economic nouns absent from the theorem.

### 8.7 Evidence register

For Draft 9, every material technical sentence should map to one row carrying
one of the repository's required epistemic labels:

- `VERIFIED (MODEL-PROVED)`;
- `VERIFIED (HOST-TESTED)`;
- `VERIFIED (SBF-EXECUTED LOCALLY)`;
- `SOURCED`;
- `INFERRED`; or
- `PROPOSED`.

The current appendix phrases “model theorem,” “prototype source,” and “basis”
are helpful but are not themselves the required labels. A compact claim ID in
the body can point to a labeled appendix row; labels need not make the prose
unreadable. Keep the evidence levels separate. In particular:

- a Lean theorem is not a Rust refinement;
- a host test is not SBF execution;
- local SBF execution is not deployment;
- a typed source relation is not source authentication;
- an immutable artifact is not loader/upgrade provenance;
- prefunded liveness arithmetic is not inclusion; and
- formal soundness is not legal compliance.

## 9. Claims that must wait for in-flight commits

No Draft 9 sentence may cite the remaining dirty work as landed until the
producing lane commits and the exact joined tree is rerun. Three narrow slices
that landed during this audit are recorded separately below so their STOPs are
not lost.

| In-flight area | Preliminary evidence observed | Maximum wording after the commit gate | Still forbidden after that narrow gate |
|---|---|---|---|
| Truth/handoff reconciliation | Dirty prose only | None until committed and checked against code/evidence | Any promotion based solely on a status-document edit |

The three newly committed slices remain non-composable at their stronger edges:

| Newly landed slice | Narrow committed claim | Still forbidden |
|---|---|---|
| `ceac012` market construction | Host-tested prefund-safe, degree-selected `CreateMarket`; older categorical real-bank constructor remains separately evidenced | A green end-to-end sealed-artifact/native blank-bank walk, source/feed/epoch construction, native bearer exit, or operatorlessness |
| `2b2ee3c` source/archive seam | Provider-neutral fixed codecs and receipt-verification relation, four focused host-workspace tests | Production provider parser/authenticator, routed instruction, live Clock/feed-head join, `Resolve` consumption, source-authenticated settlement, or CU claim |
| `f529460` settlement seam | One restricted preauthorized direct slice executes in local SBF | Candidate selection, complete entitlement construction, broader fill shapes, terminal closure, or full venue |
| `6a826b6` native SBF seam | Degree-one through degree-three point resolution and exact internal redemption execute locally against a pre-created v3 account | Completed blank-bank native creation evidence, native bearer redemption, authenticated archive-to-Resolve join, full lifecycle, deployment, or formal Rust/SBF refinement |

A single clean joined ELF and lifecycle run is still needed before composing
these claims. Independent narrow green tests do not establish that the joined
state machine is coherent.

## 10. Draft 9 edit order

1. Repair source-authentication wording first; it is the only present material
   implementation overclaim in the worked example.
2. Replace the shared “offline / tested, not formally verified” paragraph with
   the layered host, model-proof, narrow-Verus, and local-SBF account.
3. Add native B-spline semantics as its own technical contribution, preserving
   the no-pricing and no-refinement boundaries.
4. Split formal guarded-hole claims from Dragon's Clutch runtime replay and
   redemption claims.
5. Update every appendix row to an explicit epistemic label, frozen commit,
   exact path, and exact evidence scope.
6. Add only those in-flight results that have committed and survived a clean
   joined reproduction; otherwise retain their STOP wording.
7. Rebuild Draft 9 PDFs and perform a fresh line/PDF claim audit before any
   human legal, disclosure, identity, or filing review.

## 11. Commands reproduced for this audit

All commands were local and non-networked.

- `cargo test --manifest-path crates/clutch-bspline/Cargo.toml --offline --locked`: 12 passed.
- `python3 crates/clutch-bspline/oracle/check.py`: 31,814 exact cases passed; six mutants killed.
- `lake build DragonsClutch.BSpline` from `dragons-clutch/lean`: passed.
- `cargo test --manifest-path crates/clutch-bspline-accumulator/Cargo.toml --offline --locked`: 19 passed.
- `cargo test --manifest-path crates/clutch-liveness/Cargo.toml --offline --locked`: 13 passed.
- `cargo test --manifest-path research/fractional-redemption/Cargo.toml --offline --locked`: 13 passed.
- `cargo test --manifest-path research/source-profile-v1/Cargo.toml --offline --locked`: six passed.
- `cargo test --manifest-path programs/solana-reference/Cargo.toml --offline --locked`: 49 passed.
- `cargo test --manifest-path crates/clutch-kernel/Cargo.toml --offline --locked`: 23 passed.
- `cargo test --manifest-path crates/clutch-accumulator/Cargo.toml --offline --locked`: 24 unit and two doc tests passed.
- `cargo test --manifest-path crates/clutch-batch/Cargo.toml --offline --locked`: 61 passed.

These reruns occurred while unrelated SBF integration files were dirty. The
pure crates named above were not dirty, but this is not a clean joined release
attestation. The committed runtime claims remain pinned to their own recorded
source commits and ELF digests.
