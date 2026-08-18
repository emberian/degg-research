//! Domain A of the Shielded-versus-Clear differential.
//!
//! The comparison, its two sides, and precisely what it is not evidence about
//! are documented on [`degg_shielded_baseline::differential`]. Domain B, the
//! `17^4` books with quantity in `1..=2`, is the `degg-shielded-differ`
//! binary: it takes about half a minute in release and does not belong in the
//! default suite.

use degg_shielded_baseline::differential::run_domain;

#[test]
fn the_shielded_run_equals_the_clear_lowering_over_domain_a() {
    let cases = run_domain(1).expect("no divergence");
    assert_eq!(cases, 9usize.pow(4));
}
