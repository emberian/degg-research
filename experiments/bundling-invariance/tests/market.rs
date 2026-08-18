//! The collateral ledger: what the two costless operations cost.

use degg_bundling_invariance::market::{Market, Op};
use degg_bundling_invariance::payoff::Payoff;

/// States visited by the exhaustive operation sweep, pinned so that a change to
/// the operation set or to a precondition shows up as a diff.
const MARKET_STATES_VISITED: u64 = 7_820;

fn alphabet() -> Vec<Op> {
    vec![
        Op::Deposit {
            holder: 0,
            units: 1,
        },
        Op::Deposit {
            holder: 1,
            units: 2,
        },
        Op::Recombine {
            holder: 0,
            units: 1,
        },
        Op::Recombine {
            holder: 1,
            units: 1,
        },
        Op::Transfer {
            from: 0,
            to: 1,
            cell: 0,
            units: 1,
        },
        Op::Transfer {
            from: 1,
            to: 0,
            cell: 1,
            units: 1,
        },
        Op::Split {
            holder: 0,
            take: Payoff::new(vec![1, 0]),
        },
        Op::Split {
            holder: 1,
            take: Payoff::new(vec![0, 1]),
        },
        Op::Bundle { to: 0, from: 1 },
        Op::Bundle { to: 1, from: 0 },
    ]
}

fn walk(market: &Market, ops: &[Op], depth: usize, checked: &mut u64) {
    *checked += 1;
    assert!(market.conserved(), "conservation identity failed");
    if depth == 0 {
        return;
    }
    for op in ops {
        let mut next = market.clone();
        let before_outstanding = next.outstanding();
        let before_locked = next.collateral_locked();
        match next.apply(op) {
            Ok(()) => {
                let moves_collateral = Market::moves_collateral(op);
                assert_eq!(
                    next.collateral_locked() != before_locked
                        || next.outstanding() != before_outstanding,
                    moves_collateral && next.collateral_locked() != before_locked,
                    "only deposit and recombination may change collateral or claims outstanding: {op}"
                );
                if !moves_collateral {
                    assert_eq!(
                        next.outstanding(),
                        before_outstanding,
                        "{op} changed the claims outstanding"
                    );
                    assert_eq!(
                        next.collateral_locked(),
                        before_locked,
                        "{op} moved collateral"
                    );
                }
                walk(&next, ops, depth - 1, checked);
            }
            Err(_) => {
                assert_eq!(&next, market, "a refused operation changed the market");
            }
        }
    }
}

#[test]
fn every_operation_sequence_preserves_the_conservation_identity() {
    let ops = alphabet();
    let market = Market::open(2, 2);
    let mut checked = 0;
    walk(&market, &ops, 5, &mut checked);
    assert_eq!(
        checked, MARKET_STATES_VISITED,
        "the exhaustive operation sweep changed shape"
    );
}

#[test]
fn bundling_and_unbundling_never_move_collateral() {
    let mut market = Market::open(4, 2);
    market
        .apply(&Op::Deposit {
            holder: 0,
            units: 3,
        })
        .expect("deposit");
    let outstanding = market.outstanding();
    let locked = market.collateral_locked();
    for op in [
        Op::Split {
            holder: 0,
            take: Payoff::new(vec![1, 2, 0, 3]),
        },
        Op::Bundle { to: 1, from: 2 },
        Op::Transfer {
            from: 0,
            to: 1,
            cell: 2,
            units: 2,
        },
    ] {
        market.apply(&op).expect("costless operation");
        assert_eq!(market.outstanding(), outstanding);
        assert_eq!(market.collateral_locked(), locked);
        assert!(market.conserved());
    }
}

#[test]
fn claims_outstanding_are_level_across_cells_and_equal_the_collateral() {
    let mut market = Market::open(5, 3);
    for units in 1..=3 {
        market
            .apply(&Op::Deposit { holder: 1, units })
            .expect("deposit");
    }
    assert_eq!(market.collateral_locked(), 6);
    assert_eq!(market.outstanding(), Payoff::complete_set(5, 6));
    assert_eq!(market.backing_required(), 6);
    market
        .apply(&Op::Recombine {
            holder: 1,
            units: 4,
        })
        .expect("recombine");
    assert_eq!(market.collateral_locked(), 2);
    assert_eq!(market.outstanding(), Payoff::complete_set(5, 2));
    assert!(market.conserved());
}

#[test]
fn a_position_without_a_complete_set_cannot_recombine() {
    let mut market = Market::open(3, 1);
    market
        .apply(&Op::Deposit {
            holder: 0,
            units: 1,
        })
        .expect("deposit");
    market
        .apply(&Op::Split {
            holder: 0,
            take: Payoff::new(vec![0, 1, 0]),
        })
        .expect("split");
    let before = market.clone();
    assert!(
        market
            .apply(&Op::Recombine {
                holder: 0,
                units: 1
            })
            .is_err()
    );
    assert_eq!(market, before);
}
