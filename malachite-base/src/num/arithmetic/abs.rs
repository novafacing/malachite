use crate::num::arithmetic::traits::{Abs, AbsAssign, UnsignedAbs};

macro_rules! impl_abs_primitive_int {
    ($u:ident, $s:ident) => {
        impl Abs for $s {
            type Output = $s;

            /// This is a wrapper over the `abs` functions in the standard library, for example
            /// [this one](i32::abs).
            #[inline]
            fn abs(self) -> $s {
                $s::abs(self)
            }
        }

        impl AbsAssign for $s {
            /// Replaces a number with its absolute value.
            ///
            /// $x \gets |x|$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::abs#abs_assign).
            #[inline]
            fn abs_assign(&mut self) {
                *self = self.abs();
            }
        }

        impl UnsignedAbs for $s {
            type Output = $u;

            /// This is a wrapper over the `unsigned_abs` functions in the standard library, for
            /// example [this one](i32::unsigned_abs).
            #[inline]
            fn unsigned_abs(self) -> $u {
                self.unsigned_abs()
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_abs_primitive_int);

macro_rules! impl_abs_primitive_float {
    ($f:ident) => {
        impl Abs for $f {
            type Output = $f;

            /// This is a wrapper over the `abs` functions in the standard library, for example
            /// [this one](f32::abs).
            #[inline]
            fn abs(self) -> $f {
                $f::abs(self)
            }
        }

        impl AbsAssign for $f {
            /// Replaces a number with its absolute value.
            ///
            /// $x \gets |x|$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::abs#abs_assign).
            #[inline]
            fn abs_assign(&mut self) {
                *self = self.abs();
            }
        }
    };
}
apply_to_primitive_floats!(impl_abs_primitive_float);
