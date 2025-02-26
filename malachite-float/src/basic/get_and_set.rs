use crate::InnerFloat::Finite;
use crate::{significand_bits, Float};
use malachite_base::num::arithmetic::traits::{
    RoundToMultipleOfPowerOf2, RoundToMultipleOfPowerOf2Assign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use std::cmp::Ordering;

impl Float {
    /// Gets the significand of a [`Float`], taking the [`Float`] by value.
    ///
    /// The significand is the smallest positive integer which is some power of 2 times the
    /// [`Float`], and whose number of significant bits is a multiple of the limb width. If the
    /// [`Float`] is NaN, infinite, or zero, then `None` is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_nz::natural::Natural;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.to_significand(), None);
    /// assert_eq!(Float::INFINITY.to_significand(), None);
    /// assert_eq!(Float::ZERO.to_significand(), None);
    ///
    /// assert_eq!(Float::ONE.to_significand(), Some(Natural::power_of_2(63)));
    /// assert_eq!(
    ///     Float::from(std::f64::consts::PI).to_significand().unwrap(),
    ///     14488038916154245120u64
    /// );
    /// ```
    #[inline]
    pub fn to_significand(&self) -> Option<Natural> {
        match self {
            Float(Finite { significand, .. }) => Some(significand.clone()),
            _ => None,
        }
    }

    /// Gets the significand of a [`Float`], taking the [`Float`] by reference.
    ///
    /// The significand is the smallest positive integer which is some power of 2 times the
    /// [`Float`], and whose number of significant bits is a multiple of the limb width. If the
    /// [`Float`] is NaN, infinite, or zero, then `None` is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_nz::natural::Natural;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.into_significand(), None);
    /// assert_eq!(Float::INFINITY.into_significand(), None);
    /// assert_eq!(Float::ZERO.into_significand(), None);
    ///
    /// assert_eq!(Float::ONE.into_significand(), Some(Natural::power_of_2(63)));
    /// assert_eq!(
    ///     Float::from(std::f64::consts::PI).into_significand().unwrap(),
    ///     14488038916154245120u64
    /// );
    /// ```
    #[allow(clippy::missing_const_for_fn)] // destructor doesn't work with const
    #[inline]
    pub fn into_significand(self) -> Option<Natural> {
        match self {
            Float(Finite { significand, .. }) => Some(significand),
            _ => None,
        }
    }

    /// Returns a reference to the significand of a [`Float`].
    ///
    /// The significand is the smallest positive integer which is some power of 2 times the
    /// [`Float`], and whose number of significant bits is a multiple of the limb width. If the
    /// [`Float`] is NaN, infinite, or zero, then `None` is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_nz::natural::Natural;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.significand_ref(), None);
    /// assert_eq!(Float::INFINITY.significand_ref(), None);
    /// assert_eq!(Float::ZERO.significand_ref(), None);
    ///
    /// assert_eq!(*Float::ONE.significand_ref().unwrap(), Natural::power_of_2(63));
    /// assert_eq!(
    ///     *Float::from(std::f64::consts::PI).significand_ref().unwrap(),
    ///     14488038916154245120u64
    /// );
    /// ```
    #[inline]
    pub const fn significand_ref(&self) -> Option<&Natural> {
        match self {
            Float(Finite { significand, .. }) => Some(significand),
            _ => None,
        }
    }

    /// Returns a [`Float`]'s exponent.
    ///
    /// $$
    /// f(\text{NaN}) = f(\pm\infty) = f(\pm 0.0) = \text{None},
    /// $$
    ///
    /// and, if $x$ is finite and nonzero,
    ///
    /// $$
    /// f(x) = \operatorname{Some}(\lfloor \log_2 x \rfloor + 1).
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_nz::natural::Natural;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.get_exponent(), None);
    /// assert_eq!(Float::INFINITY.get_exponent(), None);
    /// assert_eq!(Float::ZERO.get_exponent(), None);
    ///
    /// assert_eq!(Float::ONE.get_exponent(), Some(1));
    /// assert_eq!(Float::from(std::f64::consts::PI).get_exponent(), Some(2));
    /// assert_eq!(Float::power_of_2(100u64).get_exponent(), Some(101));
    /// assert_eq!(Float::power_of_2(-100i64).get_exponent(), Some(-99));
    /// ```
    #[inline]
    pub const fn get_exponent(&self) -> Option<i64> {
        match self {
            Float(Finite { exponent, .. }) => Some(*exponent),
            _ => None,
        }
    }

    /// Returns a [`Float`]'s precision. The precision is a positive integer denoting how many of
    /// the [`Float`]'s bits are significant.
    ///
    /// Only [`Float`]s that are finite and nonzero have a precision. For other [`Float`]s, `None`
    /// is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_nz::natural::Natural;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.get_prec(), None);
    /// assert_eq!(Float::INFINITY.get_prec(), None);
    /// assert_eq!(Float::ZERO.get_prec(), None);
    ///
    /// assert_eq!(Float::ONE.get_prec(), Some(1));
    /// assert_eq!(Float::one_prec(100).get_prec(), Some(100));
    /// assert_eq!(Float::from(std::f64::consts::PI).get_prec(), Some(53));
    /// ```
    #[inline]
    pub const fn get_prec(&self) -> Option<u64> {
        match self {
            Float(Finite { precision, .. }) => Some(*precision),
            _ => None,
        }
    }

    /// Changes a [`Float`]'s precision. If the precision decreases, rounding may be necessary, and
    /// will use the provided [`RoundingMode`].
    ///
    /// Returns an [`Ordering`], indicating whether the final value is less than, greater than, or
    /// equal to the original value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is [`RoundingMode::Exact`] but setting the desired
    /// precision requires rounding.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let original_x = Float::from(1.0f64 / 3.0);
    /// assert_eq!(original_x.to_string(), "0.33333333333333331");
    /// assert_eq!(original_x.get_prec(), Some(53));
    ///
    /// let mut x = original_x.clone();
    /// assert_eq!(x.set_prec_round(100, RoundingMode::Exact), Ordering::Equal);
    /// assert_eq!(x.to_string(), "0.3333333333333333148296162562474");
    /// assert_eq!(x.get_prec(), Some(100));
    ///
    /// let mut x = original_x.clone();
    /// assert_eq!(x.set_prec_round(10, RoundingMode::Floor), Ordering::Less);
    /// assert_eq!(x.to_string(), "0.333");
    /// assert_eq!(x.get_prec(), Some(10));
    ///
    /// let mut x = original_x.clone();
    /// assert_eq!(x.set_prec_round(10, RoundingMode::Ceiling), Ordering::Greater);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// ```
    pub fn set_prec_round(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        assert_ne!(prec, 0);
        match self {
            Float(Finite {
                sign,
                exponent,
                precision,
                significand,
            }) => {
                let target_bits = prec
                    .round_to_multiple_of_power_of_2(Limb::LOG_WIDTH, RoundingMode::Ceiling)
                    .0;
                let significant_bits = significand_bits(significand);
                let o;
                if target_bits > significant_bits {
                    *significand <<= target_bits - significant_bits;
                    o = Ordering::Equal;
                } else {
                    let limb_count = significand.limb_count();
                    let abs_rm = if *sign { rm } else { -rm };
                    o = significand
                        .round_to_multiple_of_power_of_2_assign(significant_bits - prec, abs_rm);
                    if significand.limb_count() > limb_count {
                        *significand >>= 1;
                        *exponent = exponent.checked_add(1).unwrap();
                    }
                    *significand >>= significant_bits - target_bits;
                }
                *precision = prec;
                if *sign {
                    o
                } else {
                    o.reverse()
                }
            }
            _ => Ordering::Equal,
        }
    }

    /// Changes a [`Float`]'s precision. If the precision decreases, rounding may be necessary, and
    /// [`RoundingMode::Nearest`] will be used.
    ///
    /// Returns an [`Ordering`], indicating whether the final value is less than, greater than, or
    /// equal to the original value.
    ///
    /// To use a different rounding mode, try [`Float::set_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering;
    ///
    /// let original_x = Float::from(1.0f64 / 3.0);
    /// assert_eq!(original_x.to_string(), "0.33333333333333331");
    /// assert_eq!(original_x.get_prec(), Some(53));
    ///
    /// let mut x = original_x.clone();
    /// assert_eq!(x.set_prec(100), Ordering::Equal);
    /// assert_eq!(x.to_string(), "0.3333333333333333148296162562474");
    /// assert_eq!(x.get_prec(), Some(100));
    ///
    /// let mut x = original_x.clone();
    /// assert_eq!(x.set_prec(10), Ordering::Greater);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// ```
    #[inline]
    pub fn set_prec(&mut self, p: u64) -> Ordering {
        self.set_prec_round(p, RoundingMode::Nearest)
    }
}
