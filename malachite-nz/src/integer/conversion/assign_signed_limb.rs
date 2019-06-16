use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::conversion::traits::Assign;

use integer::Integer;
use platform::SignedLimb;

/// Assigns a `SignedLimb` to an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::conversion::traits::Assign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(456);
///     x.assign(-123);
///     assert_eq!(x.to_string(), "-123");
/// }
/// ```
impl Assign<SignedLimb> for Integer {
    fn assign(&mut self, other: SignedLimb) {
        self.sign = other >= 0;
        self.abs.assign(other.unsigned_abs());
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl Assign<i32> for Integer {
    #[inline]
    fn assign(&mut self, other: i32) {
        self.assign(SignedLimb::from(other));
    }
}
