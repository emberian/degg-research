# Data Questions 10, 12, 15, 17, and 18 — data quality is a specification problem

Internal research memo. See [README.md](README.md) for status and citation rules.

## The questions, verbatim

> 10. Which reported data elements (whether disseminated or not), while
> conceptually useful, are never or rarely populated in practice, and why?

> 12. Are there data elements that, while conceptually useful, are persistently
> difficult to report consistently across registrants, asset classes, products,
> or market structures? If so, please identify and explain. What factors most
> commonly contribute to, and what transaction types (e.g., allocated trades)
> are most susceptible to, inaccuracies or inconsistencies in swap or SBS data
> reporting (whether or not related to specific data elements)? Could guidance
> on or amendments to the swap and SBS data reporting rules resolve such
> difficulties, inaccuracies, or inconsistencies? . . . Are these data elements
> static, and therefore could potentially be collected in a reference table, or
> do these elements change over time? With respect to the difficulties,
> inaccuracies, or inconsistencies described in response to this question, are
> there practical steps the Commissions could take to improve data quality and
> transparency without significantly increasing reporting burdens?

> 15. Should SDRs and SBSDRs take additional steps to validate reported
> transaction data to ensure accurate and high-quality public dissemination and
> regulatory reporting, and if so, for what specific data elements? Would such
> additional validations reduce the costs of addressing data quality issues?
> What specific additional validations should SDRs and SBSDRs add to ensure
> higher data quality?

> 17. Are there particular validation rules or reporting obligations related to
> lifecycle events that could be simplified? . . .

> 18. Should the Commissions consider implementing a materiality or de minimis
> threshold for the requirement that reporting firms correct errors in data for
> swaps and SBS, such as for those that have terminated, matured, or are
> otherwise no longer open? If so, what materiality or de minimis threshold is
> appropriate?

— 91 Fed. Reg. 37877, 37880 (June 24, 2026).

## Positions

1. **An element that is not a deterministic function of facts the reporter is
   required to retain will be reported inconsistently, and validation cannot
   repair it.** The remedy is specification, not enforcement: for each element,
   name the source of truth, name the retention obligation that guarantees the
   inputs still exist when the element is computed, and publish the derivation
   where the element is derived.
2. **"Unsupported" should be a reportable value.** A schema that forces a number
   where the reporter's retained information cannot produce one converts a
   refusal into a wrong answer. Once that happens the framework can no longer
   distinguish a measurement from an approximation, and the resulting field is
   worse than an empty one because it looks populated. This is also the honest
   answer to Question 10: an element that is rarely populated is frequently an
   element nobody can compute, and the correct fix is to say so in the
   vocabulary rather than to press harder for a value.
3. **Repository validation should be graded, and the grades should be named.**
   Three kinds exist and they catch different things: *format* validation, which
   asks whether the field parses and falls in range; *cross-record consistency*,
   which asks whether the value agrees with related records; and
   *recomputation*, which asks whether the reported value follows from retained
   inputs under the published rule. Only the third catches substantive error,
   and it is available only for derived elements whose inputs the repository
   holds. Answering Question 15 concretely: the elements worth new validation
   are the derived ones, and the way to make them validatable is to require the
   inputs alongside the output.
4. **On Question 18, a materiality threshold should govern whether a correction
   is required, never whether the discrepancy is recorded.** A known,
   uncorrected discrepancy is itself a fact about the regulatory record. If the
   threshold suppresses the record of the discrepancy rather than the obligation
   to fix it, the record ends up looking more accurate than it is, and the
   people relying on it have no way to know which parts were checked.
5. **Corrections should supersede, never overwrite, including corrections to
   terminated or matured transactions.** The superseded record and the link
   between it and its replacement are what let a supervisor reconstruct what was
   believed at the time, which is often the question in an examination.

## Argument

The notice states the diagnosis this memo builds on: complexity "may have
contributed unintentionally, yet significantly, to a potential lack of
integrity in the accuracy, completeness, and timeliness of data reported to SDRs
and SBSDRs," and "[t]he collection, verification, and collation of pertinent
information from disparate systems across reporting counterparties has led to
large amounts of potentially inconsistent swap and SBS data reports."
91 Fed. Reg. at 37878. That is a description of a specification failure rather
than a compliance failure. When several firms compute an element from different
internal systems and no published rule says what the element is a function of,
the reports differ because the firms are answering different questions, and each
firm is answering its own question correctly.

Position 2 is the one this program has direct experience with. The design's
observation component is an interval-summary structure that combines
observations — source authentication is the design's input contract, not
implemented in the prototype — and refuses to answer a question its retained
information cannot support, rather than approximating it. That refusal is a
feature: "the rule rejects this" and "the backend cannot answer this" are
different facts, and a system that conflates them misreports both. Applied to
reporting, a failure taxonomy that distinguishes rejected, pending, unsupported,
expired, corrected, and superseded is not vocabulary decoration; it is what lets
a repository count how much of its data was actually measured.

Position 3's recomputation grade is available more often than it looks, because
it does not require the repository to hold everything — only the inputs to the
specific derived element. Where the derivation is deterministic over inputs
that are already reported, the validation is a re-execution of the published
rule, and disagreement about what happened reduces to disagreement about the
inputs, which is where a supervisor wants it. Where the inputs are not reported,
the honest options are to require them or to accept that the element cannot be
validated, and the framework should say which one it chose.

The counterargument to position 4 is volume: recording every immaterial
discrepancy on a matured transaction produces a large pile of open items nobody
will work. The answer is that recording is not the same as queueing. A
discrepancy noted, classified below threshold, and closed is one row; the pile
is a consequence of treating every recorded item as an obligation, which
position 4 explicitly does not.

*Basis for statements about the submitter's artifacts in this memo:* the
observation accumulator described above is an offline research prototype whose
deterministic tests pass; it is tested, not formally verified, and not deployed.
The correction discipline in position 5 corresponds to machine-checked
statements in the program's formal models that a correction's authorized actor,
target record, permitted fields, and governing rule version can be fixed in
advance and enforced mechanically; those are theorems about simplified models,
not properties of any deployed control and not a reporting adapter. The program
has no experience operating a reporting obligation and offers no view on which
existing elements are rarely populated.

## Evidence this program could build

An **answerable-versus-approximated corpus**. Using the existing observation
accumulator, construct a set of queries paired with retained-information sets,
and classify each query three ways: answerable exactly, refusable with a stated
reason, or answerable only by approximation. Then run the same corpus through a
schema that forbids refusal and record what value the reporter is forced to
emit and how far it is from the exact answer where an exact answer happens to be
computable. Bounded: the accumulator and its refusal semantics already exist;
this is a corpus plus a comparison harness, deterministic and offline. The
deliverable is the concrete form of position 2 — a table of cases in which a
no-refusal schema silently converts a correct refusal into a wrong number.

## Needs verification

The CFTC technical specification and its element count (the notice states the
CFTC requires reporting of up to 128 data elements) are relied on as the
notice's own statement. The specification itself was not retrieved for the
program's citation ledger, and no element-level claim should be made in a filing
without reading it. This memo deliberately identifies no specific element.

## Filing-worthiness

**Strong.** Positions 1 through 5 are answers to the questions actually asked,
they are argued from a structural point rather than from operating experience,
and each is cheap for the Commissions to adopt. The memo's discipline is that it
names no element — which is also its limit, and a filing should say so.
