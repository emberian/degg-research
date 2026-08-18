#!/usr/bin/env python3
"""Deterministic transcript/leakage laboratory for bounded frequent batches.

This is an offline measurement harness, not a cryptographic implementation.  A
Trace contains synthetic ground truth.  ``observe`` projects that truth onto a
named actor's transcript surface, while ``infer`` records only deductions made
from the projected surface.  The Dark surface is deliberately labelled as a
PROPOSED hypothesis; this module does not establish anonymity, FHE, vFHE, or
cryptographic security.
"""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Iterable, List, Mapping, Optional, Sequence, Tuple


RELATION = "specialized-frequent-batch/v1"
PRICE_GRID = (100, 101, 102, 103)
CAPACITY = 8
EPOCH_WIDTH = 10
FIXED_WIRE_BYTES = 64


@dataclass(frozen=True)
class Order:
    slot: int
    owner: str
    side: str
    limit: int
    quantity: int
    submitted_tick: int
    wire_bytes: int
    accepted: bool = True

    @classmethod
    def from_json(cls, item: Mapping[str, Any]) -> "Order":
        return cls(
            slot=int(item["slot"]), owner=str(item["owner"]), side=str(item["side"]),
            limit=int(item["limit"]), quantity=int(item["quantity"]),
            submitted_tick=int(item["submitted_tick"]), wire_bytes=int(item["wire_bytes"]),
            accepted=bool(item.get("accepted", True)),
        )

    def as_dict(self) -> Dict[str, Any]:
        return {
            "slot": self.slot, "owner": self.owner, "side": self.side,
            "limit": self.limit, "quantity": self.quantity,
            "submitted_tick": self.submitted_tick, "wire_bytes": self.wire_bytes,
            "accepted": self.accepted,
        }


@dataclass(frozen=True)
class Trace:
    trace_id: str
    batch_id: str
    market_id: str
    epoch: int
    cutoff_tick: int
    root: str
    orders: Tuple[Order, ...]
    abort: Optional[str] = None
    audit_requests: int = 0
    audit_match_count: int = 0
    audit_record_ids: Tuple[str, ...] = ()

    @classmethod
    def from_json(cls, item: Mapping[str, Any]) -> "Trace":
        audit = item.get("audit", {})
        return cls(
            trace_id=str(item["trace_id"]), batch_id=str(item["batch_id"]),
            market_id=str(item["market_id"]), epoch=int(item["epoch"]),
            cutoff_tick=int(item["cutoff_tick"]), root=str(item["root"]),
            orders=tuple(Order.from_json(x) for x in item.get("orders", [])),
            abort=item.get("abort"), audit_requests=int(audit.get("requests", 0)),
            audit_match_count=int(audit.get("match_count", 0)),
            audit_record_ids=tuple(str(x) for x in audit.get("record_ids", [])),
        )

    def as_dict(self) -> Dict[str, Any]:
        result: Dict[str, Any] = {
            "trace_id": self.trace_id, "batch_id": self.batch_id, "market_id": self.market_id,
            "epoch": self.epoch, "cutoff_tick": self.cutoff_tick, "root": self.root,
            "orders": [o.as_dict() for o in self.orders], "abort": self.abort,
        }
        result["audit"] = {
            "requests": self.audit_requests, "match_count": self.audit_match_count,
            "record_ids": list(self.audit_record_ids),
        }
        return result


def _eligible(trace: Trace) -> Tuple[Order, ...]:
    return tuple(o for o in trace.orders if o.accepted and o.quantity > 0)


def clear_result(trace: Trace) -> Dict[str, Any]:
    """Compute exact integer uniform-price clearing and pro-rata fills."""
    if trace.abort:
        return {"price": None, "volume": 0, "fills": {}}
    orders = _eligible(trace)
    best_price: Optional[int] = None
    best_volume = 0
    for price in PRICE_GRID:
        demand = sum(o.quantity for o in orders if o.side == "buy" and o.limit >= price)
        supply = sum(o.quantity for o in orders if o.side == "sell" and o.limit <= price)
        volume = min(demand, supply)
        # Ascending scan + strict > is the frozen lowest-price tie rule.
        if volume > best_volume:
            best_price, best_volume = price, volume
    if best_volume == 0:
        return {"price": None, "volume": 0, "fills": {}}

    fills: Dict[str, Dict[str, int]] = {}
    for side in ("buy", "sell"):
        eligible = tuple(o for o in orders if o.side == side and
                         ((o.limit >= best_price) if side == "buy" else (o.limit <= best_price)))
        total = sum(o.quantity for o in eligible)
        residuals: List[Tuple[int, int, Order]] = []
        for o in eligible:
            numerator = o.quantity * best_volume
            base, remainder = divmod(numerator, total)
            residuals.append((remainder, o.slot, o))
            fills.setdefault(o.owner, {})[str(o.slot)] = base
        residual = best_volume - sum(fills[o.owner][str(o.slot)] for o in eligible)
        for _, _, o in sorted(residuals, key=lambda x: (-x[0], x[1]))[:residual]:
            fills[o.owner][str(o.slot)] += 1
    return {"price": best_price, "volume": best_volume, "fills": fills}


def validate_trace(trace: Trace) -> None:
    if len(trace.orders) > CAPACITY:
        raise ValueError("trace exceeds frozen capacity")
    if len({o.slot for o in trace.orders}) != len(trace.orders):
        raise ValueError("duplicate slot")
    if any(o.slot < 0 or o.slot >= CAPACITY for o in trace.orders):
        raise ValueError("slot outside capacity")
    if any(o.side not in ("buy", "sell") for o in trace.orders):
        raise ValueError("invalid side")
    if any(o.limit not in PRICE_GRID for o in trace.orders):
        raise ValueError("limit outside price grid")
    if any(o.quantity < 1 or o.quantity > 15 for o in trace.orders if o.accepted):
        raise ValueError("quantity outside bounded domain")
    if any(o.submitted_tick > trace.cutoff_tick for o in trace.orders if o.accepted):
        raise ValueError("accepted order is late")
    if len(trace.root) != 64 or any(c not in "0123456789abcdef" for c in trace.root):
        raise ValueError("root must be 32-byte lowercase hex")
    if trace.audit_requests < 0 or trace.audit_requests > 2:
        raise ValueError("audit request budget exceeded")
    if len(trace.audit_record_ids) > 4:
        raise ValueError("audit record budget exceeded")


def _public_base(trace: Trace) -> Dict[str, Any]:
    return {
        "relation": RELATION, "batch_id": trace.batch_id, "market_id": trace.market_id,
        "root": trace.root, "capacity": CAPACITY,
    }


def _surface(trace: Trace, mode: str) -> Dict[str, Any]:
    """Return transcript fields mechanically present for the public observer."""
    if mode not in ("Clear", "Shielded", "Dark"):
        raise ValueError("mode must be Clear, Shielded, or Dark")
    result = clear_result(trace)
    out = _public_base(trace)
    if mode == "Clear":
        out.update({
            "timing": {"epoch": trace.epoch, "cutoff_tick": trace.cutoff_tick,
                       "submitted_ticks": [o.submitted_tick for o in trace.orders]},
            "participation": {"count": len(_eligible(trace)),
                              "owners": [o.owner for o in _eligible(trace)]},
            "size_bucket": {"wire_bytes": [o.wire_bytes for o in trace.orders]},
            "orders": [o.as_dict() for o in trace.orders],
            "abort": trace.abort, "clearing": result,
            "local_fills": result["fills"],
            "regulator_audit": {"requests": trace.audit_requests,
                                 "record_ids": list(trace.audit_record_ids),
                                 "match_count": trace.audit_match_count},
        })
    else:
        # Shielded and the hypothetical Dark surface deliberately use the same
        # fixed public wire shape.  Their actor permissions differ below.
        out.update({
            "timing": {"epoch": trace.epoch, "cutoff_tick": trace.cutoff_tick},
            "participation": {"capacity": CAPACITY},
            "size_bucket": {"wire_bytes": FIXED_WIRE_BYTES, "slots": CAPACITY},
            "abort": trace.abort, "clearing": result,
            "local_fills": "recipient-only",
            "regulator_audit": "threshold-selective" if mode == "Shielded" else "bounded-query",
        })
    return out


def _truth_for_owner(trace: Trace, owner: str) -> Dict[str, Any]:
    result = clear_result(trace)
    mine = [o.as_dict() for o in trace.orders if o.owner == owner]
    return {"own_orders": mine, "own_fill": result["fills"].get(owner, {})}


def observe(trace: Trace, mode: str, observer: str = "public") -> Dict[str, Any]:
    """Project a trace onto one actor's transcript surface."""
    validate_trace(trace)
    public = _surface(trace, mode)
    if observer == "public":
        return public
    if observer == "executor":
        if mode == "Dark":
            # PROPOSED surface: a Dark evaluator receives only the declared
            # public transcript; this is not a claim about a real backend.
            return public
        return {**public, "private_executor_input": [o.as_dict() for o in trace.orders],
                "private_exact_sizes": [o.wire_bytes for o in trace.orders]}
    if observer.startswith("owner:"):
        owner = observer.split(":", 1)[1]
        return {**public, "authorized_local": _truth_for_owner(trace, owner)}
    if observer == "regulator":
        if mode == "Dark":
            return {**public, "authorized_audit": {
                "requests": min(trace.audit_requests, 2),
                "match_count": min(trace.audit_match_count, 4),
                "record_receipts": list(trace.audit_record_ids[:4]),
            }}
        return {**public, "authorized_audit": {
            "requests": trace.audit_requests,
            "orders": [o.as_dict() for o in trace.orders],
            "record_ids": list(trace.audit_record_ids),
        }}
    raise ValueError("unknown observer")


def infer(surface: Mapping[str, Any], mode: str) -> Dict[str, Any]:
    """List deductions from one observed surface, separate from disclosures."""
    if mode not in ("Clear", "Shielded", "Dark"):
        raise ValueError("invalid mode")
    clearing = surface["clearing"]
    inferred: Dict[str, Any] = {
        "batch_closed": True,
        "trade_occurred": int(clearing["volume"]) > 0,
        "volume_is_zero_or_positive": int(clearing["volume"]) >= 0,
        "capacity_upper_bound_on_occupied_orders": int(surface["capacity"]),
    }
    if clearing["price"] is not None:
        inferred["price_is_in_public_grid"] = clearing["price"] in PRICE_GRID
    if mode == "Clear":
        inferred["participant_count_equals_observed_count"] = surface["participation"]["count"]
        inferred["owner_list_is_complete"] = True
        inferred["exact_order_count"] = len(surface["orders"])
    else:
        inferred["occupied_count_is_not_identified"] = True
        inferred["exact_timing_is_not_in_transcript"] = True
        inferred["wire_shape_is_fixed"] = surface["size_bucket"]["wire_bytes"] == FIXED_WIRE_BYTES
    if surface["abort"] is not None:
        inferred["result_is_not_finalized"] = True
    return inferred


def enumerate_observations(trace: Trace) -> List[Dict[str, Any]]:
    """Produce mechanically revealed and inferred facts for all supported roles."""
    rows: List[Dict[str, Any]] = []
    owners = sorted({o.owner for o in trace.orders})
    for mode in ("Clear", "Shielded", "Dark"):
        for observer in ("public", "executor", "regulator", *(f"owner:{x}" for x in owners)):
            projected = observe(trace, mode, observer)
            rows.append({"mode": mode, "observer": observer,
                         "claim_status": "PROPOSED" if mode == "Dark" else "MEASURED",
                         "mechanically_revealed": projected,
                         "inferred": infer(projected, mode)})
    return rows


def canonical_report(traces: Iterable[Trace]) -> Dict[str, Any]:
    rows = []
    for trace in traces:
        validate_trace(trace)
        rows.append({"trace": trace.trace_id, "ground_truth": trace.as_dict(),
                     "clear_result": clear_result(trace), "observations": enumerate_observations(trace)})
    return {"schema": "leakage-lab/v1", "claim_boundary": "offline transcript projection; no cryptographic claim", "traces": rows}


def load_vectors(path: Path) -> List[Trace]:
    with path.open(encoding="utf-8") as handle:
        raw = json.load(handle)
    return [Trace.from_json(x) for x in raw["traces"]]


def main(argv: Optional[Sequence[str]] = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--vectors", type=Path, default=Path(__file__).with_name("vectors") / "v1.json")
    parser.add_argument("--mode", choices=("all", "Clear", "Shielded", "Dark"), default="all")
    args = parser.parse_args(argv)
    traces = load_vectors(args.vectors)
    report = canonical_report(traces)
    if args.mode != "all":
        for row in report["traces"]:
            row["observations"] = [x for x in row["observations"] if x["mode"] == args.mode]
    json.dump(report, sys.stdout, sort_keys=True, separators=(",", ":"))
    sys.stdout.write("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
