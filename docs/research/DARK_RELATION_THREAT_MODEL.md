# Frequent-batch transcript threat model and leakage laboratory

Status: PROPOSED threat model; VERIFIED offline transcript measurements are
implemented in [`experiments/leakage-lab`](../../experiments/leakage-lab/).
The laboratory is a deterministic accounting exercise over synthetic traces.
It is not a protocol, proof, cryptographic construction, anonymity system, FHE
system, vFHE system, or deployment plan.

## Scope and claim boundary

The subject is a bounded specialized frequent batch with eight padded slots and
the public price grid `[100, 101, 102, 103]`. Each synthetic order has an owner,
side, limit, integer quantity in `1..15`, slot, submission tick, and wire size.
The batch also carries a 32-byte input-root string, cutoff epoch, typed abort
class, and bounded audit metadata. The lab computes a uniform-price result with
an exact integer accumulator and lowest-price tie break, then projects that
ground truth onto actor-specific transcript surfaces.

VERIFIED: the harness uses no network, randomness, floating point, external
packages, keys, chain state, or private data. Four checked-in traces exercise a
crossing batch, no crossing, unavailable input abort, and a lowest-price tie.
`python3 -m unittest -v` runs the tests. Canonical JSON uses sorted keys and
compact separators, so repeated runs produce byte-identical reports.

PROPOSED: a future implementation might aim to approximate the Dark surface.
Nothing in this document or the harness demonstrates that such an
implementation exists. In particular, a root, fixed padding, an opaque field,
or an actor label does not itself hide information.

## Actors and observations

The laboratory distinguishes an observation from the underlying synthetic
truth. An observation is the exact map returned by `observe(trace, mode,
observer)`. The `inferred` map is a second, conservative map containing facts
that follow from the observation, such as `trade_occurred = (volume > 0)`.
Keeping these maps separate prevents an inferred fact from being mistaken for a
wire-level disclosure.

Actors:

- `public`: a transcript reader with no private role;
- `executor`: the named computation actor in Clear and Shielded; in the
  hypothetical Dark surface it receives only the declared public surface;
- `owner:<name>`: a participant who already knows its own submitted input and
  authorized local fill; and
- `regulator`: an actor exercising the declared audit plane.

The owner’s own input and output are out of scope for privacy against that owner.
The laboratory does not try to remove correlation with external timing,
identity, settlement, endpoint compromise, or a user’s chosen order. Those are
threat-model inputs, not claims silently solved by this experiment.

## Transcript surfaces

The following table describes the public projection. “Exact” means the field is
mechanically present; “bucket” means only the stated coarse value is present;
“recipient” means it is absent from the public projection but may be returned to
the named owner; “bounded query” is the separate regulator view.

| Fact | Clear | Shielded | Dark (hypothetical) |
|---|---|---|---|
| Relation, batch, market | exact | exact | exact |
| Timing | epoch, cutoff, every submission tick | epoch and cutoff | epoch and cutoff |
| Participation | occupied count and owner list | capacity 8 | capacity 8 |
| Size bucket | every actual wire size | fixed 64-byte shape | fixed 64-byte shape |
| Order-root | root string | root string | root string |
| Order fields | every order row | absent from public; executor gets rows | absent from public and proposed executor view |
| Abort | exact typed class | exact typed class | exact typed class |
| Clearing price/volume | exact result | exact result | exact result |
| Local fills | all owners’ fills | recipient-only | recipient-only |
| Regulator audit | public metadata; regulator gets rows | threshold-selective metadata; executor remains authorized | bounded query result and opaque receipts |

The root is intentionally treated as a transcript field, not a verified
commitment. `order-root` therefore means “the string was revealed,” not “the
string proves inclusion.” A real relation would need a separate binding,
finality, availability, and non-equivocation contract.

Shielded is a named-executor surface: the executor receives private orders in
the model. That is a measurement of this surface, not a statement that the
executor is trustworthy. Dark is a frozen leakage hypothesis: public facts and
authorized local outputs only, subject to a future corruption and availability
model that this lab does not implement.

## Mechanical leakage and safe inferences

For each trace and each mode/actor pair, the harness emits all mechanically
present fields under `mechanically_revealed` and separately emits deductions
under `inferred`. Examples from `balanced-cross`:

- Clear/public mechanically reveals four occupied orders, their three distinct
  owners, submission ticks `94, 98, 95, 99`, and sizes `64, 96, 64, 128`.
  It therefore permits the exact participant count and exact order count to be
  inferred.
- Shielded/public mechanically reveals capacity `8`, one fixed wire-size
  bucket, and the public result. It permits the public to infer that a trade
  occurred when volume is `7`, but not the occupied count from this transcript.
- Dark/public has the same proposed public shape in this lab. It mechanically
  reveals the root and result, and the harness marks the unimplemented surface
  `PROPOSED`; it does not mark occupancy, identity, or exact timing as revealed.
- In a typed abort trace, the abort class is public and the result is not
  finalized. A no-cross trace and an availability-abort trace are distinct
  ground-truth states even though both have volume zero. Treating them as the
  same “no trade” outcome would erase an availability disclosure and create a
  liveness error.
- A clearing price in the frozen grid and a nonnegative integer volume are
  public in every modeled surface. The price/volume pair can imply trade/no
  trade; it does not by itself identify an order or prove root binding.

INFERRED: fixed capacity bounds occupied orders by eight. It does not establish
that eight participants exist, that slots are occupied, or that a padded wire
has cryptographic indistinguishability. INFERRED: a public root can be used for
transcript correlation across observers if the same root is repeated. Whether
that correlation identifies a person depends on external data and is not
measured here.

## Clearing and local-output semantics

For each price `p`, the exact synthetic oracle computes:

```text
demand[p] = sum(q for buy orders with limit >= p)
supply[p] = sum(q for sell orders with limit <= p)
volume[p] = min(demand[p], supply[p])
```

The first price with maximal volume is selected by ascending scan and strict
greater-than update. Allocations use integer pro-rata division; residual atoms
go to largest remainders, then lowest slot index. Tests assert the balanced
crossing result `(price=101, volume=7)` and the lowest-price tie result
`price=100`. These are semantic test measurements, not confidentiality results.

The public result is deliberately small but not harmless: price and volume can
reveal market conditions, strategy response, and temporal correlation. A local
fill is recipient-only in Shielded and Dark projections, while an owner’s own
order and fill are authorized local knowledge. Public settlement is outside this
lab; if a composed system publishes account-and-amount transfers, those
transfers must be added to the leakage function before making an end-to-end
claim.

## Concrete disclosure-budget contract

The proposed Dark transcript is versioned `specialized-frequent-batch/v1` and is
accepted by the harness only under this budget:

1. Capacity is exactly 8. Public timing is one epoch and cutoff tick. Exact
   submission ticks, retry timing, and queue position are not public fields.
2. Public wire shape is a fixed 64-byte bucket per slot. Actual input size and
   variable message count are not public fields.
3. The public batch emits relation/batch/market identifiers, one 32-byte root,
   one typed final status or abort, and one exact clearing price/no-trade plus
   nonnegative volume. No private diagnostic, partial result, or secret-bearing
   failure text is allowed.
4. Public participation is only capacity 8. Occupied count, owner list, side,
   limit, quantity, reservation, and order position are excluded.
5. Local fills are recipient-only. A local output may contain only that owner’s
   bounded fill and delta; the public transcript cannot contain a fill vector.
6. The regulator plane allows at most 2 requests per batch and at most 4 opaque
   record receipts. A Dark regulator response contains a bounded match count
   and those receipts, not raw order rows. A Shielded threshold audit is a
   different, explicitly broader surface.

The vector validator rejects a synthetic trace that exceeds capacity or either
audit budget. Changing any budget, fixed shape, public result, or abort rule is
a new relation/surface version, not a silent implementation detail.

This is a disclosure accounting contract, not differential privacy, a simulator
proof, or a cryptographic security definition. It does not promise anonymity,
noninterference, confidentiality under corruption, resistance to traffic
analysis, or hidden settlement.

## Threat cases and falsifiers

The following cases must remain separate in future experiments:

- a public exact submission tick identifies an order even if payload fields are
  hidden;
- an unpadded number of messages reveals participation;
- variable ciphertext/proof sizes reveal quantity or branch behavior;
- a root published to some observers but not others permits equivocation;
- an unavailable payload is converted to an empty slot rather than a typed
  abort;
- retries or abort timing reveal whether a batch crossed;
- local-fill delivery leaks a user’s order through a public notification;
- the audit plane silently decrypts the entire book despite a “Dark” label; and
- public settlement reconstructs the participant graph.

Any such observation is a falsifier of the proposed surface contract. A timing,
traffic, endpoint, or settlement measurement must be added to the transcript
before a stronger claim is considered.

REJECTED: “Dark because encrypted,” “Dark because threshold,” “Dark because a
root is public,” “Dark because a proof is verified,” and “Dark except for a
master audit key.” The last is Shielded or another explicitly named modality
unless a future, separately reviewed disclosure definition says otherwise.

## Reproduction and evidence labels

From `experiments/leakage-lab/`:

```sh
python3 -m unittest -v
python3 leakage_lab.py --vectors vectors/v1.json > report.json
```

The tests and vector file are the reproducibility artifact. Test outcomes and
the exact integer outputs are VERIFIED measurements of this offline harness.
The privacy surfaces, budget, and Dark role behavior are PROPOSED hypotheses.
The deductions explicitly emitted by `infer` are INFERRED from each projected
surface. No output should be promoted to a production, cryptographic, legal, or
regulatory conclusion without an independent threat model and backend-specific
evidence.
