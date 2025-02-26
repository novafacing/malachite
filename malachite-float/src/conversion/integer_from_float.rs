use crate::InnerFloat::{Finite, Zero};
use crate::{significand_bits, Float};
use malachite_base::num::arithmetic::traits::{DivisibleByPowerOf2, ShrRound};
use malachite_base::num::basic::traits::{One, Zero as ZeroTrait};
use malachite_base::num::conversion::from::SignedFromFloatError;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;

impl RoundingFrom<Float> for Integer {
    /// Converts a [`Float`] to an [`Integer`], using a specified [`RoundingMode`] and taking the
    /// [`Float`] by value. An [`Ordering`] is also returned, indicating whether the returned value
    /// is less than, equal to, or greater than the original value.
    ///
    /// If the [`Float`] is NaN or infinite, the function will panic regardless of the rounding
    /// mode.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.complexity()`.
    ///
    /// # Panics
    /// Panics if the [`Float`] is not an integer and `rm` is `Exact`, or if the [`Float`] is NaN
    /// or infinite.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NegativeInfinity;
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(Float::from(1.5), RoundingMode::Floor).to_debug_string(),
    ///     "(1, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Float::from(1.5), RoundingMode::Ceiling).to_debug_string(),
    ///     "(2, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Float::from(1.5), RoundingMode::Nearest).to_debug_string(),
    ///     "(2, Greater)"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(Float::from(-1.5), RoundingMode::Floor).to_debug_string(),
    ///     "(-2, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Float::from(-1.5), RoundingMode::Ceiling).to_debug_string(),
    ///     "(-1, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(Float::from(-1.5), RoundingMode::Nearest).to_debug_string(),
    ///     "(-2, Less)"
    /// );
    /// ```
    fn rounding_from(f: Float, rm: RoundingMode) -> (Integer, Ordering) {
        match f {
            float_either_zero!() => (Integer::ZERO, Ordering::Equal),
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                let abs_rm = if sign { rm } else { -rm };
                let (abs_i, abs_o) = if exponent < 0 {
                    match abs_rm {
                        RoundingMode::Floor | RoundingMode::Down | RoundingMode::Nearest => {
                            (Natural::ZERO, Ordering::Less)
                        }
                        RoundingMode::Ceiling | RoundingMode::Up => {
                            (Natural::ONE, Ordering::Greater)
                        }
                        RoundingMode::Exact => {
                            panic!("Cannot convert Float to Integer using {rm}")
                        }
                    }
                } else {
                    let sb = significand_bits(&significand);
                    let eb = exponent.unsigned_abs();
                    if sb >= eb {
                        significand.shr_round(sb - eb, abs_rm)
                    } else {
                        (significand << (eb - sb), Ordering::Equal)
                    }
                };
                if sign {
                    (Integer::from(abs_i), abs_o)
                } else {
                    (-abs_i, abs_o.reverse())
                }
            }
            _ => panic!("Can't convert {f} to Integer using {rm}"),
        }
    }
}

impl<'a> RoundingFrom<&'a Float> for Integer {
    /// Converts a [`Float`] to an [`Integer`], using a specified [`RoundingMode`] and taking the
    /// [`Float`] by reference. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// If the [`Float`] is NaN or infinite, the function will panic regardless of the rounding
    /// mode.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.complexity()`.
    ///
    /// # Panics
    /// Panics if the [`Float`] is not an integer and `rm` is `Exact`, or if the [`Float`] is NaN
    /// or infinite.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NegativeInfinity;
    /// use malachite_base::num::conversion::traits::RoundingFrom;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(&Float::from(1.5), RoundingMode::Floor).to_debug_string(),
    ///     "(1, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&Float::from(1.5), RoundingMode::Ceiling).to_debug_string(),
    ///     "(2, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&Float::from(1.5), RoundingMode::Nearest).to_debug_string(),
    ///     "(2, Greater)"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::rounding_from(&Float::from(-1.5), RoundingMode::Floor).to_debug_string(),
    ///     "(-2, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&Float::from(-1.5), RoundingMode::Ceiling).to_debug_string(),
    ///     "(-1, Greater)"
    /// );
    /// assert_eq!(
    ///     Integer::rounding_from(&Float::from(-1.5), RoundingMode::Nearest).to_debug_string(),
    ///     "(-2, Less)"
    /// );
    /// ```
    fn rounding_from(f: &'a Float, rm: RoundingMode) -> (Integer, Ordering) {
        match f {
            float_either_zero!() => (Integer::ZERO, Ordering::Equal),
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                if *significand == 0u32 {
                    (Integer::ZERO, Ordering::Equal)
                } else {
                    let abs_rm = if *sign { rm } else { -rm };
                    let (abs_i, abs_o) = if *exponent < 0 {
                        match abs_rm {
                            RoundingMode::Floor | RoundingMode::Down | RoundingMode::Nearest => {
                                (Natural::ZERO, Ordering::Less)
                            }
                            RoundingMode::Ceiling | RoundingMode::Up => {
                                (Natural::ONE, Ordering::Greater)
                            }
                            RoundingMode::Exact => {
                                panic!("Cannot convert Float to Integer using {rm}")
                            }
                        }
                    } else {
                        let sb = significand_bits(significand);
                        let eb = exponent.unsigned_abs();
                        if sb >= eb {
                            significand.shr_round(sb - eb, abs_rm)
                        } else {
                            (significand << (eb - sb), Ordering::Equal)
                        }
                    };
                    if *sign {
                        (Integer::from(abs_i), abs_o)
                    } else {
                        (-abs_i, abs_o.reverse())
                    }
                }
            }
            _ => panic!("Can't convert {f} to Integer using {rm}"),
        }
    }
}

impl TryFrom<Float> for Integer {
    type Error = SignedFromFloatError;

    /// Converts a [`Float`] to an [`Integer`], taking the [`Float`] by value. If the [`Float`] is
    /// not equal to an integer, an error is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
    /// use malachite_base::num::conversion::from::SignedFromFloatError::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::try_from(Float::ZERO).unwrap(), 0);
    /// assert_eq!(Integer::try_from(Float::from(123.0)).unwrap(), 123);
    /// assert_eq!(Integer::try_from(Float::from(-123.0)).unwrap(), -123);
    ///
    /// assert_eq!(Integer::try_from(Float::from(1.5)), Err(FloatNonIntegerOrOutOfRange));
    /// assert_eq!(Integer::try_from(Float::INFINITY), Err(FloatInfiniteOrNan));
    /// assert_eq!(Integer::try_from(Float::NAN), Err(FloatInfiniteOrNan));
    /// ```
    fn try_from(f: Float) -> Result<Integer, Self::Error> {
        match f {
            Float(Zero { .. }) => Ok(Integer::ZERO),
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                if exponent <= 0 {
                    Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange)
                } else {
                    let sb = significand_bits(&significand);
                    let eb = exponent.unsigned_abs();
                    if sb >= eb {
                        let bits = sb - eb;
                        if significand.divisible_by_power_of_2(bits) {
                            Ok(Integer::from_sign_and_abs(sign, significand >> bits))
                        } else {
                            Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange)
                        }
                    } else {
                        Ok(Integer::from_sign_and_abs(sign, significand << (eb - sb)))
                    }
                }
            }
            _ => Err(SignedFromFloatError::FloatInfiniteOrNan),
        }
    }
}

impl<'a> TryFrom<&'a Float> for Integer {
    type Error = SignedFromFloatError;

    /// Converts a [`Float`] to an [`Integer`], taking the [`Float`] by reference. If the [`Float`]
    /// is not equal to an integer, an error is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
    /// use malachite_base::num::conversion::from::SignedFromFloatError::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::try_from(&Float::ZERO).unwrap(), 0);
    /// assert_eq!(Integer::try_from(&Float::from(123.0)).unwrap(), 123);
    /// assert_eq!(Integer::try_from(&Float::from(-123.0)).unwrap(), -123);
    ///
    /// assert_eq!(Integer::try_from(&Float::from(1.5)), Err(FloatNonIntegerOrOutOfRange));
    /// assert_eq!(Integer::try_from(&Float::INFINITY), Err(FloatInfiniteOrNan));
    /// assert_eq!(Integer::try_from(&Float::NAN), Err(FloatInfiniteOrNan));
    /// ```
    fn try_from(f: &'a Float) -> Result<Integer, Self::Error> {
        match f {
            Float(Zero { .. }) => Ok(Integer::ZERO),
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                if *exponent <= 0 {
                    Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange)
                } else {
                    let sb = significand_bits(significand);
                    let eb = exponent.unsigned_abs();
                    if sb >= eb {
                        let bits = sb - eb;
                        if significand.divisible_by_power_of_2(bits) {
                            Ok(Integer::from_sign_and_abs(*sign, significand >> bits))
                        } else {
                            Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange)
                        }
                    } else {
                        Ok(Integer::from_sign_and_abs(*sign, significand << (eb - sb)))
                    }
                }
            }
            _ => Err(SignedFromFloatError::FloatInfiniteOrNan),
        }
    }
}

impl<'a> ConvertibleFrom<&'a Float> for Integer {
    /// Determines whether a [`Float`] can be converted to an [`Integer`], taking the [`Float`] by
    /// reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `f.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::convertible_from(&Float::ZERO), true);
    /// assert_eq!(Integer::convertible_from(&Float::from(123.0)), true);
    /// assert_eq!(Integer::convertible_from(&Float::from(-123.0)), true);
    ///
    /// assert_eq!(Integer::convertible_from(&Float::from(1.5)), false);
    /// assert_eq!(Integer::convertible_from(&Float::INFINITY), false);
    /// assert_eq!(Integer::convertible_from(&Float::NAN), false);
    /// ```
    fn convertible_from(f: &'a Float) -> bool {
        match f {
            Float(Zero { .. }) => true,
            Float(Finite {
                exponent,
                significand,
                ..
            }) => {
                *significand == 0u32
                    || *exponent > 0 && {
                        let sb = significand_bits(significand);
                        let eb = exponent.unsigned_abs();
                        sb < eb || significand.divisible_by_power_of_2(sb - eb)
                    }
            }
            _ => false,
        }
    }
}
