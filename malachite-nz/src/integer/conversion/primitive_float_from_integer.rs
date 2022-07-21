use crate::integer::Integer;
use malachite_base::num::conversion::traits::{CheckedFrom, ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;

macro_rules! float_impls {
    ($f: ident) => {
        impl<'a> RoundingFrom<&'a Integer> for $f {
            /// Converts an [`Integer`] to a primitive float according to a specified
            /// [`RoundingMode`](malachite_base::rounding_modes::RoundingMode).
            ///
            /// - If the rounding mode is `Floor` the largest float less than or equal to the
            ///   [`Integer`] is returned. If the [`Integer`] is greater than the maximum finite
            ///   float, then the maximum finite float is returned. If it is smaller than the
            ///   minimum finite float, then negative infinity is returned.
            /// - If the rounding mode is `Ceiling`, the smallest float greater than or equal to
            ///   the [`Integer`] is returned. If the [`Integer`] is greater than the maximum
            ///   finite float, then positive infinity is returned. If it is smaller than the
            ///   minimum finite float, then the minimum finite float is returned.
            /// - If the rounding mode is `Down`, then the rounding proceeds as with `Floor` if the
            ///   [`Integer`] is non-negative and as with `Ceiling` if the [`Integer`] is negative.
            /// - If the rounding mode is `Up`, then the rounding proceeds as with `Ceiling` if the
            ///   [`Integer`] is non-negative and as with `Floor` if the [`Integer`] is negative.
            /// - If the rounding mode is `Nearest`, then the nearest float is returned. If the
            ///   [`Integer`] is exactly between two floats, the float with the zero
            ///   least-significant bit in its representation is selected. If the [`Integer`] is
            ///   greater than the maximum finite float, then the maximum finite float is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Panics
            /// Panics if the rounding mode is `Exact` and `value` cannot be represented exactly.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_integer#rounding_from).
            fn rounding_from(value: &'a Integer, rm: RoundingMode) -> $f {
                if value.sign {
                    $f::rounding_from(&value.abs, rm)
                } else {
                    -$f::rounding_from(&value.abs, -rm)
                }
            }
        }

        impl<'a> From<&'a Integer> for $f {
            /// Converts an [`Integer`] to a primitive float.
            ///
            /// If there are two nearest floats, the one whose least-significant bit is zero is
            /// chosen. If the [`Integer`] is larger than the maximum finite float, then the result
            /// is the maximum finite float.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_integer#from).
            fn from(value: &'a Integer) -> $f {
                let abs = $f::from(&value.abs);
                if value.sign {
                    abs
                } else {
                    -abs
                }
            }
        }

        impl<'a> CheckedFrom<&'a Integer> for $f {
            /// Converts an [`Integer`] to a primitive float.
            ///
            /// If the input isn't exactly equal to some float, `None` is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_integer#checked_from).
            fn checked_from(value: &'a Integer) -> Option<$f> {
                $f::checked_from(&value.abs).map(|f| if value.sign { f } else { -f })
            }
        }

        impl<'a> ConvertibleFrom<&'a Integer> for $f {
            /// Determines whether an [`Integer`] can be exactly converted to a primitive float.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::primitive_float_from_integer#convertible_from).
            fn convertible_from(value: &'a Integer) -> bool {
                $f::convertible_from(&value.abs)
            }
        }
    };
}
apply_to_primitive_floats!(float_impls);
