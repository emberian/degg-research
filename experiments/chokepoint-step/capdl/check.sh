#!/bin/sh
# Check the capability theorems (bare Lean 4.33.1, about 1 min; decide +kernel over the spec).
# `./gen.sh --check` first confirms the committed specs still match breadstuffs/sel4/chokepoint.system.
set -eu
LEAN="${LEAN:-$HOME/.elan/toolchains/leanprover--lean4---v4.33.1/bin/lean}"
HERE="$(cd "$(dirname "$0")" && pwd)"
O="${OLEANS:-${TMPDIR:-/tmp}/chokepoint-capdl-oleans}"
mkdir -p "$O"
cd "$HERE"
for m in SystemCaps ChokepointSpec ChokepointMutant ChokepointCapTheorems; do
  LEAN_PATH="$O" "$LEAN" --root=. -o "$O/$m.olean" "$m.lean"
done
echo "capdl: all green"
