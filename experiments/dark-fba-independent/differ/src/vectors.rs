//! Reproduction of the published golden vector corpus from the independent
//! oracle.
//!
//! The eight fixtures are rebuilt here from the existing crate's vector
//! generator, and every number in the rendered lines is computed by
//! `degg-batch-oracle`. The line layout is transcribed from the published file
//! `experiments/dark-fba/vectors/v1.txt`, since a serialization shape is a
//! rendering convention rather than a semantic property of the relation.
//!
//! The refusal lines are the one place where the two implementations cannot
//! agree byte-for-byte by construction: each chose its own refusal taxonomy and
//! spelling. Those lines are therefore rendered twice, once in this oracle's
//! own vocabulary and once through the declared vocabulary map in
//! [`crate::adapter::expected_toy_refusal`], so that the reproduction can be
//! judged on content and on bytes separately.

use std::fmt::Write as _;

use crate::adapter::expected_toy_refusal;
use degg_batch_oracle as mine;
use degg_batch_oracle::book::{Batch, Boundary, Direction, Mode, Order, Slot};
use degg_batch_oracle::curve::Clearing;

const PUBLISHED: &str = include_str!("../../../dark-fba/vectors/v1.txt");

fn context(root_byte: u8, mode: Mode) -> Batch {
    Batch {
        batch: 7,
        market: 9,
        cutoff: 10,
        accepted_input_root: [root_byte; 32],
        slots: [Slot::Vacant; 4],
        boundary: Boundary::SATISFIED,
        mode,
    }
}

#[allow(clippy::too_many_arguments)]
fn place(
    batch: &mut Batch,
    index: usize,
    owner: u8,
    direction: Direction,
    limit_index: u8,
    quantity: u32,
    reserved: u64,
    nullifier: u64,
) {
    batch.slots[index] = Slot::Taken(Order {
        batch: batch.batch,
        market: batch.market,
        owner,
        direction,
        limit_index,
        quantity,
        reserved,
        nullifier,
        arrival: batch.cutoff,
        authorized: true,
        eligible: true,
        included: true,
    });
}

fn balanced() -> Batch {
    let mut batch = context(0x11, Mode::ShieldedSingleExecutor);
    place(&mut batch, 0, 0, Direction::Buy, 2, 5, 15, 101);
    place(&mut batch, 1, 1, Direction::Buy, 1, 3, 6, 102);
    place(&mut batch, 2, 2, Direction::Sell, 0, 4, 4, 103);
    place(&mut batch, 3, 3, Direction::Sell, 2, 4, 4, 104);
    batch
}

fn tie() -> Batch {
    let mut batch = context(0x22, Mode::Clear);
    place(&mut batch, 0, 0, Direction::Buy, 2, 4, 12, 201);
    place(&mut batch, 1, 1, Direction::Sell, 1, 4, 4, 202);
    batch
}

fn no_trade() -> Batch {
    let mut batch = context(0x33, Mode::Clear);
    place(&mut batch, 0, 0, Direction::Buy, 0, 2, 2, 301);
    place(&mut batch, 1, 1, Direction::Sell, 3, 2, 2, 302);
    batch
}

fn edited(mut batch: Batch, index: usize, edit: impl FnOnce(&mut Order)) -> Batch {
    if let Slot::Taken(ref mut order) = batch.slots[index] {
        edit(order);
    }
    batch
}

fn corpus() -> Vec<(&'static str, Batch)> {
    let mut unavailable = tie();
    unavailable.boundary.payloads_available = false;
    let mut dark = tie();
    dark.mode = Mode::DarkTarget;
    let duplicate = edited(tie(), 1, |order| order.nullifier = 201);
    vec![
        ("balanced-residual", balanced()),
        ("price-tie-low", tie()),
        ("no-trade", no_trade()),
        ("late-order", edited(tie(), 0, |order| order.arrival = 11)),
        ("payload-unavailable", unavailable),
        ("duplicate-nullifier", duplicate),
        (
            "under-reserved",
            edited(tie(), 0, |order| order.reserved -= 1),
        ),
        ("dark-target", dark),
    ]
}

fn render_case(name: &str, batch: &Batch, translate: bool) -> String {
    match mine::evaluate(batch) {
        mine::Outcome::Refused(refusal) => {
            let error = if translate {
                match expected_toy_refusal(refusal, batch) {
                    Some(mapped) => format!("{mapped}"),
                    None => "unmapped".to_owned(),
                }
            } else {
                refusal.class().to_owned()
            };
            format!("case={name}|status=refused|error={error}\n")
        }
        mine::Outcome::Settled(settlement) => {
            let (tick, volume) = match settlement.clearing {
                Clearing::NoTrade => ("none".to_owned(), 0),
                Clearing::Trade { tick, volume, .. } => (tick.to_string(), volume),
            };
            let executed = match batch.mode {
                Mode::Clear => "clear",
                Mode::ShieldedSingleExecutor => "shielded-single-executor",
                Mode::DarkTarget => "unreachable",
            };
            let fills = settlement
                .fills
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(",");
            let owners = settlement
                .owners
                .iter()
                .enumerate()
                .map(|(index, owner)| {
                    format!(
                        "o{index}:b{}:s{}:db{}:dq{}:rb{}:rq{}",
                        owner.bought_base,
                        owner.sold_base,
                        owner.base_delta,
                        owner.quote_delta,
                        owner.released_base_reservation,
                        owner.released_quote_reservation
                    )
                })
                .collect::<Vec<_>>()
                .join(";");
            let root = batch.accepted_input_root.iter().fold(
                String::with_capacity(64),
                |mut text, byte| {
                    let _ = write!(text, "{byte:02x}");
                    text
                },
            );
            format!(
                "case={name}|status=ok|executed={executed}|tick={tick}|volume={volume}|fills={fills}|owners={owners}|root={root}\n"
            )
        }
    }
}

/// Render the whole corpus from the independent oracle.
///
/// With `translate` set, refusal classes are spelled through the declared
/// vocabulary map; otherwise they carry this oracle's own class tags.
pub fn reproduce(translate: bool) -> String {
    let mut output = String::from("dark-fba-vectors-v1\n");
    for (name, batch) in corpus() {
        output.push_str(&render_case(name, &batch, translate));
    }
    output
}

/// Compare both renderings against the published corpus and write them out.
pub fn report() {
    println!("== published vector corpus reproduction ==");
    let native = reproduce(false);
    let translated = reproduce(true);
    let published: Vec<&str> = PUBLISHED.lines().collect();

    for (label, produced) in [
        ("own-vocabulary", &native),
        ("mapped-vocabulary", &translated),
    ] {
        let lines: Vec<&str> = produced.lines().collect();
        let matched = lines
            .iter()
            .zip(published.iter())
            .filter(|(left, right)| left == right)
            .count();
        println!(
            "  {label}: {matched}/{} lines byte-identical, whole file {}",
            published.len(),
            if *produced == PUBLISHED {
                "IDENTICAL"
            } else {
                "differs"
            }
        );
        for (index, (left, right)) in lines.iter().zip(published.iter()).enumerate() {
            if left != right {
                println!("    line {}: independent {left}", index + 1);
                println!("    line {}: published   {right}", index + 1);
            }
        }
        if lines.len() != published.len() {
            println!(
                "    line count differs: {} produced, {} published",
                lines.len(),
                published.len()
            );
        }
    }

    let out = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../vectors/v1-independent-reproduction.txt"
    );
    if let Some(parent) = std::path::Path::new(out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    match std::fs::write(out, &translated) {
        Ok(()) => println!("  mapped-vocabulary reproduction written to {out}"),
        Err(error) => println!("  could not write reproduction: {error}"),
    }
    println!();
}
