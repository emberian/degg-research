# Leakage laboratory (`leakage-lab/v1`)

Status: PROPOSED experiment with a VERIFIED deterministic transcript projector.
The vectors are synthetic and bounded. The lab is offline, uses only the Python
standard library, and does not contain a protocol, network client, key, proof,
FHE implementation, vFHE implementation, or anonymity claim.

## What is measured

`leakage_lab.py` computes an exact integer frequent-batch result over four price
ticks, then projects each trace onto Clear, Shielded, and a hypothetical Dark
transcript surface. For every surface it reports:

- mechanically present facts (timing, participation, size bucket, order root,
  abort, price/volume, local fills, and regulator-audit fields); and
- deductions from those fields, in a separate `inferred` object.

The supported observers are `public`, `executor`, `owner:<name>`, and
`regulator`. A Shielded executor receives synthetic order inputs. The
hypothetical Dark executor receives only the declared public surface; this is a
testable design hypothesis, not evidence that a backend can provide it.

## Disclosure-budget contract

The Dark surface is accepted by this lab only under this concrete proposed
contract:

1. A batch has at most 8 slots and emits one fixed 64-byte input commitment per
   slot. Public timing is one epoch and cutoff tick; exact submission times are
   excluded.
2. Public participation is a capacity of 8, never an occupied count or owner
   list. Public wire size is one fixed 64-byte bucket and never the input's
   actual byte length.
3. Exactly one 32-byte lowercase-hex input root, one typed abort/final status,
   and one clearing price (or no-trade) plus nonnegative volume are public.
   The root is a transcript field only; this lab does not verify a binding.
4. Local fill records are recipient-only, and are not present in the public
   surface. The owner may see its own synthetic order and fill.
5. The regulator plane permits at most 2 requests per batch and at most 4
   opaque record receipts per batch. A Dark regulator result contains only a
   bounded match count and receipts, never raw order rows. The vector validator
   rejects a trace over either budget.
6. Abort is a public typed class. No private diagnostic, retry count, partial
   clearing result, or hidden winning/losing subset may be added to the public
   transcript without a new contract version.

This is a disclosure accounting rule, not differential privacy and not a
security proof. Timing, endpoint compromise, self-knowledge, correlation with
outside data, and public settlement are not magically removed by the contract.

## Run

From this directory:

```sh
python3 -m unittest -v
python3 leakage_lab.py --vectors vectors/v1.json > report.json
python3 leakage_lab.py --mode Dark --vectors vectors/v1.json
```

The JSON output uses sorted keys and compact separators for byte-stable reports.
The canonical inputs and expected semantic cases are in `vectors/v1.json`:
balanced crossing, no crossing, availability abort, and a lowest-price tie.
