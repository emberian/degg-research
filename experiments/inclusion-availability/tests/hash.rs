//! The hash primitive is checked against the standard before anything is built
//! on top of it. A commitment scheme whose hash is subtly wrong would still
//! pass every self-consistency test in this crate.

use degg_inclusion_availability::hash::{Sha256, hex, sha256, tagged};

fn digest(message: &[u8]) -> String {
    hex(&sha256(&[message]))
}

#[test]
fn published_sha256_vectors_reproduce() {
    // FIPS 180-4 and the NIST byte-oriented test vectors.
    assert_eq!(
        digest(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert_eq!(
        digest(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(
        digest(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
    );
    assert_eq!(
        digest(b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu"),
        "cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1"
    );
}

#[test]
fn one_million_a_vector_reproduces() {
    let mut hasher = Sha256::new();
    for _ in 0..1_000 {
        hasher.update(&[b'a'; 1_000]);
    }
    assert_eq!(
        hex(&hasher.finish()),
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
    );
}

#[test]
fn streaming_agrees_with_one_shot_at_every_split() {
    let message: Vec<u8> = (0u16..300).map(|i| (i % 251) as u8).collect();
    let expected = sha256(&[&message]);
    for split in 0..=message.len() {
        let mut hasher = Sha256::new();
        hasher.update(&message[..split]);
        hasher.update(&message[split..]);
        assert_eq!(hasher.finish(), expected, "split at {split}");
    }
}

#[test]
fn tagged_hash_separates_domains() {
    let body: &[u8] = b"the same bytes";
    let left = tagged(b"degg/test/left", &[body]);
    let right = tagged(b"degg/test/right", &[body]);
    assert_ne!(left, right);
    assert_ne!(left, sha256(&[body]));
}

#[test]
fn tagged_hash_is_not_a_bare_concatenation() {
    // Under a bare concatenation these two would collide; under the tagged
    // construction the part boundaries are still not delimited, which is why
    // every commitment in the crate uses fixed-width fields.
    let split = tagged(b"degg/test/parts", &[b"ab", b"c"]);
    let joined = tagged(b"degg/test/parts", &[b"abc"]);
    assert_eq!(split, joined);
}
