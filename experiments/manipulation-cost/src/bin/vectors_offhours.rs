//! Print the byte-stable off-hours manipulation-cost corpus on standard output.

use degg_manipulation_cost::session::render_vectors_v2;

fn main() {
    match render_vectors_v2() {
        Ok(text) => print!("{text}"),
        Err(error) => {
            eprintln!("manipulation-cost off-hours vectors failed: {error:?}");
            std::process::exit(1);
        }
    }
}
