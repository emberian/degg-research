import hashlib
import json
import unittest
from pathlib import Path

from leakage_lab import (
    CAPACITY,
    FIXED_WIRE_BYTES,
    Trace,
    canonical_report,
    clear_result,
    enumerate_observations,
    load_vectors,
    observe,
    validate_trace,
)


ROOT = Path(__file__).parent


class LeakageLabTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.traces = load_vectors(ROOT / "vectors" / "v1.json")

    def test_vectors_are_bounded_and_valid(self):
        self.assertEqual(len(self.traces), 4)
        for trace in self.traces:
            validate_trace(trace)
            self.assertLessEqual(len(trace.orders), CAPACITY)

    def test_exact_crossing_and_lowest_tie(self):
        crossing = clear_result(self.traces[0])
        self.assertEqual(crossing["price"], 101)
        self.assertEqual(crossing["volume"], 7)
        self.assertEqual(crossing["fills"]["alice"]["0"], 5)
        self.assertEqual(crossing["fills"]["bob"]["1"], 2)
        self.assertEqual(crossing["fills"]["carol"]["2"], 3)
        self.assertEqual(crossing["fills"]["dave"]["3"], 4)
        tie = clear_result(self.traces[3])
        self.assertEqual(tie["price"], 100)
        self.assertEqual(tie["volume"], 5)

    def test_abort_and_no_cross_do_not_become_no_trade(self):
        no_cross = clear_result(self.traces[1])
        self.assertIsNone(no_cross["price"])
        self.assertEqual(no_cross["volume"], 0)
        aborted = clear_result(self.traces[2])
        self.assertIsNone(aborted["price"])
        self.assertEqual(aborted["volume"], 0)
        self.assertEqual(self.traces[2].abort, "UNAVAILABLE_BEFORE_CUTOFF")

    def test_mode_surfaces_have_distinct_disclosure(self):
        trace = self.traces[0]
        clear = observe(trace, "Clear")
        shielded = observe(trace, "Shielded")
        dark = observe(trace, "Dark")
        self.assertEqual(clear["participation"]["count"], 4)
        self.assertNotIn("count", shielded["participation"])
        self.assertNotIn("orders", shielded)
        self.assertEqual(shielded["size_bucket"]["wire_bytes"], FIXED_WIRE_BYTES)
        self.assertEqual(dark["local_fills"], "recipient-only")
        self.assertEqual(dark["regulator_audit"], "bounded-query")
        self.assertEqual(shielded["regulator_audit"], "threshold-selective")

    def test_role_boundaries(self):
        trace = self.traces[0]
        executor_clear = observe(trace, "Shielded", "executor")
        self.assertIn("private_executor_input", executor_clear)
        executor_dark = observe(trace, "Dark", "executor")
        self.assertNotIn("private_executor_input", executor_dark)
        owner = observe(trace, "Dark", "owner:alice")
        self.assertEqual(owner["authorized_local"]["own_fill"], {"0": 5})
        regulator = observe(trace, "Dark", "regulator")
        self.assertEqual(regulator["authorized_audit"]["requests"], 1)
        self.assertNotIn("orders", regulator["authorized_audit"])

    def test_enumeration_marks_dark_as_hypothesis(self):
        rows = enumerate_observations(self.traces[0])
        self.assertEqual(len(rows), 3 * (3 + 4))
        self.assertTrue(all(r["claim_status"] == "PROPOSED" for r in rows if r["mode"] == "Dark"))
        self.assertTrue(all(r["claim_status"] == "MEASURED" for r in rows if r["mode"] != "Dark"))
        public_dark = next(r for r in rows if r["mode"] == "Dark" and r["observer"] == "public")
        self.assertTrue(public_dark["inferred"]["occupied_count_is_not_identified"])
        self.assertTrue(public_dark["inferred"]["trade_occurred"])

    def test_report_is_byte_deterministic(self):
        report_a = json.dumps(canonical_report(self.traces), sort_keys=True, separators=(",", ":"))
        report_b = json.dumps(canonical_report(self.traces), sort_keys=True, separators=(",", ":"))
        self.assertEqual(report_a, report_b)
        self.assertEqual(hashlib.sha256(report_a.encode()).hexdigest(),
                         hashlib.sha256(report_b.encode()).hexdigest())


if __name__ == "__main__":
    unittest.main()
