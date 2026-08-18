//! Exact largest-remainder allocation, residual ranks, and conservation.

mod common;

use common::{batch, order};
use degg_batch_oracle::apportion::{Claim, largest_remainder};
use degg_batch_oracle::book::Direction::*;
use degg_batch_oracle::book::{Direction, Slot};
use degg_batch_oracle::curve::Clearing;
use degg_batch_oracle::params::{QUANTITY_CEILING, SLOTS, TICKS};
use degg_batch_oracle::settle::{Settlement, audit};
use degg_batch_oracle::{Outcome, evaluate};

fn settled(specs: &[(u8, Direction, u8, u32)]) -> Settlement {
    match evaluate(&batch(specs)) {
        Outcome::Settled(settlement) => settlement,
        Outcome::Refused(refusal) => panic!("unexpected refusal {refusal:?}"),
    }
}

#[test]
fn largest_remainder_matches_the_worked_specification_example() {
    // q = [3, 3, 1], V = 5, T = 7.
    // base = [15/7, 15/7, 5/7] = [2, 2, 0]; rem = [1, 1, 5]; residual = 1.
    let claims = [
        Claim {
            rank: 0,
            quantity: 3,
        },
        Claim {
            rank: 1,
            quantity: 3,
        },
        Claim {
            rank: 2,
            quantity: 1,
        },
    ];
    assert_eq!(largest_remainder(&claims, 5), Some([2, 2, 1, 0]));
}

#[test]
fn allocation_is_exact_on_both_sides() {
    // Same numbers as the unit example, driven through the whole relation.
    let settlement = settled(&[
        (0, Buy, 3, 3),
        (1, Buy, 3, 3),
        (2, Buy, 3, 1),
        (3, Sell, 0, 5),
    ]);
    assert_eq!(
        settlement.clearing,
        Clearing::Trade {
            tick: 0,
            price: 1,
            volume: 5
        }
    );
    assert_eq!(settlement.fills, [2, 2, 1, 5]);
}

#[test]
fn equal_remainders_go_to_the_earliest_canonical_rank() {
    let claims = [
        Claim {
            rank: 0,
            quantity: 1,
        },
        Claim {
            rank: 1,
            quantity: 1,
        },
    ];
    assert_eq!(largest_remainder(&claims, 1), Some([1, 0, 0, 0]));
    // Rank, not argument position, decides the tie.
    let reversed = [
        Claim {
            rank: 1,
            quantity: 1,
        },
        Claim {
            rank: 0,
            quantity: 1,
        },
    ];
    assert_eq!(largest_remainder(&reversed, 1), Some([0, 1, 0, 0]));
}

#[test]
fn canonical_rank_is_economically_material() {
    // Two identical buys and one unit of supply: public price and volume are
    // fixed, but the single residual atom follows the slot order. Reordering
    // the owners moves real value, which is why the rank must be frozen by the
    // admission log rather than chosen by a builder.
    let first = settled(&[(0, Buy, 3, 1), (1, Buy, 3, 1), (2, Sell, 0, 1)]);
    let second = settled(&[(1, Buy, 3, 1), (0, Buy, 3, 1), (2, Sell, 0, 1)]);
    assert_eq!(first.clearing, second.clearing);
    assert_eq!(first.fills, [1, 0, 1, 0]);
    assert_eq!(second.fills, [1, 0, 1, 0]);
    assert_eq!(first.owners[0].bought_base, 1);
    assert_eq!(first.owners[1].bought_base, 0);
    assert_eq!(second.owners[0].bought_base, 0);
    assert_eq!(second.owners[1].bought_base, 1);
}

#[test]
fn no_atom_becomes_dust() {
    for target in 0..=60u32 {
        for a in 1..=QUANTITY_CEILING {
            for b in 1..=QUANTITY_CEILING {
                let claims = [
                    Claim {
                        rank: 0,
                        quantity: a,
                    },
                    Claim {
                        rank: 1,
                        quantity: b,
                    },
                ];
                if target > a + b {
                    continue;
                }
                let award = largest_remainder(&claims, target).expect("feasible");
                assert_eq!(award[0] + award[1], target, "q=({a},{b}) V={target}");
                assert!(award[0] <= a && award[1] <= b);
            }
        }
    }
}

#[test]
fn ineligible_orders_receive_nothing_and_keep_their_reservation() {
    // The batch clears at tick 0. The buy at limit 0 is eligible; a sell at
    // limit 3 is not, and must be untouched.
    let settlement = settled(&[(0, Buy, 0, 4), (1, Sell, 0, 4), (2, Sell, 3, 9)]);
    assert_eq!(
        settlement.clearing,
        Clearing::Trade {
            tick: 0,
            price: 1,
            volume: 4
        }
    );
    assert_eq!(settlement.fills, [4, 4, 0, 0]);
    assert_eq!(settlement.owners[2].released_base_reservation, 9);
    assert_eq!(settlement.owners[2].base_delta, 0);
    assert_eq!(settlement.owners[2].quote_delta, 0);
}

#[test]
fn owner_deltas_use_the_uniform_clearing_price() {
    // Buy at limit 3 (price 4) clears at tick 0 (price 1): the buyer pays the
    // uniform price and the rest of the reservation is released.
    let settlement = settled(&[(0, Buy, 3, 5), (1, Sell, 0, 5)]);
    assert_eq!(settlement.owners[0].base_delta, 5);
    assert_eq!(settlement.owners[0].quote_delta, -5);
    assert_eq!(settlement.owners[0].released_quote_reservation, 15);
    assert_eq!(settlement.owners[1].base_delta, -5);
    assert_eq!(settlement.owners[1].quote_delta, 5);
    assert_eq!(settlement.owners[1].released_base_reservation, 0);
}

#[test]
fn no_trade_releases_every_reservation() {
    let settlement = settled(&[(0, Buy, 0, 7), (1, Sell, 2, 7)]);
    assert_eq!(settlement.clearing, Clearing::NoTrade);
    assert_eq!(settlement.owners[0].released_quote_reservation, 7);
    assert_eq!(settlement.owners[1].released_base_reservation, 7);
    for owner in settlement.owners {
        assert_eq!(owner.base_delta, 0);
        assert_eq!(owner.quote_delta, 0);
    }
}

#[test]
fn one_owner_can_hold_several_slots() {
    let settlement = settled(&[(0, Buy, 3, 4), (0, Sell, 0, 4)]);
    assert_eq!(
        settlement.clearing,
        Clearing::Trade {
            tick: 0,
            price: 1,
            volume: 4
        }
    );
    // A self-match nets to zero for the owner but still moves public volume.
    assert_eq!(settlement.owners[0].bought_base, 4);
    assert_eq!(settlement.owners[0].sold_base, 4);
    assert_eq!(settlement.owners[0].base_delta, 0);
    assert_eq!(settlement.owners[0].quote_delta, 0);
}

#[test]
fn every_one_buy_one_sell_book_conserves_and_respects_limits() {
    let mut traded = 0usize;
    for buy_limit in 0..TICKS as u8 {
        for sell_limit in 0..TICKS as u8 {
            for buy_quantity in 1..=QUANTITY_CEILING {
                for sell_quantity in 1..=QUANTITY_CEILING {
                    let fixture = batch(&[
                        (0, Buy, buy_limit, buy_quantity),
                        (1, Sell, sell_limit, sell_quantity),
                    ]);
                    let book = degg_batch_oracle::admit::screen(&fixture).expect("admissible");
                    let Outcome::Settled(settlement) = evaluate(&fixture) else {
                        panic!("valid book refused")
                    };
                    audit(&book, &settlement).expect("conservation");
                    match settlement.clearing {
                        Clearing::NoTrade => assert!(buy_limit < sell_limit),
                        Clearing::Trade { tick, volume, .. } => {
                            traded += 1;
                            assert!(sell_limit <= tick && tick <= buy_limit);
                            assert_eq!(volume, buy_quantity.min(sell_quantity));
                            assert_eq!(settlement.fills[0], volume);
                            assert_eq!(settlement.fills[1], volume);
                        }
                    }
                }
            }
        }
    }
    assert_eq!(
        traded,
        10 * QUANTITY_CEILING as usize * QUANTITY_CEILING as usize
    );
}

#[test]
fn every_bounded_four_slot_book_conserves() {
    // Exhaustive over all four slots with quantities 1..=4: 33 states per slot,
    // 33^4 = 1_185_921 books.
    let states = slot_states(4);
    assert_eq!(states.len(), 33);
    let mut count = 0usize;
    let mut traded = 0usize;
    for a in &states {
        for b in &states {
            for c in &states {
                for d in &states {
                    let mut fixture = batch(&[]);
                    for (index, state) in [a, b, c, d].into_iter().enumerate() {
                        fixture.slots[index] = match state {
                            None => Slot::Vacant,
                            Some(spec) => Slot::Taken(order(index, *spec)),
                        };
                    }
                    let book = degg_batch_oracle::admit::screen(&fixture).expect("admissible");
                    let Outcome::Settled(settlement) = evaluate(&fixture) else {
                        panic!("valid book refused")
                    };
                    audit(&book, &settlement).expect("conservation");
                    count += 1;
                    if settlement.clearing != Clearing::NoTrade {
                        traded += 1;
                    }
                }
            }
        }
    }
    assert_eq!(count, 33usize.pow(4));
    assert!(traded > 0);
}

type SlotState = Option<(u8, Direction, u8, u32)>;

fn slot_states(quantity_ceiling: u32) -> Vec<SlotState> {
    let mut states: Vec<SlotState> = vec![None];
    for direction in [Buy, Sell] {
        for limit in 0..TICKS as u8 {
            for quantity in 1..=quantity_ceiling {
                states.push(Some((0, direction, limit, quantity)));
            }
        }
    }
    assert!(states.len() <= 1 + 2 * TICKS * quantity_ceiling as usize);
    let _ = SLOTS;
    states
}
