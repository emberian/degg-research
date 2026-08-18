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
side by side.

## Filing-worthiness

**Strong.** The invariance argument is a direct, self-contained answer to the
question actually asked, it is argued entirely from verified statutory text,
and the supporting experiment is cheap.
