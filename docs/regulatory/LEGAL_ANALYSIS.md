# In-house legal analysis for the August 2026 regulatory packet

Status: **research work-product prepared without retained counsel; not legal
advice; prepared for final courtesy review by the user's designated reviewer.**
Prepared 2026-08-18. Nothing here authorizes a filing; the user approves and
submits per `SUBMISSION_WEEK_PLAN.md`.

Claim labels follow `AGENTS.md`: **VERIFIED** (directly inspected primary
source or artifact), **SOURCED** (cited source, not independently reproduced),
**INFERRED** (stated deduction from identified premises), **PROPOSED** (a
drafting or process choice). A label introducing a section or paragraph governs
its material claims until the next label.

Citation integrity: every legal citation in this document was verified by
fetching the primary source on **2026-08-18** (GPO/govinfo PDFs, the eCFR
versioner API at the 2026-08-14 issue date, uscode.house.gov, law.cornell.edu,
cftc.gov, fincen.gov, and the Regulations.gov v4 API). Section 9 is the
per-citation ledger, including the two items deliberately left unverified and
not relied on. Where a statute is verified through its verbatim quotation in a
fetched primary document (e.g., a Federal Register notice quoting the CEA),
the ledger says so explicitly.

The four filings analyzed are the Draft 5 bodies under
`docs/regulatory/typst/{definitions,data-reporting,iac,iac-cover}/body.typ`,
read in full for this analysis on 2026-08-18. Their technical-claim ceilings
are owned by `DRAFT3_CLAIM_AUDIT.md`, the three `DRAFT4_CLAIM_AUDIT*` ledgers,
and `DRAFT5_CLAIM_DELTA.md`; this document does not re-audit technical claims
and takes those ledgers as given.

---

## 1. The comment-filing act itself

**VERIFIED (5 U.S.C. 553(c), fetched 2026-08-18):** For notice-and-comment
rulemaking, "the agency shall give interested persons an opportunity to
participate in the rule making through submission of written data, views, or
arguments." Section 553 imposes no qualification, bar-membership, or counsel
requirement on commenters; 553(e) separately gives "interested persons" the
right to petition for issuance, amendment, or repeal of a rule. The two joint
requests are requests for comment (docketed as proposed-rule-stage documents),
and each states that "the Commissions are requesting comments from the public
on all aspects of these questions" (91 FR 37876). Anyone may file; an
independent researcher filing in their own name is squarely the invited
audience.

**VERIFIED (91 FR 37873-76 and 91 FR 37877 et seq., GPO PDFs fetched
2026-08-18):** Both joint notices carry identical publication terms, which are
the operative "consequences of filing":

- CFTC route: Regulations.gov, mail, or hand delivery/courier to the Secretary
  of the Commission; "Please submit your comments using only one of these
  methods"; Regulations.gov encouraged. English required (or translation).
- SEC route: internet comment form or email to rule-comments@sec.gov with the
  file number on the subject line, or paper to the SEC Secretary; "please use
  only one method of submission"; all comments posted on sec.gov.
- Public posting: "Comments (regardless of submission method) will be
  published without review for, and without removal of, any personal
  identifying information or information your business may consider
  confidential." The CFTC reserves the right to screen out inappropriate
  content, but a screened submission "will be retained in the record" and "may
  be accessible under the FOIA." Confidential-treatment requests go through 17
  CFR 145.9 (verified current: petition procedure for FOIA nondisclosure) and
  require contacting agency personnel *before* submission.

**INFERRED:** The one-method instruction is stated per agency: the CFTC
section restricts among the CFTC's three methods and the SEC section among the
SEC's three. The notice's structure therefore permits a commenter on a joint
request to file once through a CFTC method and once through an SEC method
without violating either instruction. Whether to use one agency's route or
both is a judgment call, carried to Section 7.

**VERIFIED (18 U.S.C. 1001(a), fetched 2026-08-18):** "Whoever, in any matter
within the jurisdiction of the executive, legislative, or judicial branch of
the Government of the United States, knowingly and willfully — (1) falsifies,
conceals, or covers up by any trick, scheme, or device a material fact;
(2) makes any materially false, fictitious, or fraudulent statement or
representation; or (3) makes or uses any false writing or document knowing the
same to contain any materially false, fictitious, or fraudulent statement or
entry" commits a felony (up to 5 years).

**INFERRED (application):** A comment letter or written statement submitted to
the CFTC and SEC is a statement in a matter within federal agency
jurisdiction. No oath is involved and the statute requires knowing and willful
falsity as to a material fact — good-faith analysis, labeled proposals, and
honest hedges are not exposure. But this is exactly why the repository's
claim-ceiling discipline matters and is not ceremony: every material technical
sentence in the four filings has an audited "allowed wording" ceiling
(`DRAFT3_CLAIM_AUDIT.md` V-01..V-16 and successors) and an in-document
evidentiary-basis appendix, so the filed text cannot honestly drift above what
the artifacts support. The single most important accuracy control at filing
time is mechanical: the filed PDFs must be byte-identical to the audited
Draft 5 texts (hashes recorded in `DRAFT5_CLAIM_DELTA.md`) or must be
re-audited.

**INFERRED (identity and affiliation):** The truthfulness obligation for
identity is minimal and fully satisfiable: sign as who you are, claim no
affiliation, credential, or representative capacity you do not have. The
drafts sign as "an independent software and formal-methods researcher" with
`[FULL NAME]` and `[AFFILIATION, IF ANY]` placeholders — accurate once
completed. Filing pro se is not the practice of law and requires no license.
Petitioning context: participation in an invited public-comment process is
classic petitioning activity under the First Amendment's petition clause
(text verified 2026-08-18); the operative protections here, though, are
simpler — the agencies asked, and 5 U.S.C. 553(c) obliges them to accept.

---

## 2. Definitions filing (RIN 3038-AF71 / FR Doc 2026-12743): exposure map

**VERIFIED (Draft 5 body read in full 2026-08-18):** The comment answers
Question 1 only, proposes analytical criteria, and holds the
no-classification-position line. Specifically: it walks a worked example
through stages and "asks which facts should decide its classification"; its
Limits section states "I do not assert that [the listed properties] removes an
arrangement from either Commission's jurisdiction. I do not offer a
classification of any deployed product, and I do not request permission to
deploy one"; the scope paragraph states "This scope choice is not a claim that
any example falls outside the CEA or any other law"; and the prototype is
described as "not a deployed system, a product, or an offer, and I do not ask
either Commission to approve it." No sentence asserts that any instrument is
or is not a swap, SBS, security, or excluded instrument.

**VERIFIED (line-level flag review):** Sentences nearest the line, none of
which crosses it:

1. "Whatever this text is, it does not yet look like an agreement, contract,
   or transaction, because there are no parties." (Authored-policy stage.)
   This is the closest approach to a legal conclusion in the packet. It is
   hedged as appearance ("whatever this text is," "does not yet look like")
   and the stage table immediately re-poses the point as a question. The
   Draft 4 audit already held this sentence for reviewer sign-off (F-5).
   Carried to Section 7 as John question 1.
2. "Individual claims are ordinary transferable assets." Descriptive of
   transferability, which criterion 5 then treats as a factual input; "assets"
   is generic, not a statutory category. Acceptable; no change needed.
3. "On its face this is performance of whatever instrument was created
   earlier, not the creation of a new one --- but a design in which settlement
   issued a new continuing claim would present a different question."
   Double-hedged and immediately relativized. Acceptable.

Everything else that sounds legal is either an accurate description of the
governing law (verified below) or an explicit question to the Commissions.

**Context paragraphs so the reviewer does not reconstruct them:**

**VERIFIED (7 U.S.C. 1a(47), uscode.house.gov fetched 2026-08-18; also quoted
verbatim at 91 FR 37874-75):** The CEA defines "swap" through six prongs;
the two that matter here are (A)(i), option-style instruments ("a put, call,
cap, floor, collar, or similar option of any kind ... based on the value of"
rates, currencies, commodities, indices, quantitative measures, or other
financial or economic interests), and (A)(ii), the event prong: "any purchase,
sale, payment, or delivery (other than a dividend on an equity security) that
is dependent on the occurrence, nonoccurrence, or the extent of the occurrence
of an event or contingency associated with a potential financial, economic, or
commercial consequence." Section 1a(47)(B) then excludes, among other things,
futures, security futures products, physically-settled security forwards,
options on securities subject to the Securities Act and Exchange Act, notes/
bonds/evidences of indebtedness that are securities, and SBS. A fully
collateralized categorical claim paying on a digital-asset price band is the
kind of structure the prongs were written to reach *if* an agreement,
contract, or transaction exists — which is precisely the staging question the
comment poses rather than answers. The comment's criteria 1-3 (binding
effect, consideration/funding, contingency/reference/payout) track the
statutory predicate ("agreement, contract, or transaction" plus the prong
tests) without asserting where the example lands.

**VERIFIED (77 FR 48208, GPO PDF first pages fetched 2026-08-18):** The 2012
joint Product Definitions Adopting Release — "Further Definition of 'Swap,'
'Security-Based Swap,' and 'Security-Based Swap Agreement'; Mixed Swaps;
Security-Based Swap Agreement Recordkeeping," 77 FR 48208 (Aug. 13, 2012),
joint final rule and interpretations under Dodd-Frank 712(d)(1) (codified at
15 U.S.C. 8302(d)(1), per 91 FR 35809 n.31) — is the further-definition
framework the 2026 request builds on; the 2026 request itself asks (Question
1) whether the Commissions should issue new or revised rules or
interpretations "[t]aking into account the rules and interpretations the
Commissions adopted in the Product Definitions Adopting Release." The
comment's "milestone-and-economic-function matrix inside the existing
statutory definitions and joint interpretations" is therefore an answer in
the form the request contemplates: interpretive guidance within the 2012
framework, not statutory change. Note a date discrepancy *inside the
agencies' own documents*: the 2026 definitions request's footnote 7 dates the
release "Aug. 12, 2012," while the GPO publication is Vol. 77, No. 156,
Monday, **August 13, 2012** (and 91 FR 35809 n.31 says Aug. 13). The
filings' source notes use August 13, which is correct.

**VERIFIED (15 U.S.C. 78c(a)(68)(A), uscode.house.gov fetched 2026-08-18; also
quoted at 91 FR 37875):** An SBS is a swap based on (I) a narrow-based
security index, (II) a single security or loan, or (III) "the occurrence,
nonoccurrence, or extent of the occurrence of an event relating to a single
issuer of a security or the issuers of securities in a narrow-based security
index, provided that such event directly affects the financial statements,
financial condition, or financial obligations of the issuer." **INFERRED:**
the worked example references an onchain digital-asset price and no security,
loan, issuer, or securities index, so none of the three SBS prongs is
implicated on its face; the comment's criterion 4 (security nexus) says
exactly this contrast — the identical program pointed at a single issuer's
security "would raise a different nexus" — without concluding the
classification of either variant. That is the correct posture: the nexus
facts, not the software, carry the jurisdictional weight.

**VERIFIED (mixed swaps, via 91 FR 37875 n.18 quoting 77 FR 48210 n.10 &
48291):** A mixed swap is a subset of SBS that is *also* based on
commodity-style underliers or non-issuer contingencies; the Commissions have
stated its scope "is, and is intended to be, narrow." **INFERRED:** the
worked example cannot be a mixed swap without first having a security nexus,
so the mixed-swap category enters only through the comment's paired-example
row that changes the reference object — again a question posed, not answered.

**INFERRED (net exposure assessment for this filing):** The filing's legal
risk is not classification error — it classifies nothing — but description
error about the law it recites. Every legal recital in the body was checked
against tonight's fetches: the Question 1 paraphrase, the 1a(47) framework,
the 3(a)(68) definition, the 2012-release role, the 40.11/proposal
description, and the CEA's separate facility definitions (7 U.S.C. 1a(15)
DCO, 1a(50) SEF referenced in the notice, 1a(51) trading facility; DCO
credit-substitution core verified at 1a(15)). All are accurate as written.
Residual exposure: low.

---

## 3. Event-contract boundary

**VERIFIED (7 U.S.C. 7a-2(c)(5)(C), law.cornell.edu fetched 2026-08-18):**
CEA 5c(c)(5)(C) is the special rule for event contracts: for agreements in
excluded commodities based on an occurrence or contingency, the Commission
may determine that contracts involving "activity that is unlawful under any
Federal or State law," "terrorism," "assassination," "war," "gaming," or
"other similar activity determined by the Commission ... to be contrary to
the public interest" may not be listed or made available for clearing or
trading "on or through a registered entity," with a 90-day review clock.

**VERIFIED (17 CFR 40.11, eCFR versioner API at 2026-08-14, fetched
2026-08-18):** Current 40.11(a) prohibits a *registered entity* from listing
for trading or accepting for clearing contracts "based upon an excluded
commodity, as defined in Section 1a(19)(iv) of the Act," that involve, relate
to, or reference terrorism, assassination, war, gaming, or unlawful activity
((a)(1)), or similar activity the Commission determines contrary to the
public interest ((a)(2)); 40.11(b) is reserved; 40.11(c) provides the 90-day
review mechanics.

**VERIFIED (91 FR 35806, GPO PDF fetched 2026-08-18):** The June 12, 2026
proposal ("Prediction Markets; Public Interest Determinations," RIN
3038-AF65, FR Doc 2026-11854; comments closed July 27, 2026) would amend part
40: factors for the public-interest determination, a definition of "gaming,"
a rule that contracts "involve" an activity "if their settlement is
determined by an occurrence, extent of an occurrence, or contingency in the
activity," and a new appendix F to part 40. It is proposed, not current law
— which is exactly how all three filings describe it.

**VERIFIED (7 U.S.C. 1a(19)(iv), fetched 2026-08-18):** The excluded-commodity
category that 40.11 keys on is "an occurrence, extent of an occurrence, or
contingency (other than a change in the price, rate, value, or level of a
commodity ...)" beyond the parties' control with financial, commercial, or
economic consequence.

**INFERRED (why the filings' scope choice works):** The examples reference
ledger states, program events, prices, price ranges, and path statistics of
digital assets. Two independent layers of distance follow. First, none of the
enumerated activities (terrorism, assassination, war, gaming, unlawful
activity) is involved, related to, or referenced, under the current rule's
words or the proposal's settlement-determined "involve" test — the examples
would not trip 40.11 even if listed by a registered entity, and nothing is
being listed. Second, a price-band market's underlying is precisely "a change
in the price, rate, value, or level of a commodity," which 1a(19)(iv)
carves *out* of the occurrence/contingency category — so the examples sit in
ordinary commodity-derivatives territory rather than in the event-contract
special rule at all. The filings assert only the first layer and expressly
decline to claim any exemption ("This scope choice is not a claim that any
example falls outside the CEA or any other law") — the correct ceiling,
because the second layer is an interpretive deduction, the "prediction
markets" policy fight is active, and 40.11 addresses registered entities, not
commenters. Nothing in any filing asks the Commission to bless an event
contract, list anything, or apply 40.11 to the researcher.

---

## 4. IAC written statement (Docket CFTC-2026-1717)

**VERIFIED (5 U.S.C. ch. 10, law.cornell.edu fetched 2026-08-18; IAC Charter
PDF, cftc.gov, fetched 2026-08-18):** The Federal Advisory Committee Act (as
recodified by Pub. L. 117-286 at 5 U.S.C. 1001-1014) governs. 5 U.S.C.
1009(a)(2) requires Federal Register notice of meetings — and is the exact
authority the meeting notice invokes ("Authority: 5 U.S.C. 1009(a)(2)").
1009(a)(3): "Interested persons shall be permitted to attend, appear before,
or file statements with any advisory committee," subject to reasonable rules.
1009(b)-(c) make committee records and minutes public. The amended IAC
Charter (March 3, 2026) states, "In accordance with 5 U.S.C. § 1008(b), the
duties of the IAC shall be solely advisory."

**VERIFIED (91 FR 51697, FR Doc 2026-16328, GPO PDF fetched 2026-08-18; live
docket via Regulations.gov API fetched 2026-08-18):** The meeting is August
20, 2026, 1:00-4:00 p.m. EDT, virtual for the public; written statements "in
connection with the meeting" are due **Thursday, August 27, 2026**, referencing
"Innovation Advisory Committee," by one method (Regulations.gov encouraged),
in English, with the same publish-without-review terms as the joint notices.
The live docket **CFTC-2026-1717** / document **CFTC-2026-1717-0001** confirms
FR doc 2026-16328, 91 FR 51697, a comment window of 2026-08-11 through
2026-08-27 11:59 p.m. ET (API `commentEndDate` 2026-08-28T03:59:59Z), and
`allowLateComments: false` — do not plan on grace. The CFTC event page
(fetched 2026-08-18) posts no more specific agenda; the notice's topic list
(crypto assets, AI, prediction markets, recent CFTC activity) remains the
best statement of scope.

**INFERRED (what a written statement is and is not):** It is public input
into an advisory committee's record — published, FOIA-reachable, quotable. It
is not a comment in a rulemaking (no agency response obligation attaches), not
a petition under 5 U.S.C. 553(e), not a request for relief, and it creates no
status for the submitter. The Draft 5 statement says this itself ("The IAC is
advisory. A written statement or a meeting appearance is not Commission
approval of anything, and I do not treat it as one") — verified accurate
against the charter. The asks (a milestone taxonomy, audit-trail criteria,
proof objectives, functional guidance, a predeployment path) are requests
that the Committee *recommend work*, which is the one thing an advisory
committee can properly do.

**VERIFIED (statement's legal recitals checked):** The Commission's
descriptions the statement cites are real and accurately characterized: the
March 16, 2026 ANPRM ("Prediction Markets," 91 FR 12516; comment period
closed) states "the term 'event contract' is not a defined term in the CEA or
the CFTC's regulations," that a prediction market offering swaps or futures
to the general public "must register with the CFTC as a designated contract
market," and that SEF trading is limited to eligible contract participants
(all three quotes fetched from the CFTC's posting 2026-08-18; the underlying
statutes — CEA 2(e), 7 U.S.C. 2(e); CEA 5h, 7 U.S.C. 7b-3; ECP, 7 U.S.C.
1a(18) — verified as quoted at 91 FR 35809 nn.32-33). 17 CFR 38.7's
regulatory-vs-business data separation: text fetched and accurate. Part 39's
"fully collateralized position" concept: definition fetched at 17 CFR 39.2
("funds ... sufficient to cover the maximum possible loss that a party or
counterparty could incur upon liquidation or expiration"), which incidentally
is the regulatory vocabulary closest to the design's Hoard invariant — a
useful talking point, not a compliance claim.

---

## 5. Publication vs. operation: the honest landscape

The filings ask the agencies to distinguish publishing software from
operating, soliciting, and intermediating. The current landscape justifies
the ask precisely because the answer is unsettled: the registration triggers
are functional and broad, and the closest authorities are a nonbinding staff
letter and an analogy from a different statute.

**VERIFIED (registration triggers, all fetched or verified-as-quoted
2026-08-18):**

- *Venue:* futures and retail swaps must trade on a designated contract
  market (CEA 5, 7 U.S.C. 7; CEA 2(e), 7 U.S.C. 2(e) — retail swap
  transactions "not between eligible contract participants" must be on a DCM);
  SEFs (CEA 5h, 7 U.S.C. 7b-3) serve ECPs. "Trading facility" and "swap
  execution facility" are defined functionally in 7 U.S.C. 1a.
- *Intermediary:* an FCM is defined by "soliciting or in accepting orders"
  (7 U.S.C. 1a(28)); an introducing broker likewise solicits or accepts
  orders without holding customer funds (7 U.S.C. 1a(31); CEA 4d(g), 7 U.S.C.
  6d(g) makes unregistered IB activity unlawful, as recited in Staff Letter
  26-09). The CFTC "has long construed the terms 'soliciting and accepting'
  orders to cover a wide range of activities" (Letter 26-09, quoting its own
  precedent).
- *Clearing:* a DCO is defined by credit substitution, netting, or otherwise
  mutualizing/allocating counterparty risk (7 U.S.C. 1a(15)); part 39 imposes
  substantial institutional requirements even where positions are fully
  collateralized (17 CFR 39.2, 39.11 context).

**VERIFIED (CFTC Staff Letter No. 26-09, cftc.gov PDF fetched 2026-08-18):**
March 17, 2026, **Market Participants Division** (Thomas J. Smith, Acting
Director), a no-action position under 17 CFR 140.99: the Division will not
recommend enforcement against Phantom Technologies for unregistered IB/AP
activity where Phantom passively provides front-end interface software
(mobile/browser) through which users transmit orders directly to
"Collaborators" (a registered DCM, FCMs, or IBs), subject to conditions —
no affirmative involvement with particular orders, no buy/sell signals, no
discretion over routing or execution, disclosure and risk-statement
conditions, no statutory disqualification — and expressly "until the
effective date of a Commission rulemaking or guidance addressing the
application of the IB registration requirement to software providers." The
letter states it "binds only the issuing Division ... and not the Commission"
(quoting 17 CFR 140.99(a)(2)). Two consequences: (1) the filings' description
("frontend facts can matter, in a narrow, conditional, nonbinding staff
analysis") is accurate; (2) the letter itself anticipates the rulemaking the
filings ask for. **Flag for the coordinator (typst lane owns the file):**
`typst/iac/sources.typ` source note 3 attributes the letter to the "Division
of Market Oversight." The letter is from the **Market Participants Division**.
This must be corrected before filing; the body text does not name a division,
so only the source note is affected.

**VERIFIED (FinCEN guidance FIN-2019-G001, May 9, 2019, fincen.gov PDF
fetched 2026-08-18):** Under the BSA's money-transmission rules — a different
statute, useful only as analogous reasoning — FinCEN distinguishes roles, not
technology: "An anonymizing software provider is not a money transmitter,"
because "suppliers of tools (communications, hardware, or software) that may
be utilized in money transmission ... are engaged in trade and not money
transmission" (citing the exemption at 31 CFR 1010.100(ff)(5)(ii) for
"delivery, communication, or network access services"). But an anonymizing
*services* provider — who accepts and retransmits value — is a money
transmitter; and "when DApps perform money transmission, the definition of
money transmitter will apply to the DApp, the owners/operators of the DApp,
or both." The guidance is interpretive, "does not establish any new
regulatory expectations," and is FinCEN's, not the CFTC's or SEC's.

**INFERRED (the honest bottom line, which the filings already state):**
Publication is not a safe harbor and the filings never say it is. The
functional triggers turn on control, solicitation, order handling, custody,
compensation, and operation — the exact fact list the IAC statement offers.
What the landscape lacks is a Commission-level articulation of *which*
combinations cross the line; the only on-point authority is one fact-bound
staff letter that binds no one but its Division and expires into the very
rulemaking the filings request. Asking for that guidance is therefore
well-founded, and asking is all the filings do. Nothing in this analysis, and
nothing in the filings, concludes that publishing this research (or any
software) is free of registration consequences.

---

## 6. Risk register for the project itself

**INFERRED unless marked otherwise.** Assessment scale: negligible / low /
moderate. "Basis" cites the sections above and the verified filing texts.

| # | Risk | Assessment | Basis | Confidence |
|---|---|---|---|---|
| R-1 | Filing itself creates regulatory status or obligations for the researcher | Negligible | Commenting is invited participation (5 U.S.C. 553(c); 91 FR 37876; 5 U.S.C. 1009(a)(3)); it creates no registration, no license, no continuing duty; the IAC is solely advisory (charter, 5 U.S.C. 1008(b)). §1, §4 | High |
| R-2 | False-statement exposure (18 U.S.C. 1001) from technical claims | Low | Statute requires knowing and willful material falsity (§1). Every material technical claim carries an audited ceiling and an in-filing basis appendix (VERIFIED: appendix tables present in all three bodies); the Draft 5 hashes freeze the audited text. Residual risk is drift between audited text and filed artifact — controlled by the hash gate. | High |
| R-3 | Filing read as solicitation, offer, or marketing of a product | Low | VERIFIED: each body expressly states the prototype "is not a deployed system, a product, or an offer"; no terms are offered to anyone, no venue exists, no compensation, no customers, no funds ("no deployed program, no keys, no customers, and no funds"). The worked example is framed as a lens for classification/reporting questions. §2 | High |
| R-4 | Filing read as admission of operating an unregistered venue/intermediary | Low | VERIFIED: bodies state no deployment, no live orders, no accepted funds, "does not presently compose into a production ... system"; activity described is authoring specs, models, offline prototypes — none of which matches the functional triggers in §5 (no orders, no users, no custody, no compensation). | High |
| R-5 | Event-contract/prediction-market controversy attaches to the researcher | Negligible-to-low | §3: examples avoid all enumerated activities; no listing or clearing is proposed; no exemption is claimed; 40.11 governs registered entities. The filings' restraint here is verified sentence-level. | High |
| R-6 | Permanent public posting of identity/PII | Accepted, bounded | VERIFIED: both notices publish "without review ... without removal of" PII; mitigation is procedural — sign with name and a durable public contact only, no address/phone beyond what the submitter wants permanent, no third-party personal data (none present in Draft 5). Identity-gate checklist already in `SUBMISSION_WEEK_PLAN.md` §4. | High |
| R-7 | CBI / secret / vulnerability leakage | Low | VERIFIED: Draft 5 bodies contain no keys, no addresses, no counterparties, no unpublished vulnerability details; sources notes state no source code included; disclosure-scrub gate remains in the plan. Residual: the "cite frozen public commits" option in sources.typ should be exercised only if the sibling repos are actually public at filing time (coordinator decision, already conditioned in the sources notes). | High |
| R-8 | Misstatement of law in the filings (accuracy risk distinct from R-2) | Low | Every legal recital in the three bodies checked against primary sources fetched 2026-08-18 (§2-§5). One defect found and flagged: the Letter 26-09 division misattribution in `iac/sources.typ` (§5). One agency-side oddity noted: FR Doc 2026-12743 n.7 misdates the 2012 release (§2); the filings use the correct date. | High |
| R-9 | Copyright in quoted material | Negligible | Filings quote U.S. statutes, regulations, and FR text (U.S. government works, not copyrightable) and the researcher's own writing; no third-party expressive content. The repository's own license/provenance gates (DRAFT3 ledger, human gate 4) remain for figures/fixtures if any are added. | High |
| R-10 | AML/sanctions/money-transmission exposure from the filings | Negligible | No value is transmitted, offered, or held; §5's FinCEN analysis is context in the *analysis*, not activity. Filing a comment implicates no BSA obligation. | High |
| R-11 | The packet's "without retained counsel" posture itself | Accepted | Commenting requires no counsel (§1). The reviewer relationship is a courtesy review, not an engagement: this document is labeled research work-product and not legal advice, and nothing in the packet represents that counsel has been retained or has approved the filings. `SUBMISSION_WEEK_PLAN.md` gates now say exactly this. | High |

---

## 7. Open judgment calls for the reviewer

**PROPOSED:** Kept deliberately short; each is answerable in a minute or two,
and each states the in-house tentative answer so a nod suffices.

1. **The authored-policy sentence.** "Whatever this text is, it does not yet
   look like an agreement, contract, or transaction, because there are no
   parties" (definitions body, stage walk). Tentative answer: keep — it is
   appearance-hedged, and the table converts it into a question. John's
   judgment matters because he can read it the way an agency lawyer would:
   if it scans as a conclusion of law rather than an observation, the fix is
   a one-clause recast ("no one is yet bound and there are no parties —
   whether anything cognizable exists at this stage is the question"), which
   the typst lane can apply in minutes.
2. **Signature block hygiene.** Tentative answer: sign name + "independent
   researcher" + one durable email; omit postal address and phone from the
   public artifacts (mail methods don't require a return address inside the
   comment text). John's judgment: is anything about this identity
   presentation imprudent, given permanent unredacted posting?
3. **One route or two for the joint comments.** The one-method instruction is
   per-agency (§1), so filing each joint comment once via CFTC
   (Regulations.gov, RIN-referenced) *and* once via SEC (internet form,
   File-No.-referenced) appears permitted and puts the comment in both
   agencies' records; the notices are silent on cross-posting. Tentative
   answer: file both routes, once each, identical artifact. John's judgment:
   any reason this double-presence is unwise or should be single-route CFTC?
4. **The risk register.** Does anything in Section 6 strike him as wrong,
   mis-weighted, or missing — in particular R-2's reliance on the maintained
   claim-audit ledger as the 18 U.S.C. 1001 accuracy control, and R-3/R-4's
   reading that the filings cannot reasonably be read as an offer or as
   operating activity?

Everything else this analysis touched was resolved in-house with recorded
reasoning and is presented above for a glance, not a question.

---

## 8. Items flagged to the coordinator (not John questions)

**VERIFIED findings requiring action by the lanes that own the files:**

1. `typst/iac/sources.typ` note 3: change "Division of Market Oversight" to
   "Market Participants Division" for Staff Letter 26-09 (§5). Body text
   unaffected.
2. `README.md` "Before any filing" item 1 still carries the retained-counsel
   gate ("Have U.S. commodities counsel review..."). `SUBMISSION_WEEK_PLAN.md`
   now states the actual process; README should be aligned by its owner.
3. The IAC docket does not allow late comments (`allowLateComments: false`,
   Regulations.gov API 2026-08-18); the effective electronic cutoff is
   Aug. 27, 11:59 p.m. ET. Plan buffer accordingly.
4. Date nit available if ever useful: FR Doc 2026-12743 n.7 misdates the 2012
   Product Definitions release as Aug. 12, 2012 (GPO: Aug. 13, 2012). No
   action needed; the filings are already correct.

---

## 9. Citation ledger

Every citation used in this analysis, its evidence path, and its retrieval
date. "Fetched" = primary source retrieved and read 2026-08-18 in this
session. Verified count: **28 fetched-primary items**; **2 items deliberately
not verified and not relied on**.

| # | Authority | Evidence path (2026-08-18) |
|---|---|---|
| 1 | 91 FR 37873 (June 24, 2026), FR Doc 2026-12743, RIN 3038-AF71, SEC File S7-2026-21, SEC RIN 3235-AN79, Release 33-11424/34-105735 (joint definitions request; deadline Aug. 24, 2026) | Fetched: GPO PDF, govinfo FR-2026-06-24, full text extracted |
| 2 | 91 FR 37877 (June 24, 2026), FR Doc 2026-12742, RIN 3038-AF70, SEC File S7-2026-22, SEC RIN 3235-AN78, Release 34-105734 (joint data request; deadline Aug. 24, 2026; Questions 3, 8, 19 read verbatim) | Fetched: GPO PDF, govinfo FR-2026-06-24, full text extracted |
| 3 | 91 FR 51697 (Aug. 11, 2026), FR Doc 2026-16328 (IAC meeting notice; statements due Aug. 27, 2026; authority 5 U.S.C. 1009(a)(2)) | Fetched: GPO PDF, govinfo FR-2026-08-11, full text extracted |
| 4 | Docket CFTC-2026-1717; Document CFTC-2026-1717-0001 (window 2026-08-11 to 2026-08-28T03:59:59Z; no late comments) | Fetched: Regulations.gov API v4, dockets + documents endpoints |
| 5 | 17 CFR 40.11 (current) | Fetched: eCFR versioner API, title 17 @ 2026-08-14, full section text |
| 6 | 91 FR 35806 (June 12, 2026), FR Doc 2026-11854, RIN 3038-AF65 (Prediction Markets NPRM: factors, "involve" rule, gaming definition, appendix F; proposed, not current law) | Fetched: GPO PDF, govinfo FR-2026-06-12, full text extracted |
| 7 | 91 FR 12516 (Mar. 16, 2026) Prediction Markets ANPRM (event-contract-not-defined; DCM/general-public; SEF/ECP statements; comment period closed) | Fetched: cftc.gov posting of the ANPRM, quotes extracted |
| 8 | 7 U.S.C. 1a(47)(A)(i)-(ii), (47)(B) exclusions | Fetched: uscode.house.gov current preliminary edition; cross-checked against verbatim quotes at 91 FR 37874-75 |
| 9 | 7 U.S.C. 1a(19)(iv) (excluded commodity, occurrence prong) | Fetched: uscode.house.gov; also law.cornell.edu |
| 10 | 7 U.S.C. 1a(15) (DCO, credit-substitution core), 1a(18) (ECP), 1a(28) (FCM, soliciting/accepting orders), 1a(31) (IB), 1a(35) (NBSI) | Fetched: law.cornell.edu 7 U.S.C. 1a (page truncates after (35); (47) covered by row 8's sources) |
| 11 | 15 U.S.C. 78c(a)(68)(A) (SBS definition, three prongs incl. event prong) | Fetched: uscode.house.gov; cross-checked against 91 FR 37875 |
| 12 | 77 FR 48208 (Aug. 13, 2012) Product Definitions Adopting Release (Vol. 77, No. 156; Release 33-9338/34-67453; joint final rule and interpretations) | Fetched: GPO PDF, govinfo FR-2012-08-13, first pages (title/date/agencies/action verified) |
| 13 | Mixed-swap definition and intended-narrow-scope statements, 77 FR 48210 n.10 & 48291 | Verified as quoted at 91 FR 37875 n.18 (row 1's fetched PDF) |
| 14 | 5 U.S.C. 553(c), (e) | Fetched: law.cornell.edu, quoted |
| 15 | 18 U.S.C. 1001(a)-(c) | Fetched: law.cornell.edu, quoted |
| 16 | 7 U.S.C. 7a-2(c)(5)(C) (CEA 5c(c)(5)(C) event-contract special rule) | Fetched: law.cornell.edu, quoted |
| 17 | 5 U.S.C. 1009(a)(2), (a)(3), (b), (c); ch. 10 recodification (Pub. L. 117-286) | Fetched: law.cornell.edu |
| 18 | 5 U.S.C. 1008(b) (advisory-only duties) | Verified as cited in the fetched IAC Charter (row 19) |
| 19 | CFTC Innovation Advisory Committee Charter, amended Mar. 3, 2026 ("solely advisory"; FACA 5 U.S.C. 1001-1014) | Fetched: cftc.gov PDF |
| 20 | CFTC Staff Letter No. 26-09 (Mar. 17, 2026, Market Participants Division, no-action re IB (CEA 4d(g)) and AP (CEA 4k) registration; conditions; nonbinding per 17 CFR 140.99(a)(2)) | Fetched: cftc.gov PDF, full text extracted |
| 21 | CEA 4d(g) (7 U.S.C. 6d(g)), CEA 4k (7 U.S.C. 6k), 17 CFR 1.3 (IB/AP definitions), 17 CFR 140.99 | Verified as quoted/cited in Letter 26-09 (row 20's fetched PDF) |
| 22 | CEA 2(e) (7 U.S.C. 2(e)), CEA 5 (7 U.S.C. 7), CEA 5h (7 U.S.C. 7b-3), Dodd-Frank 712(d)(1) (15 U.S.C. 8302(d)(1)) | Verified as quoted at 91 FR 35809 nn.29-33 (row 6's fetched PDF) |
| 23 | FinCEN FIN-2019-G001 (May 9, 2019) CVC guidance (anonymizing software provider not a money transmitter; services provider is; DApps; interpretive only) | Fetched: fincen.gov PDF, full text extracted, quoted |
| 24 | 31 CFR 1010.100(ff)(5)(ii) (tools/delivery/communication exemption) | Verified as quoted in FIN-2019-G001 (row 23's fetched PDF) |
| 25 | 17 CFR 38.7; part 38 heading | Fetched: eCFR versioner API, full section text |
| 26 | 17 CFR 39.2 ("fully collateralized position" definition); part 39 heading | Fetched: eCFR versioner API |
| 27 | 17 CFR 43.4(e)-(g) (rounding, cap size), 43.5 (block-trade time delays); parts 43/45/49/145 headings; 17 CFR 145.9 (confidential treatment) | Fetched: eCFR versioner API |
| 28 | U.S. Const. amend. I (petition clause); CFTC event page (no agenda posted); CFTC "Innovation at the CFTC" page (IAC + Innovation Task Force channels) | Fetched: law.cornell.edu; cftc.gov (2 pages) |

**UNVERIFIED — flagged, not relied on:**

| Authority | Status |
|---|---|
| SEC-CFTC Memorandum of Understanding (Mar. 11, 2026), sec.gov/files/mou-sec-cftc-2026.pdf | Cited in both joint notices; not fetched this session. No conclusion in this analysis or in the filings rests on it. |
| KalshiEX LLC v. CFTC, 172 F.4th 226 (D.C. Cir.) | Appears only as quoted at 91 FR 35809 n.29. The case itself was not fetched; it is not cited independently anywhere in the packet and no conclusion rests on it. Do not add it to any filing without fetching and reading it. |

Federalregister.gov and ecfr.gov HTML pages were bot-gated during this
session; all Federal Register verification therefore used the official GPO
PDFs on govinfo.gov and the eCFR versioner API, which are the authoritative
sources in any event. The filings' own hyperlinks to federalregister.gov
remain correct for human readers.
