//! Render the checked-in confidential-energy-dispatch corpus.

fn main() {
    print!(
        "{}",
        degg_confidential_energy_dispatch::transcript::render()
    );
}
