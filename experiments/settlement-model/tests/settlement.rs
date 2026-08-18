//! Execution: exactly once per settlement nullifier, idempotent retries,
//! order-independent solvency, and exact drain of the pool.

mod common;

use common::{balanced_residual, no_trade, ready};
use degg_settlement_model::custody::{Asset, Balances, CustodyState};
use degg_settlement_model::harness::{authorize_position, occupied_positions, settle_all};
use degg_settlement_model::relation::{Execution, SettlementBook, SettlementRefusal};

#[test]
fn the_balanced_book_settles_every_position_and_drains_the_pool_exactly() {
    let mut ready = ready(&balanced_residual());
    let executions = settle_all(
        &ready.session,
        &ready.run,
        &mut ready.custody,
        &mut ready.book,
        common::NOW,
    );
    assert_eq!(executions.len(), 4);
    assert_eq!(ready.custody.pool(), Balances::zero());
    assert!(ready.custody.conserves());
    let expected = [
        (0u8, Balances { base: 5, quote: 0 }),
        (1, Balances { base: 0, quote: 6 }),
        (2, Balances { base: 1, quote: 9 }),
        (3, Balances { base: 2, quote: 6 }),
    ];
    for (owner, balances) in expected {
        assert_eq!(ready.custody.account(owner), balances, "owner {owner}");
    }
    for (_, entry) in ready.custody.entries() {
        assert!(matches!(entry.state, CustodyState::Settled { .. }));
    }
}

#[test]
fn base_and_quote_conserve_across_the_settled_book() {
    // Zero fees: everything deposited returns to the owners, redistributed by
    // the trade. 21 quote and 8 base in; 21 quote and 8 base out.
    let mut ready = ready(&balanced_residual());
    settle_all(
        &ready.session,
        &ready.run,
        &mut ready.custody,
        &mut ready.book,
        common::NOW,
    );
    let mut credited = Balances::zero();
    for owner in 0..4 {
        let account = ready.custody.account(owner);
        credited.base += account.base;
        credited.quote += account.quote;
    }
    assert_eq!(credited, ready.custody.deposited());
    assert_eq!(credited, Balances { base: 8, quote: 21 });
}

#[test]
fn a_zero_fill_position_settles_through_the_settlement_lane() {
    // Owner 1's buy misses the clearing tick: fill zero, full reservation
    // back. That is a settlement, not a refund: the batch settled and the
    // position resolves to `Settled`.
    let mut ready = ready(&balanced_residual());
    let authorization =
        authorize_position(&ready.session, &ready.run, &ready.custody, 1).expect("authorizes");
    assert_eq!(authorization.escrow_consumed, 0);
    assert_eq!(authorization.released, (Asset::Quote, 6));
    assert_eq!(authorization.acquired, (Asset::Base, 0));
    let executed = ready
        .book
        .execute(&mut ready.custody, &authorization, common::NOW)
        .expect("executes");
    assert!(matches!(executed, Execution::Executed(_)));
    let entry = ready
        .custody
        .entry(&authorization.admission_nullifier)
        .expect("entry");
    assert!(matches!(entry.state, CustodyState::Settled { .. }));
}

#[test]
fn a_retry_is_idempotent_by_settlement_nullifier_and_moves_nothing() {
    let mut ready = ready(&balanced_residual());
    let authorization =
        authorize_position(&ready.session, &ready.run, &ready.custody, 0).expect("authorizes");
    let first = ready
        .book
        .execute(&mut ready.custody, &authorization, common::NOW)
        .expect("executes");
    let Execution::Executed(execution) = first else {
        panic!("first submission executes");
    };
    let pool = ready.custody.pool();
    let account = ready.custody.account(0);
    let claims = ready.book.claims().len();
    // The retry: same instruction, crashed adapter, resubmitted.
    let retry = ready
        .book
        .execute(&mut ready.custody, &authorization, common::NOW)
        .expect("a retry is an answer, not an error");
    assert_eq!(
        retry,
        Execution::AlreadyExecuted {
            execution: execution.execution
        }
    );
    assert_eq!(ready.custody.pool(), pool);
    assert_eq!(ready.custody.account(0), account);
    assert_eq!(ready.book.claims().len(), claims);
    assert!(ready.custody.conserves());
}

#[test]
fn an_instruction_against_an_unobserved_receipt_refuses() {
    let ready = ready(&balanced_residual());
    let authorization =
        authorize_position(&ready.session, &ready.run, &ready.custody, 0).expect("authorizes");
    let mut fresh_book = SettlementBook::new();
    let mut custody = ready.custody.clone();
    assert_eq!(
        fresh_book.execute(&mut custody, &authorization, common::NOW),
        Err(SettlementRefusal::ReceiptNotObserved)
    );
}

#[test]
fn settlement_is_solvent_in_every_order_and_at_every_prefix() {
    // All 24 orders over the four occupied positions; conservation and pool
    // solvency at every step; exact drain at the end of each order. Every
    // subset of positions is a prefix of some order, so this also covers all
    // sixteen partial-settlement states.
    let base = ready(&balanced_residual());
    let positions = occupied_positions(&base.session);
    assert_eq!(positions.len(), 4);
    let mut orders_checked = 0u32;
    for a in 0..4usize {
        for b in 0..4usize {
            for c in 0..4usize {
                for d in 0..4usize {
                    let order = [a, b, c, d];
                    let mut seen = order.to_vec();
                    seen.sort_unstable();
                    seen.dedup();
                    if seen.len() != 4 {
                        continue;
                    }
                    let mut custody = base.custody.clone();
                    let mut book = base.book.clone();
                    for step in order {
                        let seq = positions[step];
                        let authorization =
                            authorize_position(&base.session, &base.run, &custody, seq)
                                .expect("authorizes");
                        let executed = book
                            .execute(&mut custody, &authorization, common::NOW)
                            .expect("solvent in every order");
                        assert!(matches!(executed, Execution::Executed(_)));
                        assert!(custody.conserves(), "conserves after each step");
                    }
                    assert_eq!(custody.pool(), Balances::zero());
                    orders_checked += 1;
                }
            }
        }
    }
    assert_eq!(orders_checked, 24);
}

#[test]
fn the_no_trade_book_settles_full_releases_and_padding_is_outside_custody() {
    let mut ready = ready(&no_trade());
    let executions = settle_all(
        &ready.session,
        &ready.run,
        &mut ready.custody,
        &mut ready.book,
        common::NOW,
    );
    assert_eq!(executions.len(), 2);
    assert_eq!(ready.custody.pool(), Balances::zero());
    // Reservations return in full: 2 quote to the buyer, 2 base to the seller.
    assert_eq!(ready.custody.account(0), Balances { base: 0, quote: 2 });
    assert_eq!(ready.custody.account(1), Balances { base: 2, quote: 0 });
    assert!(ready.custody.conserves());
    // The two padding positions have no custody entry at all.
    assert_eq!(ready.custody.entries().count(), 2);
}

#[test]
fn conflicting_execution_is_defence_in_depth() {
    // Under one observed receipt binding, the execution digest is a function
    // of that binding, the position, and the committed effect, and the
    // delivery root inside the binding commits the effect at each position.
    // Two different execution digests for one settlement nullifier therefore
    // require a hash collision. The refusal class exists so a future refactor
    // cannot turn that impossibility into a silent overwrite; this test
    // records the argument, exactly as upstream records
    // `ReceiptDefect::SequenceMismatch`.
    let mut ready = ready(&balanced_residual());
    let authorization =
        authorize_position(&ready.session, &ready.run, &ready.custody, 0).expect("authorizes");
    ready
        .book
        .execute(&mut ready.custody, &authorization, common::NOW)
        .expect("executes");
    let mut forged = authorization;
    forged.execution[0] ^= 1;
    assert_eq!(
        ready.book.execute(&mut ready.custody, &forged, common::NOW),
        Err(SettlementRefusal::ConflictingExecution {
            spent: authorization.execution,
            offered: forged.execution,
        })
    );
}
