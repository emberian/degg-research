#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_dir="$repo_root/output/pdf"
mkdir -p "$output_dir"

typst compile --root "$repo_root" \
  "$repo_root/docs/regulatory/typst/definitions/main.typ" \
  "$output_dir/joint-definitions-comment-draft-9.pdf"

typst compile --root "$repo_root" \
  "$repo_root/docs/regulatory/typst/data-reporting/main.typ" \
  "$output_dir/joint-data-reporting-comment-draft-9.pdf"

typst compile --root "$repo_root" \
  "$repo_root/docs/regulatory/typst/perpetuals/main.typ" \
  "$output_dir/cftc-perpetuals-comment-draft-2.pdf"

typst compile --root "$repo_root" \
  "$repo_root/docs/regulatory/typst/iac/main.typ" \
  "$output_dir/cftc-iac-written-statement-draft-9.pdf"

typst compile --root "$repo_root" \
  "$repo_root/docs/regulatory/typst/iac-cover/main.typ" \
  "$output_dir/cftc-iac-cover-statement-draft-9.pdf"

pdfinfo "$output_dir/joint-definitions-comment-draft-9.pdf" >/dev/null
pdfinfo "$output_dir/joint-data-reporting-comment-draft-9.pdf" >/dev/null
pdfinfo "$output_dir/cftc-perpetuals-comment-draft-2.pdf" >/dev/null
pdfinfo "$output_dir/cftc-iac-written-statement-draft-9.pdf" >/dev/null
pdfinfo "$output_dir/cftc-iac-cover-statement-draft-9.pdf" >/dev/null

printf '%s\n' "regulatory PDFs built in $output_dir"
