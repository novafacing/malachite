use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    ModMul, ModMulAssign, ModPow, ModShr, ModShrAssign, UnsignedAbs,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Two, Zero};
use std::cmp::Ordering;
use std::ops::{Shr, ShrAssign};

fn mod_shr_ref_val<'a, U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &'a Natural,
    bits: S,
    m: Natural,
) -> Natural
where
    Natural: From<U>,
    &'a Natural: Shr<U, Output = Natural>,
{
    let bits_abs = bits.unsigned_abs();
    match bits.cmp(&S::ZERO) {
        Ordering::Equal => x.clone(),
        Ordering::Greater => x >> bits_abs,
        Ordering::Less => match m {
            Natural::ONE | Natural::TWO => Natural::ZERO,
            _ => x.mod_mul(Natural::TWO.mod_pow(Natural::from(bits_abs), &m), m),
        },
    }
}

fn mod_shr_ref_ref<'a, U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &'a Natural,
    bits: S,
    m: &Natural,
) -> Natural
where
    Natural: From<U>,
    &'a Natural: Shr<U, Output = Natural>,
{
    let bits_abs = bits.unsigned_abs();
    match bits.cmp(&S::ZERO) {
        Ordering::Equal => x.clone(),
        Ordering::Greater => x >> bits_abs,
        Ordering::Less => match m {
            &Natural::ONE | &Natural::TWO => Natural::ZERO,
            _ => x.mod_mul(Natural::TWO.mod_pow(Natural::from(bits_abs), m), m),
        },
    }
}

fn mod_shr_assign<U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &mut Natural,
    bits: S,
    m: Natural,
) where
    Natural: From<U> + ShrAssign<U>,
{
    let bits_abs = bits.unsigned_abs();
    match bits.cmp(&S::ZERO) {
        Ordering::Equal => {}
        Ordering::Greater => *x >>= bits_abs,
        Ordering::Less => match m {
            Natural::ONE | Natural::TWO => *x = Natural::ZERO,
            _ => x.mod_mul_assign(Natural::TWO.mod_pow(Natural::from(bits_abs), &m), m),
        },
    }
}

fn mod_shr_assign_ref<U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &mut Natural,
    bits: S,
    m: &Natural,
) where
    Natural: From<U> + ShrAssign<U>,
{
    let bits_abs = bits.unsigned_abs();
    match bits.cmp(&S::ZERO) {
        Ordering::Equal => {}
        Ordering::Greater => *x >>= bits_abs,
        Ordering::Less => match m {
            &Natural::ONE | &Natural::TWO => *x = Natural::ZERO,
            _ => x.mod_mul_assign(Natural::TWO.mod_pow(Natural::from(bits_abs), m), m),
        },
    }
}

macro_rules! impl_mod_shr {
    ($t:ident) => {
        impl ModShr<$t, Natural> for Natural {
            type Output = Natural;

            /// Right-shifts a [`Natural`] (divides it by a power of 2) modulo another [`Natural`]
            /// $m$. Assumes the input is already reduced modulo $m$. Both [`Natural`]s are taken
            /// by value.
            ///
            /// $f(x, n, m) = y$, where $x, y < m$ and $\lfloor 2^{-n}x \rfloor \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shr#mod_shr).
            #[inline]
            fn mod_shr(mut self, bits: $t, m: Natural) -> Natural {
                self.mod_shr_assign(bits, m);
                self
            }
        }

        impl<'a> ModShr<$t, &'a Natural> for Natural {
            type Output = Natural;

            /// Right-shifts a [`Natural`] (divides it by a power of 2) modulo another [`Natural`]
            /// $m$. Assumes the input is already reduced modulo $m$. The first [`Natural`] is
            /// taken by value and the second by reference.
            ///
            /// $f(x, n, m) = y$, where $x, y < m$ and $\lfloor 2^{-n}x \rfloor \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shr#mod_shr).
            #[inline]
            fn mod_shr(mut self, bits: $t, m: &'a Natural) -> Natural {
                self.mod_shr_assign(bits, m);
                self
            }
        }

        impl<'a> ModShr<$t, Natural> for &'a Natural {
            type Output = Natural;

            /// Right-shifts a [`Natural`] (divides it by a power of 2) modulo another [`Natural`]
            /// $m$. Assumes the input is already reduced modulo $m$. The first [`Natural`] is
            /// taken by reference and the second by value.
            ///
            /// $f(x, n, m) = y$, where $x, y < m$ and $\lfloor 2^{-n}x \rfloor \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shr#mod_shr).
            #[inline]
            fn mod_shr(self, bits: $t, m: Natural) -> Natural {
                mod_shr_ref_val(self, bits, m)
            }
        }

        impl<'a, 'b> ModShr<$t, &'b Natural> for &'a Natural {
            type Output = Natural;

            /// Right-shifts a [`Natural`] (divides it by a power of 2) modulo another [`Natural`]
            /// $m$. Assumes the input is already reduced modulo $m$. Both [`Natural`]s are taken
            /// by reference.
            ///
            /// $f(x, n, m) = y$, where $x, y < m$ and $\lfloor 2^{-n}x \rfloor \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shr#mod_shr).
            #[inline]
            fn mod_shr(self, bits: $t, m: &'b Natural) -> Natural {
                mod_shr_ref_ref(self, bits, m)
            }
        }

        impl ModShrAssign<$t, Natural> for Natural {
            /// Right-shifts a [`Natural`] (divides it by a power of 2) modulo another [`Natural`]
            /// $m$, in place. Assumes the input is already reduced modulo $m$. The [`Natural`] on
            /// the right-hand side is taken by value.
            ///
            /// $x \gets y$, where $x, y < m$ and $\lfloor 2^{-n}x \rfloor \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shr#mod_shr_assign).
            #[inline]
            fn mod_shr_assign(&mut self, bits: $t, m: Natural) {
                mod_shr_assign(self, bits, m);
            }
        }

        impl<'a> ModShrAssign<$t, &'a Natural> for Natural {
            /// Right-shifts a [`Natural`] (divides it by a power of 2) modulo another [`Natural`]
            /// $m$, in place. Assumes the input is already reduced modulo $m$. The [`Natural`] on
            /// the right-hand side is taken by reference.
            ///
            /// $x \gets y$, where $x, y < m$ and $\lfloor 2^{-n}x \rfloor \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(mn \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $m$
            /// is `bits`.
            ///
            /// # Examples
            /// See [here](super::mod_shr#mod_shr_assign).
            #[inline]
            fn mod_shr_assign(&mut self, bits: $t, m: &'a Natural) {
                mod_shr_assign_ref(self, bits, m);
            }
        }
    };
}
apply_to_signeds!(impl_mod_shr);
