# AGENTS.md

This repository is a public research memory for Dark Egg systems. It is not a
production protocol, a deployment repository, a legal opinion, or a place to
accumulate undifferentiated cryptographic code.

## Mission

Study specialized confidential and verifiable market relations whose exact
semantics are useful in their own right. Prefer a small relation with a complete
leakage, correctness, availability, and settlement story over an arbitrary
encrypted computer.

The repository owns:

- current research verdicts;
- mechanism specifications and typed relation designs;
- experiment plans, measurements, falsifiers, and rejected hypotheses;
- literature and local-lineage maps;
- regulatory research questions and public-comment drafts;
- interfaces that implementation and formal repositories may later consume.

It does not own:

- production Solana programs or clients;
- a deployable dark venue;
- Minidregg or Leanuweave formal sources;
- copied Breadstuffs implementations;
- keys, live orders, customer data, mainnet actions, or regulatory submissions.

## Claim discipline

Every material claim must be marked as one of:

- VERIFIED: directly inspected source, proof, reproducible measurement, or
  authoritative primary source.
- SOURCED: supported by a cited source but not independently reproduced.
- INFERRED: a stated deduction from identified premises.
- PROPOSED: a design or experiment, not a result.
- CHAT-REPORTED: preserved for follow-up but not yet verified.
- REJECTED: falsified, superseded, or outside the honest claim boundary.

The current operational truth lives in docs/VERDICTS.md. Do not create a
competing status document.

## Privacy modes

Use exactly these words:

- Clear: the specified state and computation are public.
- Shielded: a named executor, committee, or auditor may learn private inputs.
- Dark: no actor learns anything beyond the frozen leakage function and its own
  authorized local output, within an explicit corruption model.

Do not call threshold encryption dark merely because one server cannot decrypt.
Do not silently introduce an audit master key into a Dark design. Regulatory
observability, if studied, must be a named modality with a precise access,
threshold, due-process, and leakage contract.

## Repository boundaries and provenance

Dragon's Clutch, Breadstuffs, Minidregg, Leanuweave, and this repository have
different purposes and provenance. Ideas and public mathematics may be freshly
specified here with attribution. Source code, fixtures, constants, generated
artifacts, or theorem text may cross repositories only through an explicit
provenance manifest that records the producer commit, source paths, license,
generator, hashes, and receiver checks.

Do not copy TFHE, HPU, FPGA, WGSL, or vendor code into this repository. Do not
claim clean-room status after inspecting an implementation.

## Safety and authority

Research and offline synthetic experiments are permitted. Ordinary local
commits are default work and need no authorization; pushing and external
publication remain user-directed. This supersedes any stricter commit language
in historical handoff documents. No mainnet or testnet
deployment, transaction construction, signing, key access, order solicitation,
paid data acquisition, regulator contact, public filing, or external publication
is authorized merely by work in this repository.

Regulatory documents are technical research and drafting assistance, not legal
advice. Current legal and agency claims require primary-source citations and a
retrieval date. Public-comment drafts must warn that submissions may be posted
publicly and must not contain private keys, secrets, confidential business
information, unnecessary personal data, or unpublished security details.

## Editing conventions

- Keep experiments deterministic and offline by default.
- Keep exact integers exact. Never use floating point where it changes a market,
  fee, conservation, proof, or settlement result.
- Every benchmark records hardware, software commit, parameters, corpus, warmup,
  repetitions, failure count, and raw artifact digest.
- Every cryptographic result states its trust and leakage model beside its timing.
- Every mechanism states liveness and abort behavior, not only the happy path.
- Prefer refusal to a weakened or ambiguous claim.
- Archive dead claims under notes/archive rather than deleting the history.

