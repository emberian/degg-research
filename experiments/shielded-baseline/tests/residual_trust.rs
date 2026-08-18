//! Measuring the residue: how many alternative public results survive every
//! owner's complete check battery.
//!
//! The detection suite exhibits one substitution that evades every check. That
//! is an existence proof. This suite bounds the size of the hole instead: over
//! an enumerated space of alternative published runs against one fixed
//! admitted set, it counts how many the composed verifier accepts. The honest
//! run is one of them; everything else in the count is a result the named
//! executor could publish, on that admitted set, with no owner and no public
//! verifier able to object.
//!
//! The enumeration is exact and bounded, and its bounds are the claim's:
//! the cleared tick over the module's five public values, the aggregate volume
//! over `0..=8`, and the per-position fill vector over `0..=quantity` at every
//! committed position. Reservations, sides, limits, and the admitted set
//! itself are fixed at the honest values, so this measures the executor's
//! freedom in the *result*, not its freedom in the witness.

mod common;

use common::{NOW, price_tie_low, session};
use degg_relation_ir::canon::Canonical;
use degg_relation_ir::lower::{ClearedTick, OwnerOutput, PublicOutcome, PublicResult};
use degg_relation_ir::receipt::ReceiptStatus;
use degg_shielded_baseline::executor::Tamper;
use degg_shielded_baseline::owner::{OwnedPosition, OwnerEvidence, audit};
use degg_shielded_baseline::receipt::{
    DeliveryCommitment, DeliveryEntry, ShieldedReceipt, SlotOutcome, public_check,
};

#[test]
fn the_owner_check_battery_leaves_a_measured_space_of_admissible_lies() {
    let scenario = price_tie_low();
    let mut session = session(&scenario);
    let honest = session.compute(&Tamper::None, NOW).expect("computes");
    let module_digest = session.executor.module_digest();
    let cutoff = session.cutoff;
    let binding = honest.run.receipt.cutoff;
    let positions = usize::try_from(cutoff.leaf_count).expect("bounded");

    // The honest result, for reference.
    let PublicOutcome::Settled(truth) = &honest.run.public else {
        panic!("settles");
    };
    assert_eq!(truth.tick, ClearedTick::Tick(1));
    assert_eq!(truth.volume, 4);

    let plains: Vec<Option<_>> = (0..positions)
        .map(|seq| {
            session
                .admitted
                .iter()
                .zip(&session.acks)
                .find(|(_, ack)| usize::try_from(ack.seq).expect("bounded") == seq)
                .map(|(index, _)| session.submissions[*index].plain)
        })
        .collect();
    let quantities: Vec<u64> = plains
        .iter()
        .map(|plain| plain.map_or(0, |plain| plain.quantity))
        .collect();

    let ticks = [
        ClearedTick::NoTrade,
        ClearedTick::Tick(0),
        ClearedTick::Tick(1),
        ClearedTick::Tick(2),
        ClearedTick::Tick(3),
    ];
    let max_volume: u64 = quantities.iter().sum();

    let mut enumerated = 0usize;
    let mut admissible = 0usize;
    let mut admissible_tick_volume = std::collections::BTreeSet::new();

    let mut fills = vec![0u64; positions];
    loop {
        for tick in ticks {
            for volume in 0..=max_volume {
                enumerated += 1;
                let price = match tick {
                    ClearedTick::NoTrade => 0,
                    ClearedTick::Tick(index) => {
                        session.module.params.tick_prices[usize::from(index)]
                    }
                };
                let mut entries = Vec::with_capacity(positions);
                let mut coherent = true;
                for (seq, plain) in plains.iter().enumerate() {
                    let seq = u32::try_from(seq).expect("bounded");
                    let index = usize::try_from(seq).expect("bounded");
                    match plain {
                        None => entries.push(DeliveryEntry::NoLocalOutput),
                        Some(plain) => match SlotOutcome::derive(seq, plain, price, fills[index]) {
                            Some(outcome) => entries.push(DeliveryEntry::Produced(outcome)),
                            None => coherent = false,
                        },
                    }
                }
                if !coherent {
                    continue;
                }
                let delivery = DeliveryCommitment::build(&binding, &module_digest, entries);
                let public = PublicOutcome::Settled(PublicResult {
                    relation: session.module.identity.relation.clone(),
                    batch_id: session.domain.batch,
                    market_id: session.domain.market,
                    accepted_input_root: cutoff.root,
                    tick,
                    volume,
                });
                let receipt = ShieldedReceipt::new(
                    session.executor.id(),
                    module_digest,
                    binding,
                    honest.run.receipt.input_digest,
                    public.digest(),
                    delivery.root(),
                    ReceiptStatus::Settled,
                );
                if public_check(&receipt, &cutoff, &module_digest, &public).is_err() {
                    continue;
                }
                let openings: Vec<_> = (0..u32::try_from(positions).expect("bounded"))
                    .filter_map(|seq| delivery.open(seq))
                    .collect();
                let mut accepted = true;
                for owner in 0..session.module.params.owners {
                    let mut owned = Vec::new();
                    let mut delivered = OwnerOutput {
                        owner,
                        bought: 0,
                        sold: 0,
                        base_delta: 0,
                        quote_delta: 0,
                        released_base_reservation: 0,
                        released_quote_reservation: 0,
                        owned_slot_fills: vec![None; positions],
                    };
                    for (at, index) in session.admitted.iter().enumerate() {
                        let submission = &session.submissions[*index];
                        if submission.plain.owner != owner {
                            continue;
                        }
                        let seq = session.acks[at].seq;
                        let slot = usize::try_from(seq).expect("bounded");
                        owned.push(OwnedPosition {
                            submission,
                            inclusion: &session.receipts[slot],
                            opening: openings.iter().find(|opening| opening.seq == seq),
                        });
                        if let DeliveryEntry::Produced(outcome) =
                            delivery.entry(seq).expect("committed")
                        {
                            delivered.owned_slot_fills[slot] = Some(outcome.fill);
                            match outcome.side {
                                degg_relation_ir::batch::Side::Buy => {
                                    delivered.bought += outcome.fill;
                                }
                                degg_relation_ir::batch::Side::Sell => {
                                    delivered.sold += outcome.fill;
                                }
                            }
                            delivered.base_delta += outcome.base_delta;
                            delivered.quote_delta += outcome.quote_delta;
                            delivered.released_base_reservation += outcome.released_base;
                            delivered.released_quote_reservation += outcome.released_quote;
                        }
                    }
                    let evidence = OwnerEvidence {
                        owner,
                        positions: owned,
                        cutoff: &cutoff,
                        delivered: Some(&delivered),
                    };
                    if !audit(&session.module, &evidence, &receipt, &public).is_consistent() {
                        accepted = false;
                        break;
                    }
                }
                if accepted {
                    admissible += 1;
                    admissible_tick_volume.insert((format!("{tick:?}"), volume));
                }
            }
        }
        // Odometer over the per-position fill vector.
        let mut carry = 0usize;
        loop {
            if carry >= positions {
                // Every fill vector has been enumerated.
                assert_eq!(
                    enumerated,
                    5 * usize::try_from(max_volume + 1).expect("bounded")
                        * quantities
                            .iter()
                            .map(|q| usize::try_from(q + 1).expect("bounded"))
                            .product::<usize>()
                );

                // The honest run is admissible, as it must be.
                assert!(
                    admissible_tick_volume.contains(&("Tick(1)".to_owned(), 4)),
                    "the honest result was rejected"
                );
                // And so are many others. This is the number the document
                // quotes as the size of the residue on this book.
                // VERIFIED, at exactly these bounds. Of 1,125 enumerated
                // published runs against one fixed admitted set, 377 are
                // accepted by the public check and by all four owners'
                // complete batteries.
                assert_eq!(enumerated, 1_125);
                assert_eq!(admissible, 377);

                // The sharp form of the same measurement: *every* well-formed
                // public result is admissible. A well-formed result is a
                // no-trade at volume zero, or any tick at any positive volume
                // the book could bound. All 33 of them survive, so the
                // composed verifier constrains the executor's fill vector and
                // constrains the published clearing tick and aggregate volume
                // not at all.
                let mut well_formed = std::collections::BTreeSet::new();
                well_formed.insert(("NoTrade".to_owned(), 0));
                for index in 0..4u8 {
                    for volume in 1..=max_volume {
                        well_formed.insert((format!("Tick({index})"), volume));
                    }
                }
                assert_eq!(well_formed.len(), 33);
                assert_eq!(admissible_tick_volume, well_formed);
                return;
            }
            if fills[carry] < quantities[carry] {
                fills[carry] += 1;
                break;
            }
            fills[carry] = 0;
            carry += 1;
        }
    }
}
