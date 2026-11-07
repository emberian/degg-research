import Lake
open Lake DSL

-- Lets other Lake projects depend on the device model, e.g. veil-pilot's refinement proof:
--   require chokepoint from git "https://github.com/emberian/degg-research.git" @ "<rev>" / "experiments/chokepoint-step"
-- `check.sh` stays the way to check the whole experiment, including CommitBytes and its BLAKE3.
package chokepoint

lean_lib AriadneStep
lean_lib GrayCounter
