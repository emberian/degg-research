# Independence boundary

The oracle crate below was written from `relations/CLEARING_V0.md` and
`docs/research/DARK_FBA_RELATION.md` only. Its full test suite passed before the
author read a single line of `experiments/dark-fba/src/**`. These are the SHA-256
digests of the implementation and tests at that moment (2026-08-18); anything the
differential harness later revealed was recorded as a finding, never edited into
agreement.

```text
3b4b0ac31c4ab13ed4a1bbc462a47018c37fb8a2841fa1ffea417152226bd243  oracle/Cargo.toml
88121b57de59d02945f61a5c666bd4cff2816bd8a0f1cde132d1217fa8c1319d  oracle/src/admit.rs
248db75b3c18e215c69ae5dd85fb918e86ccfa344ad7926d594a455ca85a2e56  oracle/src/apportion.rs
8123f221f3b8bcf1ff1447203f171a396c73662f365702905de035a1335cebe1  oracle/src/book.rs
2480c6d03664a5aefabc33a4b2b4d78f2f844daf829dec935d9e16ff1e721a38  oracle/src/curve.rs
2ed3c78214ea4141f7e25e03a61754f9e0dd610e489b190f79c81a8187443d66  oracle/src/lib.rs
929866088d749c41aef231c61fc09279b2d4419458cb3d071dae71204fc7b96f  oracle/src/params.rs
2ed79c0293056bd273c3f42e0ad7d24451383f78b93c7d8ff6c96831296cb17c  oracle/src/settle.rs
cc577c13025e292d3c06a027faf7728951f3ba5cbd29ee3b94d2a90e6801d95f  oracle/tests/admission.rs
b5e5c8027d0088c5159256a82c76b224bed6196b8b180813516440b4cc550f20  oracle/tests/allocation.rs
bed0fa94b4b1403aa672a0dd1417c1001c8bf327c2cc3a4c84b0110c13eef010  oracle/tests/clearing.rs
befb6e46764c41f4fbe250b21b5615a3a622b884b4551684919e9c586282d9d5  oracle/tests/common/mod.rs
```

Gate state at the boundary: 35 tests passing (16 admission, 11 allocation,
8 clearing), `cargo clippy --all-targets -- -D warnings` clean, `cargo fmt --check`
clean, zero third-party dependencies.

Re-verified after the differential run on 2026-08-18: every digest above is
unchanged. No line of the independent oracle was edited after the existing
implementation's source was read, so the differential compared the pre-read
artifact exactly. The harness under `differ/` was written after that reading and
makes no independence claim; it only translates inputs and compares outputs.

## Post-differential conformance edit, 2026-08-18

After the run above, `DARK_FBA_RELATION.md` section 4.1 froze the relation's
admission-check order for v0, and this oracle was edited to conform to it. The
independence claim above is therefore bounded: it describes the artifact that
the 300,359,333-case differential of section 13.3 compared, and nothing later.
Refusal-class agreement measured after this edit is a conformance check against
a section derived from the other implementation's behavior, not independent
corroboration; the earlier disagreement is the finding, and it stands as
recorded.

The edit is confined to the admission module and the witness struct. Four files
changed; the eight digests unchanged below are the same bytes the differential
ran against, and they include every line of curve construction, tick selection,
apportionment, and settlement, so domains A and B remain a comparison of
independently written clearing code.

```text
037f9384ff6003a69b38c441bd27c29334fdcbbfb2039f465e13291ede58931d  oracle/src/admit.rs        CHANGED
ac19591f6324781c1f168085cb486d6cada0ac68557231a4f8d99577c60f0302  oracle/src/book.rs         CHANGED
e0b57312a6c22518de0f45b466a4ad5f71aab15692b6ff38fa1c90c5f0a63837  oracle/tests/admission.rs  CHANGED
c6c266a4455149055d19585fc0af5746b785742d3e81394ac638c6e52160c90b  oracle/tests/common/mod.rs CHANGED
3b4b0ac31c4ab13ed4a1bbc462a47018c37fb8a2841fa1ffea417152226bd243  oracle/Cargo.toml          unchanged
248db75b3c18e215c69ae5dd85fb918e86ccfa344ad7926d594a455ca85a2e56  oracle/src/apportion.rs    unchanged
2480c6d03664a5aefabc33a4b2b4d78f2f844daf829dec935d9e16ff1e721a38  oracle/src/curve.rs        unchanged
2ed3c78214ea4141f7e25e03a61754f9e0dd610e489b190f79c81a8187443d66  oracle/src/lib.rs          unchanged
929866088d749c41aef231c61fc09279b2d4419458cb3d071dae71204fc7b96f  oracle/src/params.rs       unchanged
2ed79c0293056bd273c3f42e0ad7d24451383f78b93c7d8ff6c96831296cb17c  oracle/src/settle.rs       unchanged
b5e5c8027d0088c5159256a82c76b224bed6196b8b180813516440b4cc550f20  oracle/tests/allocation.rs unchanged
bed0fa94b4b1403aa672a0dd1417c1001c8bf327c2cc3a4c84b0110c13eef010  oracle/tests/clearing.rs   unchanged
```

What changed in the two source files: the quantity domain check now precedes
the limit domain check; the zero-nullifier and repeated-nullifier rules moved
from a separate batch-scoped pass into the per-slot sequence, so an earlier
slot's violation preempts a later slot's; and the witness gained the
custody-binding statement the relation requires and this oracle previously
omitted. Gate state after the edit: 40 oracle tests passing (21 admission, 11
allocation, 8 clearing) plus 8 harness tests, `cargo clippy --all-targets --
-D warnings` clean, `cargo fmt --check` clean, zero third-party dependencies.
