//! SHA-256 and the domain-separated tagged hash used by every commitment here.
//!
//! The implementation is written from FIPS 180-4 so that this experiment stays
//! dependency-free. It is a straightforward reference implementation: it is not
//! constant time, not optimised, and carries no side-channel claim. Its only
//! obligation is to agree with the standard, which the test suite checks
//! against published vectors.
//!
//! Every commitment in this crate goes through [`tagged`], never through a bare
//! [`sha256`] of concatenated fields. A tag fixes the position of a byte string
//! in the commitment structure, so a leaf preimage can never be reread as an
//! interior node, a bag step, a root, or a holder statement.

/// Round constants, the first thirty-two bits of the fractional parts of the
/// cube roots of the first sixty-four primes.
#[rustfmt::skip]
const ROUND_CONSTANTS: [u32; 64] = [
    0x428a_2f98, 0x7137_4491, 0xb5c0_fbcf, 0xe9b5_dba5, 0x3956_c25b, 0x59f1_11f1, 0x923f_82a4,
    0xab1c_5ed5, 0xd807_aa98, 0x1283_5b01, 0x2431_85be, 0x550c_7dc3, 0x72be_5d74, 0x80de_b1fe,
    0x9bdc_06a7, 0xc19b_f174, 0xe49b_69c1, 0xefbe_4786, 0x0fc1_9dc6, 0x240c_a1cc, 0x2de9_2c6f,
    0x4a74_84aa, 0x5cb0_a9dc, 0x76f9_88da, 0x983e_5152, 0xa831_c66d, 0xb003_27c8, 0xbf59_7fc7,
    0xc6e0_0bf3, 0xd5a7_9147, 0x06ca_6351, 0x1429_2967, 0x27b7_0a85, 0x2e1b_2138, 0x4d2c_6dfc,
    0x5338_0d13, 0x650a_7354, 0x766a_0abb, 0x81c2_c92e, 0x9272_2c85, 0xa2bf_e8a1, 0xa81a_664b,
    0xc24b_8b70, 0xc76c_51a3, 0xd192_e819, 0xd699_0624, 0xf40e_3585, 0x106a_a070, 0x19a4_c116,
    0x1e37_6c08, 0x2748_774c, 0x34b0_bcb5, 0x391c_0cb3, 0x4ed8_aa4a, 0x5b9c_ca4f, 0x682e_6ff3,
    0x748f_82ee, 0x78a5_636f, 0x84c8_7814, 0x8cc7_0208, 0x90be_fffa, 0xa450_6ceb, 0xbef9_a3f7,
    0xc671_78f2,
];

/// Initial state, the first thirty-two bits of the fractional parts of the
/// square roots of the first eight primes.
#[rustfmt::skip]
const INITIAL_STATE: [u32; 8] = [
    0x6a09_e667, 0xbb67_ae85, 0x3c6e_f372, 0xa54f_f53a, 0x510e_527f, 0x9b05_688c, 0x1f83_d9ab,
    0x5be0_cd19,
];

/// Streaming SHA-256 state.
#[derive(Clone, Debug)]
pub struct Sha256 {
    state: [u32; 8],
    block: [u8; 64],
    filled: usize,
    bytes: u64,
}

impl Default for Sha256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha256 {
    /// A fresh hasher.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: INITIAL_STATE,
            block: [0u8; 64],
            filled: 0,
            bytes: 0,
        }
    }

    /// Absorb one more byte string.
    pub fn update(&mut self, mut data: &[u8]) {
        self.bytes = self
            .bytes
            .checked_add(data.len() as u64)
            .expect("message length exceeds the modelled domain");
        while !data.is_empty() {
            let take = core::cmp::min(64 - self.filled, data.len());
            self.block[self.filled..self.filled + take].copy_from_slice(&data[..take]);
            self.filled += take;
            data = &data[take..];
            if self.filled == 64 {
                compress(&mut self.state, &self.block);
                self.filled = 0;
            }
        }
    }

    /// Apply the standard padding and produce the digest.
    #[must_use]
    pub fn finish(mut self) -> [u8; 32] {
        let bit_length = self.bytes.wrapping_mul(8);
        self.update(&[0x80]);
        while self.filled != 56 {
            self.update(&[0x00]);
        }
        let tail = bit_length.to_be_bytes();
        self.block[56..64].copy_from_slice(&tail);
        compress(&mut self.state, &self.block);
        self.filled = 0;

        let mut out = [0u8; 32];
        for (chunk, word) in out.chunks_exact_mut(4).zip(self.state.iter()) {
            chunk.copy_from_slice(&word.to_be_bytes());
        }
        out
    }
}

fn compress(state: &mut [u32; 8], block: &[u8; 64]) {
    let mut schedule = [0u32; 64];
    for (word, chunk) in schedule.iter_mut().zip(block.chunks_exact(4)) {
        *word = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
    }
    for i in 16..64 {
        let a = schedule[i - 15];
        let b = schedule[i - 2];
        let s0 = a.rotate_right(7) ^ a.rotate_right(18) ^ (a >> 3);
        let s1 = b.rotate_right(17) ^ b.rotate_right(19) ^ (b >> 10);
        schedule[i] = schedule[i - 16]
            .wrapping_add(s0)
            .wrapping_add(schedule[i - 7])
            .wrapping_add(s1);
    }

    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for (constant, word) in ROUND_CONSTANTS.iter().zip(schedule.iter()) {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let choose = (e & f) ^ ((!e) & g);
        let t1 = h
            .wrapping_add(s1)
            .wrapping_add(choose)
            .wrapping_add(*constant)
            .wrapping_add(*word);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let majority = (a & b) ^ (a & c) ^ (b & c);
        let t2 = s0.wrapping_add(majority);

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }

    let round = [a, b, c, d, e, f, g, h];
    for (slot, value) in state.iter_mut().zip(round.iter()) {
        *slot = slot.wrapping_add(*value);
    }
}

/// SHA-256 of the concatenation of `parts`.
///
/// This is the raw primitive. Commitments in this crate use [`tagged`] instead,
/// because a bare concatenation has no domain separation.
#[must_use]
pub fn sha256(parts: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part);
    }
    hasher.finish()
}

/// Domain-separated hash: `SHA-256(t || t || parts)` where `t = SHA-256(tag)`.
///
/// The doubled tag digest occupies a full compression block, so no choice of
/// `parts` under one tag can be reinterpreted under another tag, and no tag is
/// a prefix of the message body. The construction is the one published for
/// BIP-340; it is used here as ordinary public mathematics and carries no
/// claim beyond collision resistance of the underlying hash.
#[must_use]
pub fn tagged(tag: &[u8], parts: &[&[u8]]) -> [u8; 32] {
    let prefix = sha256(&[tag]);
    let mut hasher = Sha256::new();
    hasher.update(&prefix);
    hasher.update(&prefix);
    for part in parts {
        hasher.update(part);
    }
    hasher.finish()
}

/// Lowercase hexadecimal rendering, used only by the vector transcript.
#[must_use]
pub fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(DIGITS[usize::from(byte >> 4)] as char);
        out.push(DIGITS[usize::from(byte & 0x0f)] as char);
    }
    out
}
