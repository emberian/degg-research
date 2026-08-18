# Data Questions 2, 2a, and 20 — the reporting party should be the party that holds the record

Internal research memo. See [README.md](README.md) for status and citation rules.

## The questions, verbatim

> 2. Regulation SBSR Rule 901(a)(1) states that if an SBS is executed on a
> platform and will be submitted to clearing, the platform would have the
> obligation to report the original transaction (the "alpha") to an SBSDR and
> the clearing agency has the obligation to report the cleared transactions (the
> "beta" and "gamma"), but to the extent those transactions are not submitted
> for clearing, Rule 901(a)(2) provides that one of the counterparties is
> responsible for reporting the transaction. Should the SEC amend Regulation
> SBSR to require the platform to report platform-executed, non-cleared trades
> to the SBSDR, consistent with the CFTC's requirements, which are different
> than the SEC's requirements?

> a. Section 6(l) of the Exchange Act provides that "[i]t shall be unlawful for
> any person to effect a transaction in an SBS with or for a person that is not
> an eligible contract participant, unless such transaction is effected on a
> national securities exchange registered pursuant to subsection (b)." Who
> should be the reporting party for a transaction effected on a national
> securities exchange in an SBS issued by a clearing agency: the national
> securities exchange or the clearing agency? Who should be the reporting party
> if an uncleared transaction is effected on a platform: the platform or one of
> the counterparties?

> 20. Do commenters have suggestions regarding the reporting hierarchies that
> determine the reporting counterparty? Do commenters have any concerns
> regarding operational issues related to the reporting hierarchies?

— 91 Fed. Reg. 37877, 37879, 37880-81 (June 24, 2026).

## Positions

1. **The reporting obligation should follow the record.** Assign each group of
   fields to the party that mechanically holds those fields at the milestone
   where they come into existence. A party required to report a fact it can
   only obtain by asking someone else will report that fact late, or
   approximately, or not at all, and no validation rule downstream can repair
   it.
2. **In a platform-executed transaction, the platform and the counterparties
   hold different records, and neither holds the other's.** The platform holds
   the order lifecycle: receipt, rejection, modification, cancellation, arrival
   sequence, and the composition of the set accepted at close. The
   counterparties hold beneficial ownership, allocation, and the internal
   linkage between an account and a person. Requiring one party to report the
   whole transaction requires that party to source half of it from the other.
3. **Answering Question 2 directly: for platform-executed transactions the
   platform should report execution and order-lifecycle fields,** because it is
   the only party that holds them, and the counterparty-side identity,
   ownership, and allocation fields should be reported by, or sourced from, the
   counterparty. The same answer resolves Question 2a: the venue reports what
   the venue observes, and the clearing agency reports what clearing creates.
4. **Field-group allocation shrinks the reporting-hierarchy problem
   (Question 20).** Hierarchies exist to break ties about who reports an entire
   transaction. Once the obligation is allocated by field group, most ties do
   not arise, because for most fields exactly one party is in a position to
   know. The residual hierarchy question — which counterparty supplies the
   counterparty-side fields when both could — is narrower and easier.
5. **Whatever allocation is chosen, the record must remain joinable.** One
   lifecycle, one identifier, contributions from more than one party, and a
   stated rule for what the record looks like when a contribution is missing:
   pending, not silently absent.

## Argument

The observation behind this memo is not specific to blockchain venues, but
onchain execution makes it unusually vivid. In the design the program's data
comment walks through, the funding of a position, the orders that create
exposure, and the eventual settlement can each be visible to a different party.
The public ledger shows a deposit and a redemption but does not say who
controls the wallet. The venue sees the orders, including the ones it rejected,
and those rejections never become ledger bytes at all because they never
changed the ledger's state. The counterparty knows whose position it is. As
that comment puts it, the records a market-conduct examination cares most about
"are reported from the venue's own records or they are not reported."

That is the general principle stated in a hard case. A reporting hierarchy
built around "who is more likely to be a dealer" allocates the obligation by
sophistication, which is a reasonable proxy for who can bear the cost but not a
proxy at all for who can observe the fact. Where those two come apart — and
platform-executed non-cleared trades are exactly where they come apart — the
observation constraint should win, because cost can be shifted by contract and
observation cannot.

The counterargument is duplication: splitting a transaction report across two
parties creates a reconciliation problem where there was one report. That cost
is real and it is why position 5 matters more than the rest. But the
alternative is not "no reconciliation" — it is reconciliation performed
privately, before reporting, by a party contractually obtaining facts from the
other, with the failures invisible to the repository. Making the join explicit
moves an existing reconciliation into the open, where a missing contribution is
a reportable state rather than a silently wrong field.

*Basis for statements about the submitter's artifacts in this memo:* the
program has an offline research prototype whose batch component freezes its
policy when the book is constructed and verifies a submitted clearing by full
recomputation from the frozen book, and a separate offline deterministic
laboratory that replays four synthetic trading traces against three transcript
designs and reports, per observer role, which fields are mechanically present
and which deductions those fields enable. Both are offline research code with
passing deterministic tests; they are tested, not formally verified, and not
deployed. Neither is a reporting system, and the program has no experience
operating a reporting obligation.

## Evidence this program could build

A **record-custody matrix**. Extend the existing leakage laboratory, which
already models observers, to emit for each milestone of the worked design a
table of field against role — platform, counterparty, chain observer,
repository, regulator — marking for each cell whether that role mechanically
holds the field, can derive it, or cannot obtain it without being told. Bounded:
the observer machinery and the synthetic traces already exist; this is a new
projection and a report format, not a new experiment. The deliverable is
directly responsive: a printed argument that for a large fraction of fields
exactly one role can supply the value, which is what positions 1 through 3
assert.

## Needs verification

Regulation SBSR Rules 901(a)(1) and 901(a)(2), 17 C.F.R. 45.3(a), 17 C.F.R.
45.8, 17 C.F.R. 43.3, and 15 U.S.C. 78f(l) are relied on here only as the
notice characterizes and quotes them. None was independently retrieved for the
program's citation ledger, which contains the part headings for 17 C.F.R. parts
43, 45, and 49 and a small number of specific sections not including these. A
filing that takes a position on how Regulation SBSR should be amended must read
Rule 901 first — in particular, the existing rule may already allocate some
fields in the way position 3 recommends.

## Filing-worthiness

**Strong.** The position is a direct answer to the question asked, it is
argued from a structural fact rather than from operating experience the program
lacks, and the supporting experiment is an extension of an artifact that
already exists. It requires reading Rule 901 before filing.
