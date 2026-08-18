# Draft 5 claim delta ledger

Status: rewrite-lane delta record, prepared 2026-08-18. Draft 5 is a
style-only rewrite of all four filings ("omit needless words"; plain,
unpuffed register; hedges stated once, exactly, in the right place). This
ledger lists every place where tightening changed the material meaning or
strength of a claim, and every limitation whose expression moved, so the
follow-up adversarial audit can verify quickly. Nothing here is filed; the
coordinator commits.

Governing ceilings, unchanged: `DRAFT3_CLAIM_AUDIT.md` (V-01..V-16 and the
downgrade table), `DRAFT4_CLAIM_AUDIT.md` (V-17..V-21 and the post-audit
addendum, fixes F-1..F-7), `DRAFT4_CLAIM_AUDIT_DATA.md` (V-22..V-28),
`DRAFT4_CLAIM_AUDIT_IAC.md` (V-22..V-27 of that ledger), and the
`SUBMISSION_WEEK_PLAN.md` §2 keep-outs.

## Word counts

| Document | body.typ words (D4 → D5) | Rendered PDF words (D4 → D5) | Pages (D4 → D5) |
|---|---|---|---|
| Definitions comment | 3,853 → 3,750 (−2.7%) | 4,426 → 4,321 (−2.4%) | 9 → 9 |
| Data-reporting comment | 4,476 → 4,366 (−2.5%) | 5,033 → 4,923 (−2.2%) | 10 → 10 |
| IAC written statement | 4,288 → 4,247 (−1.0%) | 4,991 → 4,935 (−1.1%) | 10 → 9 |
| IAC cover statement | 386 → 386 (0%) | 491 → 480 (−2.2%) | 1 → 1 |

Word counts by `wc -w` (body.typ includes Typst markup; rendered counts from
`pdftotext`, including headers, source notes, and warning blocks). The
reductions are small by design: Draft 4 was already a plain-register rewrite
that passed an adversarial audit, and most remaining sentences are
ceiling-worded claim text that must survive verbatim. The cuts are
meta-discourse, restatement, and intensifiers, not analytical substance.

## Definitions comment

Material meaning or strength changes:

1. **Criterion 4 (Security nexus): "commodity price" → "onchain digital-asset
   price."** This applies fix F-6 from
   `DRAFT4_DEFINITIONS_AUDIT_VERDICT.md`. The Draft 4 post-audit addendum in
   `DRAFT4_CLAIM_AUDIT.md` records F-6 as applied, but the committed Draft 4
   text (`git show HEAD`) still reads "commodity price"; the fix was never
   actually made. Draft 5 makes it: a legal characterization becomes a factual
   description, and the nexus contrast with a single issuer's security is
   kept. Direction: downgrade (removes an asserted legal label). The paired-
   examples table row "One application refers to a commodity price" is
   unchanged — the audit verdict expressly allows "commodity price" inside a
   hypothetical row.
2. **Stage-table intro: dropped "Any staged automated product can be located
   on it."** This was F-9 in the audit verdict (an unbacked universal
   generality claim, flagged as a nit). The sentence now claims only that the
   table generalizes the walk. Direction: downgrade.

Limitations whose expression moved: none. Verified in place, verbatim: the
F-1 "not necessarily" hedge (twice: Core distinction key point and
Conclusion), F-2 "can materially change," F-3 "locks collateral that only the
market's terms can move," F-4 "whatever instrument was created earlier," F-7
"This removes reporting discretion; it does not prevent trading from
influencing the underlying price," the F-5 sentence held for counsel
("Whatever this text is, it does not yet look like…"), the scope non-claim,
element 5 and request 5's conditional wording, the entire Limits section, and
all nine appendix rows. The V-17 prototype wording ("offline research
prototype… not a deployed system, a product, or an offer"; "tested, not
formally verified"; no program/keys/customers/funds) is verbatim.

Notable non-material cuts: the executive summary's design paragraph is
compressed into one sentence-per-term list (the full description follows in
"A worked example"); "Here is the example in full. I will refer back to it
throughout," "Walk the market from birth to settlement and watch what
changes," and "naming them makes the later criteria easier to state" deleted
as meta-discourse; intensifiers deleted ("genuinely," "narrow but
important" → "one distinction"). Both tables, the seven criteria, the six
requested work products, and the appendix rows are substantively unchanged.

## Data-reporting comment

Material meaning or strength changes: none.

Limitations whose expression moved: none. Verified in place, verbatim: "not
necessarily remove identity or strategy leakage"; the risk-analysis hedge ("I
offer these as risk analysis… not as measurements of any real market");
the leakage-lab denial list ("It measures no anonymity, no cryptographic
leakage, no timing behavior, no endpoints, and no real market"); the V-23
batch-verifier sentence ("only if recomputation from the frozen book
reproduces it exactly; it never trusts the submitter's claimed quantities");
the V-15 guarded-update paragraph with its "theorems about modeled state
machines… not a deployed reporting adapter" closer; the Clear/Shielded/Dark
definitions and "regulator-observable Shielded, not Dark"; "My own research
has not produced an end-to-end Dark system; its strongest composed paths
remain Shielded"; the proof-is-not-the-evidence paragraph; the full Limits
section (including "my own research included" and the classification
scope-out); the conclusion's four "should not mean" sentences; note_refs 1–4
used exactly once each; all twelve appendix rows.

Wording changes an auditor may want to sight (verified to carry no claim
content): "surprisingly little" → "little" (close-and-match milestone);
"One further small lesson" → "One further lesson" (the accumulator claim and
its appendix basis are unchanged); "and I state exactly what stands behind
them" dropped from the guarded-updates intro (the hedged statements it
announced are all retained); the funding milestone's "two things a report
needs them not to be confused with" setup deleted while both denials ("not an
identification… not a contingent position") are kept; the milestone-walk
closing recap compressed to one bridge sentence (the three records it
restated are defined in the executive summary and used throughout).

## IAC written statement

Material meaning or strength changes: none.

Limitations whose expression moved: none. Verified in place, verbatim: the
no-jurisdictional-shortcut paragraph; the scope non-claim; "These are
theorems about modeled state machines, not deployed controls"; the anti-model
sentence; "by construction of the terms" with the structural-check sentence;
the sealing/stability-premise sentences with "deliberately agnostic" and "The
collapse is worth exactly what the evidence licensing it is worth"; the
balance-guard non-gluing claim (counterexample direction only); the V-25
prototype wording ("tested, not formally verified"; "not a deployed system, a
product, or an offer, and I do not ask the Commission to approve it"); the
Clear/Shielded/Dark definitions; "Nothing in my research has produced a
deployed Dark venue, and I make no claim that a Dark architecture satisfies
current rules"; the frozen framing ("regulator-observable Shielded as the
practical reference architecture… Dark is retained solely as a long-horizon
research boundary — not a venue label, a compliance conclusion, or a
deployment recommendation"); "do not presently compose into a production,
permissionless, end-to-end Dark market system"; "Formal verification does not
prove legal compliance, and I do not claim that it does"; the IAC-advisory
paragraph; all eleven note_refs (1, 11 exec; 10, 11 scope; 2, 3, 4, 5, 6, 7,
8, 9 as in Draft 4); all twelve appendix rows.

Non-material cuts: "My central message is that…," "Rather than define the
milestones abstractly…," "rather than a habit," "rather than hypothetical,"
"possibly much later" → "at a later moment," "for everyone at exactly the
moment" → "exactly," "genuinely held" → "held," and the appendix intro's
first sentence. The milestone table, both key points, the seven questions,
the reference architecture, and every list survive substantively intact. The
statement now fits 9 pages (was 10).

## IAC cover statement

Body: wording unchanged (only the dash-encoding conversion described under
"Metadata, build, and mechanical checks"). Every sentence is either audited
framing (the V-01 negative,
the milestone-attachment clause, the scope non-claim, the frozen Dark
framing) or already minimal; the seven questions carry forward verbatim, per
the instruction to tighten them only where a word is genuinely needless. One
page, as required.

Template (renders on the cover page — the one directed substantive change):
the cover warning block's second sentence, "Public submission would
permanently link the named submitter to the research described here," is
replaced by "…review remain required before filing." This removes the
residual defensive framing around identity and the act of submitting, per the
user's direction; the [FULL NAME] placeholders, the identity/authority human
gate, and the not-filed warning itself are unchanged. The main `filing()`
review block is untouched.

## Metadata, build, and mechanical checks

- All four `metadata.typ` files: document_kind "… - DRAFT 5"; review_label
  "DRAFT 5 FOR REVIEW - NOT FILED". Identifier lines byte-identical to Draft
  4: Definitions CFTC RIN 3038-AF71 | SEC File S7-2026-21 | SEC RIN
  3235-AN79; Data CFTC RIN 3038-AF70 | SEC File S7-2026-22 | SEC RIN
  3235-AN78; IAC Docket CFTC-2026-1717 | Document CFTC-2026-1717-0001.
  `sources.typ` files untouched in all four (FR Doc. 2026-12743 and
  2026-12742 remain in entries 1; entry "Proposed, not current law" markers
  intact; final-source gate notes unchanged).
- `scripts/build-regulatory-pdfs.sh`: all four outputs now
  `*-draft-5.pdf`; pdfinfo checks kept; run 2026-08-18 (typst 0.15.0).
  Draft 4 and earlier PDFs untouched on disk (mtimes verified).
- Dash encoding: the Draft 4 bodies contained literal em-dash characters,
  which `scripts/check.sh` forbids in Typst source (the rule predates Draft
  4; the Draft 4 lanes broke it). Draft 5 encodes them as Typst `---`, which
  renders the identical em-dash glyph; page counts and rendered word counts
  are unchanged, and `./scripts/check.sh` now passes. Source-encoding only;
  no wording change.
- Draft 5 PDFs:
  - `joint-definitions-comment-draft-5.pdf` — 9 pages, SHA-256
    `27c890b43f46e8ea372c3310fe40f6542d4e1fbba4dbbe7742f769b6cff01896`
  - `joint-data-reporting-comment-draft-5.pdf` — 10 pages, SHA-256
    `1edb197a41ef07f20647735c5a115cf40834a468087949439af6937c9e52e508`
  - `cftc-iac-written-statement-draft-5.pdf` — 9 pages, SHA-256
    `4019cb1419247d7411a49f9d8498afb95d06eefc404dc76e9900d9953c3fa64e`
  - `cftc-iac-cover-statement-draft-5.pdf` — 1 page, SHA-256
    `62857b536e76b312d03843ae938d7b7eac000e138d8fc7215953dbf381aaba52`
- PDF text checks, 2026-08-18: "DRAFT 5 FOR REVIEW - NOT FILED" renders on
  every page of all four; zero `SOURCED:`/`VERIFIED`/`INFERRED:`/`PROPOSED:`
  label strings; `[FULL NAME]` placeholders present in metadata block and
  signature of each; all protected phrases listed above grep-verified in the
  rendered text; note_ref/source_entry integrity identical to Draft 4 (no
  orphans, no dangling refs).
- No filing, network submission, regulator contact, or commit was performed
  by this lane; the coordinator commits.
