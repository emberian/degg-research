# Energy Draft 3 architecture and claim audit

Status: **internal Markdown-only planning memo; not filing-ready; not legal
advice; not filed, submitted, sent, or published by this work.** Prepared and
primary-source checked on 2026-08-19. This memo does not amend the frozen Draft
9 Typst filings or the separate perpetuals Draft 2.

**Artifact-authoring gate.** The mandatory PDF-skill marker program
`container_tools/mark_artifact_operation_started.mjs` is absent from the
documented skill location and was not found under `/Users/ember`, `/opt`, or
`/usr/local`. The marker therefore could not succeed. Under the applicable PDF
workflow, no Typst or PDF may be created, edited, re-exported, or rendered until
that gate is restored. This Markdown memo is deliberately not a filing source.

## 1. Decision

**PROPOSED: prepare a separately named Energy Draft 3 candidate for the August
26 docket.** Do not overwrite perpetuals Draft 2. Draft 2 is a narrow
reference-price/replay comment; Draft 3 should ask a different technical
question:

> Can commercial energy providers participate in a precisely specified,
> efficient planning or calculation relation while keeping plant-level
> commercial and operational inputs from routine public disclosure, and can
> the released plan or settlement calculation be made verifiably correct and
> regulator-observable under a separately declared access policy?

**INFERRED: this is a high-priority, honest fit to the docket only if it remains
a technical architecture contribution.** The Commission expressly asks about
commercial participants, analytical frameworks, source and physical data,
continuous calculation and settlement, surveillance, and operational
readiness. It does not ask this software project to opine without data on which
energy commodity needs a perpetual contract or whether any product should be
listed.

**REJECTED: treating confidential dispatch research as evidence that an energy
perpetual is useful, lawful, manipulation-resistant, or ready to list.** A
planning relation and a perpetual contract are different objects. Their
connection is limited to the Commission's questions about commercial-user
requirements, confidential physical information, integrity of calculation,
surveillance access, and settlement operations.

The candidate should preserve these statements near its beginning:

- **VERIFIED:** the author has no professional energy-market expertise, energy
  market data, operational plant data, interviews with commercial energy
  participants, or empirical study of an energy venue in this research record.
- **PROPOSED:** confidentiality may be a material participation constraint for
  producers, refiners, merchants, transporters, end-users, or other providers
  whose inputs include commercially or operationally sensitive facts.
- **VERIFIED:** the current executable evidence is Clear, bounded, synthetic,
  and offline, plus a single-executor Shielded baseline that does not provide
  technical confidentiality from that executor.
- **PROPOSED:** both a regulator-observable Shielded threshold design and a
  Dark FHE/vFHE design are research targets, not implemented systems.
- **REJECTED:** any implication that the current work contains an FHE backend,
  vFHE proof, threshold committee, private SBF settlement, production source
  adapter, deployable energy venue, or legal/listing conclusion.

## 2. Official matter and submission facts

### 2.1 Energy request for comment

**SOURCED (retrieved 2026-08-19):** the Commission's request is *Request for
Comment on the Extension of Standard Futures Contracts to 24/7 Trading and on
Perpetual Contracts Referencing Physically Delivered or Storable Energy
Commodities*, RIN 3038-AF75, 91 Fed. Reg. 38334, FR Doc. 2026-12784. The
original notice asks 67 numbered questions and says commenters need not answer
all of them. It encourages data, empirical analysis, transaction- or
market-level statistics, and supporting documents rather than conclusory
assertions.

Official sources:

- [original request, 91 Fed. Reg. 38334](https://www.federalregister.gov/documents/2026/06/25/2026-12784/request-for-comment-on-the-extension-of-standard-futures-contracts-to-247-trading-and-on-perpetual)
- [extension, 91 Fed. Reg. 47158](https://www.federalregister.gov/documents/2026/07/28/2026-15216/request-for-comment-on-the-extension-of-standard-futures-contracts-to-247-trading-and-on-perpetual)
- [Regulations.gov comment form for the extension document](https://www.regulations.gov/commenton/CFTC-2026-1388-0061)

**SOURCED (retrieved 2026-08-19):** the extension moves the deadline to
**Wednesday, August 26, 2026** and uses docket **CFTC-2026-1388**. It also asks
for concrete operational-readiness steps, investments, and system, staffing, or
process changes, with estimated implementation time. This memo recommends a
dependency sequence but no time estimate because the research record has no
empirical staffing or implementation basis for one.

### 2.2 Public-record warning

**SOURCED (retrieved 2026-08-19):** the notices state that comments are
published and warn against including personal identifying information or
business information the submitter does not want published. They direct a
submitter seeking confidential treatment to contact the listed CFTC personnel
before submission and review 17 CFR 145.9. The notices also ask commenters to
use only one submission method.

Consequently, any eventual public draft must contain no private keys, secrets,
confidential provider information, actual operational data, unnecessary
personal data, or unpublished security details. A synthetic relation schema is
appropriate; a provider's real cost curve, outage schedule, fuel constraint,
hedge, inventory, bid, or plant identity is not.

### 2.3 Other legitimate current writing windows

**SOURCED (primary-source recheck 2026-08-19):** the following are the active
windows with a material or plausible connection to this research. “Channel” is
the CFTC-side public docket unless otherwise stated.

| Deadline/status | Matter | Docket and channel | Fit |
|---|---|---|---|
| Aug. 24 | Joint further-definitions RFC, 91 Fed. Reg. 37873, RIN 3038-AF71 / SEC S7-2026-21 | CFTC-2026-1355; [CFTC comment form](https://www.regulations.gov/commenton/CFTC-2026-1355-0001) | High; Draft 10 correction needed |
| Aug. 24 | Joint swap/SBS data-reporting RFC, 91 Fed. Reg. 37877, RIN 3038-AF70 / SEC S7-2026-22 | CFTC-2026-1354; [CFTC comment form](https://www.regulations.gov/commenton/CFTC-2026-1354-0001) | High; Draft 10 correction needed |
| Aug. 26 | Energy 24/7/perpetuals RFC, 91 Fed. Reg. 38334 as extended by 91 Fed. Reg. 47158, RIN 3038-AF75 | CFTC-2026-1388; [extension comment form](https://www.regulations.gov/commenton/CFTC-2026-1388-0061) | High for the carefully bounded Energy Draft 3 |
| Aug. 27 | Innovation Advisory Committee written statements, 91 Fed. Reg. 51697 | CFTC-2026-1717; [comment form](https://www.regulations.gov/commenton/CFTC-2026-1717-0001) | High; Draft 10 correction needed |
| Aug. 31 | Joint portfolio/cross-margining RFC, 91 Fed. Reg. 39579, RIN 3038-AF72 / SEC S7-2026-23 | CFTC-2026-1420; [CFTC comment form](https://www.regulations.gov/commenton/CFTC-2026-1420-0001) | Low without empirical margin/account contribution |
| Oct. 5 | Conflicts and Affiliations NPRM, 91 Fed. Reg. 50926, RIN 3038-AF76 | CFTC-2026-1686; [comment form](https://www.regulations.gov/commenton/CFTC-2026-1686-0001) | Possible later Shielded information-barrier contribution |
| Rolling; no promised response | Innovation written input | [official written-input page](https://www.cftc.gov/About/Innovation/writteninput) | Follow-up, not a substitute for an open docket |
| Rolling; meeting not guaranteed | Innovation meeting request | [official request form](https://forms.cftc.gov/forms/InnovationMeetingRequest) | Focused predeployment questions after public docket arguments |

Other open Paperwork Reduction Act notices concern specific collections of
information rather than the architecture studied here and are out of lane.
They should not receive a comment merely because their periods are open.

**SOURCED (status rechecked 2026-08-19):** the earlier prediction-markets ANPRM,
fintech RFI, Regulation 40.11 prediction-markets proposal, and event-contract
reporting NPRM are closed. Their records remain relevant background, but they
are not current filing channels. Do not characterize an unsolicited late
submission as an ordinary timely comment.

## 3. Exact question map

The table maps only questions with a defensible connection. “Use” means a
possible Draft 3 contribution, not an answer on the underlying energy-market
facts.

| RFC question | Official subject | Claim-safe Draft 3 use | Required refusal |
|---|---|---|---|
| Q32 | Which energy commodities have unmet hedging/risk-management needs; distinguish commercial from speculative demand and identify the relevant perpetual attribute | **PROPOSED:** ask whether avoidable disclosure of provider-level operational and commercial inputs is a design constraint for commercial participation; distinguish confidentiality of planning inputs from anonymity and from product demand | No claim that a commodity needs a perpetual, that providers would use one, or that confidentiality is empirically demanded |
| Q37 | Methods to assess effects on physical markets, commercial hedgers, and the economy | **PROPOSED:** a staged analytical framework with frozen public/private fields, exact synthetic scenarios, independent Clear recomputation, failure cases, and later expert/data validation | No physical-market, price, welfare, or macroeconomic conclusion; synthetic tests are not market evidence |
| Q38–41 | Reference-price source, governance, continuous observability, liquidity, concentration, and cross-market linkage | Preserve Draft 2's reference-program argument, but update it to the exact mock-only source-provenance evidence in section 7 | No claim of a continuously available or manipulation-resistant energy source; no production provider support |
| Q42–44 | Convergence, funding, cost of carry, storage, convenience yield, seasonality, and alternatives | At most identify these as omitted economic state that a credible future relation or funding calculation must model explicitly | Do not propose a funding formula or claim that dispatch optimization captures term structure or convergence |
| Q45–49 | Deliverable supply, storage, constraints, negative prices, outages, supply shocks, delivery and logistics | **PROPOSED:** use ramp limits and forced availability only as tiny synthetic examples of private physical constraints; use signed/exact arithmetic and typed failure for negative/extreme-value tests | No deliverable-supply estimate, storage model, AC power-flow model, outage prediction, historical stress result, or delivery conclusion |
| Q50–51 | Continuous manipulation resistance and timing of funding calculation | Preserve the versioned observation program, frozen time grid, exact calculation, and explicit refusal thesis | Determinism narrows calculation discretion; it does not establish market representativeness or manipulation resistance |
| Q52 | Cross-market and physical-market surveillance; access to physical data and information-sharing | **PROPOSED:** separate routine confidentiality, bounded compliance queries, and due-process threshold disclosure; every mode names who can learn what | No assertion that bounded queries or threshold disclosure satisfies Core Principle 4 or any recordkeeping/investigative duty |
| Q53 | Required demonstration and evidentiary basis | Offer a layered evidence record: authentic source, exact relation, admitted-input commitment, independent recomputation or proof, availability/inclusion, failure/recovery, empirical market evidence, and surveillance | Clear replay proves only deterministic recomputation conditional on inputs; current research supplies only selected technical layers |
| Q60–62 | Clearing, margin, default management, margin calibration, and automatic liquidation | Usually omit. The current research does not model energy margin, default waterfalls, or liquidation. A typed-abort discussion may illustrate why failure policy must be explicit but is not an answer to these questions | No margin number, default-management proposal, liquidation claim, or fully collateralized exemption theory |
| Q63 | Operational considerations for continuous funding calculation and settlement | **HIGH fit:** distinguish calculation from value transfer; bind the released calculation to settlement; name banking-cycle, default, liveness, retry, and privacy dependencies; explain that public transfers can undo computation privacy | No continuous funding implementation, banking-rail integration, private SBF settlement, or production recovery path |
| Q66 | Objective, generally applicable criteria for supporting a perpetual contract | **PROPOSED:** technical evidence criteria in section 8, offered as necessary architecture questions rather than sufficient listing criteria | No claim that satisfying the technical criteria establishes Core Principle compliance or product suitability |

### 3.1 The precise Q32 contribution

Q32 asks for actual hedging or risk-management needs. This repository cannot
answer that factual question. The candidate can contribute a narrower research
hypothesis:

**PROPOSED:** if participation in a joint planning or settlement calculation
requires routine disclosure of plant-level cost curves, minimum/maximum output,
ramp constraints, forced availability, inventory, hedge state, or bids, then
the disclosure itself may change who participates and what they reveal. That
hypothesis should be tested with energy experts and commercial participants;
it should not be presented as established demand.

### 3.2 The precise Q37 contribution

**PROPOSED analytical ladder:** evaluate architecture before product claims.

1. Freeze the economic relation and exact integer domains.
2. Declare every public input, private input, public output, owner-local output,
   and permitted failure disclosure.
3. Build a Clear exhaustive or independently recomputed oracle over a bounded
   synthetic domain.
4. Test conservation, feasibility, canonical optimality/tie-breaking,
   overflow, omission, unavailability, and contradictory-input behavior.
5. Obtain independent energy-domain review of what the model omits.
6. Introduce real, appropriately licensed data only under an explicit data and
   confidentiality protocol.
7. Evaluate Shielded and Dark backends against the same semantic corpus.
8. Separately study physical-market, hedging, price, and distributional effects.

This framework makes falsifiers visible. It does not make a toy dispatch
relation representative of any energy market.

### 3.3 The precise Q63 contribution

The comment should distinguish four obligations that are often collapsed into
“settlement”:

1. calculate an output from the complete admitted input set;
2. establish that the calculation followed the frozen relation;
3. authorize the exact economic effects implied by that output; and
4. transfer value through an operational payment and default-management path.

**VERIFIED:** current local research contains bounded models and selected
execution evidence for parts of obligations 1–3 in other synthetic settings.
It contains no continuous energy funding path, banking integration, private
SBF settlement, default waterfall, or operational energy deployment.

**INFERRED:** confidentiality is end-to-end. A Dark calculation followed by a
public account-and-amount settlement graph is pre-settlement confidential at
most. A serious energy architecture must specify whether settlement is Clear,
Shielded, or Dark separately from computation.

## 4. Three modalities; never merge them

### 4.1 Current Clear evidence

In Clear mode, one process sees all synthetic inputs and outputs and anyone can
recompute the result.

Current reusable evidence:

- **VERIFIED:** `dark-fba/n4-k4-q15/v0` is a frozen, bounded, exact-integer
  clearing relation with canonical ties and allocation rules. It is not an
  energy relation.
- **VERIFIED:** `experiments/relation-ir` represents that relation as canonical
  data and has only a Clear lowering. The lowering agreed with two local
  oracles over **2,116,916** enumerated cases at the documented bounds. This is
  bounded differential evidence, not formal proof or energy validation.
- **VERIFIED:** the Clear relation-IR refuses Shielded and Dark lowering targets;
  visibility annotations are types, not privacy mechanisms.
- **VERIFIED:** `experiments/inclusion-availability` implements a deterministic
  offline append-only admission/receipt/abort model with **131 tests** at its
  documented bounds. Its “availability” is a declared integer, not a network,
  erasure-coding, sampling, consensus, or economic guarantee.

The correct Draft 3 use is methodological: a frozen relation, canonical bytes,
exact replay, explicit failure, and a complete input-set commitment are
checkable evidence layers. They do not establish confidential execution,
energy relevance, source truth, or market integrity.

### 4.2 Proposed regulator-observable Shielded alternative

**PROPOSED:** a named threshold committee could operate under a frozen access
and disclosure contract. The design must state:

- the committee membership and corruption threshold;
- whether members see shares, reconstruct raw inputs, or receive bounded query
  outputs;
- enrollment, key generation, rotation, replacement, recovery, and historic
  ciphertext exposure;
- who may authorize access, the legal/process predicate, scope, time range,
  notice/contest rules, and emergency handling;
- an append-only request/disclosure receipt and sanctions for unauthorized
  release;
- bounded encrypted surveillance queries versus raw-book disclosure;
- censorship, refusal, timeout, and terminal refund/recovery behavior; and
- which information remains visible through ingress, timing, identities, and
  settlement even if computation inputs are encrypted.

This is **Shielded**, not Dark: the authorized threshold may learn more than a
frozen leakage function.

**VERIFIED counterexample:** the present single-executor Shielded baseline does
not establish output correctness. Over one fixed admitted set it enumerated
1,125 alternative published runs; 377 passed every modeled check, including all
33 well-formed public results. The checks constrained the fill vector but did
not constrain clearing tick or aggregate volume. Splitting decryption among a
committee changes confidentiality trust, not this correctness gap.

Therefore the Shielded alternative still needs verifiable evaluation. A
threshold signature or attestation is not silently upgraded into a proof.

### 4.3 Proposed Dark FHE/vFHE target

**PROPOSED:** in a Dark mode, no actor learns more than the frozen public
leakage, its own input, and its authorized local output, within an explicit
corruption, topology, availability, and cryptographic model. For the energy
thesis, the target is not “arbitrary encrypted computation”; it is a small
family of exact, fixed-shape relations over provider constraints and plans.

A Dark target needs all of the following before the word is supportable:

- a concrete encrypted backend and parameter set;
- constrained key generation, refresh, release, and local-output delivery;
- a proof or other sound verifiable-evaluation object binding the exact input
  root, relation identifier, output commitments, and settlement authorization;
- non-equivocating inclusion and real payload recoverability;
- a frozen leakage function and a simulator or comparable noninterference
  statement;
- a settlement layer that does not reconstruct participant, account, and
  amount relationships; and
- explicit abort, censorship, recovery, and adaptive-corruption treatment.

**VERIFIED negative boundary:** this repository has no implemented FHE backend,
multi-party/threshold FHE protocol, vFHE system, proof-carrying release, TPU
implementation, private SBF settlement, or Dark composition. `docs/FHE_FRONTIER.md`
is a sourced research map, not implementation evidence.

## 5. Candidate bounded energy relation

This section is a Draft 3 schema, not a promoted implementation claim.

### 5.1 Proposed V0 identifier and shape

**CHAT-REPORTED / WAITING FOR COMMIT:** an engineering lane is implementing
`confidential-energy-dispatch/p3-t3-b2-q4/v0` with:

- exactly three padded provider slots;
- exactly three periods;
- exactly two buses;
- integer output `0..=4`;
- private convex two-segment costs, minimum/maximum output, initial output,
  ramp-up/ramp-down limits, per-period forced availability, and provider
  bus/owner binding; and
- public zonal demand, line limits, system reserve, epoch, instance, and domain.

The planned V0 has implicit on/off choices plus minimum output and is therefore
nonconvex. The proposed Clear oracle is exhaustive bounded search with direct
recomputation verification, not an unsupported primal-dual certificate.

**DO NOT PROMOTE** this paragraph to VERIFIED until a commit, model document,
tests, reproducible commands, corpus/hash, and exact claim ceiling land and are
independently inspected.

### 5.2 Proposed leakage declaration

The leakage declaration must be frozen before any privacy backend is selected.
The following is only a design candidate:

| Category | Proposed visibility |
|---|---|
| relation/domain/version | public |
| planning epoch and fixed dimensions | public |
| provider count | padded capacity only |
| provider identity | credentialed/private; exact authority model unresolved |
| provider bus | private unless the physical model makes it logically implied |
| cost segments | private |
| min/max, initial output, ramp, availability | private |
| demand, line limits, reserve | public in V0 |
| global status and invariant booleans | public |
| input/plan/local-output commitments | public |
| aggregate objective value | undecided; do not publish by default |
| provider-local dispatch and credits | owner-local |
| full schedule | private unless a separate operational recipient is authorized |
| failure | public bounded type; secret-bearing diagnostics forbidden |
| regulator access | separate Shielded threshold/query policy; not part of Dark leakage |
| settlement graph | no private implementation; explicit STOP |

### 5.3 Deliberately absent from V0

**REJECTED as current claims:** storage or inventory dynamics; fuel and
transport constraints; startup/shutdown costs; minimum up/down times; AC power
flow; a real transmission topology; losses; unit commitment at operational
scale; bid-stack semantics; existing hedge portfolios; collateral; margin;
liquidation; default; delivery; banking rails; source authentication for real
energy data; market clearing rules; and any chain settlement adapter.

The user's broader research objective includes inventory, hedge, bid, and
operational state. Those are future relation families, not fields that may be
smuggled into the V0 claim.

### 5.4 Promotion gate for the candidate relation

Before Draft 3 describes the relation as implemented, require:

1. a committed semantic specification and canonical relation identifier;
2. exact public/private/local-output declarations;
3. exact arithmetic widths and refusal behavior;
4. independent evaluator or independently written verifier;
5. tests for feasibility, reserve, ramp, forced outage, line, padding,
   permutation/canonical tie, overflow, infeasibility, and malformed input;
6. a complete enumeration boundary or declared corpus with zero-divergence
   results;
7. reproducible commands, toolchain, date, and artifact digest; and
8. an explicit statement that no energy expert or real data validated it.

## 6. Availability, inclusion, and fairness

**INFERRED:** a correct confidential computation over an operator-selected
subset is not a fair or complete planning result. A credible design needs:

- a cutoff-bound accepted-input commitment;
- one receipt per admitted provider input;
- constant-shape padding if participation count is outside the leakage budget;
- non-equivocation evidence;
- actual ciphertext recoverability before calculation;
- a timeout and typed abort/refund/recovery path; and
- a rule preventing an unavailable input from becoming a convenient empty
  provider slot.

**VERIFIED current boundary:** the local inclusion/availability experiment
supplies checkable offline commitment and state-machine objects but no real
network availability, cryptographic hiding, attribution, or economic liveness.
Draft 3 may use it as an executable specification of questions, not as an
operational solution.

## 7. Source provenance and current Dragon's Clutch evidence

Perpetuals Draft 2 and Draft 9 are stale where they say the source archive is
offline or not joined to Resolve. The replacement claim must be exact.

### 7.1 Supported statement

**VERIFIED by inspection of Dragon's Clutch through commit `44bed19`:**

- live real-SBF `Resolve` is bound to an exact canonical Clutch-owned sealed
  source receipt and refuses same-domain evidence-buffer or PDA substitution;
- a permissionless, canonical, predictable-PDA-prefund-safe SourceSpec, Feed,
  single-page archive, parser-admitted append, one-bucket maturity witness, and
  seal path produces the receipt `Resolve` consumes;
- the source choice is compile-time closed rather than caller-selected;
- the successful construction path was exercised in a bank only through a
  separate **NON-PRODUCTION mock-source ELF**; and
- the default artifact registers zero provider/parser releases and refuses
  source construction/mutation with `SourceReleaseUnavailable`.

The focused mock-ELF run reports 448,896 CU for SourceSpec+Feed initialization,
553,723 CU for archive initialization, 656,962 and 656,346 CU for the two
appends, and 866,718 CU for seal. These are measurements of the recorded local
mock artifact, not budget promises or evidence of a production provider.

Exact implementation records:

- Dragon's Clutch `docs/implementation/AUTHENTICATED_SOURCE_CONSTRUCTION_V1.md`
  at `44bed19`;
- `programs/clutch-sbf/svm-tests/tests/source_ingest.rs` at `44bed19`; and
- `docs/implementation/SOURCE_ARCHIVE_RUNTIME.md` on the joined source/archive
  history beginning at `0b96a3a` and continuing through the current source
  construction commits.

### 7.2 Mandatory STOP statement

**VERIFIED STOPs:** no reviewed production provider ABI, live energy price
source, production deployment authenticator, production parser release,
provider availability guarantee, multipage archive, repair-generation path,
general maturity horizon, cleanup/successor path, or deployable end-to-end
source pipeline exists. The mock uses synthetic provider bytes. Do not say
“production source authentication,” “live feed support,” “operatorless
ingestion,” or “energy oracle integration.”

### 7.3 Claim-safe Draft 3 wording

> Local real-SBF resolution is bound to the exact canonical program-owned
> sealed source receipt and rejects same-domain evidence or account
> substitution. A permissionless canonical single-page construction path now
> creates the source specification, feed, archive, parser-admitted observations,
> and one-bucket maturity receipt that resolution consumes, but this complete
> path has been exercised only in a separate non-production mock-provider ELF.
> The default artifact admits no provider release. No production energy-source
> authenticator, parser, availability proof, multipage archive, or deployable
> source pipeline exists.

## 8. Proposed objective technical criteria for Q66

These are **PROPOSED necessary evidence questions**, not sufficient listing or
legal criteria:

1. **Frozen product and calculation terms.** Does an exact, versioned relation
   identify sources, time, arithmetic, failure behavior, and every public or
   local output?
2. **Authentic inputs.** Does each admitted input bind to the intended source,
   deployment/release, account or credential, time domain, and correction
   lineage?
3. **Complete input set.** Can participants verify inclusion under one cutoff,
   detect equivocation, and recover every required payload?
4. **Exact calculation.** Can an independent implementation recompute the
   output, or can a relying party verify a proof bound to the exact relation and
   input root?
5. **Feasibility and conservation.** Are physical/economic constraints,
   arithmetic widths, rounding, and conservation stated and checked without
   silent approximation?
6. **Canonical choice.** If multiple feasible or optimal plans exist, is the
   objective and tie-break deterministic and resistant to executor choice?
7. **Continuous evidence.** Is the evidence regenerated for every funding or
   planning interval rather than extrapolated from an episodic test?
8. **Failure and recovery.** Are missing input, source outage, proof failure,
   timeout, retry, correction, rollback, and terminal disposition distinct?
9. **Surveillance and access.** Which data are routinely visible, which bounded
   queries are possible, who may authorize exceptional disclosure, and what
   immutable receipt records access?
10. **Settlement binding.** Can only the authorized, once-only economic effects
    of the verified output move value, and what does the transfer surface leak?
11. **Operational dependency map.** Which providers, committees, operators,
    frontends, sequencers, payment rails, and upgrade authorities remain
    indispensable?
12. **Empirical energy evidence.** What real data establish representativeness,
    liquidity, manipulation resistance, physical-market effects, and commercial
    utility independently of software correctness?

## 9. Engineering claim ledger for the candidate

| Candidate claim | Status | Evidence | Honest ceiling | Best RFC use |
|---|---|---|---|---|
| Exact bounded relation can be represented as canonical data and Clear-evaluated | VERIFIED for the existing non-energy FBA relation | `experiments/relation-ir`; 2,116,916-case differential at declared bounds | Method evidence only; no energy relation, proof, or privacy | Q37, Q53, Q66 |
| Complete-input commitment, receipts, non-equivocation classes, and typed abort can be modeled | VERIFIED offline | `experiments/inclusion-availability`; 131 tests at declared bounds | No real availability, network, cryptographic hiding, attribution, or liveness | Q37, Q52–53, Q66 |
| Single-executor Shielded assembly behaves like the Clear evaluator | VERIFIED at declared bounds | `experiments/shielded-baseline`; 52 tests and 90,082-case shared-evaluator differential | Executor sees everything; differential does not independently validate semantics | Q52–53, Q66 |
| Single-executor result checks leave material correctness freedom | VERIFIED bounded counterexample | 1,125 alternative runs; 377 accepted; all 33 well-formed public results accepted | One fixed admitted set; no general cryptographic theorem | Q53, Q66 |
| A threshold committee by itself solves result correctness | REJECTED | Same residual-trust counterexample | Threshold changes confidentiality trust only | Q52–53 |
| FHE/vFHE Dark computation exists locally | REJECTED | `docs/FHE_FRONTIER.md` is a sourced research map | No backend, parameters, proof, or composition | Architecture limits only |
| Confidential energy dispatch V0 is implemented | CHAT-REPORTED / WAITING FOR COMMIT | In-flight `confidential-energy-dispatch/p3-t3-b2-q4/v0` lane | Do not promote until section 5.4 gates are inspected | Q32, Q37, Q42–53, Q63, Q66 |
| Real energy cost/ramp/outage/inventory/hedge/bid data were studied | REJECTED | No such data in the inspected repository | Synthetic inputs only | Mandatory limitation |
| Live source receipt is bound to SBF Resolve | VERIFIED, scoped | Dragon's Clutch source/archive history through `44bed19` | Mock-only construction; default registry empty; no energy provider | Q38–41, Q50–53 |
| Native payout semantics execute on Solana | VERIFIED, scoped to local SBF | Dragon's Clutch `6a826b6`: categorical degree 0 and native B-spline degrees 1–3; native point resolve/internal redeem campaign | No native bearer exit; not energy dispatch; no production deployment | Supporting exact-calculation example only |
| Permissionless blank-bank market construction exists | VERIFIED, scoped | Dragon's Clutch `7cf7150`: ordinary-wallet categorical/native construction; d0 916,052 CU, d1 909,302 CU | Terms→Grid existence, Feed/Epoch, native external paths remain STOP | Operational architecture example, not operatorlessness |
| Candidate construction means candidate selection is live | REJECTED | Dragon's Clutch `1835b79` creates only `SUBMITTED` candidate/feed | Does not verify, select, clear, freeze receipt, or authorize settlement | Q53/Q63 limit |
| Canonical selection is live SBF | REJECTED | Dragon's Clutch `471462f` is an offline fixed-layout model | Live Epoch schedule/selection/entitlement chain absent | Q53/Q63 limit |
| Onchain clearing is implemented | REJECTED | Dragon's Clutch `f529460` is one prefrozen, same-page, full-fill, direct, single-claim, zero-fee slice | No general selection, partials, portfolios, fees, virtual legs, terminal closure, or private settlement | Q63 bounded example only |
| The complete protocol is formally verified | REJECTED | Separate Lean model proofs, one narrow Verus seam, tests, and SBF execution are different evidence types | No whole-protocol refinement or production proof | Q53/Q66 evidence taxonomy |
| Permissionless steps make the system operatorless | REJECTED | Construction and selected transitions remove particular privileges only | Sources, inclusion, selection, upgrades, settlement, frontends, and legal roles remain separate questions | Q66 dependency map |

## 10. Draft 2 to Energy Draft 3 delta

Draft 3 should be a new document, not a revision that erases Draft 2's narrow
history.

### Keep, but compress

- the reference price as a versioned observation program;
- frozen sampling grid, window, correction, and failure semantics;
- manipulation-cost analysis as an assumption-indexed envelope, not a venue
  fact; and
- replay as deterministic recomputation conditional on admitted inputs.

### Correct

- replace “source authentication is an assumed input contract, not
  implemented” with the exact mock-only statement in section 7;
- replace any “offline accumulator only” implication with the distinction
  between a joined canonical receipt in local real-SBF and the absence of a
  production provider release;
- keep the categorical distinction between deterministic calculation,
  authentic/representative source data, and manipulation resistance.

### Add

1. the commercial-provider confidentiality hypothesis, explicitly unvalidated;
2. the Clear / Shielded / Dark three-mode architecture;
3. the bounded energy relation schema and its omissions;
4. the single-executor residual-correctness counterexample;
5. inclusion/availability as a precondition for fair confidential computation;
6. Q63's calculation / verification / authorization / transfer split;
7. the settlement-graph leakage warning;
8. the Q66 evidence criteria; and
9. the operational-readiness dependency sequence below.

### Do not add

- a product pitch;
- an energy-market demand conclusion;
- claims about real providers, plants, commodities, prices, storage, or
  transmission;
- an FHE performance number or accelerator claim;
- “privacy-preserving,” “trustless,” “operatorless,” “formally verified,” or
  “production-ready” without a local qualifier that makes the weaker statement
  exact; or
- a view on whether any specific energy perpetual should be listed.

## 11. Operational-readiness sequence for the extension notice

The extension asks for steps, investments, system/staff/process changes, and
time estimates. The candidate can responsibly provide a **dependency order**
and explicitly decline unsupported timing estimates:

1. **Energy-domain co-design.** Energy economists, commercial users, physical
   operators, market surveillance, clearing/settlement, and privacy/security
   reviewers define the real relation and unacceptable disclosures.
2. **Data/source semantics.** Identify authoritative sources, deployment and
   parser releases, time/finality/correction rules, availability, licensing,
   and outage behavior.
3. **Clear reference relation.** Freeze exact arithmetic, constraints,
   objective, tie-break, failure states, and independent test oracles.
4. **Admission and availability.** Add authentic credentials, cutoff-bound
   inclusion, non-equivocation, recoverability, censorship evidence, and
   terminal recovery.
5. **Confidential backend selection.** Compare a named Shielded threshold
   design and specialized Dark FHE candidates against the same relation and
   leakage contract.
6. **Verifiable evaluation.** Bind a proof or independently sound verification
   object to the complete input root, relation, output, and local releases.
7. **Regulatory observability.** Specify routine data, bounded queries,
   exceptional threshold disclosure, process, notice, logging, and abuse
   response.
8. **Settlement relation.** Bind calculation to once-only value movement;
   model banking cycles, margin/default if applicable, and settlement leakage.
9. **Adversarial and formal review.** Independent implementations, overflow and
   refinement work, cryptographic review, privacy analysis, source attacks,
   failure injection, recovery, and economic red-team studies.
10. **Production operations.** Reviewed provider adapters, staffing,
    monitoring, incident response, key ceremonies, upgrade governance, audits,
    and staged pilots under an identified legal path.

**REJECTED:** attaching dates to these steps from the current research record.
Time estimates require a selected relation/backend, energy participants,
operational requirements, and staffing plan.

## 12. Draft 9 to Draft 10 engineering delta

The existing Draft 9 filings should branch to Draft 10 now rather than wait for
a production source adapter. Deadline pressure does not justify leaving a
materially stale negative claim in place.

### 12.1 Mandatory source correction

Definitions, data-reporting, IAC, and any reused perpetuals language must stop
saying there is no live archive-to-Resolve join. Use section 7.3's wording and
retain the default-empty-registry/mock-only/production-provider STOPs.

Exact stale locations inspected on 2026-08-19:

- `docs/regulatory/typst/definitions/body.typ:92-93,422`;
- `docs/regulatory/typst/data-reporting/body.typ:74,139-142,412`;
- `docs/regulatory/typst/iac/body.typ:169-171,389`; and
- `docs/regulatory/typst/perpetuals/body.typ:107-114,262`.

The first three are frozen Draft 9 sources; the last is the separate energy
Draft 2 source. None is edited by this memo.

### 12.2 Candidate, selection, and settlement

- **SUPPORTED:** local real-SBF constructs one canonical `SUBMITTED` candidate
  and feed from the narrow frozen direct book.
- **STOP:** that instruction does not verify or select the candidate, close the
  window, clear the Epoch, freeze entitlement, or authorize settlement.
- **SUPPORTED:** an offline no-std fixed-layout model authenticates full-width
  policy/domain identities, maintains an order-independent streaming top three,
  and plans once-only bounded direct selection after an immutable close.
- **STOP:** the selection authority is not live SBF.
- **SUPPORTED:** local real-SBF consumes one exact prefrozen direct entitlement
  atomically in the restricted slice described in the ledger.
- **STOP:** this is not general clearing, a full venue, or private settlement.

### 12.3 Native semantics and construction

Draft 10 may retain the exact supported claims already introduced by Draft 9:
categorical degree 0 and open-clamped B-spline degrees 1–3, separate Lean
theorems over named models, a narrow Verus transfer seam, local real-SBF
subsets, the signed categorical walk, and ordinary-wallet blank-bank
categorical/native construction. It must retain every evidence boundary:

- tests and SBF execution are not formal proof;
- Lean model theorems do not establish whole-program refinement;
- the Verus seam is not whole-protocol verification;
- local ELF execution is not deployment, audit, production readiness, or legal
  compliance; and
- native bearer exit and other STOP paths remain absent.

### 12.4 Operatorlessness

Permissionless, prefund-safe creation and permissionless consumption of a
prefrozen entitlement demonstrate bounded removal of particular actor
privileges. They do not answer who operates sources, admission, selection,
upgrades, surveillance, recovery, frontend distribution, or economic/legal
functions. “Operatorless” remains PROPOSED and must be decomposed milestone by
milestone.

## 13. Filing sequence and opportunity map

This is a planning recommendation only. No filing or contact is authorized by
this memo.

1. **August 24 — joint definitions Draft 10.** Correct source provenance and
   preserve guarded-hole/instrument-definition questions. Official matter:
   [91 Fed. Reg. 37873](https://www.federalregister.gov/documents/2026/06/24/2026-12743/joint-request-for-comment-on-further-definition-of-swap-and-security-based-swap-and-on-alternative),
   docket CFTC-2026-1355.
2. **August 24 — joint data Draft 10.** Correct source provenance and use the
   exact evidence taxonomy; do not claim a production reporting system.
   Official matter: [91 Fed. Reg. 37877](https://www.federalregister.gov/documents/2026/06/24/2026-12742/joint-request-for-comment-on-swap-and-security-based-swap-data-reporting),
   docket CFTC-2026-1354.
3. **August 26 — Energy Draft 3 candidate.** High priority if it can pass
   domain, legal, privacy, source, and claim review. Keep it technical and
   synthetic; respond mainly to Q32, Q37, Q38–41, Q50–53, Q63, and Q66, with
   the carefully bounded Q42–49 connections above.
4. **August 27 — IAC Draft 10.** Present the broader research questions and
   exact STOPs; do not use the IAC statement to evade a topic-specific docket.
   Official notice: [91 Fed. Reg. 51697](https://www.federalregister.gov/documents/2026/08/11/2026-16328/innovation-advisory-committee),
   docket CFTC-2026-1717.
5. **After docket comments — rolling Innovation channel.** A concise meeting
   request or written input can link the public comments and ask focused
   predeployment questions. The Commission's
   [written-input page](https://www.cftc.gov/About/Innovation/writteninput)
   says input is posted without modification; the
   [meeting-request form](https://forms.cftc.gov/forms/InnovationMeetingRequest)
   does not guarantee a meeting; and the
   [meeting log](https://www.cftc.gov/About/Innovation/meetings) is public.
6. **August 31 portfolio/cross-margining RFC — defer absent a real margin
   contribution.** The current energy work does not answer empirical portfolio
   margin, bankruptcy, account, or cross-margin questions. Official matter:
   [91 Fed. Reg. 39579](https://www.federalregister.gov/documents/2026/06/30/2026-13182/joint-request-for-comment-on-further-implementation-of-portfolio-margining-and-cross-margining-of),
   docket CFTC-2026-1420.
7. **October 5 conflicts/affiliations NPRM — later possible Shielded use.** A
   future comment may address information barriers and access governance, but
   it should not be forced into this week's packet. Official matter:
   [91 Fed. Reg. 50926](https://www.federalregister.gov/documents/2026/08/06/2026-15948/conflicts-and-affiliations),
   docket CFTC-2026-1686.

Closed event-contract, prediction-market, fintech, and event-contract-reporting
comment periods are not legitimate current submission windows. Do not late-file
into a closed docket unless the agency reopens it or counsel identifies an
appropriate procedural vehicle.

## 14. Public-comment and ex parte risk

**SOURCED (retrieved 2026-08-19):** the CFTC's
[submission instructions](https://www.cftc.gov/LawRegulation/PublicComments/HowtoSubmit/index.htm)
direct post-April 28 comments through Regulations.gov, say to use only one
method, and warn that comments are public. An eventual filing should be the
minimal public technical record needed to answer the questions.

**SOURCED (retrieved 2026-08-19):** the Commission's
[ex parte communications policy](https://www.cftc.gov/LawRegulation/FederalRegister/finalrules/2019-27103.html)
provides for publication of substantive written or oral ex parte communications
containing significant or material information about the merits of a proposed
rule, and for disclosure of meetings concerning proposed rules, including
attendee names and affiliations.

Practical consequences:

- file docket-specific merits in the docket rather than using Innovation email
  or a meeting as a private substitute;
- assume written input, attachments, meeting identity, and substantive meeting
  content may become public;
- do not submit unpublished security details or provider confidential
  information;
- decide legal-name, affiliation, and project-lineage disclosures consciously;
- label independent technical research accurately and avoid implying an
  employer or nonprofit affiliation without authorization; and
- obtain legal and energy-domain review before any submission.

## 15. Release gates for an actual Energy Draft 3

All must be true before the Markdown plan becomes a filing candidate:

- [ ] mandatory artifact marker restored and succeeds exactly once for the
      artifact-authoring session;
- [ ] energy relation commit/tests inspected, or every reference remains
      explicitly PROPOSED/CHAT-REPORTED;
- [ ] independent energy-domain review of vocabulary and omissions;
- [ ] legal review of scope, statements, identity, affiliation, and submission
      channel;
- [ ] official docket/deadline/comment-document rechecked on filing day;
- [ ] exact source-provenance wording updated to the default-empty/mock-only
      boundary;
- [ ] no FHE, vFHE, threshold, private settlement, production provider,
      deployment, operatorless, formal-proof, or compliance overclaim;
- [ ] all citations use primary official sources and current retrieval dates;
- [ ] privacy/security review for secrets, personal data, business information,
      and unpublished security details;
- [ ] Typst compilation plus full PDF render-and-visual inspection after the
      artifact gate is available; and
- [ ] source/build/output manifest frozen and the author expressly authorizes
      filing.

Until then, the correct label is: **a promising, high-priority technical
comment architecture with a deliberately unfinished evidence boundary.**
