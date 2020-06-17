use std::str::FromStr;

use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::random::special_random_natural_below::special_random_natural_below;
use malachite_nz::natural::Natural;
use rand::{IsaacRng, SeedableRng, StdRng};
use rust_wheels::iterators::common::EXAMPLE_SEED;

use malachite_test::common::test_properties;
use malachite_test::inputs::natural::positive_naturals;

#[test]
fn test_special_random_natural_below() {
    let test = |n, out| {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let x = special_random_natural_below(&mut rng, &Natural::from_str(n).unwrap());
        assert_eq!(format!("{:b}", x), out);
        assert!(x.is_valid());
    };
    test("1", "0");
    test("10", "101");
    test("1000000", "11111000110000110");
    test("1000000000000", "1000000001111111000000000111101110000011");
}

#[test]
#[should_panic]
fn special_random_natural_below_fail() {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    special_random_natural_below(&mut rng, &Natural::ZERO);
}

#[test]
fn special_random_natural_below_properties() {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    test_properties(positive_naturals, |n| {
        let x = special_random_natural_below(&mut rng, n);
        assert!(x.is_valid());
        assert!(x < *n);
    });
}
