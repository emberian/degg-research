#!/usr/bin/env bash
set -euo pipefail

# Draft 10 regulatory PDFs. Draft 9 sources and outputs are frozen; this
# script builds only from the *-draft-10 sibling source directories created
# by the Draft 10 engineering-claim-delta pass.

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_dir="$repo_root/output/pdf"
mkdir -p "$output_dir"

typst compile --root "$repo_root" \
  "$repo_root/docs/regulatory/typst/definitions-draft-10/main.typ" \
  "$output_dir/joint-definitions-comment-draft-10.pdf"

typst compile --root "$repo_root" \
  "$repo_root/docs/regulatory/typst/data-reporting-draft-10/main.typ" \
  "$output_dir/joint-data-reporting-comment-draft-10.pdf"

typst compile --root "$repo_root" \
  "$repo_root/docs/regulatory/typst/iac-draft-10/main.typ" \
  "$output_dir/cftc-iac-written-statement-draft-10.pdf"

typst compile --root "$repo_root" \
  "$repo_root/docs/regulatory/typst/iac-cover-draft-10/main.typ" \
  "$output_dir/cftc-iac-cover-statement-draft-10.pdf"

pdfinfo "$output_dir/joint-definitions-comment-draft-10.pdf" >/dev/null
pdfinfo "$output_dir/joint-data-reporting-comment-draft-10.pdf" >/dev/null
pdfinfo "$output_dir/cftc-iac-written-statement-draft-10.pdf" >/dev/null
pdfinfo "$output_dir/cftc-iac-cover-statement-draft-10.pdf" >/dev/null

printf '%s\n' "draft 10 regulatory PDFs built in $output_dir"
