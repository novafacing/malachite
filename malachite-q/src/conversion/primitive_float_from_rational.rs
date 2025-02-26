use crate::Rational;
use malachite_base::num::arithmetic::traits::{
    DivRound, DivisibleByPowerOf2, IsPowerOf2, NegAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, ExactFrom, RawMantissaAndExponent, RoundingFrom, SciMantissaAndExponent,
    WrappingFrom,
};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode;
use std::cmp::Ordering;

fn abs_is_neg_power_of_2(x: &Rational) -> bool {
    x.numerator == 1u32 && x.denominator.is_power_of_2()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FloatFromRationalError;

macro_rules! float_impls {
    ($f: ident) => {
        impl RoundingFrom<Rational> for $f {
            /// Converts a [`Rational`] to a value of a primitive float according to a specified
            /// [`RoundingMode`](malachite_base::rounding_modes::RoundingMode), taking the
            /// [`Rational`] by value.
            ///
            /// - If the rounding mode is `Floor`, the largest float less than or equal to the
            ///   [`Rational`] is returned. If the [`Rational`] is greater than the maximum finite
            ///   float, then the maximum finite float is returned. If it is smaller than the
            ///   minimum finite float, then negative infinity is returned. If it is between zero
            ///   and the minimum positive float, then positive zero is returned.
            /// - If the rounding mode is `Ceiling`, the smallest float greater than or equal to
            ///   the [`Rational`] is returned. If the [`Rational`] is greater than the maximum
            ///   finite float, then positive infinity is returned. If it is smaller than the
            ///   minimum finite float, then the minimum finite float is returned. If it is between
            ///   zero and the maximum negative float, then negative zero is returned.
            /// - If the rounding mode is `Down`, then the rounding proceeds as with `Floor` if the
            ///   [`Rational`] is non-negative and as with `Ceiling` if the [`Rational`] is
            ///   negative. If the [`Rational`] is between the maximum negative float and the
            ///   minimum positive float, then positive zero is returned when the [`Rational`] is
            ///   non-negative and negative zero otherwise.
            /// - If the rounding mode is `Up`, then the rounding proceeds as with `Ceiling` if the
            ///   [`Rational`] is non-negative and as with `Floor` if the [`Rational`] is negative.
            ///   Positive zero is only returned when the [`Rational`] is zero, and negative zero
            ///   is never returned.
            /// - If the rounding mode is `Nearest`, then the nearest float is returned. If the
            ///   [`Rational`] is exactly between two floats, the float with the zero
            ///   least-significant bit in its representation is selected. If the [`Rational`] is
            ///   greater than the maximum finite float, then the maximum finite float is returned.
            ///   If the [`Rational`] is closer to zero than to any float (or if there is a tie
            ///   between zero and another float), then positive or negative zero is returned,
            ///   depending on the [`Rational`]'s sign.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Panics
            /// Panics if the rounding mode is `Exact` and `value` cannot be represented exactly.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_rational#rounding_from).
            fn rounding_from(mut value: Rational, mut rm: RoundingMode) -> ($f, Ordering) {
                if value == 0u32 {
                    (0.0, Ordering::Equal)
                } else {
                    let sign = value.sign;
                    if !sign {
                        rm.neg_assign();
                    }
                    let mut exponent = value.floor_log_base_2_abs();
                    let (f, o) = if exponent > $f::MAX_EXPONENT {
                        match rm {
                            RoundingMode::Exact => {
                                panic!("Value cannot be represented exactly as a float")
                            }
                            RoundingMode::Floor | RoundingMode::Down | RoundingMode::Nearest => {
                                ($f::MAX_FINITE, Ordering::Less)
                            }
                            _ => ($f::INFINITY, Ordering::Greater),
                        }
                    } else if exponent >= $f::MIN_NORMAL_EXPONENT {
                        value >>= exponent - i64::wrapping_from($f::MANTISSA_WIDTH);
                        let (n, d) = value.into_numerator_and_denominator();
                        let (mut mantissa, o) = n.div_round(d, rm);
                        let mut bits = mantissa.significant_bits();
                        let mut done = false;
                        if bits > $f::MANTISSA_WIDTH + 1 {
                            if exponent == $f::MAX_EXPONENT {
                                done = true;
                            } else {
                                bits -= 1;
                                mantissa >>= 1; // lsb is zero
                                exponent += 1;
                            }
                        }
                        if done {
                            match rm {
                                RoundingMode::Exact => {
                                    panic!("Value cannot be represented exactly as a float")
                                }
                                RoundingMode::Floor
                                | RoundingMode::Down
                                | RoundingMode::Nearest => ($f::MAX_FINITE, Ordering::Less),
                                _ => ($f::INFINITY, Ordering::Greater),
                            }
                        } else {
                            assert_eq!(bits, $f::MANTISSA_WIDTH + 1);
                            mantissa.clear_bit($f::MANTISSA_WIDTH);
                            (
                                $f::from_raw_mantissa_and_exponent(
                                    u64::exact_from(&mantissa),
                                    u64::exact_from(exponent + $f::MAX_EXPONENT),
                                ),
                                o,
                            )
                        }
                    } else if exponent >= $f::MIN_EXPONENT {
                        let target_width = u64::wrapping_from(exponent - $f::MIN_EXPONENT + 1);
                        value >>= $f::MIN_EXPONENT;
                        let (n, d) = value.into_numerator_and_denominator();
                        let (mantissa, o) = n.div_round(d, rm);
                        (
                            if mantissa.significant_bits() > target_width
                                && exponent == $f::MIN_NORMAL_EXPONENT - 1
                            {
                                $f::MIN_POSITIVE_NORMAL
                            } else {
                                $f::from_raw_mantissa_and_exponent(u64::exact_from(&mantissa), 0)
                            },
                            o,
                        )
                    } else {
                        match rm {
                            RoundingMode::Exact => {
                                panic!("Value cannot be represented exactly as a float")
                            }
                            RoundingMode::Floor | RoundingMode::Down => (0.0, Ordering::Less),
                            RoundingMode::Nearest => {
                                if exponent == $f::MIN_EXPONENT - 1
                                    && !abs_is_neg_power_of_2(&value)
                                {
                                    ($f::MIN_POSITIVE_SUBNORMAL, Ordering::Greater)
                                } else {
                                    (0.0, Ordering::Less)
                                }
                            }
                            _ => ($f::MIN_POSITIVE_SUBNORMAL, Ordering::Greater),
                        }
                    };
                    if sign {
                        (f, o)
                    } else {
                        (-f, o.reverse())
                    }
                }
            }
        }

        impl TryFrom<Rational> for $f {
            type Error = FloatFromRationalError;

            /// Converts a [`Rational`] to a primitive float, taking the [`Rational`] by value. If
            /// the input isn't exactly equal to any float, an error is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_rational#try_from).
            fn try_from(value: Rational) -> Result<$f, Self::Error> {
                if value == 0 {
                    Ok(0.0)
                } else {
                    let sign = value.sign;
                    let (mantissa, exponent, _) = value
                        .sci_mantissa_and_exponent_round(RoundingMode::Exact)
                        .ok_or(FloatFromRationalError)?;
                    let f = $f::from_sci_mantissa_and_exponent(mantissa, i64::exact_from(exponent))
                        .ok_or(FloatFromRationalError);
                    if sign {
                        f
                    } else {
                        f.map(|x| -x)
                    }
                }
            }
        }

        impl ConvertibleFrom<Rational> for $f {
            /// Determines whether a [`Rational`] can be exactly converted to a primitive float,
            /// taking the [`Rational`] by value.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_rational#convertible_from).
            fn convertible_from(value: Rational) -> bool {
                if value == 0 {
                    true
                } else {
                    if let Some((mantissa, exponent, _)) =
                        value.sci_mantissa_and_exponent_round::<$f>(RoundingMode::Exact)
                    {
                        let exponent = i64::exact_from(exponent);
                        if !($f::MIN_EXPONENT..=$f::MAX_EXPONENT).contains(&exponent) {
                            return false;
                        }
                        let (orig_mantissa, orig_exponent) = mantissa.raw_mantissa_and_exponent();
                        orig_exponent == u64::wrapping_from($f::MAX_EXPONENT)
                            && exponent >= $f::MIN_NORMAL_EXPONENT
                            || orig_mantissa.divisible_by_power_of_2(u64::wrapping_from(
                                $f::MIN_NORMAL_EXPONENT - exponent,
                            ))
                    } else {
                        false
                    }
                }
            }
        }

        impl<'a> RoundingFrom<&'a Rational> for $f {
            /// Converts a [`Rational`] to a value of a primitive float according to a specified
            /// [`RoundingMode`](malachite_base::rounding_modes::RoundingMode), taking the
            /// [`Rational`] by reference.
            ///
            /// - If the rounding mode is `Floor`, the largest float less than or equal to the
            ///   [`Rational`] is returned. If the [`Rational`] is greater than the maximum finite
            ///   float, then the maximum finite float is returned. If it is smaller than the
            ///   minimum finite float, then negative infinity is returned. If it is between zero
            ///   and the minimum positive float, then positive zero is returned.
            /// - If the rounding mode is `Ceiling`, the smallest float greater than or equal to
            ///   the [`Rational`] is returned. If the [`Rational`] is greater than the maximum
            ///   finite float, then positive infinity is returned. If it is smaller than the
            ///   minimum finite float, then the minimum finite float is returned. If it is between
            ///   zero and the maximum negative float, then negative zero is returned.
            /// - If the rounding mode is `Down`, then the rounding proceeds as with `Floor` if the
            ///   [`Rational`] is non-negative and as with `Ceiling` if the [`Rational`] is
            ///   negative. If the [`Rational`] is between the maximum negative float and the
            ///   minimum positive float, then positive zero is returned when the [`Rational`] is
            ///   non-negative and negative zero otherwise.
            /// - If the rounding mode is `Up`, then the rounding proceeds as with `Ceiling` if the
            ///   [`Rational`] is non-negative and as with `Floor` if the [`Rational`] is negative.
            ///   Positive zero is only returned when the [`Rational`] is zero, and negative zero
            ///   is never returned.
            /// - If the rounding mode is `Nearest`, then the nearest float is returned. If the
            ///   [`Rational`] is exactly between two floats, the float with the zero
            ///   least-significant bit in its representation is selected. If the [`Rational`] is
            ///   greater than the maximum finite float, then the maximum finite float is returned.
            ///   If the [`Rational`] is closer to zero than to any float (or if there is a tie
            ///   between zero and another float), then positive or negative zero is returned,
            ///   depending on the [`Rational`]'s sign.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Panics
            /// Panics if the rounding mode is `Exact` and `value` cannot be represented exactly.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_rational#rounding_from).
            fn rounding_from(value: &'a Rational, mut rm: RoundingMode) -> ($f, Ordering) {
                if *value == 0u32 {
                    (0.0, Ordering::Equal)
                } else {
                    if !value.sign {
                        rm.neg_assign();
                    }
                    let mut exponent = value.floor_log_base_2_abs();
                    let (f, o) = if exponent > $f::MAX_EXPONENT {
                        match rm {
                            RoundingMode::Exact => {
                                panic!("Value cannot be represented exactly as a float")
                            }
                            RoundingMode::Floor | RoundingMode::Down | RoundingMode::Nearest => {
                                ($f::MAX_FINITE, Ordering::Less)
                            }
                            _ => ($f::INFINITY, Ordering::Greater),
                        }
                    } else if exponent >= $f::MIN_NORMAL_EXPONENT {
                        let x = value >> exponent - i64::wrapping_from($f::MANTISSA_WIDTH);
                        let (n, d) = x.into_numerator_and_denominator();
                        let (mut mantissa, o) = n.div_round(d, rm);
                        let mut bits = mantissa.significant_bits();
                        let mut done = false;
                        if bits > $f::MANTISSA_WIDTH + 1 {
                            if exponent == $f::MAX_EXPONENT {
                                done = true;
                            } else {
                                bits -= 1;
                                mantissa >>= 1; // lsb is zero
                                exponent += 1;
                            }
                        }
                        if done {
                            match rm {
                                RoundingMode::Exact => {
                                    panic!("Value cannot be represented exactly as a float")
                                }
                                RoundingMode::Floor
                                | RoundingMode::Down
                                | RoundingMode::Nearest => ($f::MAX_FINITE, Ordering::Less),
                                _ => ($f::INFINITY, Ordering::Greater),
                            }
                        } else {
                            assert_eq!(bits, $f::MANTISSA_WIDTH + 1);
                            mantissa.clear_bit($f::MANTISSA_WIDTH);
                            (
                                $f::from_raw_mantissa_and_exponent(
                                    u64::exact_from(&mantissa),
                                    u64::exact_from(exponent + $f::MAX_EXPONENT),
                                ),
                                o,
                            )
                        }
                    } else if exponent >= $f::MIN_EXPONENT {
                        let target_width = u64::wrapping_from(exponent - $f::MIN_EXPONENT + 1);
                        let x = value >> $f::MIN_EXPONENT;
                        let (n, d) = x.into_numerator_and_denominator();
                        let (mantissa, o) = n.div_round(d, rm);
                        (
                            if mantissa.significant_bits() > target_width
                                && exponent == $f::MIN_NORMAL_EXPONENT - 1
                            {
                                $f::MIN_POSITIVE_NORMAL
                            } else {
                                $f::from_raw_mantissa_and_exponent(u64::exact_from(&mantissa), 0)
                            },
                            o,
                        )
                    } else {
                        match rm {
                            RoundingMode::Exact => {
                                panic!("Value cannot be represented exactly as a float")
                            }
                            RoundingMode::Floor | RoundingMode::Down => (0.0, Ordering::Less),
                            RoundingMode::Nearest => {
                                if exponent == $f::MIN_EXPONENT - 1
                                    && !abs_is_neg_power_of_2(&value)
                                {
                                    ($f::MIN_POSITIVE_SUBNORMAL, Ordering::Greater)
                                } else {
                                    (0.0, Ordering::Less)
                                }
                            }
                            _ => ($f::MIN_POSITIVE_SUBNORMAL, Ordering::Greater),
                        }
                    };
                    if value.sign {
                        (f, o)
                    } else {
                        (-f, o.reverse())
                    }
                }
            }
        }

        impl<'a> TryFrom<&'a Rational> for $f {
            type Error = FloatFromRationalError;

            /// Converts a [`Rational`] to a primitive float, taking the [`Rational`] by reference.
            /// If the input isn't exactly equal to any float, an error is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_rational#try_from).
            fn try_from(value: &'a Rational) -> Result<$f, Self::Error> {
                if *value == 0 {
                    Ok(0.0)
                } else {
                    let (mantissa, exponent, _) = value
                        .sci_mantissa_and_exponent_round_ref(RoundingMode::Exact)
                        .ok_or(FloatFromRationalError)?;
                    let f = $f::from_sci_mantissa_and_exponent(mantissa, i64::exact_from(exponent))
                        .ok_or(FloatFromRationalError);
                    if value.sign {
                        f
                    } else {
                        f.map(|x| -x)
                    }
                }
            }
        }

        impl<'a> ConvertibleFrom<&'a Rational> for $f {
            /// Determines whether a [`Rational`] can be exactly converted to a primitive float,
            /// taking the [`Rational`] by reference.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_rational#convertible_from).
            fn convertible_from(value: &'a Rational) -> bool {
                if *value == 0 {
                    true
                } else {
                    if let Some((mantissa, exponent, _)) =
                        value.sci_mantissa_and_exponent_round_ref::<$f>(RoundingMode::Exact)
                    {
                        let exponent = i64::exact_from(exponent);
                        if !($f::MIN_EXPONENT..=$f::MAX_EXPONENT).contains(&exponent) {
                            return false;
                        }
                        let (orig_mantissa, orig_exponent) = mantissa.raw_mantissa_and_exponent();
                        orig_exponent == u64::wrapping_from($f::MAX_EXPONENT)
                            && exponent >= $f::MIN_NORMAL_EXPONENT
                            || orig_mantissa.divisible_by_power_of_2(u64::wrapping_from(
                                $f::MIN_NORMAL_EXPONENT - exponent,
                            ))
                    } else {
                        false
                    }
                }
            }
        }
    };
}
apply_to_primitive_floats!(float_impls);
