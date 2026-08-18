//! Drivers shared by the tests and the corpus renderer.
//!
//! Everything here composes the shielded-baseline session with this crate's
//! custody and settlement objects; nothing here adds a rule of its own.

use degg_relation_ir::batch::Side;
use degg_shielded_baseline::scenario::{Run, Session};

use crate::authorize::{AuthorizationDefect, SettlementAuthorization, SettlementInputs, authorize};
use crate::custody::{Asset, CustodyLedger};
use crate::relation::{Execution, SettlementBook, SettlementRefusal};

/// The asset a side reserves: quote for a buy, base for a sell.
#[must_use]
pub fn spending_asset(side: Side) -> Asset {
    match side {
        Side::Buy => Asset::Quote,
        Side::Sell => Asset::Base,
    }
}

/// Escrow every admitted submission of a session into a fresh custody ledger.
///
/// Only admitted submissions are escrowed. The pre-admission window — an
/// escrow whose submission the log then refuses — has no release rule in this
/// model and is recorded as an open item in the specification.
#[must_use]
pub fn escrow_admitted(session: &Session) -> CustodyLedger {
    let mut custody = CustodyLedger::new();
    for index in &session.admitted {
        let submission = &session.submissions[*index];
        custody
            .escrow(
                submission.request.nullifier,
                submission.plain.owner,
                spending_asset(submission.plain.side),
                submission.plain.reserved,
                submission.request.arrival_epoch,
            )
            .expect("admitted nullifiers are distinct");
    }
    custody
}

/// The committed positions of a session's admitted submissions, in position
/// order.
#[must_use]
pub fn occupied_positions(session: &Session) -> Vec<u32> {
    let mut positions: Vec<u32> = session.acks.iter().map(|ack| ack.seq).collect();
    positions.sort_unstable();
    positions
}

/// The settlement inputs for one committed position of one published run.
#[must_use]
pub fn settlement_inputs<'a>(session: &'a Session, run: &'a Run, seq: u32) -> SettlementInputs<'a> {
    SettlementInputs {
        receipt: &run.run.receipt,
        cutoff: &session.cutoff,
        outcome: &run.run.public,
        phase: run.phase,
        module: &session.module,
        inclusion: &session.receipts[usize::try_from(seq).expect("bounded capacity")],
        opening: run
            .openings
            .iter()
            .find(|opening| opening.seq == seq)
            .expect("the run opens every committed position"),
    }
}

/// Authorize one position of one run against one custody ledger.
pub fn authorize_position(
    session: &Session,
    run: &Run,
    custody: &CustodyLedger,
    seq: u32,
) -> Result<SettlementAuthorization, AuthorizationDefect> {
    authorize(&settlement_inputs(session, run, seq), custody)
}

/// Authorize and execute every occupied position, in position order.
///
/// Returns the executions in that order. Panics on any refusal, so it is a
/// driver for runs the caller expects to settle cleanly; the refusal tests
/// call [`authorize_position`] and [`SettlementBook::execute`] directly.
pub fn settle_all(
    session: &Session,
    run: &Run,
    custody: &mut CustodyLedger,
    book: &mut SettlementBook,
    epoch: u64,
) -> Vec<crate::relation::SettlementExecution> {
    let mut executed = Vec::new();
    for seq in occupied_positions(session) {
        let authorization =
            authorize_position(session, run, custody, seq).expect("an honest run authorizes");
        match book.execute(custody, &authorization, epoch) {
            Ok(Execution::Executed(execution)) => executed.push(execution),
            Ok(Execution::AlreadyExecuted { .. }) => {
                unreachable!("settle_all executes each position once")
            }
            Err(refusal) => panic_on(refusal),
        }
    }
    executed
}

fn panic_on(refusal: SettlementRefusal) -> ! {
    panic!("an honest run settles, got {refusal:?}")
}
