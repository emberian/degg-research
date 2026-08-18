# Draft 5 consolidated claim ledger

Status: consolidated filing-preflight ledger, prepared 2026-08-18 by the
Draft 5 adversarial audit lane. This is a local research control, not legal
advice, a filing authorization, a source audit for the sibling repositories,
or a conclusion about any product or jurisdiction.

This ledger consolidates and renumbers the four prior ledgers into one ID
space, resolving the row-ID collision in which `DRAFT4_CLAIM_AUDIT_DATA.md`
and `DRAFT4_CLAIM_AUDIT_IAC.md` each independently assigned V-22 and up.
It also records the Draft 5 changes (from `DRAFT5_CLAIM_DELTA.md`, verified
by `DRAFT5_PACKET_AUDIT_VERDICT.md`) and the post-verdict repairs the audit
lane applied directly to the Draft 5 sources.

Governing master: `DRAFT3_CLAIM_AUDIT.md` remains the ceiling authority for
V-01 through V-16, the not-locally-VERIFIED downgrade table, and the gate
definitions (S/A/P/D/B/R). Nothing here raises any ceiling.

## ID space and renumbering map

| Consolidated ID | Origin | Old ID | Subject |
|---|---|---|---|
| V-01 … V-16 | DRAFT3_CLAIM_AUDIT.md | unchanged | Master ceilings and downgrade families |
| V-17 | DRAFT4_CLAIM_AUDIT.md | V-17 | Dragon's Clutch prototype existence/status wording (definitions) |
| V-18 | DRAFT4_CLAIM_AUDIT.md | V-18 | Worked example's market terms as hypothetical design |
| V-19 | DRAFT4_CLAIM_AUDIT.md | V-19 | Regulation 40.11 scope disclaimer (definitions) |
| V-20 | DRAFT4_CLAIM_AUDIT.md | V-20 | Definitions traceability appendix accuracy |
| V-21 | DRAFT4_CLAIM_AUDIT.md | V-21 | Definitions identifier preservation |
| V-22 | DRAFT4_CLAIM_AUDIT_DATA.md | V-22 | Worked-example-as-records milestone walk |
| V-23 | DRAFT4_CLAIM_AUDIT_DATA.md | V-23 | Batch verifier full-recomputation claim (data) |
| V-24 | DRAFT4_CLAIM_AUDIT_DATA.md | V-24 | Accumulator refusal claim (data) |
| V-25 | DRAFT4_CLAIM_AUDIT_DATA.md | V-25 | Leakage-laboratory description and hedges |
| V-26 | DRAFT4_CLAIM_AUDIT_DATA.md | V-26 | Parts 43/45/49 sourced characterizations |
| V-27 | DRAFT4_CLAIM_AUDIT_DATA.md | V-27 | Data traceability appendix accuracy |
| V-28 | DRAFT4_CLAIM_AUDIT_DATA.md | V-28 | Data identifier preservation |
| V-29 | DRAFT4_CLAIM_AUDIT_IAC.md | V-22 | Five-milestone taxonomy as PROPOSED framing |
| V-30 | DRAFT4_CLAIM_AUDIT_IAC.md | V-23 | Sealing/finality stability-premise claims |
| V-31 | DRAFT4_CLAIM_AUDIT_IAC.md | V-24 | Balance-guard non-gluing claim (counterexample direction only) |
| V-32 | DRAFT4_CLAIM_AUDIT_IAC.md | V-25 | Compressed worked market and prototype wording (IAC) |
| V-33 | DRAFT4_CLAIM_AUDIT_IAC.md | V-26 | IAC identifier, deadline, and note_ref integrity |
| V-34 | DRAFT4_CLAIM_AUDIT_IAC.md | V-27 | IAC cover register and seven questions |
| V-35 … V-38 | this ledger | new | Draft 5 changes and audit repairs (below) |

Any citation of "V-22 … V-27" in `DRAFT4_CLAIM_AUDIT_IAC.md` or documents
quoting it should be read through this map. The three Draft 4 per-filing
ledgers remain on disk as the full row text for V-17 through V-34; this file
owns the ID space from Draft 5 forward.

## New rows for Draft 5

| ID | Change and allowed wording | Evidence and boundary |
|---|---|---|
| V-35 | Definitions Criterion 4: "commodity price" → "onchain digital-asset price" for the worked example (fix F-6 of `DRAFT4_DEFINITIONS_AUDIT_VERDICT.md`). Direction: downgrade — a legal characterization becomes a factual description. The paired-examples table row "One application refers to a commodity price" is retained deliberately: the verdict expressly allows the phrase inside a hypothetical row, where it is the varied hypothesis of a jurisdictional question, not an assertion about the worked design. | `typst/definitions/body.typ` (Criterion 4). Record correction: the Draft 4 post-audit addendum in `DRAFT4_CLAIM_AUDIT.md` recorded F-6 as applied, but the committed Draft 4 text and rebuilt Draft 4 PDF still read "commodity price"; the fix actually landed in Draft 5 (verified against `git show f087fea` and the Draft 4 PDF text). |
| V-36 | Definitions stage-table intro: the universal "Any staged automated product can be located on it" removed (F-9). The table now claims only to generalize the walk. Direction: downgrade. | `typst/definitions/body.typ` ("The table generalizes this walk; …"). |
| V-37 | Shared template, cover warning block: the sentence "Public submission would permanently link the named submitter to the research described here" removed; the block now reads "*Review draft - not filed.* Identity, privacy, legal, source, and live-docket review remain required before filing." The not-filed warning, review requirements, and [FULL NAME] placeholders are unchanged. The removed sentence existed only in `cover_filing` (used by the IAC cover); the main `filing()` review block never contained it and is untouched. | `typst/shared/template.typ` `cover_filing`; verified absent from all four rendered Draft 5 PDFs. |
| V-38 | Monotone-coupling wording set (audit repairs, applied 2026-08-18). Rule: a filing may depend on engineering state through one monotone claim — "I have built an offline research prototype of this design's accounting; it is tested, not formally verified" — plus the ceiling-mandated present-tense negatives (not deployed / no funds / do not presently compose), which the filing-day gate re-verifies. Sentences whose truth tracked the repositories' current API surface or simple-present behavior were rewritten to built-anchored or design-anchored form, superseding the corresponding Draft 4 phrasings as the allowed wording: (a) the definitions and data appendix accounting rows drop "claim materialization" (a prototype-API term absent from the filing's own worked example) and say "has been implemented offline with passing deterministic tests"; the definitions body attributes the operation list to "the worked example's accounting --- deposit, recombination, resolution, redemption"; (b) V-23 wording becomes "I built the batch verifier to accept a submitted clearing only if recomputation from the frozen book reproduces it exactly, never trusting the submitter's claimed quantities"; (c) V-24 wording becomes "I built the observation accumulator to refuse a question its retained information cannot support rather than approximate it"; (d) the IAC structural-check sentence becomes "I made the check structural: … and I built every transition --- market construction included --- to refuse, as an invariant violation, any state whose collateral falls below that maximum"; (e) the IAC batch sentence becomes "I built my batch prototype to do exactly this: freeze …, derive …, and accept … recomputed from scratch --- never trusting the submitter's claimed quantities"; (f) appendix rows use "was built to accept / was built to refuse / was built to replay"; (g) the IAC Dark-definition article aligned to "a frozen leakage function" (matching the data-reporting definition and IAC question 6). All changes are narrowings or register decouplings; no ceiling is raised. | `DRAFT5_PACKET_AUDIT_VERDICT.md` findings M-1 … M-6, M-9, C2-1, with file:line for each. |

## Evidence re-pin (2026-08-18, this audit)

- dragons-clutch HEAD is `d60ccf3` ("Kernel: transfer_internal, terminal
  complete-set redemption, structural transactionality"), two commits past
  the `245c965` pin in the Draft 4 ledgers and four past the original
  `fa4efb4e` pin. Committed test counts by source inspection: clutch-kernel
  16, clutch-accumulator 10, clutch-batch 9. `cargo test --offline --locked`
  re-run by this lane on the working tree: 16 + 10 + 28 = 54 passed, 0
  failed (the batch surplus is uncommitted in-progress `relation_v1` work —
  19 additional tests plus a `pub mod` declaration and a visibility change;
  the `propose`/`verify` semantics the filings describe are untouched by the
  dirty diff). The 9 committed batch tests are among the 28 passing.
- Kernel public API at `d60ccf3` adds `redeem_complete_set` and
  `transfer_internal`. Both call `check_invariants` before their first
  write, as every other transition does, so the pre-repair IAC sentence
  ("around every transition") happened to remain true; after the V-38
  repairs no filing sentence depends on the API surface either way. No
  filing states a test count, and no filing contains a "the prototype has no
  transfer" sentence.
- The filings' claim remains exactly "tested, not formally verified."
  dragons-clutch `toolchain/PINNED_PROOF_TOOLS.md` states the pinned Verus
  and Rocq record no verification result, and the `verus/` stubs carry no
  passing proof. Do not upgrade this claim on installed tools alone.
  (Cross-repo note, not a filing defect: at re-pin time the dragons-clutch
  `README.md` Status section still read "Verus is not yet installed or
  pinned," contradicting its own `toolchain/PINNED_PROOF_TOOLS.md`; that
  README was corrected later the same night (dragons-clutch commit
  `a23c7e9`). The filing wording is unaffected either way because it
  asserts only the tested-not-verified boundary.)
- On filing day, this section is re-pinned once at the frozen commit; the
  filings themselves carry no per-commit specifics after V-38.

## Gates before any filing edition (carried forward, unchanged in force)

1. **Identity and authority** — user-owned; placeholders remain.
2. **Legal review** — the analysis is performed in-house and recorded in the
   repository's legal-analysis materials, with a final courtesy review of the
   finished packet by the user's designated reviewer. The gate itself is
   unchanged: no filing edition proceeds without that review of the final
   text. This ledger and the verdicts record reasoning so the review can be
   fast; they are not legal advice.
3. **Current docket** — re-verify every identifier, deadline, method, and
   agenda immediately before filing; retrieval dates are not currency.
4. **Copyright/provenance** — freeze commits, paths, and hashes if public
   reproducibility is claimed; otherwise keep the limited description.
5. **Disclosure** — final scan for secrets, personal data, and anything not
   intended for permanent public posting.

## Supersession

`DRAFT4_CLAIM_AUDIT.md`, `DRAFT4_CLAIM_AUDIT_DATA.md`, and
`DRAFT4_CLAIM_AUDIT_IAC.md` remain the authoritative full row text for
V-17 … V-34 (via the map above) and are marked with pointers to this file.
`DRAFT5_CLAIM_DELTA.md` is the rewrite lane's account of Draft 5 and was
verified by `DRAFT5_PACKET_AUDIT_VERDICT.md`; where V-38 supersedes a
phrasing that the delta had verified verbatim, this ledger controls.

---

# Draft 6 section (appended 2026-08-18 by the Draft 6 rewrite lane)

Draft 6 rewrites all four filings from an interrogative register to a
declarative one under four coordinator directives: (1) positions stated and
argued instead of questions asked; (2) audience ontology --- staff-attorney
categories, research idiolect demoted to at most one-clause credentials;
(3) ceiling-exhaust deletion --- defensive self-audit sentences removed
unless a reasonable reader would otherwise draw a wrong, decision-relevant
inference; (4) hard length budget (definitions 6 pp, data 7 pp, IAC 6 pp,
cover 1 p; all met). Register rule, stated once here: **positions are the
commenter's advocacy and are not bound by the factual-claim ceilings; every
factual sentence about the submitter's artifacts keeps its exact ceiling
wording. Ceilings bound the wording of claims a filing makes; they never
generate sentences. Deleting a claim entirely is always permitted and
violates no ceiling.** Nothing in this section raises any ceiling.

## Draft 6 position register

Positions are advocacy (PROPOSED analysis by the commenter), each argued in
the filing from the verified legal base. "Basis" points into
`LEGAL_ANALYSIS.md` (LA) sections whose citations were fetch-verified
2026-08-18; no citation outside LA §9 or the existing `sources.typ` entries
was introduced. "Counterargument" is the strongest objection the filing
addresses in text.

### Definitions comment (RIN 3038-AF71 / S7-2026-21)

| ID | Position (short form) | Basis | Counterargument addressed |
|---|---|---|---|
| P-D1 | Classify staged programs milestone by milestone, as interpretive guidance within the 2012 framework | LA §2 (1a(47) predicate; 77 FR 48208 role; Question 1 framing) | Stage-relabeling gamesmanship; answered: the matrix reads binding effect, consideration, exposure --- facts, not labels |
| P-D2 | Publication of market software, without more, should not itself be a regulated activity; operative events are funding and interaction | LA §2 (statutory predicate requires agreement/contract/transaction); LA §5 (publication is not a safe harbor either) | Publication as a step in solicitation/operation; answered via "without more" plus P-D7 functional analysis |
| P-D3 | Formation where binding effect and consideration coincide (funding, in the worked example) | LA §2 (1a(47)(A)(i) option prong analogy) | Executory bilateral formation by mutual promises; answered: irrevocable instructions move formation earlier, revocable ones bind nobody |
| P-D4 | Instrument formation and holder's contingent exposure are separate findings; a complete set plus recombination right is fully hedged; exposure starts at the first unbalancing transaction | LA §2 (1a(47)(A)(ii) dependence analysis) | Administrability ("use funding for everything"); answered: two findings serve different rules |
| P-D5 | The reference object and payout structure allocate the category; the worked example implicates no SBS prong on its face, the issuer-referencing variant would | LA §2 (78c(a)(68)(A) three prongs, INFERRED application; mixed-swap narrowness) | One program spans both jurisdictions; answered: that is why only the reference object can carry the allocation |
| P-D6 | Prefunding, loss ceilings, fail-closed terms are risk facts, not classification exclusions --- and the Commissions should say so | LA §2 (1a(47)(B) exclusions are instrument types) | Industry prefunding-exclusion reading and reflexive regulator distrust; both addressed |
| P-D7 | Separate instrument / venue / intermediary / clearing findings; "the software" is not a unit of classification | LA §2 (CEA facility definitions verified); LA §5 | "Code is one thing" coarseness in both exempting and condemning directions |

Kept as genuine questions: weight of transferability where not statutorily
required (policy choice). Non-position on Questions 12-15 retained.

### Data-reporting comment (RIN 3038-AF70 / S7-2026-22)

| ID | Position (short form) | Basis | Counterargument addressed |
|---|---|---|---|
| P-R1 | Report normalized economic lifecycle events bound to ledger sources; provenance fields add to, never replace, economic/counterparty fields | LA §9 rows 1-2 (Question 3 read verbatim); the record-walk evidence | "The ledger is already the record"; answered: ownership, rejections, offchain events, fork meaning all absent |
| P-R2 | Hedged complete-set funding is not a contingent-position print; report exposure at the first unbalancing transaction | Same economics as P-D4; LA §2 | Hash-to-trade administrability; answered: false positives at funding, omissions at close |
| P-R3 | Corrections supersede, never overwrite | Part 45 lifecycle framework (LA §9 row 27) | Tape simplicity; answered: silent overwrite misleads |
| P-R4 | Public dissemination as an explicit versioned leakage policy; confidential record stays exact and timely regardless | 17 CFR part 43 delays/caps (LA §9 row 27); parts 45/49 | Transparency absolutism; answered: the law already prices this trade-off |
| P-R5 | A proof satisfies a reporting element only where the required proposition is exactly what its statement establishes; otherwise retain committed evidence | Analytical; within the request's Question 19 scope | Proof-as-complete-report |
| P-R6 | Publish reporting rules as governed executable validators over the three-record structure | Question 19's own ask (LA §9 row 2) | An opaque implementation silently becoming the law |

Kept as a genuine question: whether a Dark architecture can satisfy each
obligation without a general opening path (research boundary, tested
obligation by obligation).

### IAC written statement (Docket CFTC-2026-1717)

| ID | Position (short form) | Basis | Counterargument addressed |
|---|---|---|---|
| P-I1 | Adopt the milestone taxonomy (publication, funding, close, resolution, settlement) as the shared factual clock | LA §4 (advisory asks proper); LA §2 | Taxonomy rigidity; answered: milestones are events, not product categories |
| P-I2 | Publication without more is not operation; guidance should state which functional combinations cross the line | LA §5 (Letter 26-09 VERIFIED; functional triggers; FinCEN analogy context) | Two symmetric errors named and rejected |
| P-I3 | Exposure arises at funding; the published template creates none; prefunding fixes the ceiling in the same act | LA §2; V-38(d) fact support | Template-liability view; answered by the no-party, no-consideration analysis |
| P-I4 | Prefunded atomic settlement without novation/credit/mutualization performs no credit intermediation; clearing analysis turns on custody, settlement control, default handling | LA §5 (7 U.S.C. 1a(15) credit-substitution core); LA §4 (17 CFR 39.2 fully collateralized position) | Part 39 coexistence; answered: collateralization does not excuse an entity performing clearing functions --- the position is about when functions arise, not relief once they do |
| P-I5 | Real-time DCM fields are those surveillance/reconstruction obligations consume; the rest may stay encrypted if exact linked records are timely recoverable | LA §4 (part 38 obligations; 38.7 separation VERIFIED) | Plaintext-by-default; answered: the objective is access sufficiency |
| P-I6 | Governed threshold disclosure can satisfy access objectives; regulator-observable Shielded is the reference architecture | LA §4 (38.7); repository privacy-modes definitions | "Encrypted means unavailable"; Dark fenced as the open research question |
| P-I7 | Proofs admitted as evidence of exactly their encoded propositions, with negative cases and retained records; structured predeployment path | LA §4 (innovation channels); LA §5 (26-09 expires into rulemaking) | Proofs replacing surveillance/governance |

Kept as genuine questions: Dark structural-preclusion (research boundary);
the procedural vehicle for predeployment review (Commission's choice).

### IAC cover statement

C-1 through C-7 are the one-sentence forms of P-I1 through P-I7, each
naming what it settles. The scope paragraph and the Clear/Shielded/Dark
framing sentence are preserved from Draft 5 verbatim. This supersedes the
V-34 "seven questions" register description; V-34's identifier, note, and
warning-block requirements remain in force and are met.

## Factual-row survival (V-17 ... V-38)

Verified against the Draft 6 sources on 2026-08-18:

- **Wording survives verbatim or narrowed:** V-17 ("not a deployed system,
  a product, or an offer" restored verbatim in all three bodies; "tested,
  not formally verified" in all three), V-18, V-19 (scope disclaimer
  verbatim in definitions, IAC, cover), V-21, V-22 (walk compressed,
  factual content unchanged), V-23 (verbatim), V-24 (verbatim), V-26
  (parts 43/45/49 sentences verbatim), V-28, V-32, V-33 (all 11 IAC
  note_refs still referenced; identifiers and deadline unchanged), V-35
  ("onchain digital-asset price" retained in the P-D5 argument), V-36
  (table intro claims only to walk the example), V-37 (template untouched),
  V-38 (all retained sentences verbatim: the (b) batch-verifier, (c)
  accumulator, (d) structural-collateral, (e) batch-prototype wordings, and
  the (a)/(f) appendix "has been implemented offline with passing
  deterministic tests" / "was built to accept / refuse / replay" rows).
- **Claims deleted entirely (permitted; no ceiling violated):** the
  guarded-commitment and candidate-result expository sections (V-03/V-04/
  V-06/V-07-governed mechanics) are no longer load-bearing content anywhere;
  the definitions paired-examples table (including the V-35-noted
  "commodity price" hypothetical row); the definitions prose stage walk
  (folded into the stage table); the data separate-outputs table; the data
  governance failure-mode list; the IAC reference-architecture numbered list
  (folded to one sentence); the IAC between-milestones events paragraph;
  V-15's guarded-update reporting paragraph in its Draft 5 form (replaced by
  the narrower N-4 credential below); V-25's three-design enumeration
  (compressed to "three transcript designs"; all hedge sentences retained);
  V-30/V-31 model-mechanics prose (replaced by the narrower N-2/N-3
  credentials below). V-20/V-27 appendices remain accurate for the claims
  each filing still makes; rows for deleted claims were removed with them.
- **Register notes:** V-29's taxonomy is now advocated (P-I1) rather than
  offered --- an advocacy-register change, not a factual upgrade; the
  Draft 5 hedged sentence "Whatever this text is, it does not yet look like
  an agreement..." is superseded by position P-D2/P-D3 advocacy (see the
  John packet question 1 update).

## New factual claims and ceilings (Draft 6)

| ID | Claim as worded in Draft 6 | Ceiling |
|---|---|---|
| N-1 | "An order fixes who is acting, which balances may change, and the exact limits of the permitted fill; a fill outside those limits fails and changes nothing --- a property I have machine-checked in formal models of this order pattern" (definitions); "an order can be executed exactly as committed or not at all, a property I have machine-checked in formal models of the commitment pattern" and "a committed redemption executes at most once" / "no primitive allows an obligation whose amount or obligor is fixed later" (IAC) | Audience-language restatement of V-03/V-04 (accepted fill equals committed transition; guard violation fails closed; replay/nullifier refusal; no value- or authority-bearing hole). Must keep "formal models" / "machine-checked"; never "deployed controls," "cryptographically enforced," or a compliance property |
| N-2 | "Declaring an outcome before the evidence window closes is not caution but error: a declaration the remaining evidence can falsify, a failure mode I have exhibited concretely in a formal model" (IAC) | V-30 counterexample direction only; model-only; no oracle or legal finality validated |
| N-3 | "Two withdrawals can each be valid against the same pool and jointly overdraw it ... a fact I have machine-checked" (IAC) | V-31 counterexample direction only; model-only; supports serialization as design choice, not a deployed guarantee |
| N-4 | "In my formal models I have machine-checked that a correction's authorized actor, target record, permitted fields, and governing rule version can be fixed in advance and enforced mechanically" (data) | Narrowed V-15/V-04 restatement; never a reporting adapter, compliance implementation, or accepted schema |
| N-5 | "A small deterministic laboratory that replays four synthetic trading traces against three transcript designs and records which fields each design mechanically reveals and which deductions those fields enable" (data) | Compressed V-25/V-11; the hedge set ("measures no anonymity, cryptographic leakage, timing behavior, or real market") must travel with any use |

## Draft 6 artifacts (built 2026-08-18; Typst embeds a creation timestamp, so a rebuild changes the hash --- these identify the pre-typography build; see the dated status note below)

- `joint-definitions-comment-draft-6.pdf` --- 6 pages, SHA-256
  `dfdd8784db2559008586ceb85ddbbde926ad8633387d9e9a9245617b85dc0f00`
- `joint-data-reporting-comment-draft-6.pdf` --- 7 pages, SHA-256
  `c2a646c40cbcba0d97ca7f11a786caec4139128c3f6198ae00008b874103b78d`
- `cftc-iac-written-statement-draft-6.pdf` --- 6 pages, SHA-256
  `c6918fa545066d5c0aabe7c1ccc097a9491465f3d57a1d938b6ada9b9bc8edb0`
- `cftc-iac-cover-statement-draft-6.pdf` --- 1 page, SHA-256
  `63315caa69a6910af8abf081c2766d448b6ec04e8ee6cdfbbaa1043c66c7069f`

> Status 2026-08-19 (drift review): the typography commit 0c22ae7 rebuilt all
> four PDFs (STIX Two Text; the three long documents gained one page each), so
> the hashes and page counts above no longer identify the files at `output/pdf/`
> --- they identify the pre-typography build. Content was verified preserved as
> character multisets in that commit. The row is deliberately not re-pinned
> here while the typst lane is live; the filing-day freeze (gate 4) re-pins
> hashes once, at the frozen commit.

Body word counts (Typst source, `wc -w`), Draft 5 to Draft 6: definitions
3753 to 2695; data-reporting 4377 to 2968; IAC 4251 to 2708; cover 386 to
462 (positions carry more content than questions at the same length class).
Earlier draft PDFs are untouched. `scripts/build-regulatory-pdfs.sh` now
builds the draft-6 filenames; `scripts/check.sh` passes.
