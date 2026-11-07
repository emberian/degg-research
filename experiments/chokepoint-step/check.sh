#!/bin/sh
# Check the chokepoint experiment with bare Lean (no lake, no Mathlib).
# CommitBytes imports minidregg's Lean BLAKE3 (Compiler/Blake3.lean, core-only) from its
# source tree, compiled here into a private olean dir: one BLAKE3, no copy.
set -eu
LEAN="${LEAN:-$HOME/.elan/toolchains/leanprover--lean4---v4.33.1/bin/lean}"
MD="${MINIDREGG:-/Users/ember/dev/minidregg}"
HERE="$(cd "$(dirname "$0")" && pwd)"
O="${OLEANS:-${TMPDIR:-/tmp}/chokepoint-oleans}"
mkdir -p "$O/Compiler"
(cd "$MD" && "$LEAN" --root=. -o "$O/Compiler/Blake3.olean" Compiler/Blake3.lean)
(cd "$HERE" && LEAN_PATH="$O" "$LEAN" --root=. -o "$O/CommitStep.olean" CommitStep.lean)
(cd "$HERE" && LEAN_PATH="$O" "$LEAN" --root=. -o "$O/CommitBytes.olean" CommitBytes.lean)
(cd "$HERE" && LEAN_PATH="$O" "$LEAN" --root=. -o "$O/GrayCounter.olean" GrayCounter.lean)
(cd "$HERE" && LEAN_PATH="$O" "$LEAN" --root=. -o "$O/AriadneStep.olean" AriadneStep.lean)
echo "chokepoint: all green"
