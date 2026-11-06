#!/bin/sh
# Regenerate the Lean CapDL specs from breadstuffs/sel4/chokepoint.system with the Microkit tool:
#   ChokepointSpec.lean  — the real image's CapDL spec;
#   ChokepointMutant.lean — the same system with persist ALSO mapping nic_mmio (the falsifier).
# Needs the PD ELFs from `make build-chokepoint` in breadstuffs/sel4. Fails if the committed
# ChokepointSpec.lean differs from what the current chokepoint.system produces (--check).
set -eu
SEL4="${SEL4:-/Users/ember/dev/breadstuffs/sel4}"
MK="${MICROKIT:-$HOME/sel4-sdk/microkit-sdk-2.2.0/bin/microkit}"
HERE="$(cd "$(dirname "$0")" && pwd)"
T="${TMPDIR:-/tmp}/chokepoint-capdl.$$"; mkdir -p "$T"
B="$SEL4/build-chokepoint"
"$MK" "$SEL4/chokepoint.system" --search-path "$B" --board qemu_virt_aarch64 --config debug \
  -o "$T/real.img" -r "$T/real.txt" --capdl-json "$T/real.json" >/dev/null
sed 's|<map mr="durable_log"|<map mr="nic_mmio" vaddr="0x6_000_000_000" perms="rw" cached="false" />\n        &|' \
  "$SEL4/chokepoint.system" > "$T/mutant.system"
grep -c 'mr="nic_mmio"' "$T/mutant.system" | grep -qx 2
"$MK" "$T/mutant.system" --search-path "$B" --board qemu_virt_aarch64 --config debug \
  -o "$T/mutant.img" -r "$T/mutant.txt" --capdl-json "$T/mutant.json" >/dev/null
python3 -I "$HERE/capdl_to_lean.py" "$T/real.json" chokepoint > "$T/ChokepointSpec.lean"
python3 -I "$HERE/capdl_to_lean.py" "$T/mutant.json" chokepointMutant > "$T/ChokepointMutant.lean"
if [ "${1:-}" = "--check" ]; then
  cmp "$T/ChokepointSpec.lean" "$HERE/ChokepointSpec.lean"
  cmp "$T/ChokepointMutant.lean" "$HERE/ChokepointMutant.lean"
  echo "capdl: committed specs match chokepoint.system"
else
  cp "$T/ChokepointSpec.lean" "$T/ChokepointMutant.lean" "$HERE/"
  echo "capdl: regenerated ($(shasum -a 256 "$T/real.json" | cut -c1-16) real spec)"
fi
rm -rf "$T"
