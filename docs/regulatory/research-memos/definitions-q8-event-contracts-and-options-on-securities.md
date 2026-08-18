# Definitions Question 8 — event contracts on securities, and the options exclusion

Internal research memo. See [README.md](README.md) for status and citation rules.

## The question, verbatim

> 8. Some event contracts settle by reference to a security or group or index
> of securities (whether narrow-based or broad-based). While certain of these
> event contracts may generally fall within the prongs of the "swap" and
> "security-based swap" definitions described above, there is a statutory
> exclusion from the definitions of "swap" and "security-based swap" for "any
> put, call, straddle, option, or privilege on any security, certificate of
> deposit, or group or index of securities, including any interest therein or
> based on the value thereof, that is subject to [the Securities Act and the
> Exchange Act]". Are there circumstances in which an event contract that
> references one or more securities should or should not be considered a "put,
> call, straddle, option, or privilege on" a security or group or index of
> securities for purposes of the exclusion from the definitions of "swap" and
> "security-based swap"? Is there a need for greater clarity regarding when an
> event contract is a "put, call, straddle, option, or privilege" on an
> "interest" in, or "based on the value" of, "any security . . . or group or
> index of securities" (whether narrow-based or broad-based), that is subject
> to the Securities Act and the Exchange Act and therefore not a swap or SBS?
> What are the characteristics of event contracts based on a security or index
> that are swaps or SBS, that distinguish them from options on securities,
> including in particular binary options that already trade as standardized
> options on securities on national securities exchanges?

— 91 Fed. Reg. 37873, 37876 (June 24, 2026).

## Positions

1. **Classification should be invariant under bundling and unbundling.** A
   contract that pays a fixed amount if a stated outcome occurs is a
   cash-or-nothing claim. A set of such claims covering every possible outcome
   of one reference variable, without overlap, is a portfolio of cash-or-nothing
   claims on that variable, and the complete set is economically identical to
   the collateral it was issued against. If a cash-or-nothing claim on a
   security's value is a "put, call, straddle, option, or privilege on" that
   security, then so is every claim in such a set, and so is any bounded payoff
   assembled from them. A rule that answers differently for the bundle and for
   its parts can be defeated by an operation these designs perform routinely,
   at will, and for no cost.
2. **Payoff shape is not the distinguishing characteristic the question is
   looking for.** A standardized binary option on a security listed on a
   national securities exchange and a categorical event claim settling on the
   same security's price at the same time pay the same amounts in the same
   states of the world. Whatever distinguishes them, it is not the payoff.
3. **The reference object and the settlement function distinguish them.** An
   instrument whose settlement amount is a function of the price or value of a
   security, or of a group or index of securities, should be analyzed under the
   same exclusion as an option on that reference, whatever the instrument is
   called. An instrument whose settlement amount is a function of a fact about
   an issuer, rather than of a security's price or value, is where the
   security-based swap event prong does its work, and the exclusion should not
   be read to swallow it.
4. **State the test in the form a drafter can apply before listing.** Three
   facts decide it, and all three are readable off the contract's own frozen
   terms: (i) name the reference variable; (ii) state whether the settlement
   amount is a function of the price or value of a security or index; and
   (iii) state whether the outcome set is exhaustive and non-overlapping over
   that variable. Products whose answers to (i) and (ii) match should not
   receive different treatment because of (iii).

## Argument

The exclusion's operative words are the broadest in the sentence. "Privilege"
is not a term of art with a narrow payoff shape attached to it, and the
exclusion covers instruments "on any security . . . or group or index of
securities, including any interest therein or based on the value thereof."
7 U.S.C. 1a(47)(B)(iii). The swap definition's own option prong is drawn just
as broadly: it reaches "a put, call, cap, floor, collar, or similar option of
any kind that is for the purchase or sale, or based on the value, of" the
listed reference classes. 7 U.S.C. 1a(47)(A)(i). Neither text turns on whether
the payoff is linear, capped, or all-or-nothing.

What creates the overlap the question is about is that a price band is
simultaneously a contingency and a function of value. The event prong reaches
a payment "dependent on the occurrence, nonoccurrence, or the extent of the
occurrence of an event or contingency associated with a potential financial,
economic, or commercial consequence." 7 U.S.C. 1a(47)(A)(ii). Every
cash-or-nothing claim on a price satisfies that description in words, and also
satisfies the option prong's "based on the value" description in words. The
statute anticipates this collision elsewhere and resolves it in favor of the
price reading: the excluded-commodity definition the event-contract special
rule keys on expressly carves out "a change in the price, rate, value, or
level of a commodity" from the occurrence-and-contingency category.
7 U.S.C. 1a(19)(iv). The CEA therefore already contains an instance of the
distinction position 3 asks the Commissions to state generally — price-referencing
contracts are treated as price-referencing contracts, not as occurrence
contracts, even where the words of the occurrence category would otherwise
reach them.

The security-based swap event prong supplies the other half. It reaches an
event "relating to a single issuer of a security or the issuers of securities
in [a narrow-based security index], provided that such event directly affects
the financial statements, financial condition, or financial obligations of the
issuer." 15 U.S.C. 78c(a)(68)(A). That is an issuer-fact test, not a price
test, and reading the options exclusion to cover issuer-fact contracts because
they happen to be all-or-nothing would leave the prong with little to do.

The counterargument is administrability: if bundling invariance is required, a
venue could list an exhaustive set of claims to convert options into event
contracts, or list a single claim to convert an event contract into an option.
That is the argument for the position rather than against it. Under invariance,
neither relabeling changes the answer, so neither is worth doing.

*Basis for statements about the submitter's artifacts in this memo:* the
complete-set issuance, recombination, and redemption accounting described
above is implemented in an offline pure-Rust research prototype whose
deterministic tests pass; it is tested, not formally verified, and it is not
deployed, funded, offered, or operating.

## Evidence this program could build

A **paired-payoff corpus**. Take four payoff shapes a market participant would
recognize — a digital, a range, a capped directional position, and a tail
position — and express each one twice: once as a bespoke instrument with its
own settlement function, and once as a portfolio over an exhaustive,
non-overlapping basis of claims on the same reference. Emit, for each pair, the
exact integer cashflow vector across every state of the world, and show the two
vectors are equal in every state. Bounded: one reference variable, a small
number of outcomes, deterministic integer arithmetic, no market data, offline.
The deliverable is a table a lawyer can read: two instruments that a
form-based rule would classify differently, with identical cashflows printed
side by side. This experiment has been built and run; see the addendum below.

## Filing-worthiness

**Strong.** The invariance argument is a direct, self-contained answer to the
question actually asked, it is argued entirely from verified statutory text,
and the supporting experiment is cheap.

## Addendum, 2026-08-18: the paired-payoff corpus, run

The experiment proposed above has been built and run. It lives in
[../../../experiments/bundling-invariance/README.md](../../../experiments/bundling-invariance/README.md),
a dependency-free offline Rust crate with deterministic tests and a byte-stable
output corpus. It went further than the proposal in one respect: rather than
printing four worked pairs, it enumerates a bounded corpus exhaustively and
checks every decomposition of every member.

**Claim ceiling, stated before the results.** This is a synthetic corpus over a
toy model. It exhibits the arbitrage structurally: it shows that particular
classification criteria, applied to particular payoff objects, answer differently
for a bundle and for its parts, and that the conversion between the two costs
nothing in the model. It asserts nothing about any real rule's text, any real
instrument, any real venue, or how any authority would classify anything. The
category labels in it are the criteria's own vocabulary, invented for the
experiment; none is a statutory category and none is a legal conclusion. The
results are VERIFIED as deterministic offline measurements of that crate at
exactly the bounds below, and nothing else; the crate is tested, not formally
verified, and not deployed. That classification criteria should have to survive
bundling invariance remains PROPOSED.

### What was modelled

A payoff object is an exhaustive, non-overlapping partition of one reference
variable into cells, a nonnegative integer payoff vector over those cells, and
three economic facts a criterion may read: what the settlement amount is a
function of (one security's price, an index's value, or an issuer fact), whether
the maximum payout is locked against collateral before the claim exists, and
whether the claim is transferable. A classification criterion is any decidable
function from those facts to a category label, together with its own aggregation
story — the criterion's account of how the labels of the parts bear on the label
of the whole.

The two costless operations are the ones the worked example's terms already
perform: bundling (hold the parts together; the payoffs add cell by cell) and its
inverse. Claims are created in exactly one way, a deposit that mints one claim on
every cell against one unit of collateral, and destroyed in exactly one way, a
recombination that surrenders one claim on every cell for that unit back. The
crate carries a collateral ledger so that "costless" is computed rather than
asserted: the claims outstanding on each cell are the same number on every cell,
and that number is the collateral locked. Splitting a position, bundling two
positions, and transferring a claim all leave both quantities untouched. That is
checked over every operation sequence of length five drawn from a ten-operation
alphabet, 7,820 states in all.

### Bounds

Full enumeration inside these bounds, and no claim outside them: partitions of 2
through 5 cells; payouts of 0 through 3 per cell; all 1,360 resulting payoff
vectors; 12 fact profiles; 16,320 payoff objects. For each object, two families
of decomposition were checked — every way of splitting it into two parts, and its
statement as its individual one-cell claims — giving 56,936 distinct
decompositions per criterion, or 683,232 counting fact profiles.

A criterion is treated as bundling-invariant when, for every object and every
decomposition, parts that all receive the same label fix the whole's label. Each
criterion was also allowed to declare additional labels it considers acceptable
for a whole whose parts are unanimous, and violations were counted under both
readings, so no result below depends on an uncharitable reading of a candidate.

### Per-criterion results

| Criterion | Reads | Violations, strict | Violations, under its own aggregation story |
|---|---|---|---|
| Binary payout makes it an event contract | payoff | 49,032 | 49,032 |
| Payout bounded by 2 makes it excluded | payoff | 180,696 | 180,696 |
| More than two distinct amounts makes it a portfolio | payoff | 159,120 | 113,472 |
| Outcome set (one outcome, some, or all) decides | payoff | 27,420 | 0 |
| Pays in some state makes it a claim | payoff | 0 | 0 |
| Prefunded makes it excluded | facts | 0 | 0 |
| Reference variable and settlement function decide (control) | facts | 0 | 0 |
| One label for everything (degenerate control) | nothing | 0 | 0 |

The control is the test Position 4 of this memo proposes. It is invariant, and it
still separates objects: it gives price-referencing and index-referencing objects
one label and issuer-fact objects another, whatever their payoffs.

### One witness, worked

Take a market on one security's closing price on a stated date, with two
outcomes: at or below a stated level, and above it. Two claims are listed. The
first pays one unit if the price is at or below the level and nothing otherwise.
The second pays one unit if the price is above and nothing otherwise. Read alone,
each pays a fixed amount on a stated outcome and nothing otherwise, so a
criterion that classifies by payoff shape — a binary payout makes it an event
contract — calls each of them an event contract.

A holder who owns one of each owns a position that pays one unit whichever way
the price goes. Read as one object, that position does not pay a fixed amount on
a stated outcome and nothing otherwise; it pays the same amount in every state.
The same criterion, applied to the same holder's same claims, calls that position
something else.

Nothing happened in between. Moving the two claims into one account, or out of
one, mints no claim and cancels none. The claims outstanding on each outcome are
the same before and after, so the collateral the terms require is the same before
and after, and no payment is made by anyone to anyone. The holder is free to hold
either label, in either direction, at any time, for nothing. In the worked
example's design the second direction is available too: the pair is a complete
set, and its holder may exchange it for the collateral it was issued against
before resolution.

That is the smallest violation in the corpus — two cells, one unit each — and it
was found by the sweep rather than chosen for the memo. The remaining 49,031
violations of that one criterion are variations on it.

The same corpus contains the table the proposal above asked for. A digital, a
range, a capped directional position, and a tail position were each expressed
twice, once as a bespoke instrument and once as a portfolio over the exhaustive
basis, with the exact integer cashflow in every state printed on both sides. The
two sides are equal in every state for all four, and for every other vector in
the corpus. The capped directional position alone receives different answers from
four of the eight criteria depending on which of the two equal expressions is
read.

### Three findings the experiment was not looking for

1. **A payoff-shape criterion can be invariant.** The criterion "it is a claim if
   it pays in some state" reads only the payoff vector and survives with no
   violations. The reason is structural: the outcomes a bundle pays in are
   exactly the union of the outcomes its parts pay in, so a criterion whose label
   classes are closed under that union is invariant. This is worth stating
   plainly because it means invariance is not a blanket objection to reading the
   payoff. It is an objection to reading the payoff in ways that bundling
   changes — how large the largest payout is, how many distinct amounts appear,
   how many outcomes are covered.
2. **Invariance does not make a criterion a good one.** "A prefunded claim is
   excluded" is bundling-invariant, because prefunding is a fact the parts and
   the bundle share. This memo objects to that criterion, but the objection is
   Position 6's — prefunding is a risk fact, not a classification exclusion — and
   not an arbitrage objection. A criterion that gives everything one label is
   also invariant, and separates nothing. Invariance is a filter, not a
   qualification.
3. **A criterion can survive by declining to answer.** The outcome-set criterion
   had 27,420 violations under the strict reading and none under its own declared
   story. But the story it needs in order to survive permits three different
   labels — an option, a portfolio, or collateral — for the same unanimous parts.
   A rule that answers "one of three, depending" has not classified the object.
   The corpus records that property explicitly rather than crediting the
   criterion with a pass.

### How severe a constraint invariance is

The eight candidates are tests somebody might write down. The experiment also
counted, exhaustively, how many of **every** two-label criterion that reads only
which outcomes an object pays in are invariant: 12 of 16 over two cells, 52 of
256 over three, and 300 of 65,536 over four. At five cells there are more than
four billion such criteria and the experiment says nothing about them. The
surviving fraction falls steeply as the partition grows, which is the
quantitative form of the position: invariance is a demanding test, and most
plausible-sounding form-based criteria fail it.

