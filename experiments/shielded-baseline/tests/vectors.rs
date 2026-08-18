//! The checked-in corpus is the model's byte identity.

use degg_shielded_baseline::transcript::render;

#[test]
fn the_checked_in_corpus_reproduces_byte_for_byte() {
    let expected = include_str!("../vectors/v1.txt");
    let rendered = render();
    assert_eq!(
        rendered, expected,
        "the transcript changed; regenerate vectors/v1.txt only with an intended semantic change"
    );
}

#[test]
fn rendering_is_deterministic_within_a_process() {
    assert_eq!(render(), render());
}
