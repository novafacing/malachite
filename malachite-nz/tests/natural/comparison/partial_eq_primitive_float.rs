use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::common::natural_to_rug_integer;
use malachite_nz_test_util::generators::natural_primitive_float_pair_gen;
use rug;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_eq_primitive_float() {
    let test = |u, v: f32, out| {
        assert_eq!(Natural::from_str(u).unwrap() == v, out);
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Natural::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);

        let v = v as f64;
        assert_eq!(Natural::from_str(u).unwrap() == v, out);
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Natural::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("5", f32::NAN, false);
    test("5", f32::POSITIVE_INFINITY, false);
    test("5", f32::NEGATIVE_INFINITY, false);

    test("0", 0.0, true);
    test("0", -0.0, true);
    test("0", 5.0, false);
    test("0", -5.0, false);
    test("123", 123.0, true);
    test("123", 5.0, false);
    test("123", -123.0, false);
    test("1000000000000", 123.0, false);

    test("1208925819614629174706175", 1.2089258e24, false);
    test("1208925819614629174706176", 1.2089258e24, true);
    test("1208925819614629174706177", 1.2089258e24, false);
    test("1208925819614629174706175", -1.2089258e24, false);
    test("1208925819614629174706176", -1.2089258e24, false);
    test("1208925819614629174706177", -1.2089258e24, false);
}

fn partial_eq_primitive_float_properties_helper<
    T: PartialEq<Natural> + PartialEq<rug::Integer> + PrimitiveFloat,
>()
where
    Natural: From<T> + PartialEq<T> + PartialOrd<T>,
    rug::Integer: PartialEq<T>,
{
    natural_primitive_float_pair_gen::<T>().test_properties(|(n, f)| {
        let eq = n == f;
        assert_eq!(natural_to_rug_integer(&n) == f, eq);
        assert_eq!(f == n, eq);
        assert_eq!(f == natural_to_rug_integer(&n), eq);
        assert_eq!(n.partial_cmp(&f) == Some(Ordering::Equal), eq);
        if eq {
            assert!(f.is_integer());
        }
    });
}

#[test]
fn partial_eq_primitive_float_properties() {
    apply_fn_to_primitive_floats!(partial_eq_primitive_float_properties_helper);
}
