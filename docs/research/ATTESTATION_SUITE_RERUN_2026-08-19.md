# Attestation suite rerun: a reproduced test record

Converting the attestation crates' recorded-but-unreproduced test claims into a
freshly executed measurement on a clean tree.

Status: reproduction record, executed 2026-08-18. This document exists to move
three Breadstuffs attestation test suites from *recorded* (a claim that a suite
passes, carried forward from an earlier session's report) to *reproduced* (a
suite observed passing, on a named commit, on named hardware, with the exact
commands and counts written down). It is a measurement log, not a design
document and not a soundness argument. No source file was modified to produce
it.

Filename note: this record is filed under the `2026-08-19` slug it was
commissioned under; the run itself executed on 2026-08-18 (UTC timestamps
below). The timestamps, not the filename, are the measurement.

## Claim discipline

Labels follow `AGENTS.md`. In this document:

- VERIFIED marks facts obtained by running the command and reading its output
  in this session.
- CHAT-REPORTED marks the prior status of these same suites: asserted in
  earlier session notes but not reproduced. This is the "gate R" state that
  this record is intended to retire.
- INFERRED marks a stated deduction from the observations.

The gate conversion this record performs is exactly: the three suites below
move from CHAT-REPORTED to VERIFIED. Nothing else in the repository's verdict
set is touched.

## 1. What was tested, and what pins it

VERIFIED. The tested tree is the Breadstuffs working repository at commit

    436c2a865a0a0e6b8222050ef27464750a0471d7

with root tree object `98f3fdd433d44c214c2e2ea61bd4db7959a97b12`.

VERIFIED. The tree shipped to the test host was produced by `git archive HEAD`,
so it contains the committed tree exactly and none of the working-copy
modifications. At the time of the run the local working copy carried three
tracked modifications and six untracked files:

    MM circuit-prove/tests/leaf_vs_recursion_sweep.rs
    MM fhegg-fhe/src/bin/ntt_four_step_bench.rs
    MM fhegg-fhe/tests/kpz_encoding_depth.rs

INFERRED. None of those three modifications is in the compiled surface of this
run. Two are in `fhegg-fhe`, which no suite here selects. The third is a
sibling integration-test file in `circuit-prove`; because suite 3 was filtered
to `--test dyck_parse_tamper`, that file was never compiled. The record
therefore pins the committed tree, not a dirty tree.

VERIFIED. Byte-identity of the shipped tree was spot-checked by SHA-256 on both
ends:

    Cargo.lock
      51be5800e17dc464edc4b5c8c34abf242543b23e211a6c036c9cb5e2d1326033
    circuit/descriptors/by-name/dyck-parse.json
      00628482829968f579a6e75c749f78a0567bae2b008780e39b4e22513545ef8a

Both matched between the local repository and the test host.

## 2. Environment

VERIFIED. Test host `persvati`, a 24-core Linux box:

    Linux persvati 6.17.0-40-generic #40-Ubuntu SMP PREEMPT_DYNAMIC
      Fri Jun 19 16:42:13 UTC 2026 x86_64 GNU/Linux
    nproc                24
    memory               83 GiB total, 76 GiB available at launch

VERIFIED. Toolchain, resolved through the repository's own pin
(`rust-toolchain.toml`, `channel = "nightly-2026-06-21"`):

    rustc 1.98.0-nightly (8b6558a02 2026-06-20)
    cargo 1.98.0-nightly (a595d0da2 2026-06-20)
    active toolchain  nightly-2026-06-21-x86_64-unknown-linux-gnu
      (overridden by the tree's rust-toolchain.toml)

INFERRED. The host default toolchain was a later nightly
(`1.98.0-nightly (13f1859f2 2026-06-27)`); the tree's pin took precedence, so
this run measures the pinned toolchain rather than the host default. That is
the intended behaviour of the date-pin.

VERIFIED. Run window: `2026-08-18T08:27:53Z` to `2026-08-18T08:32:32Z`.

VERIFIED. Working directory on the host:
`/home/ember/swarm-runs/breadstuffs-b-conversion`. The target directory started
empty, so every build below is a cold build against a warm cargo registry and
git cache already present on the host.

## 3. Commands and results

VERIFIED. Each suite was run exactly as written, from the tree root, with
`CARGO_TERM_COLOR=never`, sequentially, sharing one target directory.

| # | Command | Result | Tests | Wall |
|---|---------|--------|-------|------|
| 1 | `cargo test -p dregg-zkoracle-prove --locked` | ok (rc=0) | 61 passed, 0 failed, 0 ignored | 50 s |
| 2 | `cargo test -p dregg-zkoracle-live --test model_provenance_fused --locked` | ok (rc=0) | 6 passed, 0 failed, 0 ignored | 64 s |
| 3 | `cargo test -p dregg-circuit-prove --test dyck_parse_tamper --locked` | ok (rc=0) | 19 passed, 0 failed, 0 ignored | 99 s |

Total wall time for all three, including all compilation: 213 s (3 min 33 s).

VERIFIED. Zero failures, zero panics, and zero ignored tests across all three
suites. No test was filtered out at runtime in any suite ("0 filtered out" on
every result line).

### Suite 1 -- `dregg-zkoracle-prove`

VERIFIED. Five test binaries ran. Verbatim result lines, in execution order:

    Finished `test` profile [unoptimized + debuginfo] target(s) in 49.62s

    Running unittests src/lib.rs
    running 52 tests
    test result: ok. 52 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s

    Running tests/oracle_mark_zktls.rs
    running 2 tests
    test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    Running tests/provenance_gate.rs
    running 4 tests
    test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    Running tests/zkoracle_roundtrip.rs
    running 3 tests
    test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s

    Doc-tests dregg_zkoracle_prove
    running 0 tests
    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

VERIFIED. The 52 library tests group by module as: `attestation` (10),
`authentic` (5), `cfg` (11), `endpoints::github` (5), `endpoints::price` (3),
`injection` (3), `render` (11), `sigv4` (2), `zk_leg` (3). The three
integration binaries contributed `a_tampered_price_never_becomes_a_graded_mark`
and `a_genuine_zktls_price_mints_a_graded_mark`; `policy_admits_exactly_what_it_says`,
`a_fixture_attestation_self_describes_as_a_test_double`,
`a_live_leg_is_refused_fail_closed_without_the_backend` and
`fixture_only_attestation_is_refused_on_the_live_path`; and
`injection_catch_discriminates`,
`full_zkoracle_attestation_accepts_and_hostiles_are_refused` and
`stark_carrying_attestation_roundtrips_and_bad_leg_refuses`.

VERIFIED, and worth flagging against the prior record. `zkoracle-prove/Cargo.toml`
carries an inline note describing a measured 2026-07-27 state in which this
package ran "51 lib tests and FOUR test binaries that reported running 0 tests"
under `-p` selection, because a workspace-wide feature unification decided which
suite existed. This run observes 52 library tests and three integration binaries
each carrying real tests (2, 4, 3). INFERRED: the crate split that replaced the
`tlsn-live` feature with a dependency edge did what it was intended to do, and
the package's `-p` suite is no longer selection-dependent in the way that note
records.

### Suite 2 -- `dregg-zkoracle-live`, target `model_provenance_fused`

VERIFIED. Verbatim:

    Finished `test` profile [unoptimized + debuginfo] target(s) in 58.72s
    Running tests/model_provenance_fused.rs
    running 6 tests
    test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.23s

VERIFIED. The six tests:

    real_mpctls_presentation_authenticates_on_the_live_path
    self_signed_fixture_is_accepted_by_the_legacy_path_and_refused_on_the_live_path
    tampered_real_presentation_is_refused_by_real_crypto
    spliced_evidence_is_refused_even_with_a_real_live_leg
    stark_injection_leg_is_attached_consulted_and_discriminates_on_the_live_path
    injecting_prose_is_refused_over_a_real_live_session

VERIFIED. This target performs no external network I/O. It drives a
`FixtureNotary` and the in-process TLSNotary server fixture; the only
external-looking hostname in the file is inside a doc comment. "Live" in these
test names denotes the real MPC-TLS 2PC code path with real cryptography, not a
connection to a third-party service.

### Suite 3 -- `dregg-circuit-prove`, target `dyck_parse_tamper`

VERIFIED. Verbatim:

    Finished `test` profile [unoptimized + debuginfo] target(s) in 1m 38s
    Running tests/dyck_parse_tamper.rs
    running 19 tests
    test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

VERIFIED. Two honest-acceptance tests, one commitment-separation test, and
sixteen tamper canaries, all passing:

    emitted_dyck_brackets_proves_and_verifies
    emitted_dyck_nested_proves_and_verifies
    nested_and_flat_commitments_differ
    depth_next_carries_its_own_range
    emitted_dyck_forged_table_commitment_rejects
    tamper_dropped_remainder_rejects
    tamper_forged_remainder_symbol_rejects
    tamper_initial_depth_rejects
    tamper_input_token_rejects
    tamper_nested_remainder_source_rejects
    tamper_occupancy_hole_below_pointer_rejects
    tamper_occupancy_overclaimed_depth_rejects
    tamper_occupancy_symbol_above_pointer_rejects
    tamper_overflowing_depth_rejects
    tamper_overflowing_push_rejects
    tamper_rule_id_rejects
    tamper_stack_cell_rejects
    tamper_term_consumes_nonterminal_rejects
    tamper_wrapped_depth_rejects

INFERRED. The suite is non-vacuous in the specific sense its own header claims:
honest witnesses accept and each single-tooth tamper rejects, so acceptance is
not trivially universal. That is a property of this test set, not a proof of
the AIR.

## 4. Caveats a reader must carry

This section is the part of the record that matters most; the green counts
above are cheap without it.

VERIFIED. **Lean was not executed on the test host.** `elan`, `lake` and `lean`
are all absent from `persvati`. The `dregg-lean-ffi` build script said so
explicitly during suite 3:

    warning: dregg-lean-ffi@0.1.0: dregg-lean-ffi: cannot resolve the Lean
    sysroot (no DREGG_LEAN_SYSROOT and `lake env` failed in metatheory/) --
    skipping the archive refresh; the existing archive (if any) is used as-is.

VERIFIED. The Dyck descriptor that suite 3 exercises is a committed JSON
artifact, `circuit/descriptors/by-name/dyck-parse.json`, compiled into
`dregg-circuit` by `include_str!` and served by `descriptor_by_name` under the
name `dregg-dyck-parse-v1`. Its authoring source is
`metatheory/Dregg2/Circuit/Emit/DyckStackEmit.lean` (`dyckParseDesc`), byte-pinned
there by an `emitVmJson2` `#guard`.

INFERRED, and this is the load-bearing limit of suite 3. This run reproduces
"the *committed* emitted descriptor accepts the honest witnesses and rejects
each tamper". It does **not** re-execute the Lean emit, and it does **not**
re-check the `#guard` that pins the JSON to `dyckParseDesc`. The link from the
Lean-authored object to the JSON bytes tested here is inherited from the commit,
not reproduced by this measurement. A reader who wants that link reproduced
needs a Lean-capable host and a separate run; this record does not supply it.

INFERRED. Nothing here discharges the underlying proof-system floor. These are
Rust integration tests over finitely many witnesses. They demonstrate that the
deployed prover and verifier behave as intended on the cases enumerated in the
files; they establish nothing about all inputs, and they inherit whatever
undischarged assumptions the FRI/STARK stack carries. "The suite is green" is
the claim being converted to VERIFIED, and it is the only claim being converted.

VERIFIED. **What was deliberately not run.** `dregg-zkoracle-live` has three
further test targets that the `--test model_provenance_fused` filter excluded:
`bedrock_mpctls_live.rs` (3 tests, all three marked `#[ignore]`, targeting
`bedrock-runtime.us-east-1.amazonaws.com`), `notary_durable_key.rs` (1 test) and
`tlsn_live_roundtrip.rs` (3 tests). The Bedrock target reaches a paid external
service and was correctly out of scope; it would have been skipped by its
`#[ignore]` markers even under a wider selection. The other two were simply not
selected. No workspace-wide `cargo test` was run at any point.

INFERRED. Because suite 1 selects a package and suites 2 and 3 select single
test targets, these three commands are not a statement about the workspace's
overall health. They are three filtered measurements and should be cited as such.

## 5. Reproducing this

VERIFIED. The full recipe, as executed:

    # from a machine holding the Breadstuffs repository
    git archive --format=tar 436c2a865a0a0e6b8222050ef27464750a0471d7 \
      | gzip -1 | ssh persvati 'tar xzf - -C ~/swarm-runs/breadstuffs-b-conversion'

    # on the test host, from that directory
    cargo test -p dregg-zkoracle-prove --locked
    cargo test -p dregg-zkoracle-live --test model_provenance_fused --locked
    cargo test -p dregg-circuit-prove --test dyck_parse_tamper --locked

VERIFIED. Transport note. A full `rsync` of the working tree was measured at
30.7 GB across 167k files after excluding `target` and `.git`, against a link
benchmarked at roughly 2.1 MB/s -- about four hours. The `git archive` of the
committed tree is 213 MB gzipped and transferred in 54 s, a roughly 250-fold
reduction, and has the additional property of pinning exactly what was tested.
The bulk excluded is untracked build output: `node_modules`, `.lake`, `dist`,
`build` and `.venv` trees under various members.

VERIFIED. Host artifacts were left in place for reuse at
`/home/ember/swarm-runs/`, occupying 7.6 GB: 6.8 GB of cargo `target` output,
763 MB of source tree, and 128 KB of logs under `~/swarm-runs/logs/`. Host disk
after the run: 550 GB free of 1.9 TB.

## 6. Verdict

VERIFIED. All three named suites pass on commit
`436c2a865a0a0e6b8222050ef27464750a0471d7` under the tree's pinned nightly
toolchain, with 86 tests passing in total (61 + 6 + 19), zero failures and zero
ignored tests, in 213 s of wall time on 24 cores.

The gate conversion is complete for the suites named, and only for them, and
only at the resolution described in section 4: a green Rust test run over
enumerated cases against a committed Lean-emitted artifact whose emit step was
not re-executed here.
