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
  (Cross-repo note, not a filing defect: the dragons-clutch `README.md`
  Status section still reads "Verus is not yet installed or pinned," which
  its own `toolchain/PINNED_PROOF_TOOLS.md` contradicts; the filing wording
  is unaffected because it asserts only the tested-not-verified boundary.)
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
