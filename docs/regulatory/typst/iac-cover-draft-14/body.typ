#import "../shared/template.typ": note_ref

I build programmable markets and prove things about them. The Committee's
inaugural meeting has now been held --- under a Roadmap whose three announced
tracks, crypto capital markets, compute markets, and prediction markets, all
touch this work --- and this statement is submitted for the meeting's record.
It reports what I found, and asks the Committee to recommend six specific
work products.

The system is Dragon's Clutch: a market where anyone deposits collateral and
receives a complete set of claims, trades them in a batch that clears by a
frozen rule, and redeems after an authenticated observation decides the
outcome. It runs — in local test banks and, end to end, through a
44-transaction signed walk on a real local validator, with valueless test
units; no public cluster hosts a deployment.

Three findings drive everything I ask for.

*A staged program has no single moment of formation.* Authorship, funding,
matching, resolution, and settlement happen days apart, to different people,
creating different rights. Bilateral confirmations fuse these into one
signing; code separates them. Any guidance that does not name which milestone
it analyzes will be applied inconsistently, because there is no longer one
event to point at.

*Removing settlement discretion relocates manipulation risk; it does not
remove it.* My design lets no one choose the outcome after the fact. The
attacker's response is to move the thing being observed — and they know in
advance exactly which statistic, at which instants, decides the payout.
Surveillance therefore needs the reference specification in the contract's
terms, machine-readably, or it does not know what to watch.

*Verification has a hard ceiling — and my first reading of it was wrong.*
Re-executing a batch clearing inside one Solana transaction once consumed
exactly the 1,400,000-unit limit and failed. The cause was a software hash
implementation, not the architecture: with the runtime's hashing syscall the
same selection completes at 16 percent of the ceiling, and the staged
successor clears every measured row with wide margin. A measured stop is
evidence about an artifact, not an architecture — and at scale the
alternatives are still staging or succinct proof.

= What I ask the Committee to recommend

1. A *milestone taxonomy* — publication, instrument creation, funding, close,
   resolution, settlement — as the shared factual clock for classifying and
   supervising staged transactions.
2. *Functional guidance separating software publication from operation*,
   naming which combinations of solicitation, order handling, custody,
   oracle control, and transaction-linked compensation cross the line.
3. *A statement of which facts govern the clearing analysis* of fully
   prefunded, atomically settled designs — prefunding removes novation and
   loss mutualization, but the statute also reaches multilateral settlement
   and netting, and that question stays open.
4. *Privacy-compatible audit-trail criteria* separating the public record,
   the exact confidential regulatory record, and governed disclosure. A
   transaction hash performs none of the functions an audit trail performs.
5. *Proof and control objectives with published negative cases*, admitting
   proofs as evidence of exactly what their statements encode — and a
   structured path to present a design's factual matrix before anything is
   at stake.
6. *A reference-specification requirement* for contracts settling by
   observing a named venue, carried machine-readably, so surveillance can
   evaluate a manipulation-cost envelope instead of one context-free number.

= The question I most want the Committee to take up early

The meeting's agenda included agentic finance. The sharpest version of
the publication-versus-operation question is a market participant that is an
AI agent with no operator: a published specification fixes the operating
loop, and prepaid executors submit steps a ledger accepts only when their
certificates verify.

Parts of that certificate stack are real in my research — a Lean-authored
parse-and-guard proof system joined to a genuine two-party TLS notarization,
tested for refusal. Parts are not: proving the whole execution history is an
open gap, so the executing host is trusted, and the transcript leg pins a
named notary, who is an operator for that function. No such agent exists.

The question is worth asking before one does: when no one operates, which
operator functions — supervision, recordkeeping, emergency authority,
accountability for harm — can verifiable conduct evidence satisfy, which
attach to the specification's author or its executors, and which have no
bearer at all.

= Scope

The attached statement uses Clear, Shielded, and Dark as exact
information-flow terms, and every technical claim carries its evidentiary
basis in an appendix. The examples reference objectively verifiable
crypto-native facts — ledger states, program events, prices, ranges, path
statistics — not politics, sports, gaming, or subjective social events. The
Committee's duties are advisory; I ask only that it recommend work.

Respectfully submitted, [FULL NAME]
