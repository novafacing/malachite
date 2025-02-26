use crate::num::arithmetic::traits::ModPowerOf2Inverse;
use crate::num::basic::unsigneds::PrimitiveUnsigned;

// Uses Newton's method, as described by Colin Plumb in
// https://groups.google.com/g/sci.crypt/c/UI-UMbUnYGk/m/hX2-wQVyE3oJ.
pub_test! {mod_power_of_2_inverse_fast<T: PrimitiveUnsigned>(x: T, pow: u64) -> Option<T> {
    assert_ne!(x, T::ZERO);
    assert!(pow <= T::WIDTH);
    assert!(x.significant_bits() <= pow);
    if x.even() {
        return None;
    } else if x == T::ONE {
        return Some(T::ONE);
    }
    let mut small_pow = 2;
    let mut inverse = x.mod_power_of_2(2);
    while small_pow < pow {
        small_pow <<= 1;
        if small_pow > pow {
            small_pow = pow;
        }
        // inverse <- inverse * (2 - inverse * x) mod 2^small_pow
        inverse.mod_power_of_2_mul_assign(
            T::TWO.mod_power_of_2_sub(
                inverse.mod_power_of_2_mul(x.mod_power_of_2(small_pow), small_pow),
                small_pow,
            ),
            small_pow,
        );
    }
    Some(inverse)
}}

macro_rules! impl_mod_power_of_2_inverse {
    ($u:ident) => {
        impl ModPowerOf2Inverse for $u {
            type Output = $u;

            /// Computes the multiplicative inverse of a number modulo $2^k$. Assumes the number is
            /// already reduced modulo $2^k$.
            ///
            /// Returns `None` if $x$ is even.
            ///
            /// $f(x, k) = y$, where $x, y < 2^k$, $x$ is odd, and $xy \equiv 1 \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_inverse#mod_power_of_2_inverse).
            #[inline]
            fn mod_power_of_2_inverse(self, pow: u64) -> Option<$u> {
                mod_power_of_2_inverse_fast(self, pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_inverse);
