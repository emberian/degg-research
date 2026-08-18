# Definitions Questions 7 and 10 — standards that look outside the terms

Internal research memo. See [README.md](README.md) for status and citation rules.

## The questions, verbatim

> 7. Regarding the SBS Event Contract Prong, is additional clarity necessary
> regarding when an event "directly affects" the financial statements,
> financial condition, or financial obligations of an issuer? Should the
> Commissions further address the circumstances when a swap does or does not
> satisfy the SBS Event Contract Prong? How should any further clarifications
> relate to the Commissions' rules and guidance on credit default swaps in the
> Product Definitions Adopting Release, and should such rules and guidance be
> revised or clarified?

> 10. Security forwards, if intended to be physically settled at the time the
> contract is entered into, are excluded from the definitions of "swap" and
> "security-based swap." In the Product Definitions Adopting Release, the
> Commissions declined to provide a bright-line test for determining whether a
> security forward is intended to be physically settled. In light of innovative
> products and product structures, should the Commissions provide additional
> clarity as to the meaning of the phrase "for deferred shipment or delivery,
> so long as the transaction is intended to be physically settled" in the
> exclusion from the definition of "swap" set forth in CEA section
> 1a(47)(B)(ii)? What approach should be taken?

— 91 Fed. Reg. 37873, 37876 (June 24, 2026).

## Positions

1. **These two questions share a structure.** Each asks about a standard that
   is not answered by reading the contract: whether an event "directly affects"
   an issuer's financials is a question about a real-world effect, and whether
   a forward is "intended to be physically settled" is a question about a state
   of mind. Both are hard to apply to an instrument whose terms are fixed and
   executed mechanically, because there is no later moment at which either
   inquiry has anything new to read.
2. **For physical settlement, replace the intent inquiry with a reachability
   inquiry.** Ask whether any state the frozen terms can reach settles other
   than by delivery, and if so, who can bring that state about. Terms with no
   reachable cash-settlement path are physically settled. Terms with a cash
   path available at one party's election are not physically settled merely
   because that party says today that it does not intend to elect. This is not
   a new substantive standard; it is the existing standard evaluated against
   the only evidence a fully specified instrument has.
3. **For "directly affects," the Commissions should say which object the test
   is applied to.** Two readings are available: the event as the contract
   describes it, or the event as it actually occurred. Only the first can be
   applied before listing, and only the first gives a stable answer for the
   whole life of the contract. A drafter-facing formulation of the first
   reading asks whether the contract's own settlement function is a function of
   a named financial fact about a named issuer.
4. **Neither restatement narrows or broadens the standard.** Each moves the
   moment of application from after the fact to the moment of design, which is
   the moment at which a person deciding whether to build something needs the
   answer.

## Argument

The exclusion at issue in Question 10 is textually about the transaction's
character. The notice quotes the operative phrase as "for deferred shipment or
delivery, so long as the transaction is intended to be physically settled," and
locates it in the exclusion at CEA section 1a(47)(B)(ii). The notice records
that the Commissions declined to provide a bright-line test for that intent in
2012. An intent standard is workable where the parties are two firms with a
trading relationship and a documented history. It is weakest exactly where the
notice says innovation is happening: a set of terms published in advance, with
no negotiation, entered into by parties who never speak, and executed by a
machine that has no access to anyone's intent. In that setting the only
durable evidence of what the transaction is "intended" to do is what the terms
permit it to do — which is fully enumerable, because the terms enumerate it.

Question 7's standard has the same shape in the opposite direction. The prong
requires an event that "directly affects the financial statements, financial
condition, or financial obligations of the issuer." 15 U.S.C. 78c(a)(68)(A).
Read as a test about the actual effect of the actual event, the answer is not
available at listing, may differ between two contracts with identical terms
because the underlying facts differed, and cannot be relied on by anyone
deciding what to build. Read as a test about the event the contract describes,
it is available at listing and stable. The Commissions do not have to choose the
second reading; they should say which one they mean, because the two produce
different answers and each is currently defensible from the text.

The counterargument to position 2 is that a reachability test invites drafting
around it — insert a delivery path that nobody will ever use and the terms are
"physically settled" on their face. That objection is answered by the second
half of the test rather than by abandoning it: who holds the election, and
under what conditions. A delivery path nobody can practically reach is a term
with a condition on it, and the condition is in the terms.

*Basis for statements about the submitter's artifacts in this memo:* this
program's prototype has a finite, enumerated lifecycle with a closed set of
transitions and an explicit refusal class on each edge, which is what makes the
reachability question mechanical for instruments of this kind; that lifecycle
is design-level in the program's architecture documents and partially
implemented in an offline research prototype whose deterministic tests pass. It
is tested, not formally verified, and it is not deployed. The program has no
artifact that settles anything by physical delivery and no artifact that
references an issuer.

## Evidence this program could build

A **reachable-settlement-mode report**. Take the prototype's lifecycle graph,
enumerate every state reachable from market creation, and emit for each
terminal state: the settlement mode, the condition that reaches it, and the
party (if any) holding an election over that condition. Bounded: the graph is
small and already closed, so this is a traversal over existing code plus a
report format. The deliverable demonstrates the shape of the answer position 2
proposes — a one-page table produced mechanically from terms, with no
interview and no representation about intent.

## Needs verification

The notice's reference to the Commissions' credit default swap rules and
guidance points to 77 Fed. Reg. at 48267. The program's ledger records the 2012
adopting release as fetched for title, date, agencies, and action only; its
substantive content at that page was not read. Any position on how a "directly
affects" clarification should relate to the CDS guidance requires reading it
first. This memo takes no such position.

## Filing-worthiness

**Needs evidence for Question 7 — the recommendation is sound but the CDS
half of the question cannot be answered without reading the 2012 guidance.
Interesting but weak for Question 10 — the reachability restatement is a clean
idea on a substrate (physically settled security forwards) this program does
not work in, and a filing that says so would be volunteering an opinion without
experience behind it.**
