use integer::Integer::{self, Large, Small};

/// Determines whether an `Integer` is equal to an `i32`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert!(Integer::from(123) == 123);
/// assert!(Integer::from(123) != -5);
/// ```
impl PartialEq<i32> for Integer {
    fn eq(&self, other: &i32) -> bool {
        match *self {
            Small(x) => x == *other,
            Large(_) => false,
        }
    }
}

/// Determines whether an `i32` is equal to an `Integer`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert!(123 == Integer::from(123));
/// assert!(-5 != Integer::from(123));
/// ```
impl PartialEq<Integer> for i32 {
    fn eq(&self, other: &Integer) -> bool {
        match *other {
            Small(y) => y == *self,
            Large(_) => false,
        }
    }
}
