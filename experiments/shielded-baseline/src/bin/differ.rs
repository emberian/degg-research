//! Domain B of the Shielded-versus-Clear differential.
//!
//! Run in release from the repository root:
//!
//! ```sh
//! cargo run --offline --locked --release \
//!   --manifest-path experiments/shielded-baseline/Cargo.toml \
//!   --bin degg-shielded-differ
//! ```

fn main() {
    let ceiling: u64 = std::env::args()
        .nth(1)
        .and_then(|argument| argument.parse().ok())
        .unwrap_or(2);
    match degg_shielded_baseline::differential::run_domain(ceiling) {
        Ok(cases) => println!("domain quantity-ceiling={ceiling} cases={cases} divergences=0"),
        Err(reason) => {
            eprintln!("{reason}");
            std::process::exit(1);
        }
    }
}
