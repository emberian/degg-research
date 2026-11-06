#!/bin/sh
# Census of where AIR constraints are authored in a breadstuffs checkout (Lean-authored AIR law).
# Read-only. Usage: air_census.sh /Users/ember/dev/breadstuffs
# Counts syntax, not semantics: a hit is a place to look, not a proven violation.
set -eu
ROOT="${1:?breadstuffs root}"
TMP="${TMPDIR:-/tmp}/air-census.$$"
mkdir -p "$TMP"
find "$ROOT" -name '*.rs' -not -path '*/target/*' -not -path '*/.lake/*' -not -path '*/vendor/*' \
  -not -path '*/headver/*' -not -path '*/tests/*' -not -path '*/docs/*' -not -path '*/benches/*' \
  -not -path '*/examples/*' > "$TMP/src.txt"
echo "non-test .rs files: $(wc -l < "$TMP/src.txt" | tr -d ' ')"
echo "distinct include_str!(*.json) artifacts (Lean-emitted descriptors and fixtures):"
tr '\n' '\0' < "$TMP/src.txt" | xargs -0 grep -h -o 'include_str!([^)]*\.json[^)]*)' 2>/dev/null | sort -u | wc -l
echo "Rust-built 'EffectVmDescriptor2 {' literals by file:"
tr '\n' '\0' < "$TMP/src.txt" | xargs -0 grep -c 'EffectVmDescriptor2 *{' 2>/dev/null | grep -v ':0$' | sed "s#$ROOT/##" | sort -t: -k2 -rn
echo "hand 'impl Air<AB> for' blocks (non-test src):"
tr '\n' '\0' < "$TMP/src.txt" | xargs -0 grep -c 'impl<AB[^>]*> *Air<AB> for' 2>/dev/null | grep -v ':0$' | sed "s#$ROOT/##"
echo "builder assert_* constraint calls (non-test src) by file:"
tr '\n' '\0' < "$TMP/src.txt" | xargs -0 grep -c -E '\.(assert_zero|assert_eq|assert_bool|assert_one|assert_zeros|assert_bools)(_ext)?\(' 2>/dev/null | grep -v ':0$' | sed "s#$ROOT/##" | sort -t: -k2 -rn
echo "files that call their own code a Rust twin/mirror of Lean:"
tr '\n' '\0' < "$TMP/src.txt" | xargs -0 grep -l -i 'rust twin\|the twin of\|mirror of lean\|rust mirror' 2>/dev/null | wc -l
rm -rf "$TMP"
