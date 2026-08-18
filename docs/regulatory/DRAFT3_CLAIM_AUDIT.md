# Draft 3 filing-preflight claim ledger

Status: filing-preflight audit, inspected 2026-08-18. This is a local
research control, not legal advice, a filing authorization, a source audit for
the sibling repositories, or a conclusion about any product or jurisdiction.

This ledger audits the Draft 3 sources under `docs/regulatory/typst/`, the
review PDFs under `output/pdf/`, and the local status records. A theorem, test,
or repository note is evidence only for the proposition it actually states.

The local ceilings used here are:

| Gate | Meaning | Does not mean |
|---|---|---|
| S | Lean definition/theorem about a model, with stated premises | deployed cryptography, physical execution, or legal effect |
| A | Lean-owned admission/controller decision over a modeled request | signature security, custody, consensus, or physical CAS |
| P | Concrete cryptographic reduction/security game at named parameters | merely having a relation, ciphertext, or proof-shaped type |
| D | Exact artifact, handler/storage path, and consumer are joined | a model, test, or microbenchmark |
| B | Reproducible matched measurement or deterministic test corpus | security, anonymity, liveness, or production readiness |
| R | Read-only repository/provenance inspection | independent audit, clean-room status, or public reproducibility |

## Material VERIFIED claim ledger

“Allowed wording” is the strongest wording supported by the cited artifact. A
Draft 3 sentence that goes beyond it must be downgraded or reclassified.

| ID | Material claim family and allowed wording | Exact local artifact/path | Strongest actual gate | Filing boundary |
|---|---|---|---|---|
| V-01 | Dark Egg Research is a research/experiment-design repository; its local record says it has no production protocol, live credentials, deployed venue, or regulator submission. | `docs/VERDICTS.md` V0/V7; `AGENTS.md` Mission/Safety; `docs/regulatory/README.md` | R | Say “this repository records no…” or “no action is authorized here.” Do not assert a universal negative about every external system or person. |
| V-02 | Draft 3 Typst sources and four review-PDF targets exist locally and are marked for review/not filed; identity fields remain placeholders. | `docs/regulatory/README.md`; `docs/regulatory/typst/*/metadata.typ`; `output/pdf/*draft-3.pdf` | R | Treat as review artifacts only. A PDF’s existence is not a filing, acceptance, or approval. Freeze a clean source commit, build manifest, hashes, and output set before any filing edition. |
| V-03 | The weak Breadstuffs guarded-hole prototype fixes field, actor, target, and guards before a later integer; accepted fill equals the modeled state transition and a guard violation fails closed. | `/Users/ember/dev/breadstuffs/metatheory/Dregg2/Exec/GuardedHole.lean:48-70`; `/Users/ember/dev/breadstuffs/docs/DESIGN-partial-turn-promises.md:59-70,228-252` | S | Keep “weak,” “prototype,” and “modeled.” Do not call it a legal category, deployed proof circuit, authority-bearing lazy contribution, or production control. |
| V-04 | Current Minidregg formal research models eager typed advice/guarded reactions with codec, pre-root, abstract authority demand, finite footprint, guard/effect commitments, deadline, continuation, replay domain, logical CAS/nullifier refusal, and frame theorems. | `/Users/ember/dev/minidregg/docs/FORMAL_STATUS_AND_NEXT_PROOFS.md:3,36-43`; `/Users/ember/dev/minidregg/Theory/GuardedAdvice.lean:176-188,286-339`; `/Users/ember/dev/minidregg/Theory/ReactiveController.lean:240-453`; `/Users/ember/dev/minidregg/Theory/ReactiveCellTransition.lean` | S/A | Snapshot is source-inspection evidence at commit `bf45a611ec8f2bf401012376ea14b45827910b6f`, with dirty uncommitted work and no current umbrella build. Do not say “deployed,” “cryptographically bound,” or “physically atomic.” |
| V-05 | Minidregg’s private-turn/escrow model separates sealed work from public release/settlement and deliberately rejects zero-pinned proof suites as semantic evidence. | `/Users/ember/dev/minidregg/docs/FORMAL_STATUS_AND_NEXT_PROOFS.md:41,74-76`; `/Users/ember/dev/minidregg/Kernel/PrivateTurn.lean`; `/Users/ember/dev/minidregg/Kernel/PrivateEscrowSettlement.lean:107-114,537-579` | S/A | The status doc says there is no FHE executor, native BFV correctness refinement, FHE confidentiality theorem, vFHE construction, or deployed succinct verifier. Never promote “BFV,” “private,” “sealed,” or “proof” to Dark/FHE/vFHE. |
| V-06 | Leanuweave’s candidate-result model represents partial results as sets of correlated worlds/answers; deterministic image evaluation commutes with union, while determinacy requires a separately supplied stability/coordination premise. | `/Users/ember/dev/leanuweave/Uwueave/Holes.lean:79-117,555-621,899-904`; `/Users/ember/dev/leanuweave/docs/MAP.md` `Uwueave/Holes.lean` row; commit `f1450667cc87a48706c61f6d5ead71f73ab43fb1` | S | State the set/evaluation theorem only. Do not infer oracle validity, legal finality, authentication, causal provenance, or enforceable selection. |
| V-07 | Leanuweave’s guarded-gluing result is conditional on `Spanning`; witnesses show `Glues` and `IConfluent` can diverge without that premise, and one-shot behavior can be globally non-gluing but gluing within an owner seam. | `/Users/ember/dev/leanuweave/Uwueave/Gluing.lean:233-272,381-443,683-800`; `/Users/ember/dev/leanuweave/docs/MAP.md` `Uwueave/Gluing.lean` row | S | Preserve the `Spanning` and seam qualifications. “Guarded holes always glue,” “always converge,” or “one-shot admission is automatic” is overbroad. |
| V-08 | The local claim that the formal guarded-commitment/candidate-result artifacts are research objects and do not compose into a production permissionless end-to-end Dark market is supported by repository status and sibling status audits. | `docs/VERDICTS.md` V2/V3/V6/V7; `docs/LOCAL_LINEAGE.md`; `/Users/ember/dev/minidregg/docs/FORMAL_STATUS_AND_NEXT_PROOFS.md:41,72-76,84-96`; `/Users/ember/dev/breadstuffs/STATUS.md`; `/Users/ember/dev/breadstuffs/audits/AUDIT-privacy.md` | R with S/A/B components | This is a bounded local-state statement, not an independent security audit and not proof that no external implementation exists. Keep “does not presently compose,” not “cannot compose.” |
| V-09 | `relations/CLEARING_V0.md` specifies a proposed fixed-grid maximum-volume/ties-low/pro-rata relation; it is not a production mechanism. | `relations/CLEARING_V0.md:1-6,31-100` | S (specification only) | Do not describe V0 as an accepted market rule, economic recommendation, settlement protocol, or reporting standard. |
| V-10 | The Dark-FBA Rust artifact computes bounded v0 clearing/allocation arithmetic with exact integers, deterministic ties, refusal cases, conservation/limit tests, and byte-stable vectors; its process sees the full fixture and `DarkTarget` is refused. | `docs/research/DARK_FBA_RELATION.md:1-3,6-11`; `experiments/dark-fba/README.md`; `experiments/dark-fba/src/lib.rs`; `experiments/dark-fba/PROVENANCE.md`; `experiments/dark-fba/vectors/v1.txt` (SHA-256 `9a00d7393d00b5cca1e1b980a468a48cb7c21053fac8ae9e15abe2ba7fc9a767`) | B for offline semantics; no P/D | `cargo test --manifest-path experiments/dark-fba/Cargo.toml` passed 9 tests on 2026-08-18. This verifies toy arithmetic/test corpus only, not commitment binding, encryption, FHE, proof soundness, access control, availability, liveness, settlement, or Dark confidentiality. |
| V-11 | The leakage laboratory deterministically projects four synthetic traces onto Clear, named-executor Shielded, and a hypothetical Dark transcript surface, separating mechanically revealed fields from inferred deductions. | `docs/research/DARK_RELATION_THREAT_MODEL.md`; `experiments/leakage-lab/README.md`; `experiments/leakage-lab/leakage_lab.py`; `experiments/leakage-lab/test_leakage_lab.py`; `experiments/leakage-lab/vectors/v1.json` | B for synthetic transcript accounting; no P/D | `python3 -m unittest -v` passed 7 tests on 2026-08-18. The Dark row is explicitly `PROPOSED` in the harness. Do not call this an anonymity, noninterference, cryptographic leakage, timing, endpoint, settlement, or real-market measurement. |
| V-12 | The experiment records proposed leakage budgets (fixed capacity/shape, public result, recipient-only local fills, bounded audit queries) and explicitly says roots are transcript fields, not verified commitments. | `docs/research/DARK_RELATION_THREAT_MODEL.md` “Concrete disclosure-budget contract”; `experiments/leakage-lab/README.md`; `experiments/leakage-lab/leakage_lab.py` `_surface`/`observe` | B of accounting code only | Call these proposed surface contracts. A root, fixed padding, opaque field, exact arithmetic, threshold label, or proof label alone does not establish confidentiality. Public settlement and external correlation remain outside the experiment. |
| V-13 | Breadstuffs contains substantial FHE/market/proof research and historical measurements, but its local status/provenance record identifies trusted/semi-honest, complete-book-viewer, nonlinear, settlement, or liveness boundaries in the composed dark path. | `docs/VERDICTS.md` V2; `docs/LOCAL_LINEAGE.md` Breadstuffs section; `/Users/ember/dev/breadstuffs/STATUS.md`; `/Users/ember/dev/breadstuffs/fhegg-fhe/MEASURED-ENVELOPE.md`; `/Users/ember/dev/breadstuffs/fhegg-fhe/ADDITIVE-FOLD-ENVELOPE.md` | R/B for named historical measurements | Quote only the exact measured path, host, date, and real-vs-extrapolated distinction. Do not convert an FHE clear, BFV fold, threshold component, or test into a composed Dark venue, vFHE, permissionless liveness, or private settlement claim. |
| V-14 | The local provenance boundary records that this repository did not import sibling source code and does not claim clean-room status after read-only inspection. | `AGENTS.md`; `docs/LOCAL_LINEAGE.md` “Provenance manifest required”; `experiments/dark-fba/PROVENANCE.md` “Lineage boundary”; `docs/regulatory/README.md` | R / self-provenance record | Copyright and license clearance remain a human gate. Do not say “clean room,” “no third-party provenance,” or “copyright cleared.” |
| V-15 | The Draft 3 data-reporting guarded-update/candidate-state paragraph is supported by the same model evidence, but is a design pattern rather than a deployed reporting adapter. | `docs/regulatory/typst/data-reporting/body.typ:239-252`; V-04/V-06 above | S/A only | Keep the `VERIFIED (local research object)` and `INFERRED` limitation. Do not call it a compliance implementation, regulator-access mechanism, or accepted reporting schema. |
| V-16 | The Draft 3 comments expressly disclaim jurisdictional exemption, production infrastructure, customer funds/live orders, and present Dark reporting compliance. | `docs/regulatory/typst/iac/body.typ:78-91,317-323`; `docs/regulatory/typst/definitions/body.typ:246-264`; `docs/regulatory/typst/data-reporting/body.typ:292-299`; `docs/VERDICTS.md` V7 | R/document-scope check | Preserve these limits. Moving an `INFERRED` or `PROPOSED` paragraph under a `VERIFIED` label is a material claim upgrade requiring a new audit. |

## Claims that are not locally VERIFIED

These are not defects when labeled correctly. They are downgrade points if a
filing edition presents them as facts about an operating product, current law,
or a security property.

| Claim family | Current status and source | Required downgrade or gate |
|---|---|---|
| Clear/Shielded/Dark taxonomy and any end-to-end Dark statement | `docs/regulatory/typst/iac/body.typ:15-18,182-196`; `docs/regulatory/typst/data-reporting/body.typ:179-205`; `docs/research/DARK_RELATION_THREAT_MODEL.md` | `PROPOSED`/`INFERRED` only. A Dark filing claim needs an implemented backend, frozen leakage function, corruption model, ingress/intermediate/proof/settlement/availability analysis, malicious-executor correctness, and distributed-liveness evidence. None is present. |
| Independent researcher identity, affiliation, contact, and signature | `docs/regulatory/typst/iac/body.typ:5-13`; all `metadata.typ`; signature blocks | Human identity/authority gate. The repository cannot verify who will submit or whether the statement is authorized. |
| Counsel, classification, registration, DCM/SEF/DCO, CEA/Exchange Act, or Regulation 40.11 conclusions | `docs/regulatory/typst/*/sources.typ`; Draft 3 `SOURCED` paragraphs | Human counsel gate. Keep technical text as questions, factual matrices, or explicitly labeled inference. This ledger gives no legal conclusion. |
| Current deadlines, docket/document identifiers, agendas, meeting status, and current rules | `docs/regulatory/typst/*/sources.typ` (retrieved 2026-08-17); `docs/regulatory/SUBMISSION_WEEK_PLAN.md` | Current-docket gate immediately before filing: re-open official records, check extensions/corrections/agenda, exact title/RIN/File number, method, language, and receipt deadline. Retrieval date is not current verification. |
| Independent public reproducibility, independent audit, or clean-room lineage | `docs/regulatory/typst/*/sources.typ` local-material paragraphs; `docs/LOCAL_LINEAGE.md`; sibling status docs | Downgrade to “reviewed local research artifacts” unless immutable public commits, exact paths, licenses, generator/build manifest, hashes, and independent reproduction are frozen. Read-only inspection defeats a clean-room claim. |
| Cryptographic commitment binding, signature/key-custody correctness, FHE/vFHE, proof soundness, anonymity, noninterference, or confidential regulatory access | `docs/regulatory/typst/iac/body.typ:78-81,228-246`; `docs/regulatory/typst/data-reporting/body.typ:104-127`; Minidregg status `FORMAL_STATUS_AND_NEXT_PROOFS.md:41` | Downgrade to model theorem, proposed objective, or synthetic test. No local P/D gate closes these claims. |
| Production clearing, matching, settlement, collateral sufficiency, oracle validity, chain atomicity, availability, or liveness | `docs/regulatory/typst/iac/body.typ:78-91`; `docs/research/DARK_FBA_RELATION.md:9-10`; `relations/CLEARING_V0.md` “Known omissions” | Downgrade to bounded arithmetic/specification. The toy moves no assets and has no chain, proof, network, or custody adapter. |
| Empirical identity/strategy leakage or privacy benefit from the leakage lab | `docs/regulatory/typst/data-reporting/body.typ:129-176`; `experiments/leakage-lab/*` | Keep as `INFERRED`/`PROPOSED` risk analysis. The lab uses synthetic traces and no external timing, identity, endpoint, settlement, or market data. |
| No source code copied as a copyright/license conclusion | `docs/regulatory/README.md`; `experiments/dark-fba/PROVENANCE.md`; `LICENSING.md`; `NOTICE` | Human copyright/provenance gate. Review source, fixture, generated artifact, theorem/text, license, vendor/patent boundary, and public-disclosure payload. Do not state that AGPL selection resolves third-party rights. |
| Proof establishes the report, onchain means reported, or encryption means unavailable to lawful process | `docs/regulatory/typst/data-reporting/body.typ:104-127,292-317` | These are explicitly denied by Draft 3 limits. Preserve the denial; any contrary sentence is rejected/overbroad. |

## Mandatory human gates before a filing edition

No local test can discharge these gates:

1. **Identity and authority:** verify full name, affiliation (if any), safe
   public contact, signature, and authority to make each representation.
2. **Counsel:** qualified counsel reviews instrument, venue/intermediary,
   clearing/custody, state-law, sanctions/AML, tax, disclosure, copyright, and
   public-communications issues. This ledger is not legal advice.
3. **Current docket:** immediately revalidate each official notice, deadline,
   docket/document identifier, agency method, language requirement, agenda, and
   publication warning. Submit once only if the human filer authorizes it and
   saves the receipt.
4. **Copyright/provenance:** review all descriptions, links, figures, constants,
   fixtures, generated output, and quoted text. Freeze producer commits, exact
   paths, licenses, generator/build commands, hashes, and receiver checks where
   reproducibility is claimed. Do not claim clean-room status.
5. **Disclosure:** remove private keys, secrets, unnecessary personal data,
   confidential business information, unpublished vulnerabilities, and any
   implementation detail not intended for permanent public posting.

## Mechanical checks recorded for this audit

- `cargo test --manifest-path experiments/dark-fba/Cargo.toml`: 9 tests passed
  on 2026-08-18. This is the v0 offline semantic toy gate only.
- `python3 -m unittest -v` from `experiments/leakage-lab`: 7 tests passed on
  2026-08-18. This is the synthetic transcript-projector gate only.
- No network, regulator contact, filing, deployment, key access, or external
  publication was performed for this audit.

## Pre-filing disposition

The strongest honest overall description is: **source-grounded formal models
plus deterministic offline experiments, with no demonstrated end-to-end Dark
venue or production reporting system**. Identity, counsel, current docket,
copyright/provenance, final build, and public-disclosure decisions remain human
gates.
