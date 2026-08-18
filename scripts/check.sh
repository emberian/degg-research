#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

if rg -n '[[:blank:]]+$' --glob '*.md' --glob '*.sh' --glob '*.typ' .; then
  echo "trailing whitespace found" >&2
  exit 1
fi

if rg -n '[‐‑‒–—―]' --glob '*.typ' .; then
  echo "non-ASCII dash found in Typst source" >&2
  exit 1
fi

if rg -n -i \
  '(BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY|api[_-]?key[[:space:]]*[:=][[:space:]]*[^<[:space:]]|secret[[:space:]]*[:=][[:space:]]*[^<[:space:]])' \
  --glob '!scripts/check.sh' .; then
  echo "possible secret material found" >&2
  exit 1
fi

python3 - "$repo_root" <<'PY'
import pathlib
import re
import sys

root = pathlib.Path(sys.argv[1])
missing = []
pattern = re.compile(r"\[[^\]]+\]\(([^)]+)\)")

for path in root.rglob("*.md"):
    text = path.read_text(encoding="utf-8")
    for raw in pattern.findall(text):
        target = raw.strip()
        if (
            not target
            or target.startswith(("http://", "https://", "#", "mailto:"))
        ):
            continue
        target = target.split("#", 1)[0]
        resolved = (path.parent / target).resolve()
        if not resolved.exists():
            missing.append(f"{path.relative_to(root)} -> {raw}")

if missing:
    print("missing local Markdown links:", file=sys.stderr)
    for item in missing:
        print(f"  {item}", file=sys.stderr)
    raise SystemExit(1)
PY

echo "degg-research checks passed"
