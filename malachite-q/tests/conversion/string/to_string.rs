use malachite_base::strings::string_is_subset;
use malachite_base::strings::ToDebugString;
use malachite_nz_test_util::generators::integer_gen;
use malachite_q::Rational;
use malachite_q_test_util::common::{rational_to_bigrational, rational_to_rug_rational};
use malachite_q_test_util::generators::rational_gen;
use std::str::FromStr;

#[test]
pub fn test_to_string() {
    fn test(u: &str) {
        let x = Rational::from_str(u).unwrap();
        assert_eq!(x.to_string(), u);
        assert_eq!(x.to_debug_string(), u);
    }
    test("0");
    test("2");
    test("123");
    test("1000");
    test("1000000");
    test("1000000000000000");
    test("-2");
    test("-123");
    test("-1000");
    test("-1000000");
    test("-1000000000000000");
    test("99/100");
    test("101/100");
    test("22/7");
    test("-99/100");
    test("-101/100");
    test("-22/7");
}

#[test]
fn to_string_properties() {
    rational_gen().test_properties(|x| {
        let s = x.to_string();
        assert_eq!(x.to_debug_string(), s);
        assert_eq!(rational_to_bigrational(&x).to_string(), s);
        assert_eq!(rational_to_rug_rational(&x).to_string(), s);
        assert!(string_is_subset(&s, "-/0123456789"));
        if x != 0 {
            assert!(!s.starts_with('0'));
        }
    });

    integer_gen().test_properties(|x| {
        assert_eq!(Rational::from(&x).to_string(), x.to_string());
    });
}
