#!/usr/bin/env bash
# Regenerate ../t256_vectors.jsonl from minidregg's Tower256 multiply (prover/src/binary_tower_256.rs on
# lane/accel-tower256-gpu, rev 367f9370d7cef61b4c6cad2a424390b96340c4e3 for the committed file), copied verbatim
# except the TowerElem embed and the test module.  Usage: make_vectors.sh MINIDREGG_CHECKOUT [REV]
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd); REV=${2:-367f9370d7cef61b4c6cad2a424390b96340c4e3}
git -C "$1" show "$REV:prover/src/binary_tower_256.rs" | python3 -c '
import sys
s = sys.stdin.read()
s = s[:s.index("#[cfg(test)]")].replace("use crate::binary_tower::TowerElem;\n", "")
a = s.index("    /// Embed the existing level-six [`TowerElem`]"); b = s.index("    /// `y_k = fpGen k`")
sys.stdout.write(s[:a] + s[b:])' > "$HERE/src/binary_tower_256.rs"
( cd "$HERE" && cargo build -q --release && ./target/release/t256vec 4 256 ) > "$HERE/../t256_vectors.jsonl"
