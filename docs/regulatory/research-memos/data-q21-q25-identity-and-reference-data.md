# Data Questions 21-25 — product identity, counterparty identity, and reference data

Internal research memo. See [README.md](README.md) for status and citation rules.

## The questions, verbatim

> 21. What limitations with respect to the information and supervisory or
> operational utility of the transaction data, if any, arise when the UPI or
> other standardized product identifiers or classification systems are used?

> 22. Are there instances where a data standard other than the UPI should be
> used to identify products? Please explain.

> 23. Are there additional opportunities to use standardized and static
> reference data elements to capture relevant attributes of swaps and SBS?
> Please explain.

> 24. Are there instances where a data standard other than the LEI should be
> used to identify counterparties? Please explain.

> 25. Is there information that is currently reported on a trade-by-trade basis
> that could more efficiently be captured through reference data? If so, please
> explain the benefits and drawbacks of using such reference data, including:
> (1) benefits or impacts to public transparency, usability, and accessibility
> of swap and SBS transaction and pricing data; (2) impediments to accessing
> reference data; (3) management of changes to the reference data that may occur
> over time; and (4) costs of maintaining and validating the reference data. In
> addition to these items, please describe any other risks, benefits, and costs
> associated with using standardized and static reference data.

— 91 Fed. Reg. 37877, 37881 (June 24, 2026).

## Positions

1. **A classification-based product identifier describes a product by its
   attributes; a programmable product's economics live in its terms, and some of
   those terms are not attributes.** Two instruments can agree on asset class,
   underlier, settlement currency, and every other attribute a taxonomy records,
   and still pay different amounts in some state of the world, because they
   differ in an edge-case rule: what happens when an observation is missing,
   when a value lands exactly on a boundary, when the source is upgraded, when
   the window closes with insufficient coverage. Those rules decide real money
   in exactly the situations supervisors care about, and no attribute list
   carries them.
2. **The transaction record should carry, alongside the product identifier, a
   digest of the frozen terms and the version of the schema that decodes them.**
   The identifier keeps its aggregation function — it is what lets someone count
   activity by product type — and the digest supplies identity, which is what
   lets someone confirm that two records describe the same instrument. Neither
   replaces the other, and the failure mode of using only the first is silent:
   two economically different products aggregate into one bucket and nothing
   looks wrong.
3. **Reference data is the right home for anything constant across every
   transaction in a product's life** (Questions 23 and 25): the outcome
   boundaries, the payout structure, the observation specification, the
   settlement rule, the failure rules. The transaction record should reference
   that material by digest rather than restate it per trade.
4. **Addressing reference data by digest answers the change-management
   sub-question in Question 25 directly.** A digest cannot be changed in place.
   If the terms change, the digest changes, and every record that pointed at the
   old digest still points at exactly what it described when it was written.
   The common failure of reference tables — a row edited in place, silently
   re-describing every historical transaction that referenced it — is not
   available.
5. **On Question 24: a wallet address is not a counterparty identifier.** It
   identifies an account. The mapping between accounts and legal persons is
   many-to-many in both directions: one person can control many accounts, and one
   account can be controlled by an arrangement involving several persons. The
   LEI should stay as the counterparty standard, and the account address should
   be carried as an attribute of the transaction in the confidential record, with
   the account-to-party linkage maintained separately. Substituting the address
   for the identifier would produce a record that looks precise and identifies
   nobody.

## Argument

The notice describes the current arrangement: unique product identifiers issued
by the Derivatives Service Bureau identify products, legal entity identifiers
managed by the Global Legal Entity Identifier Foundation identify
counterparties, and for natural persons acting as private individuals the
reporting counterparty submits its own identifier followed by a unique
identifier it assigns and maintains. 91 Fed. Reg. at 37881 and n.26. That
architecture assumes a product's economically relevant characteristics can be
expressed as a bounded set of attributes drawn in advance from a taxonomy.

That assumption holds for a large and important class of instruments and fails
for instruments whose terms are compiled rather than selected. In the design
this program's comments describe, a market is defined by a set of outcome
boundaries, a frozen observation specification with a named source and window,
a payout structure, and a deterministic rule for every failure mode. Change only
the rule for a value landing exactly on a boundary and you have a different
instrument, with a different price, that every attribute-based identifier will
report as the same product. This is not a hypothetical about future systems;
it is a property of any product whose terms are written rather than chosen from
a list, which includes a large part of what both notices call innovative
structures.

The remedy is modest and already familiar from the program's data-reporting
comment, which lists "[p]roduct or rule identifier, terms digest, and schema
version" among the fields that bind an event to its economic semantics and let a
decoder reject an incompatible record. Positions 2 through 4 are that field
recommendation stated as a reference-data recommendation.

The counterargument is that a digest is not human-readable and cannot be
aggregated on. Correct, and that is why position 2 keeps both: the identifier
for grouping, the digest for identity, and a reference record that resolves the
digest to the readable terms. The cost is one field and one lookup; the benefit
is that a supervisor can tell whether two records describe the same instrument
without comparing prose.

*Basis for statements about the submitter's artifacts in this memo:* the
content-addressed market template — terms addressed by digest, with the
transaction record binding to that digest — is a design commitment in this
program's architecture documents, not an implemented component. What is
implemented is an offline pure-Rust research prototype covering the core
accounting of the worked design, whose deterministic tests pass; it is tested,
not formally verified, and it is not deployed. The program has built no product
identifier, no reference-data service, and no reporting adapter, and has no
experience operating any of them.

## Evidence this program could build

A **taxonomy-collision corpus**. Generate a set of market templates that agree
on every attribute an attribute-based product identifier would record — asset
class, underlier, settlement currency, tenor, payout type — but differ in
exactly one frozen rule each: the boundary tie rule, the missing-observation
rule, the insufficient-coverage rule, the source-upgrade rule. For each pair,
print at least one state of the world in which the two templates pay different
amounts, with exact integer cashflows. Then print each template's terms digest,
showing that the digest separates the pairs the taxonomy merges. Bounded:
templates and payout computation are within the existing prototype's scope;
this is a corpus plus a report. The deliverable is the whole argument in one
table — pairs indistinguishable under attribute identity, distinguishable under
terms identity, with the money difference printed.

## Needs verification

The notice's account of the UPI, its issuance by the Derivatives Service Bureau,
the designation order at 88 Fed. Reg. 11790, the LEI and its management, and the
treatment of natural persons is relied on as the notice's own statement. None of
those underlying sources was retrieved for the program's citation ledger. This
memo makes no claim about the UPI's actual attribute set; position 1 is argued
about attribute-based identification in general, and a filing that asserts what
the UPI does and does not carry must read the designation order and the product
classification system first. That reading is the single most important
prerequisite for this memo, because the position is strongest if the UPI's
attribute set is fixed and weakest if it admits an extensible free-form field.

## Filing-worthiness

**Strong, conditional on reading the UPI designation order.** The position is a
direct answer to Questions 21 through 25, it is specific, it costs the
Commissions one field, and the supporting corpus is cheap. The conditional is
not a formality: the whole argument turns on a factual claim about what the
identifier can express.
