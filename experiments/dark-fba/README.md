# Dark FBA Offline Toy

Status: PROPOSED relation experiment; VERIFIED only as an offline deterministic
Rust oracle after the commands below pass.

This standalone crate evaluates one fixed four-slot, four-owner, four-tick
uniform-price frequent batch. It uses exact integers, deterministic ties-low
price selection, and exact largest-remainder pro-rata allocation. It has no
network, cryptography, keys, proof system, wallet, chain adapter, or real asset
support.

The executable modes are:

- `Clear`: the caller is allowed to inspect the whole fixture.
- `ShieldedSingleExecutor`: the one process running the toy sees the whole
  fixture and all local outputs.
- `DarkTarget`: always refused because no Dark backend exists.

The public research specification is
[DARK_FBA_RELATION.md](../../docs/research/DARK_FBA_RELATION.md). Provenance is
recorded in [PROVENANCE.md](PROVENANCE.md).

Run the narrow validation:

```sh
cargo fmt --manifest-path experiments/dark-fba/Cargo.toml --check
cargo test --manifest-path experiments/dark-fba/Cargo.toml
cargo run --quiet --manifest-path experiments/dark-fba/Cargo.toml \
  --bin dark-fba-vectors
```

The checked-in `vectors/v1.txt` is compared byte-for-byte by a unit test. It is
synthetic test data, not a cryptographic artifact or external market record.

