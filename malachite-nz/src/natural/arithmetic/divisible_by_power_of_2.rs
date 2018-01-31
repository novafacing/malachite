use natural::{LIMB_BITS, LIMB_BITS_MASK, LOG_LIMB_BITS};
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Returns whether `self` is divisible by 2<sup>`pow`</sup>. If `self` is 0, the result is
    /// always true; otherwise, it is equivalent to `self.trailing_zeros().unwrap() <= pow`, but
    /// more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.divisible_by_power_of_2(100), true);
    ///     assert_eq!(Natural::from(100u32).divisible_by_power_of_2(2), true);
    ///     assert_eq!(Natural::from(100u32).divisible_by_power_of_2(3), false);
    ///     assert_eq!(Natural::trillion().divisible_by_power_of_2(12), true);
    ///     assert_eq!(Natural::trillion().divisible_by_power_of_2(13), false);
    /// }
    /// ```
    pub fn divisible_by_power_of_2(&self, pow: u32) -> bool {
        match (self, pow) {
            (_, 0) | (&Small(0), _) => true,
            (&Small(_), pow) if pow >= LIMB_BITS => false,
            (&Small(small), pow) => small & ((1 << pow) - 1) == 0,
            (&Large(ref limbs), pow) => {
                let zero_limbs = (pow >> LOG_LIMB_BITS) as usize;
                if zero_limbs >= limbs.len() || limbs.iter().take(zero_limbs).any(|&limb| limb != 0)
                {
                    return false;
                }
                limbs[zero_limbs] & ((1 << (pow & LIMB_BITS_MASK)) - 1) == 0
            }
        }
    }
}
