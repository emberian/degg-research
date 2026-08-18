# IAC written statement — addendum candidate: the operatorless agent

Status: **candidate text, not filed, not part of any draft.** Prepared
2026-08-19 by the operatorless-agents research lane as the possible
artificial-intelligence content of the IAC written statement (Docket
CFTC-2026-1717; the meeting notice's published topics expressly include
artificial intelligence, and the current draft statement is silent on it).
It enters the Draft 7 packet only on the author's express go; absent that
word, it is filed nowhere. Supporting analysis and the go/no-go
considerations are in [OPERATORLESS_AGENTS.md](OPERATORLESS_AGENTS.md).

Drafting facts. Register: filing register — first person, plain, no claim
labels, ending with a three-line basis block. Every artifact sentence is at
or below the wording ceilings of the independent artifact survey of
2026-08-19 (corrected second pass); the central artifact sentence is the
survey's own defensible-summary sentence, carried essentially verbatim.
Dashes are written `---` to match the Typst house style, so the text can be
handed to the Draft 7 lane verbatim. Length: about 370 words including
the basis block — above the half-page target, and honestly so: the
ceiling-mandated sentences (the defensible-summary sentence and the
verbatim Lean-emit caveat) account for roughly 90 of them. The sanctioned
discretionary trims, in order — the dashed clause in the second sentence,
then the first paragraph's closing clause ("two facts that cut in
different regulatory directions") — reach about 335 words without touching
any ceiling-bound sentence; going lower means dropping a required beat,
which is a go/no-go consideration, not a drafting one. Suggested
placement: a new
section after "The argument," before
"Requested work products"; it takes no numbered position, so nothing
renumbers, and its basis block travels with it in place of appendix rows
(the Draft 7 lane's call). The memo's one filing gate — the recorded-suite
re-run — has been met:
[ATTESTATION_SUITE_RERUN_2026-08-19.md](../../research/ATTESTATION_SUITE_RERUN_2026-08-19.md)
landed, agreeing with the recorded results (86 tests, zero failures, from
the pinned committed tree), and the basis block cites it with its Lean-emit
caveat. If a live provider session is performed before filing, the
corresponding sentence may be restated only to what that new record
supports, re-audited.

---

## The candidate insert, verbatim

The operatorless agent

The Committee's agenda includes artificial intelligence. The architecture
in this statement extends to a market participant that is itself an AI
agent and has no operator. A published specification fixes the agent's
entire operating loop in advance --- its instruction templates, its input
policy, the tools it may call, and the settlement effect of every step ---
and execution is prepaid, permissionless work: anyone may perform a step
offchain and submit the result, and the ledger accepts a step only if its
certificates verify against the specification. This is Position 2's
publication-versus-operation question in its sharpest form: in such a
design there is no operator to register, and no operator to trust --- two
facts that cut in different regulatory directions.

Part of the certificate stack this needs is real: my local research
artifacts implement a Lean-authored parse/guard STARK and a genuine
TLSNotary 2PC integration, joined by a shared content commitment and
tested for refusal, with no live model-provider session, no onchain posting
path, and no verifiable-inference backend. Proving the whole execution
history is a named, machine-readable gap, so the executing host is
currently trusted; and the transcript leg pins a named notary, which is an
operator for that function. These artifacts have enumerated trusted roles;
they are not an operatorless system, and no such agent exists.

I request approval of nothing. I ask the Committee to take up one question
early: when no one operates, which operator functions --- supervision,
recordkeeping, emergency authority, accountability for harm --- can
verifiable conduct evidence satisfy, which attach to the specification's
author or its executors, and which have no bearer at all.

Basis. Offline research artifacts and one pinned third-party integration,
reviewed by the submitter; test suites independently reproduced from the
pinned committed tree --- 86 tests, zero failures, under the repository's
own toolchain pin (record of 2026-08-19) --- with the Lean emit step not
re-run: the committed emitted descriptor, whose Lean pinning is inherited
from the commit, is what the tamper canaries exercised.

No live model-provider session (a live exchange-API MPC-TLS session was recorded 2026-07-11), no onchain posting path, no verifiable-inference
backend, no deployed agent, no funded market; the executing host and the
pinned notary are trusted.

Research artifacts and open design questions --- not products, offers, or
compliance conclusions.
