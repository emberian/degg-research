//! Print the byte-stable manipulation-cost corpus on standard output.

use degg_manipulation_cost::table::render_vectors_v1;

fn main() {
    match render_vectors_v1() {
        Ok(text) => print!("{text}"),
        Err(error) => {
            eprintln!("manipulation-cost vectors failed: {error:?}");
            std::process::exit(1);
        }
    }
}
