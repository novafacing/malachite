use itertools::Itertools;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{BitConvertible, LeadingZeros, LowMask, NotAssign};

use integer::conversion::to_twos_complement_limbs::limbs_twos_complement_in_place;
use integer::Integer;
use natural::arithmetic::shr::limbs_slice_shr_in_place;
use natural::logic::bit_convertible::{limbs_asc_from_bits_asc, limbs_asc_from_bits_desc};
use natural::Natural;
use platform::{Limb, SignedLimb};

/// Given the bits of a non-negative `Integer`, in ascending order, checks whether the most
/// significant bit is `false`; if it isn't, appends an extra `false` bit. This way the `Integer`'s
/// non-negativity is preserved in its bits.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_nz::integer::logic::bit_convertible::bits_to_twos_complement_bits_non_negative;
///
/// let mut bits = vec![false, true, false];
/// bits_to_twos_complement_bits_non_negative(&mut bits);
/// assert_eq!(bits, &[false, true, false]);
///
/// let mut bits = vec![true, false, true];
/// bits_to_twos_complement_bits_non_negative(&mut bits);
/// assert_eq!(bits, &[true, false, true, false]);
/// ```
pub fn bits_to_twos_complement_bits_non_negative(bits: &mut Vec<bool>) {
    if !bits.is_empty() && *bits.last().unwrap() {
        // Sign-extend with an extra false bit to indicate a positive Integer
        bits.push(false);
    }
}

/// Given the bits of the absolute value of a negative `Integer`, in ascending order, converts the
/// bits to two's complement. Returns whether there is a carry left over from the two's complement
/// conversion process.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `bits.len()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::logic::bit_convertible::bits_slice_to_twos_complement_bits_negative;
///
/// let mut bits = &mut [true, false, true];
/// assert!(!bits_slice_to_twos_complement_bits_negative(bits));
/// assert_eq!(bits, &[true, true, false]);
///
/// let mut bits = &mut [false, false, false];
/// assert!(bits_slice_to_twos_complement_bits_negative(bits));
/// assert_eq!(bits, &[false, false, false]);
/// ```
pub fn bits_slice_to_twos_complement_bits_negative(bits: &mut [bool]) -> bool {
    let mut true_seen = false;
    for bit in bits.iter_mut() {
        if true_seen {
            bit.not_assign();
        } else if *bit {
            true_seen = true;
        }
    }
    !true_seen
}

/// Given the bits of the absolute value of a negative `Integer`, in ascending order, converts the
/// bits to two's complement and checks whether the most significant bit is `true`; if it isn't,
/// appends an extra `true` bit. This way the `Integer`'s negativity is preserved in its bits. The
/// bits cannot be empty or contain only `false`s.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `bits.len()`
///
/// # Panics
/// Panics if `bits` contains only `false`s.
///
/// # Examples
/// ```
/// use malachite_nz::integer::logic::bit_convertible::bits_vec_to_twos_complement_bits_negative;
///
/// let mut bits = vec![true, false, false];
/// bits_vec_to_twos_complement_bits_negative(&mut bits);
/// assert_eq!(bits, &[true, true, true]);
///
/// let mut bits = vec![true, false, true];
/// bits_vec_to_twos_complement_bits_negative(&mut bits);
/// assert_eq!(bits, &[true, true, false, true]);
/// ```
pub fn bits_vec_to_twos_complement_bits_negative(bits: &mut Vec<bool>) {
    assert!(!bits_slice_to_twos_complement_bits_negative(bits));
    if bits.last() == Some(&false) {
        // Sign-extend with an extra true bit to indicate a negative Integer
        bits.push(true);
    }
}

fn limbs_asc_from_negative_twos_complement_limbs_asc(mut xs: Vec<Limb>) -> Vec<Limb> {
    let x_high = xs.last_mut().unwrap();
    let leading_zeros = LeadingZeros::leading_zeros(*x_high);
    if leading_zeros != 0 {
        *x_high |= !Limb::low_mask(Limb::WIDTH - leading_zeros);
    }
    assert!(!limbs_twos_complement_in_place(&mut xs));
    xs
}

fn limbs_asc_from_negative_twos_complement_bits_asc(bits: &[bool]) -> Vec<Limb> {
    limbs_asc_from_negative_twos_complement_limbs_asc(limbs_asc_from_bits_asc(bits))
}

fn limbs_asc_from_negative_twos_complement_bits_desc(bits: &[bool]) -> Vec<Limb> {
    limbs_asc_from_negative_twos_complement_limbs_asc(limbs_asc_from_bits_desc(bits))
}

fn from_bit_iterator_helper(mut limbs: Vec<Limb>, sign_bit: bool, last_width: usize) -> Integer {
    if sign_bit {
        if last_width != usize::exact_from(Limb::WIDTH) {
            *limbs.last_mut().unwrap() |= !Limb::low_mask(u64::exact_from(last_width));
        }
        assert!(!limbs_twos_complement_in_place(&mut limbs));
        -Natural::from_owned_limbs_asc(limbs)
    } else {
        Integer::from(Natural::from_owned_limbs_asc(limbs))
    }
}

impl BitConvertible for Integer {
    /// Returns the bits of an `Integer` in ascending order, so that less significant bits have
    /// lower indices in the output vector. The bits are in two's complement, and the most
    /// significant bit indicates the sign; if the bit is `false`, the `Integer` is positive, and if
    /// the bit is `true` it is negative. There are no trailing `false` bits if the `Integer` is
    /// positive or trailing `true` bits if the `Integer` is negative, except as necessary to
    /// include the correct sign bit. Zero is a special case: it contains no bits.
    ///
    /// This method is more efficient than `to_bits_desc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::ZERO.to_bits_asc().is_empty());
    /// // 105 = 01101001b, with a leading false bit to indicate sign
    /// assert_eq!(
    ///     Integer::from(105).to_bits_asc(),
    ///     vec![true, false, false, true, false, true, true, false]
    /// );
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(
    ///     Integer::from(-105).to_bits_asc(),
    ///     vec![true, true, true, false, true, false, false, true]
    /// );
    /// ```
    fn to_bits_asc(&self) -> Vec<bool> {
        let mut bits = self.abs.to_bits_asc();
        if self.sign {
            bits_to_twos_complement_bits_non_negative(&mut bits);
        } else {
            bits_vec_to_twos_complement_bits_negative(&mut bits);
        }
        bits
    }

    /// Returns the bits of an `Integer` in descending order, so that less significant bits have
    /// higher indices in the output vector. The bits are in two's complement, and the most
    /// significant bit indicates the sign; if the bit is `false`, the `Integer` is positive, and if
    /// the bit is `true` it is negative. There are no leading `false` bits if the `Integer` is
    /// non-negative or `true` bits if `Integer` is negative, except as necessary to include the
    /// correct sign bit. Zero is a special case: it contains no bits.
    ///
    /// This is similar to how BigIntegers in Java are represented.
    ///
    /// This method is less efficient than `to_bits_asc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::ZERO.to_bits_desc().is_empty());
    /// // 105 = 01101001b, with a leading false bit to indicate sign
    /// assert_eq!(
    ///     Integer::from(105).to_bits_desc(),
    ///     vec![false, true, true, false, true, false, false, true]
    /// );
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(
    ///     Integer::from(-105).to_bits_desc(),
    ///     vec![true, false, false, true, false, true, true, true]
    /// );
    /// ```
    fn to_bits_desc(&self) -> Vec<bool> {
        let mut bits = self.to_bits_asc();
        bits.reverse();
        bits
    }

    /// Converts a slice of bits to an `Integer`, in ascending order, so that less significant bits
    /// have lower indices in the input slice. The bits are in two's complement, and the most
    /// significant bit indicates the sign; if the bit is `false`, the `Integer` is non-negative,
    /// and if the bit is `true` it is negative. If `bits` is empty, zero is returned.
    ///
    /// This method is more efficient than `from_bits_desc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `bits.len()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from_bits_asc(&[]).to_string(), "0");
    /// // 105 = 01101001b, with a leading false bit to indicate sign
    /// assert_eq!(
    ///     Integer::from_bits_asc(
    ///         &[true, false, false, true, false, true, true, false]
    ///     ).to_string(),
    ///     "105"
    /// );
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(
    ///     Integer::from_bits_asc(
    ///         &[true, true, true, false, true, false, false, true]
    ///     ).to_string(),
    ///     "-105"
    /// );
    /// ```
    fn from_bits_asc(bits: &[bool]) -> Integer {
        match bits {
            &[] => Integer::ZERO,
            &[.., false] => Integer::from(Natural::from_bits_asc(bits)),
            bits => -Natural::from_owned_limbs_asc(
                limbs_asc_from_negative_twos_complement_bits_asc(bits),
            ),
        }
    }

    /// Converts a slice of bits to an `Integer`, in descending order, so that less significant bits
    /// have higher indices in the input slice. The bits are in two's complement, and the most
    /// significant bit indicates the sign; if the bit is `false`, the `Integer` is non-negative,
    /// and if the bit is `true` it is negative. If `bits` is empty, zero is returned.
    ///
    /// This method is less efficient than `from_bits_asc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `bits.len()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from_bits_desc(&[]).to_string(), "0");
    /// // 105 = 01101001b, with a leading false bit to indicate sign
    /// assert_eq!(
    ///     Integer::from_bits_desc(
    ///         &[false, true, true, false, true, false, false, true]
    ///     ).to_string(),
    ///     "105"
    /// );
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(
    ///     Integer::from_bits_desc(
    ///         &[true, false, false, true, false, true, true, true]
    ///     ).to_string(),
    ///     "-105"
    /// );
    /// ```
    fn from_bits_desc(bits: &[bool]) -> Integer {
        match bits {
            &[] => Integer::ZERO,
            &[false, ..] => Integer::from(Natural::from_bits_desc(bits)),
            bits => -Natural::from_owned_limbs_asc(
                limbs_asc_from_negative_twos_complement_bits_desc(bits),
            ),
        }
    }

    /// TODO doc
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_nz::integer::Integer;
    /// use std::iter::empty;
    ///
    /// assert_eq!(Integer::from_bit_iterator_asc(empty()), 0);
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Integer::from_bit_iterator_asc(
    ///         [true, false, false, true, false, true, true, false].iter().cloned()
    ///     ),
    ///     105
    /// );
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(
    ///     Integer::from_bit_iterator_asc(
    ///         [true, true, true, false, true, false, false, true].iter().cloned()
    ///     ),
    ///     -105
    /// );
    /// ```
    fn from_bit_iterator_asc<I: Iterator<Item = bool>>(xs: I) -> Integer {
        const WIDTH: usize = Limb::WIDTH as usize;
        let mut limbs = Vec::new();
        let mut buffer = [false; WIDTH];
        let mut last_width = 0;
        let mut last_bit = false;
        for chunk in &xs.chunks(WIDTH) {
            let mut i = 0;
            for bit in chunk {
                buffer[i] = bit;
                i += 1;
            }
            last_width = i;
            last_bit = buffer[last_width - 1];
            limbs.push(Limb::from_bits_asc(&buffer[..last_width]));
        }
        from_bit_iterator_helper(limbs, last_bit, last_width)
    }

    /// TODO doc
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitConvertible;
    /// use malachite_nz::integer::Integer;
    /// use std::iter::empty;
    ///
    /// assert_eq!(Integer::from_bit_iterator_desc(empty()), 0);
    /// // 105 = 1101001b
    /// assert_eq!(
    ///     Integer::from_bit_iterator_desc(
    ///         [false, true, true, false, true, false, false, true].iter().cloned()
    ///     ),
    ///     105
    /// );
    /// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
    /// assert_eq!(
    ///     Integer::from_bit_iterator_desc(
    ///         [true, false, false, true, false, true, true, true].iter().cloned()
    ///     ),
    ///     -105
    /// );
    /// ```
    fn from_bit_iterator_desc<I: Iterator<Item = bool>>(xs: I) -> Integer {
        const WIDTH: usize = Limb::WIDTH as usize;
        let mut limbs = Vec::new();
        let mut buffer = [false; WIDTH];
        let mut last_width = 0;
        let mut first_bit = false;
        let mut first = true;
        for chunk in &xs.chunks(WIDTH) {
            let mut i = 0;
            for bit in chunk {
                if first {
                    first_bit = bit;
                    first = false;
                }
                buffer[i] = bit;
                i += 1;
            }
            last_width = i;
            limbs.push(Limb::from_bits_desc(&buffer[..last_width]));
        }
        match limbs.len() {
            0 => Integer::ZERO,
            1 => {
                if first_bit {
                    if last_width != WIDTH {
                        limbs[0] |= !Limb::low_mask(u64::exact_from(last_width));
                    }
                    Integer::from(SignedLimb::wrapping_from(limbs[0]))
                } else {
                    Integer::from(limbs[0])
                }
            }
            _ => {
                limbs.reverse();
                if last_width != WIDTH {
                    let smallest_limb = limbs[0];
                    limbs[0] = 0;
                    limbs_slice_shr_in_place(
                        &mut limbs,
                        Limb::WIDTH - u64::wrapping_from(last_width),
                    );
                    limbs[0] |= smallest_limb;
                }
                from_bit_iterator_helper(limbs, first_bit, last_width)
            }
        }
    }
}
