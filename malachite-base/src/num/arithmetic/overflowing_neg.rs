use crate::num::arithmetic::traits::{OverflowingNeg, OverflowingNegAssign};

macro_rules! impl_overflowing_neg {
    ($t:ident) => {
        impl OverflowingNeg for $t {
            type Output = $t;

            /// This is a wrapper over the `overflowing_neg` functions in the standard library, for
            /// example [this one](u32::overflowing_neg).
            #[inline]
            fn overflowing_neg(self) -> ($t, bool) {
                $t::overflowing_neg(self)
            }
        }

        impl OverflowingNegAssign for $t {
            /// Negates a number in place.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow occurred. If an
            /// overflow occurred, then the wrapped value is assigned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::overflowing_neg#overflowing_neg_assign).
            #[inline]
            fn overflowing_neg_assign(&mut self) -> bool {
                let overflow;
                (*self, overflow) = self.overflowing_neg();
                overflow
            }
        }
    };
}
apply_to_primitive_ints!(impl_overflowing_neg);
