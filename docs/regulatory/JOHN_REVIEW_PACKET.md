# Courtesy-review memo

Prepared 2026-08-18 for the final courtesy review. Everything below is
preprocessed: the in-house analysis (`LEGAL_ANALYSIS.md`, attached) records
the reasoning and primary-source verification behind each judgment, so each
row here should cost a glance and only Section 3 asks for your time. Thank
you — this is a favor and it is built to respect your hour.

## 1. What is being filed

I am filing three public comments as an independent researcher, in my own
name, with no client, product, offer, or deployed system involved. Two are
responses to joint CFTC/SEC requests for comment due **Monday, August 24,
2026** — one on product definitions (CFTC RIN 3038-AF71 / SEC File S7-2026-21,
answering Question 1 with proposed analytical criteria and no classification
position) and one on data reporting (RIN 3038-AF70 / S7-2026-22, answering
Questions 3, 8, and 19). The third is a written statement to the CFTC's
Innovation Advisory Committee, docket CFTC-2026-1717, due **Thursday, August
27, 2026** (hard electronic cutoff 11:59 p.m. ET; the docket does not accept
late comments).

## 2. Material legal judgments made in-house

Bases cite sections of the attached `LEGAL_ANALYSIS.md` (LA). Every legal
citation behind these rows was verified against the primary source on
2026-08-18; LA §9 is the per-citation ledger.

| Judgment | Basis | Confidence |
|---|---|---|
| Anyone may file these comments; no counsel or credential is required; filing pro se is not practicing law | LA §1 (5 U.S.C. 553(c); the notices' own invitations; 5 U.S.C. 1009(a)(3) for the IAC) | High |
| Comments are published permanently, unredacted, without agency review of PII/CBI; mitigation is what we omit, and nothing sensitive is in the drafts | LA §1, §6 R-6/R-7 (both notices' ADDRESSES terms, verified verbatim) | High |
| 18 U.S.C. 1001 applies to statements in these submissions; the maintained claim-audit ledger and hash-frozen final texts are the accuracy control; residual risk low | LA §1, §6 R-2 | High |
| The definitions comment actually holds its no-classification-position line; three near-the-line sentences reviewed, none crosses; one carried to you (Q1) | LA §2 (sentence-level review of the Draft 5 text) | High |
| The filings' legal recitals (swap definition, SBS prongs, mixed swaps, 2012 release, 40.11 + June 2026 proposal, facility definitions) are accurate as written | LA §2, §3, §5 (all recitals checked against fetched primary text) | High |
| The crypto-native-objective-events scope keeps the examples clear of the 40.11/CEA 5c(c)(5)(C) enumerated activities without claiming any exemption — the correct ceiling | LA §3 | High |
| The IAC statement is input to a solely-advisory FACA committee; it creates no status and requests only that the Committee recommend work | LA §4 (charter, 5 U.S.C. ch. 10, meeting notice) | High |
| Publication-vs-operation is genuinely unsettled (functional triggers; one nonbinding staff letter; FinCEN analogy), so the filings' request for guidance is well-founded — and the filings never claim publication is safe | LA §5 | High |
| Filing creates no meaningful exposure for the researcher: no solicitation/offer content, no operating activity described, prototype descriptions carry audited evidence ceilings, no CBI/PII | LA §6 risk register R-1..R-11 | High |
| Two defects found and routed: a source note misattributes CFTC Staff Letter 26-09 to the wrong division (fix queued before filing); the repo README still names a retained-counsel gate (being aligned) | LA §8 | High |

## 3. Questions for you (each ~1 minute; tentative answers included)

1. **One sentence.** The definitions comment says, of a published-but-unsigned
   template: "Whatever this text is, it does not yet look like an agreement,
   contract, or transaction, because there are no parties." I read this as a
   hedged observation, not a legal conclusion, and plan to keep it. Would an
   agency lawyer read it the same way, or should it be recast as a pure
   question? (LA §2 item 1; a one-clause fix is staged if you prefer it.)
2. **Signature block.** I plan to sign name + "independent researcher" + one
   durable email, omitting postal address and phone from the permanently
   public artifacts. Anything imprudent in that presentation?
3. **Routing.** The joint notices' "use only one method" language is stated
   per agency, so I read them as permitting one submission via the CFTC route
   *and* one via the SEC route for each joint comment (identical artifact,
   once per agency), which puts the comment in both records. Tentative plan:
   file both routes. Any reason to prefer single-route instead? (LA §1, §7.3.)
4. **Risk register.** Does anything in LA §6 strike you as wrong, mis-weighted,
   or missing — especially R-2 (resting the 1001 accuracy control on the
   maintained claim-audit ledger) and R-3/R-4 (that the filings cannot
   reasonably be read as an offer or as operating activity)?

A nod or a one-line reply per item is exactly the right amount of effort; if
any answer is "needs a conversation," say so and I will not treat silence as
sign-off.

## 4. Attachments

1. `joint-definitions-comment-draft-5.pdf` (9 pp.)
2. `joint-data-reporting-comment-draft-5.pdf` (10 pp.)
3. `cftc-iac-written-statement-draft-5.pdf` (9 pp.)
4. `cftc-iac-cover-statement-draft-5.pdf` (1 p.)
5. `LEGAL_ANALYSIS.md` (the in-house analysis this memo indexes)

Every material technical claim in the four PDFs carries a one-line
evidentiary basis in its own appendix and is governed by a maintained
evidence ledger in the repository (claim audits for Drafts 3-5, with SHA-256
hashes of the exact Draft 5 artifacts); you do not need to verify any
technical claim to review the legal posture. The drafts are watermarked
"DRAFT 5 FOR REVIEW - NOT FILED" and identity placeholders remain visibly
unresolved until the final identity gate.
