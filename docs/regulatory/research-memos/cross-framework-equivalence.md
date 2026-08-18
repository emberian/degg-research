# Definitions Questions 12-14 and Data Question 4 — equivalence is a claim about rules, not about outcomes

Internal research memo. See [README.md](README.md) for status and citation rules.

## The questions, verbatim

From the definitions notice, 91 Fed. Reg. 37873, 37876-77 (June 24, 2026):

> 12. Where trading in economically related or functionally similar product
> classes implicates both SEC and CFTC regulatory interests, are there
> circumstances in which compliance with one Commission's regulatory framework
> could appropriately satisfy substantially similar requirements of the other
> Commission (alternative compliance)? In this case, how should "substantially
> similar" be viewed? Should it contemplate scope, objectives and/or outcomes
> of requirements? Supervisory compliance programs? Enforcement authority?
> Other considerations/standards?

> 13. Title VII provides that the Commissions may adopt rules to further define
> terms included in Title VII, but it also limits the exemptive authority of
> each Commission over certain provisions related to swaps and SBS. In light of
> these provisions, under what circumstances should the Commissions
> consider/pursue joint or coordinated notice registration, tailored rules,
> rules of procedure, tailored trade reporting rules, deemed filing, or other
> joint or coordinated approaches to facilitate alternative compliance?

> 14. What considerations should guide surveillance, examination, and
> enforcement under an alternative compliance approach? How could enhanced
> sharing of information and data help fulfil the Commissions' regulatory
> mandates under an alternative compliance approach? How could the Commissions
> more effectively coordinate to examine and enforce their regulatory
> requirements?

From the data notice, 91 Fed. Reg. 37877, 37879-80 (June 24, 2026):

> 4. Given the scheduled expiration of the 2019 Compliance Statement in 2029,
> should the SEC consider amendments to its SBS Reporting Rules to more fully
> harmonize those rules with the CFTC's swap reporting rules?

> a. If so, what should be the scope of such amendments? Should the SEC's and
> CFTC's rules be identical? . . . Should the SEC adopt a technical
> specification for reporting that is identical to the CFTC's technical
> specification? Are there areas (e.g., asset classes, product types, underlier
> reporting requirements) where any technical specifications that the SEC may
> adopt should differ from the CFTC's technical specifications? If the SEC makes
> changes to more fully harmonize the SEC's SBS Reporting Rules with the CFTC's
> swap reporting rules, what measures should the Commissions undertake to ensure
> that reporting rules remain harmonized over time?

## Positions

1. **"Substantially similar" should be asserted obligation by obligation, not
   framework by framework.** The unit that can be compared is a requirement
   together with the evidence that discharges it. A determination at the level
   of an entire framework cannot be audited, because nobody can say which
   sentence of framework A is doing the work of which sentence of framework B.
2. **Agreement on observed outcomes is not equivalence.** Two rules that produce
   the same result on every case anyone has examined can differ on cases nobody
   has examined, and the cases nobody examined are where a compliance regime
   fails. The way to establish equivalence is to compare the rules, and that
   comparison is mechanical only where each rule is written as a condition over
   named fields.
3. **The durable form of an equivalence determination is a crosswalk with
   conformance cases.** For each obligation in framework A: the obligation in
   framework B said to satisfy it; the records and fields that discharge it; and
   a set of test cases — including cases the two frameworks are known to treat
   differently — that a supervisor or a firm can run to confirm the mapping still
   holds. A determination without conformance cases is an assertion; with them
   it is a control.
4. **Every equivalence determination should be versioned, dated, and paired
   with a re-examination trigger.** When either framework's rule changes, the
   determination is stale until re-made. Harmonization achieved once and left
   unmanaged drifts, and the drift is invisible precisely because both sides
   believe the question was settled.
5. **On Question 4a: identical technical specifications are the cheapest way to
   stay harmonized,** because a shared specification makes divergence a visible
   conflict at the moment of change rather than a discovery years later. Where
   the specifications must differ, the difference should be an explicit,
   enumerated exception list carried in the specification itself, not an
   unstated delta that each firm reconstructs on its own.
6. **On Questions 13 and 14: sharing data across two agencies is easy when both
   hold records under one normalized event model and hard when they hold two
   dialects.** "Deemed filing" and coordinated reporting are attractive for the
   same reason: they are cheap when the two records mean the same thing field by
   field, and they are a translation project otherwise. The prerequisite for the
   coordination the questions contemplate is a shared semantic model, not a
   shared pipe.
7. **A related point on the data notice's Question 14 (separate dissemination by
   each repository):** fragmented public dissemination is a much smaller problem
   when every repository publishes under one event model with one set of field
   meanings, because aggregation then becomes mechanical rather than a
   reconciliation exercise per repository.

## Argument

The methodological claim in position 2 is the load-bearing one and it is not
specific to securities law. When two specifications are said to be equivalent,
the claim can be made at two levels: that they agree on the sample of behaviors
someone tested, or that they agree as rules. The first is cheap and does not
support the conclusion people draw from it. The second is more work but is the
only version that supports reliance, because a firm relying on alternative
compliance is relying on the untested cases just as much as the tested ones.
Writing a requirement as a condition over named fields is what makes the second
comparison possible, which is why this memo's recommendation is continuous with
the machine-readable-rules position the program has already filed on the data
notice's Question 19: the same governance apparatus — versioned schemas,
deterministic predicates, conformance vectors, retained historical versions,
human-readable parity mapping each provision to its data elements — is what an
equivalence determination needs in order to be re-checkable.

Position 4 is the one most likely to be omitted, because it is about the period
after the interesting work is done. The data notice supplies its own example of
the risk: reporting has been broadly harmonized in practice through universal
reliance on a 2019 compliance statement that is scheduled to expire, and the
notice now asks what should replace it. An equivalence determination with no
version, no date, and no trigger reproduces that situation on a longer clock.

The reporting frameworks the questions concern are 17 C.F.R. parts 43, 45, and
49 on the CFTC side and Regulation SBSR and the SBSDR rules on the SEC side, as
the notice describes them. Nothing in this memo turns on the content of any
particular rule in those parts.

*Basis for statements about the submitter's artifacts in this memo:* this memo
makes no factual claim about the program's artifacts. The conformance-vector
discipline it recommends is a practice the program applies to its own
experiments — checked-in canonical vectors compared byte-for-byte by tests in
an offline research crate — and that crate is offline research code, tested, not
formally verified, and not deployed.

## Evidence this program could build

A **two-dialect divergence demonstration**. Express one lifecycle event under
two schema versions that differ in a single field's definition — a rounding
rule, a timestamp basis, an enumeration boundary. Generate a conformance corpus,
and exhibit: (i) a large set of inputs on which both dialects agree, and (ii) at
least one input on which they disagree, that outcome sampling from ordinary
traffic would be unlikely to surface. Then show that comparing the two
predicates directly finds the divergence immediately. Bounded: two small
schemas, a generated corpus, deterministic, offline. The deliverable is the
argument for position 2 in a form a reader can check in a minute rather than
accept on authority.

## Needs verification

- Both notices cite a March 11, 2026 SEC-CFTC Memorandum of Understanding
  regarding harmonization. It was not retrieved for the program's citation
  ledger. No claim here rests on it, and it should not be cited in a filing
  before it is read — it is the document most likely to already contain a
  coordination framework this memo would otherwise duplicate.
- The data notice's account of the 2019 Compliance Statement, its universal
  reliance, and its November 5, 2029 expiration is relied on as the notice's own
  characterization; the underlying release was not retrieved.
- Regulation SBSR (17 C.F.R. 242.900-909) and the SBSDR rules (17 C.F.R.
  240.13n-1 through 13n-12) were not retrieved. Only the part headings for
  17 C.F.R. parts 43, 45, and 49 and a small number of CFTC sections are in the
  verified ledger.

## Note for the coordinator

The definitions filing states that the submitter "take[s] no position on
Questions 12 through 15 concerning alternative compliance." This memo is a
position on Questions 12 through 14. Folding it in requires reversing that
sentence, which was presumably a deliberate scope choice — the alternative
compliance discussion is institutional and this program has no supervisory
experience. The narrower option is to fold only the harmonization-maintenance
material (positions 3 through 5) into the *data* filing under its Question 4,
where the program is already speaking and where no no-position statement is in
the way.

## Filing-worthiness

**Strong as method, on the data notice's Question 4. Interesting but out of
character for the definitions filing as currently scoped** — the institutional
questions in 12 and 13 invite an answer this program cannot support with
experience, and the no-position sentence is a defensible place to stay.
