// Emits Tower256 reference vectors from minidregg's prover/src/binary_tower_256.rs (copied verbatim apart from
// the TowerElem embed): per r, the 256 basis images r*e_j, then test pairs (x, r*x).  One JSON object per line.
#[allow(dead_code)]
mod binary_tower_256;
use binary_tower_256::Tower256;
fn mix(s: &mut u64) -> u64 { *s = s.wrapping_add(0x9E3779B97F4A7C15); let mut z = *s;
  z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9); z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB); z ^ (z >> 31) }
fn hex(b: [u8; 32]) -> String { b.iter().map(|x| format!("{:02x}", x)).collect() }
fn main() {
  let a: Vec<String> = std::env::args().collect();
  let nr: usize = a[1].parse().unwrap(); let nx: usize = a[2].parse().unwrap();
  let mut s = 0x7477_3235_36u64;
  for _ in 0..nr {
    let r = Tower256::from_limbs([mix(&mut s), mix(&mut s), mix(&mut s), mix(&mut s)]);
    let cols: Vec<String> = (0..256).map(|j| { let mut l = [0u64; 4]; l[j / 64] = 1u64 << (j % 64);
      format!("\"{}\"", hex(r.mul(Tower256::from_limbs(l)).to_le_bytes())) }).collect();
    let tests: Vec<String> = (0..nx).map(|_| { let x = Tower256::from_limbs([mix(&mut s), mix(&mut s), mix(&mut s), mix(&mut s)]);
      format!("[\"{}\", \"{}\"]", hex(x.to_le_bytes()), hex(r.mul(x).to_le_bytes())) }).collect();
    println!("{{\"r\": \"{}\", \"cols\": [{}], \"tests\": [{}]}}", hex(r.to_le_bytes()), cols.join(", "), tests.join(", "));
  }
}
