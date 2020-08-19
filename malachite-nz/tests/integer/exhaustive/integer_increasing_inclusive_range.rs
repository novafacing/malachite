use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::strings::ToDebugString;

use malachite_nz::integer::exhaustive::integer_increasing_inclusive_range;
use malachite_nz::integer::Integer;

fn expected_range_len(a: &Integer, b: &Integer) -> usize {
    match (*a >= 0, *b >= 0) {
        (false, false) => usize::exact_from(-a) - usize::exact_from(-b) + 1,
        (false, true) => usize::exact_from(-a) + usize::exact_from(b) + 1,
        (true, false) => panic!(),
        (true, true) => usize::exact_from(b) - usize::exact_from(a) + 1,
    }
}

fn integer_increasing_inclusive_range_helper(a: Integer, b: Integer, values: &str) {
    let xs = integer_increasing_inclusive_range(a.clone(), b.clone())
        .take(20)
        .collect::<Vec<_>>()
        .to_debug_string();
    assert_eq!(xs, values);
    let len = expected_range_len(&a, &b);
    assert_eq!(
        integer_increasing_inclusive_range(a.clone(), b.clone()).count(),
        len
    );
    let mut init = integer_increasing_inclusive_range(a, b)
        .rev()
        .skip(len.saturating_sub(20))
        .collect::<Vec<_>>();
    init.reverse();
    assert_eq!(xs, init.to_debug_string());
}

#[test]
fn test_integer_increasing_inclusive_range() {
    integer_increasing_inclusive_range_helper(Integer::ZERO, Integer::ZERO, "[0]");
    integer_increasing_inclusive_range_helper(Integer::ZERO, Integer::ONE, "[0, 1]");
    integer_increasing_inclusive_range_helper(
        Integer::ZERO,
        Integer::from(10),
        "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]",
    );
    integer_increasing_inclusive_range_helper(
        Integer::from(10),
        Integer::from(20),
        "[10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]",
    );
    integer_increasing_inclusive_range_helper(
        Integer::from(-20),
        Integer::from(-10),
        "[-20, -19, -18, -17, -16, -15, -14, -13, -12, -11, -10]",
    );
    integer_increasing_inclusive_range_helper(
        Integer::from(-100),
        Integer::from(100),
        "[-100, -99, -98, -97, -96, -95, -94, -93, -92, -91, -90, -89, -88, -87, -86, -85, -84, \
        -83, -82, -81]",
    );
}

#[test]
#[should_panic]
fn integer_increasing_inclusive_range_fail() {
    integer_increasing_inclusive_range(Integer::ONE, Integer::ZERO);
}
