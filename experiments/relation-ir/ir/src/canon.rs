//! Canonical byte encoding `degg-cbe/v1`.
//!
//! Every IR object has exactly one byte string. The encoding is injective over
//! the frozen object family and is used only to name objects by digest; it is
//! not a wire protocol, not a commitment scheme, and it hides nothing.
//!
//! Rules:
//!
//! - an object encodes as `u32be(len(tag)) ++ tag ++ body`, where `tag` is the
//!   object's ASCII domain-separation tag;
//! - `u8`/`u32`/`u64` are big-endian fixed width; `i64` is two's-complement
//!   big-endian; `bool` is one byte, `0` or `1`;
//! - strings and byte strings are `u32be(len) ++ raw`;
//! - 32-byte digests are raw;
//! - lists are `u32be(count) ++ elements`;
//! - enum variants are `u32be(code) ++ payload fields in declared order`;
//! - options are `0x00`, or `0x01 ++ payload`;
//! - a nested composite object is `u32be(len(bytes)) ++ bytes` of its own full
//!   tagged encoding.

use crate::sha256::sha256;

/// Deterministic byte sink with the `degg-cbe/v1` primitive writers.
#[derive(Default)]
pub struct Sink {
    bytes: Vec<u8>,
}

impl Sink {
    /// Write one `u8`.
    pub fn u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    /// Write one big-endian `u32`.
    pub fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    /// Write one big-endian `u64`.
    pub fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    /// Write one big-endian two's-complement `i64`.
    pub fn i64(&mut self, value: i64) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    /// Write one boolean byte.
    pub fn bool(&mut self, value: bool) {
        self.bytes.push(u8::from(value));
    }

    /// Write a length-prefixed string.
    pub fn str(&mut self, value: &str) {
        self.u32(value.len() as u32);
        self.bytes.extend_from_slice(value.as_bytes());
    }

    /// Write a raw 32-byte digest.
    pub fn digest(&mut self, value: &[u8; 32]) {
        self.bytes.extend_from_slice(value);
    }

    /// Write a list header.
    pub fn count(&mut self, value: usize) {
        self.u32(value as u32);
    }

    /// Write a nested composite object: length prefix plus its tagged bytes.
    pub fn nested(&mut self, object: &impl Canonical) {
        let bytes = object.canonical_bytes();
        self.u32(bytes.len() as u32);
        self.bytes.extend_from_slice(&bytes);
    }

    /// Write an option: absent marker, or present marker plus payload.
    pub fn option<T>(&mut self, value: Option<&T>, mut write: impl FnMut(&mut Self, &T)) {
        match value {
            None => self.u8(0),
            Some(value) => {
                self.u8(1);
                write(self, value);
            }
        }
    }

    fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

/// An object with a canonical `degg-cbe/v1` byte encoding.
pub trait Canonical {
    /// ASCII domain-separation tag; unique per object family.
    fn tag(&self) -> &'static str;

    /// Write the body, after the tag.
    fn body(&self, sink: &mut Sink);

    /// The full canonical bytes: tagged and framed.
    fn canonical_bytes(&self) -> Vec<u8> {
        let mut sink = Sink::default();
        sink.str(self.tag());
        self.body(&mut sink);
        sink.finish()
    }

    /// SHA-256 of the canonical bytes.
    fn digest(&self) -> [u8; 32] {
        sha256(&self.canonical_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Pair(u32, &'static str);

    impl Canonical for Pair {
        fn tag(&self) -> &'static str {
            "test/pair"
        }
        fn body(&self, sink: &mut Sink) {
            sink.u32(self.0);
            sink.str(self.1);
        }
    }

    #[test]
    fn framing_is_tag_prefixed_and_deterministic() {
        let bytes = Pair(7, "hi").canonical_bytes();
        let mut expected = vec![0, 0, 0, 9];
        expected.extend_from_slice(b"test/pair");
        expected.extend_from_slice(&[0, 0, 0, 7]);
        expected.extend_from_slice(&[0, 0, 0, 2]);
        expected.extend_from_slice(b"hi");
        assert_eq!(bytes, expected);
        assert_eq!(bytes, Pair(7, "hi").canonical_bytes());
        assert_ne!(bytes, Pair(8, "hi").canonical_bytes());
    }
}
