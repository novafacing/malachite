use std::mem::size_of;

use malachite_base::rounding_mode::RoundingMode;

#[test]
fn test_size() {
    assert_eq!(size_of::<RoundingMode>(), 1);
}
