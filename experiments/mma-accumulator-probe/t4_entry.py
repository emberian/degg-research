"""Kaggle T4 dry-run entry (initiative 17, workload 3): mma_run.py with exactly the --shrink parameters."""
import os
import pathlib
import subprocess
import sys

here = pathlib.Path(__file__).resolve().parent
out = pathlib.Path("/tmp/accel/mma-out")
# bare-word presets (the Kaggle harness cannot pass dashed arguments through --args)
PRESETS = {"rotation": ["--sections", "copy,f16parity,f16ip,ltparity,towermul"]}
extra = [x for w in sys.argv[1:] for x in PRESETS.get(w, [w])]
cmd = [sys.executable, "-u", str(here / "mma_run.py"), "--out", str(out), "--shrink", "--sku", "T4"] + extra
print("RUN", " ".join(cmd), flush=True)
rc = subprocess.call(cmd)
# a short log of the work dir sizes (results.json itself goes to ACCEL_RESULTS from inside mma_run.py)
print("rc", rc, "results at", os.environ.get("ACCEL_RESULTS"), flush=True)
sys.exit(rc)
