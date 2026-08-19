use degg_confidential_energy_dispatch::transcript::render;

#[test]
fn checked_in_corpus_is_byte_identical() {
    assert_eq!(render(), include_str!("../vectors/v1.txt"));
}

#[test]
fn renderer_is_deterministic() {
    assert_eq!(render(), render());
}
