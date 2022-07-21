use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::NotAssign;
use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use std::mem::swap;
use std::ops::{Sub, SubAssign};

impl Sub<Integer> for Integer {
    type Output = Integer;

    /// Subtracts an [`Integer`] by another [`Integer`], taking both by value.
    ///
    /// $$
    /// f(x, y) = x - y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$ (only if the underlying [`Vec`] needs to reallocate)
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO - Integer::from(123), -123);
    /// assert_eq!(Integer::from(123) - Integer::ZERO, 123);
    /// assert_eq!(Integer::from(456) - Integer::from(-123), 579);
    /// assert_eq!(
    ///     -Integer::from(10u32).pow(12) - -Integer::from(10u32).pow(12) * Integer::from(2u32),
    ///     1000000000000u64
    /// );
    /// ```
    #[inline]
    fn sub(mut self, other: Integer) -> Integer {
        self -= other;
        self
    }
}

impl<'a> Sub<&'a Integer> for Integer {
    type Output = Integer;

    /// Subtracts an [`Integer`] by another [`Integer`], taking the first by value and the second
    /// by reference.
    ///
    /// $$
    /// f(x, y) = x - y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO - &Integer::from(123), -123);
    /// assert_eq!(Integer::from(123) - &Integer::ZERO, 123);
    /// assert_eq!(Integer::from(456) - &Integer::from(-123), 579);
    /// assert_eq!(
    ///     -Integer::from(10u32).pow(12) - &(-Integer::from(10u32).pow(12) * Integer::from(2u32)),
    ///     1000000000000u64
    /// );
    /// ```
    #[inline]
    fn sub(mut self, other: &'a Integer) -> Integer {
        self -= other;
        self
    }
}

impl<'a> Sub<Integer> for &'a Integer {
    type Output = Integer;

    /// Subtracts an [`Integer`] by another [`Integer`], taking the first by reference and the
    /// second by value.
    ///
    /// $$
    /// f(x, y) = x - y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(&Integer::ZERO - Integer::from(123), -123);
    /// assert_eq!(&Integer::from(123) - Integer::ZERO, 123);
    /// assert_eq!(&Integer::from(456) - Integer::from(-123), 579);
    /// assert_eq!(
    ///     &-Integer::from(10u32).pow(12) - -Integer::from(10u32).pow(12) * Integer::from(2u32),
    ///     1000000000000u64
    /// );
    /// ```
    fn sub(self, mut other: Integer) -> Integer {
        other -= self;
        -other
    }
}

impl<'a, 'b> Sub<&'a Integer> for &'b Integer {
    type Output = Integer;

    /// Subtracts an [`Integer`] by another [`Integer`], taking both by reference.
    ///
    /// $$
    /// f(x, y) = x - y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(&Integer::ZERO - &Integer::from(123), -123);
    /// assert_eq!(&Integer::from(123) - &Integer::ZERO, 123);
    /// assert_eq!(&Integer::from(456) - &Integer::from(-123), 579);
    /// assert_eq!(
    ///     &-Integer::from(10u32).pow(12) -
    ///             &(-Integer::from(10u32).pow(12) * Integer::from(2u32)),
    ///     1000000000000u64
    /// );
    /// ```
    fn sub(self, other: &'a Integer) -> Integer {
        match (self, other) {
            (x, y) if std::ptr::eq(x, y) => Integer::ZERO,
            (integer_zero!(), y) => -y.clone(),
            (x, &integer_zero!()) => x.clone(),
            // e.g. 10 - -5 or -10 - 5; sign of result is sign of self
            (
                &Integer {
                    sign: sx,
                    abs: ref ax,
                },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) if sx == (!sy && *ay != 0) => Integer {
                sign: sx,
                abs: ax + ay,
            },
            // e.g. 10 - 5, -10 - -5, or 5 - 5; sign of result is sign of self
            (
                &Integer {
                    sign: sx,
                    abs: ref ax,
                },
                &Integer { abs: ref ay, .. },
            ) if sx && *ax == *ay || *ax > *ay => Integer {
                sign: sx,
                abs: ax - ay,
            },
            // e.g. 5 - 10, -5 - -10, or -5 - -5; sign of result is opposite of sign of other
            (
                &Integer { abs: ref ax, .. },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) => Integer {
                sign: !sy,
                abs: ay - ax,
            },
        }
    }
}

impl SubAssign<Integer> for Integer {
    /// Subtracts an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by value.
    ///
    /// $$
    /// x \gets x - y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$ (only if the underlying [`Vec`] needs to reallocate)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::ZERO;
    /// x -= -Integer::from(10u32).pow(12);
    /// x -= Integer::from(10u32).pow(12) * Integer::from(2u32);
    /// x -= -Integer::from(10u32).pow(12) * Integer::from(3u32);
    /// x -= Integer::from(10u32).pow(12) * Integer::from(4u32);
    /// assert_eq!(x, -2000000000000i64);
    /// ```
    fn sub_assign(&mut self, mut other: Integer) {
        match (&mut *self, &other) {
            (_, &integer_zero!()) => {}
            (&mut integer_zero!(), _) => {
                *self = other;
                self.neg_assign();
            }
            // e.g. 10 - -5 or -10 - 5; sign of self is unchanged
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) if sx == (!sy && *ay != 0) => *ax += ay,
            // e.g. 10 - 5, -10 - -5, or 5 - 5; sign of self is unchanged
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Integer { abs: ref ay, .. },
            ) if sx && *ax == *ay || *ax > *ay => *ax -= ay,
            // e.g. 5 - 10, -5 - -10, or -5 - -5; sign of self is flipped
            _ => {
                swap(self, &mut other);
                self.abs -= other.abs;
                self.sign.not_assign();
            }
        }
    }
}

impl<'a> SubAssign<&'a Integer> for Integer {
    /// Subtracts an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by reference.
    ///
    /// $$
    /// x \gets x - y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::ZERO;
    /// x -= &(-Integer::from(10u32).pow(12));
    /// x -= &(Integer::from(10u32).pow(12) * Integer::from(2u32));
    /// x -= &(-Integer::from(10u32).pow(12) * Integer::from(3u32));
    /// x -= &(Integer::from(10u32).pow(12) * Integer::from(4u32));
    /// assert_eq!(x, -2000000000000i64);
    /// ```
    fn sub_assign(&mut self, other: &'a Integer) {
        match (&mut *self, other) {
            (_, &integer_zero!()) => {}
            (&mut integer_zero!(), y) => *self = -y.clone(),
            // e.g. 10 - -5 or -10 - 5; sign of self is unchanged
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) if sx == (!sy && *ay != 0) => *ax += ay,
            // e.g. 10 - 5, -10 - -5, or 5 - 5; sign of self is unchanged
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Integer { abs: ref ay, .. },
            ) if sx && *ax == *ay || *ax > *ay => *ax -= ay,
            (
                &mut Integer {
                    sign: ref mut sx,
                    abs: ref mut ax,
                },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) => {
                *sx = !sy;
                *ax = ay - &*ax;
            }
        }
    }
}
