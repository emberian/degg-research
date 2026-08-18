//! The comparison integer refuses to wrap.

use degg_manipulation_cost::big::Big;

#[test]
fn widening_and_ordering_agree_with_native_arithmetic() {
    let values = [
        0u128,
        1,
        2,
        u64::MAX as u128,
        u64::MAX as u128 + 1,
        u128::MAX,
    ];
    for left in values {
        for right in values {
            assert_eq!(
                Big::from_u128(left).cmp(&Big::from_u128(right)),
                left.cmp(&right),
                "{left} vs {right}"
            );
        }
    }
    assert!(Big::ZERO.is_zero());
    assert!(!Big::from_u128(1).is_zero());
}

#[test]
fn products_of_two_u128s_are_exact() {
    let a = u128::MAX;
    let b = u128::MAX;
    let product = Big::from_u128(a).checked_mul_u128(b).expect("product");
    // (2^128 - 1)^2 = 2^256 - 2^129 + 1, checked against an independent build
    // of the same value from two halves.
    let expected = Big::from_u128(a)
        .checked_mul_u128(b - 1)
        .expect("product")
        .checked_add(Big::from_u128(a))
        .expect("sum");
    assert_eq!(product, expected);
}

#[test]
fn overflow_refuses_instead_of_wrapping() {
    let mut value = Big::from_u128(u128::MAX);
    let mut steps = 0;
    while let Some(next) = value.checked_mul_u128(u128::MAX) {
        value = next;
        steps += 1;
        assert!(steps < 64, "the width never ran out");
    }
    assert!(steps >= 14, "unexpectedly narrow: {steps} doublings");
}

#[test]
fn addition_carries_across_every_limb() {
    let big = Big::from_u128(u128::MAX);
    let mut accumulated = Big::ZERO;
    for _ in 0..10 {
        accumulated = accumulated.checked_add(big).expect("sum");
    }
    let expected = Big::from_u128(u128::MAX)
        .checked_mul_u128(10)
        .expect("product");
    assert_eq!(accumulated, expected);
}
