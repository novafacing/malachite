use crate::Float;
use crate::InnerFloat::{Finite, Zero};
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;

fn float_partial_eq_unsigned<T: PrimitiveUnsigned>(x: &Float, y: &T) -> bool
where
    Natural: From<T>,
{
    match x {
        float_either_zero!() => *y == T::ZERO,
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            *y != T::ZERO
                && *sign
                && *exponent >= 0
                && y.significant_bits() == exponent.unsigned_abs()
                && significand.cmp_normalized(&Natural::from(*y)) == Ordering::Equal
        }
        _ => false,
    }
}

macro_rules! impl_partial_eq_unsigned {
    ($t: ident) => {
        impl PartialEq<$t> for Float {
            /// Determines whether a [`Float`] is equal to an unsigned primitive integer.
            ///
            /// Infinity, negative infinity, and NaN are not equal to any primitive integer. Both
            /// the [`Float`] zero and the [`Float`] negative zero are equal to the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                float_partial_eq_unsigned(self, other)
            }
        }

        impl PartialEq<Float> for $t {
            /// Determines whether an unsigned primitive integer is equal to a [`Float`].
            ///
            /// No primitive integer is equal to infinity, negative infinity, or NaN. The integer
            /// zero is equal to both the [`Float`] zero and the [`Float`] negative zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &Float) -> bool {
                other == self
            }
        }
    };
}
apply_to_unsigneds!(impl_partial_eq_unsigned);

fn float_partial_eq_signed<T: PrimitiveSigned>(x: &Float, y: &T) -> bool
where
    Natural: From<<T as UnsignedAbs>::Output>,
{
    match x {
        float_either_zero!() => *y == T::ZERO,
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            *y != T::ZERO
                && *sign == (*y >= T::ZERO)
                && *exponent >= 0
                && y.significant_bits() == exponent.unsigned_abs()
                && significand.cmp_normalized(&Natural::from(y.unsigned_abs())) == Ordering::Equal
        }
        _ => false,
    }
}

macro_rules! impl_partial_eq_signed {
    ($t: ident) => {
        impl PartialEq<$t> for Float {
            /// Determines whether a [`Float`] is equal to a signed primitive integer.
            ///
            /// Infinity, negative infinity, and NaN are not equal to any primitive integer. Both
            /// the [`Float`] zero and the [`Float`] negative zero are equal to the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                float_partial_eq_signed(self, other)
            }
        }

        impl PartialEq<Float> for $t {
            /// Determines whether a signed primitive integer is equal to a [`Float`].
            ///
            /// No primitive integer is equal to infinity, negative infinity, or NaN. The integer
            /// zero is equal to both the [`Float`] zero and the [`Float`] negative zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq(&self, other: &Float) -> bool {
                other == self
            }
        }
    };
}
apply_to_signeds!(impl_partial_eq_signed);
