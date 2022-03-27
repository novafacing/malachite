use malachite_base::num::arithmetic::traits::{FloorLogBase, Pow, PowerOf2, RoundToMultiple};
use malachite_base::num::conversion::string::options::{
    FromSciStringOptions, SciSizeOptions, ToSciOptions,
};
use malachite_base::num::conversion::traits::{FromSciString, ToSci};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::strings::string_is_subset;
use malachite_base_test_util::generators::{signed_gen, signed_to_sci_options_pair_gen_var_1};
use malachite_base_test_util::num::conversion::string::from_sci_string::DECIMAL_SCI_STRING_CHARS;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::generators::{integer_gen, integer_to_sci_options_pair_gen_var_1};
use std::collections::HashMap;
use std::str::FromStr;

#[test]
pub fn test_to_sci() {
    assert_eq!(
        Integer::power_of_2(1000000).to_sci().to_string(),
        "9.900656229295898e301029"
    );
    assert_eq!(
        (-Integer::power_of_2(1000000)).to_sci().to_string(),
        "-9.900656229295898e301029"
    );

    fn test_i(x: Integer, out: &str) {
        assert_eq!(x.to_sci().to_string(), out);
        assert_eq!(
            x.to_sci_with_options(ToSciOptions::default()).to_string(),
            out
        );
    }
    fn test(s: &str, out: &str) {
        test_i(Integer::from_str(s).unwrap(), out);
    }
    test("0", "0");
    test("1", "1");
    test("10", "10");
    test("100", "100");
    test("1000", "1000");
    test("10000", "10000");
    test("100000", "100000");
    test("1000000", "1000000");
    test("10000000", "10000000");
    test("100000000", "100000000");
    test("1000000000", "1000000000");
    test("10000000000", "10000000000");
    test("100000000000", "100000000000");
    test("1000000000000", "1000000000000");
    test("10000000000000", "10000000000000");
    test("100000000000000", "100000000000000");
    test("1000000000000000", "1000000000000000");
    test("10000000000000000", "1e16");
    test("100000000000000000", "1e17");
    test_i(Integer::from(u64::MAX), "1.844674407370955e19");
    test_i(Integer::from(u128::MAX), "3.402823669209385e38");
    test_i(Integer::from(i64::MAX), "9.223372036854776e18");
    test_i(Integer::from(i128::MAX), "1.701411834604692e38");

    test("999999999999999", "999999999999999");
    test("9999999999999999", "9999999999999999");
    test("99999999999999999", "1e17");
    test("999999999999999999", "1e18");

    test("-1", "-1");
    test("-10", "-10");
    test("-100", "-100");
    test("-1000", "-1000");
    test("-10000", "-10000");
    test("-100000", "-100000");
    test("-1000000", "-1000000");
    test("-10000000", "-10000000");
    test("-100000000", "-100000000");
    test("-1000000000", "-1000000000");
    test("-10000000000", "-10000000000");
    test("-100000000000", "-100000000000");
    test("-1000000000000", "-1000000000000");
    test("-10000000000000", "-10000000000000");
    test("-100000000000000", "-100000000000000");
    test("-1000000000000000", "-1000000000000000");
    test("-10000000000000000", "-1e16");
    test("-100000000000000000", "-1e17");
    test_i(Integer::from(i64::MIN), "-9.223372036854776e18");
    test_i(Integer::from(i128::MIN), "-1.701411834604692e38");
}

#[test]
pub fn test_to_sci_with_options() {
    fn test_i(x: Integer, options: ToSciOptions, out: &str) {
        assert_eq!(x.to_sci_with_options(options).to_string(), out);
    }
    fn test(s: &str, options: ToSciOptions, out: &str) {
        test_i(Integer::from_str(s).unwrap(), options, out);
    }
    // For tests with the default options, see `test_to_sci`

    let mut options = ToSciOptions::default();
    options.set_include_trailing_zeros(true);
    test("0", options, "0.000000000000000");
    test("1", options, "1.000000000000000");
    test("10", options, "10.00000000000000");
    test("100", options, "100.0000000000000");
    test("1000", options, "1000.000000000000");
    test("10000", options, "10000.00000000000");
    test("100000", options, "100000.0000000000");
    test("1000000", options, "1000000.000000000");
    test("10000000", options, "10000000.00000000");
    test("100000000", options, "100000000.0000000");
    test("1000000000", options, "1000000000.000000");
    test("10000000000", options, "10000000000.00000");
    test("100000000000", options, "100000000000.0000");
    test("1000000000000", options, "1000000000000.000");
    test("10000000000000", options, "10000000000000.00");
    test("100000000000000", options, "100000000000000.0");
    test("1000000000000000", options, "1000000000000000");
    test("10000000000000000", options, "1.000000000000000e16");
    test("100000000000000000", options, "1.000000000000000e17");
    test_i(Integer::from(u64::MAX), options, "1.844674407370955e19");
    test_i(Integer::from(u128::MAX), options, "3.402823669209385e38");

    test("999999999999999", options, "999999999999999.0");
    test("9999999999999999", options, "9999999999999999");
    test("99999999999999999", options, "1.000000000000000e17");
    test("999999999999999999", options, "1.000000000000000e18");

    options = ToSciOptions::default();
    options.set_base(2);
    test_i(Integer::from(u128::MAX), options, "1e128");
    options.set_base(3);
    test_i(Integer::from(u128::MAX), options, "2.022011021210021e80");
    options.set_base(4);
    test_i(Integer::from(u128::MAX), options, "1e64");
    options.set_base(5);
    test_i(Integer::from(u128::MAX), options, "1.103111044120131e55");
    options.set_base(8);
    test_i(Integer::from(u128::MAX), options, "4e42");
    // When base >= 15, there is a mandatory sign after the exponent indicator "e", to distinguish
    // it from the digit "e"
    options.set_base(16);
    test_i(Integer::from(u128::MAX), options, "1e+32");
    options.set_base(32);
    test_i(Integer::from(u128::MAX), options, "8e+25");
    options.set_base(36);
    test_i(Integer::from(u128::MAX), options, "f.5lxx1zz5pnorynqe+24");

    // The sign can be forced in other cases too
    options.set_base(3);
    options.set_force_exponent_plus_sign(true);
    test_i(Integer::from(u128::MAX), options, "2.022011021210021e+80");

    // The digits can be uppercase, and so can the exponent indicator
    options = ToSciOptions::default();
    options.set_base(36);
    options.set_uppercase();
    test_i(Integer::from(u128::MAX), options, "F.5LXX1ZZ5PNORYNQe+24");

    options.set_lowercase();
    options.set_e_uppercase();
    test_i(Integer::from(u128::MAX), options, "f.5lxx1zz5pnorynqE+24");

    options.set_uppercase();
    test_i(Integer::from(u128::MAX), options, "F.5LXX1ZZ5PNORYNQE+24");

    options = ToSciOptions::default();
    options.set_size_complete();
    options.set_base(2);
    test_i(
        Integer::from(u128::MAX),
        options,
        "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111\
        111111111111111111111111111111111111111",
    );
    options.set_base(3);
    test_i(
        Integer::from(u128::MAX),
        options,
        "202201102121002021012000211012011021221022212021111001022110211020010021100121010",
    );
    options.set_base(4);
    test_i(
        Integer::from(u128::MAX),
        options,
        "3333333333333333333333333333333333333333333333333333333333333333",
    );
    options.set_base(5);
    test_i(
        Integer::from(u128::MAX),
        options,
        "11031110441201303134210404233413032443021130230130231310",
    );
    options.set_base(8);
    test_i(
        Integer::from(u128::MAX),
        options,
        "3777777777777777777777777777777777777777777",
    );
    options.set_base(16);
    test_i(
        Integer::from(u128::MAX),
        options,
        "ffffffffffffffffffffffffffffffff",
    );
    options.set_base(32);
    test_i(
        Integer::from(u128::MAX),
        options,
        "7vvvvvvvvvvvvvvvvvvvvvvvvv",
    );
    options.set_base(36);
    test_i(
        Integer::from(u128::MAX),
        options,
        "f5lxx1zz5pnorynqglhzmsp33",
    );

    options = ToSciOptions::default();
    options.set_precision(4);
    options.set_include_trailing_zeros(true);
    test("0", options, "0.000");
    test("1", options, "1.000");
    test("10", options, "10.00");
    test("100", options, "100.0");
    test("1000", options, "1000");
    test("10000", options, "1.000e4");
    test("9", options, "9.000");
    test("99", options, "99.00");
    test("999", options, "999.0");
    test("9999", options, "9999");
    test("99999", options, "1.000e5");

    options.set_include_trailing_zeros(false);
    test("0", options, "0");
    test("1", options, "1");
    test("10", options, "10");
    test("100", options, "100");
    test("1000", options, "1000");
    test("10000", options, "1e4");
    test("9", options, "9");
    test("99", options, "99");
    test("999", options, "999");
    test("9999", options, "9999");
    test("99999", options, "1e5");

    options = ToSciOptions::default();
    options.set_precision(1);
    options.set_include_trailing_zeros(true); // doesn't matter when precision is 1
    test("0", options, "0");
    test("1", options, "1");
    test("10", options, "1e1");
    test("100", options, "1e2");
    test("1000", options, "1e3");
    test("10000", options, "1e4");
    test("9", options, "9");
    test("99", options, "1e2");
    test("999", options, "1e3");
    test("9999", options, "1e4");
    test("99999", options, "1e5");

    options.set_include_trailing_zeros(false);
    test("0", options, "0");
    test("1", options, "1");
    test("10", options, "1e1");
    test("100", options, "1e2");
    test("1000", options, "1e3");
    test("10000", options, "1e4");
    test("9", options, "9");
    test("99", options, "1e2");
    test("999", options, "1e3");
    test("9999", options, "1e4");
    test("99999", options, "1e5");

    options = ToSciOptions::default();
    options.set_scale(2);
    options.set_include_trailing_zeros(true);
    test("0", options, "0.00");
    test("1", options, "1.00");
    test("10", options, "10.00");
    test("100", options, "100.00");
    test("1000", options, "1000.00");
    test("10000", options, "10000.00");
    test("9", options, "9.00");
    test("99", options, "99.00");
    test("999", options, "999.00");
    test("9999", options, "9999.00");
    test("99999", options, "99999.00");

    options.set_include_trailing_zeros(false);
    test("0", options, "0");
    test("1", options, "1");
    test("10", options, "10");
    test("100", options, "100");
    test("1000", options, "1000");
    test("10000", options, "10000");
    test("9", options, "9");
    test("99", options, "99");
    test("999", options, "999");
    test("9999", options, "9999");
    test("99999", options, "99999");

    options = ToSciOptions::default();
    options.set_scale(0);
    options.set_include_trailing_zeros(true); // doesn't matter when scale is 0
    test("0", options, "0");
    test("1", options, "1");
    test("10", options, "10");
    test("100", options, "100");
    test("1000", options, "1000");
    test("10000", options, "10000");
    test("9", options, "9");
    test("99", options, "99");
    test("999", options, "999");
    test("9999", options, "9999");
    test("99999", options, "99999");

    options.set_include_trailing_zeros(false);
    test("0", options, "0");
    test("1", options, "1");
    test("10", options, "10");
    test("100", options, "100");
    test("1000", options, "1000");
    test("10000", options, "10000");
    test("9", options, "9");
    test("99", options, "99");
    test("999", options, "999");
    test("9999", options, "9999");
    test("99999", options, "99999");

    options = ToSciOptions::default();
    options.set_precision(2);
    options.set_rounding_mode(RoundingMode::Nearest); // This is the default
    test("123", options, "1.2e2");
    options.set_rounding_mode(RoundingMode::Down);
    test("123", options, "1.2e2");
    options.set_rounding_mode(RoundingMode::Floor);
    test("123", options, "1.2e2");
    options.set_rounding_mode(RoundingMode::Up);
    test("123", options, "1.3e2");
    options.set_rounding_mode(RoundingMode::Ceiling);
    test("123", options, "1.3e2");

    options.set_rounding_mode(RoundingMode::Nearest);
    test("135", options, "1.4e2");
    options.set_rounding_mode(RoundingMode::Down);
    test("135", options, "1.3e2");
    options.set_rounding_mode(RoundingMode::Floor);
    test("135", options, "1.3e2");
    options.set_rounding_mode(RoundingMode::Up);
    test("135", options, "1.4e2");
    options.set_rounding_mode(RoundingMode::Ceiling);
    test("135", options, "1.4e2");

    options.set_rounding_mode(RoundingMode::Exact);
    test("140", options, "1.4e2");

    options.set_rounding_mode(RoundingMode::Nearest);
    test("999", options, "1e3");
    options.set_rounding_mode(RoundingMode::Down);
    test("999", options, "9.9e2");
    options.set_rounding_mode(RoundingMode::Floor);
    test("999", options, "9.9e2");
    options.set_rounding_mode(RoundingMode::Up);
    test("999", options, "1e3");
    options.set_rounding_mode(RoundingMode::Ceiling);
    test("999", options, "1e3");

    let mut options = ToSciOptions::default();
    options.set_include_trailing_zeros(true);
    test_i(Integer::from(i64::MAX), options, "9.223372036854776e18");
    test_i(Integer::from(i128::MAX), options, "1.701411834604692e38");
    test("-1", options, "-1.000000000000000");
    test("-10", options, "-10.00000000000000");
    test("-100", options, "-100.0000000000000");
    test("-1000", options, "-1000.000000000000");
    test("-10000", options, "-10000.00000000000");
    test("-100000", options, "-100000.0000000000");
    test("-1000000", options, "-1000000.000000000");
    test("-10000000", options, "-10000000.00000000");
    test("-100000000", options, "-100000000.0000000");
    test("-1000000000", options, "-1000000000.000000");
    test("-10000000000", options, "-10000000000.00000");
    test("-100000000000", options, "-100000000000.0000");
    test("-1000000000000", options, "-1000000000000.000");
    test("-10000000000000", options, "-10000000000000.00");
    test("-100000000000000", options, "-100000000000000.0");
    test("-1000000000000000", options, "-1000000000000000");
    test("-10000000000000000", options, "-1.000000000000000e16");
    test("-100000000000000000", options, "-1.000000000000000e17");
    test_i(Integer::from(i64::MIN), options, "-9.223372036854776e18");
    test_i(Integer::from(i128::MIN), options, "-1.701411834604692e38");

    test("-999999999999999", options, "-999999999999999.0");
    test("-9999999999999999", options, "-9999999999999999");
    test("-99999999999999999", options, "-1.000000000000000e17");
    test("-999999999999999999", options, "-1.000000000000000e18");

    options = ToSciOptions::default();
    options.set_base(2);
    test_i(Integer::from(i128::MAX), options, "1e127");
    test_i(Integer::from(i128::MIN), options, "-1e127");
    options.set_base(3);
    test_i(Integer::from(i128::MAX), options, "1.01100201022001e80");
    test_i(Integer::from(i128::MIN), options, "-1.01100201022001e80");
    options.set_base(4);
    test_i(Integer::from(i128::MAX), options, "2e63");
    test_i(Integer::from(i128::MIN), options, "-2e63");
    options.set_base(5);
    test_i(Integer::from(i128::MAX), options, "3.013030220323124e54");
    test_i(Integer::from(i128::MIN), options, "-3.013030220323124e54");
    options.set_base(8);
    test_i(Integer::from(i128::MAX), options, "2e42");
    test_i(Integer::from(i128::MIN), options, "-2e42");
    // When base >= 15, there is a mandatory sign after the exponent indicator "e", to distinguish
    // it from the digit "e"
    options.set_base(16);
    test_i(Integer::from(i128::MAX), options, "8e+31");
    test_i(Integer::from(i128::MIN), options, "-8e+31");
    options.set_base(32);
    test_i(Integer::from(i128::MAX), options, "4e+25");
    test_i(Integer::from(i128::MIN), options, "-4e+25");
    options.set_base(36);
    test_i(Integer::from(i128::MAX), options, "7.ksyyizzkutudzbve+24");
    test_i(Integer::from(i128::MIN), options, "-7.ksyyizzkutudzbve+24");

    // The sign can be forced in other cases too
    options.set_base(3);
    options.set_force_exponent_plus_sign(true);
    test_i(Integer::from(i128::MAX), options, "1.01100201022001e+80");
    test_i(Integer::from(i128::MIN), options, "-1.01100201022001e+80");

    // The digits can be uppercase, and so can the exponent indicator
    options = ToSciOptions::default();
    options.set_base(36);
    options.set_uppercase();
    test_i(Integer::from(i128::MAX), options, "7.KSYYIZZKUTUDZBVe+24");
    test_i(Integer::from(i128::MIN), options, "-7.KSYYIZZKUTUDZBVe+24");

    options.set_lowercase();
    options.set_e_uppercase();
    test_i(Integer::from(i128::MAX), options, "7.ksyyizzkutudzbvE+24");
    test_i(Integer::from(i128::MIN), options, "-7.ksyyizzkutudzbvE+24");

    options.set_uppercase();
    test_i(Integer::from(i128::MAX), options, "7.KSYYIZZKUTUDZBVE+24");
    test_i(Integer::from(i128::MIN), options, "-7.KSYYIZZKUTUDZBVE+24");

    options = ToSciOptions::default();
    options.set_size_complete();
    options.set_base(2);
    test_i(
        Integer::from(i128::MAX),
        options,
        "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111\
        11111111111111111111111111111111111111",
    );
    test_i(
        Integer::from(i128::MIN),
        options,
        "-1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000",
    );
    options.set_base(3);
    test_i(
        Integer::from(i128::MAX),
        options,
        "101100201022001010121000102002120122110122221010202000122201220121120010200022001",
    );
    test_i(
        Integer::from(i128::MIN),
        options,
        "-101100201022001010121000102002120122110122221010202000122201220121120010200022002",
    );
    options.set_base(4);
    test_i(
        Integer::from(i128::MAX),
        options,
        "1333333333333333333333333333333333333333333333333333333333333333",
    );
    test_i(
        Integer::from(i128::MIN),
        options,
        "-2000000000000000000000000000000000000000000000000000000000000000",
    );
    options.set_base(5);
    test_i(
        Integer::from(i128::MAX),
        options,
        "3013030220323124042102424341431241221233040112312340402",
    );
    test_i(
        Integer::from(i128::MIN),
        options,
        "-3013030220323124042102424341431241221233040112312340403",
    );
    options.set_base(8);
    test_i(
        Integer::from(i128::MAX),
        options,
        "1777777777777777777777777777777777777777777",
    );
    test_i(
        Integer::from(i128::MIN),
        options,
        "-2000000000000000000000000000000000000000000",
    );
    options.set_base(16);
    test_i(
        Integer::from(i128::MAX),
        options,
        "7fffffffffffffffffffffffffffffff",
    );
    test_i(
        Integer::from(i128::MIN),
        options,
        "-80000000000000000000000000000000",
    );
    options.set_base(32);
    test_i(
        Integer::from(i128::MAX),
        options,
        "3vvvvvvvvvvvvvvvvvvvvvvvvv",
    );
    test_i(
        Integer::from(i128::MIN),
        options,
        "-40000000000000000000000000",
    );
    options.set_base(36);
    test_i(
        Integer::from(i128::MAX),
        options,
        "7ksyyizzkutudzbv8aqztecjj",
    );
    test_i(
        Integer::from(i128::MIN),
        options,
        "-7ksyyizzkutudzbv8aqztecjk",
    );

    options = ToSciOptions::default();
    options.set_precision(4);
    options.set_include_trailing_zeros(true);
    test("-1", options, "-1.000");
    test("-10", options, "-10.00");
    test("-100", options, "-100.0");
    test("-1000", options, "-1000");
    test("-10000", options, "-1.000e4");
    test("-9", options, "-9.000");
    test("-99", options, "-99.00");
    test("-999", options, "-999.0");
    test("-9999", options, "-9999");
    test("-99999", options, "-1.000e5");

    options.set_include_trailing_zeros(false);
    test("-1", options, "-1");
    test("-10", options, "-10");
    test("-100", options, "-100");
    test("-1000", options, "-1000");
    test("-10000", options, "-1e4");
    test("-9", options, "-9");
    test("-99", options, "-99");
    test("-999", options, "-999");
    test("-9999", options, "-9999");
    test("-99999", options, "-1e5");

    options = ToSciOptions::default();
    options.set_precision(1);
    options.set_include_trailing_zeros(true); // doesn't matter when precision is 1
    test("-1", options, "-1");
    test("-10", options, "-1e1");
    test("-100", options, "-1e2");
    test("-1000", options, "-1e3");
    test("-10000", options, "-1e4");
    test("-9", options, "-9");
    test("-99", options, "-1e2");
    test("-999", options, "-1e3");
    test("-9999", options, "-1e4");
    test("-99999", options, "-1e5");

    options.set_include_trailing_zeros(false);
    test("-1", options, "-1");
    test("-10", options, "-1e1");
    test("-100", options, "-1e2");
    test("-1000", options, "-1e3");
    test("-10000", options, "-1e4");
    test("-9", options, "-9");
    test("-99", options, "-1e2");
    test("-999", options, "-1e3");
    test("-9999", options, "-1e4");
    test("-99999", options, "-1e5");

    options = ToSciOptions::default();
    options.set_scale(2);
    options.set_include_trailing_zeros(true);
    test("-1", options, "-1.00");
    test("-10", options, "-10.00");
    test("-100", options, "-100.00");
    test("-1000", options, "-1000.00");
    test("-10000", options, "-10000.00");
    test("-9", options, "-9.00");
    test("-99", options, "-99.00");
    test("-999", options, "-999.00");
    test("-9999", options, "-9999.00");
    test("-99999", options, "-99999.00");

    options.set_include_trailing_zeros(false);
    test("-1", options, "-1");
    test("-10", options, "-10");
    test("-100", options, "-100");
    test("-1000", options, "-1000");
    test("-10000", options, "-10000");
    test("-9", options, "-9");
    test("-99", options, "-99");
    test("-999", options, "-999");
    test("-9999", options, "-9999");
    test("-99999", options, "-99999");

    options = ToSciOptions::default();
    options.set_scale(0);
    options.set_include_trailing_zeros(true); // doesn't matter when scale is 0
    test("-1", options, "-1");
    test("-10", options, "-10");
    test("-100", options, "-100");
    test("-1000", options, "-1000");
    test("-10000", options, "-10000");
    test("-9", options, "-9");
    test("-99", options, "-99");
    test("-999", options, "-999");
    test("-9999", options, "-9999");
    test("-99999", options, "-99999");

    options.set_include_trailing_zeros(false);
    test("-1", options, "-1");
    test("-10", options, "-10");
    test("-100", options, "-100");
    test("-1000", options, "-1000");
    test("-10000", options, "-10000");
    test("-9", options, "-9");
    test("-99", options, "-99");
    test("-999", options, "-999");
    test("-9999", options, "-9999");
    test("-99999", options, "-99999");

    options = ToSciOptions::default();
    options.set_precision(2);
    options.set_rounding_mode(RoundingMode::Nearest); // This is the default
    test("-123", options, "-1.2e2");
    options.set_rounding_mode(RoundingMode::Down);
    test("-123", options, "-1.2e2");
    options.set_rounding_mode(RoundingMode::Floor);
    test("-123", options, "-1.3e2");
    options.set_rounding_mode(RoundingMode::Up);
    test("-123", options, "-1.3e2");
    options.set_rounding_mode(RoundingMode::Ceiling);
    test("-123", options, "-1.2e2");

    options.set_rounding_mode(RoundingMode::Nearest);
    test("-135", options, "-1.4e2");
    options.set_rounding_mode(RoundingMode::Down);
    test("-135", options, "-1.3e2");
    options.set_rounding_mode(RoundingMode::Floor);
    test("-135", options, "-1.4e2");
    options.set_rounding_mode(RoundingMode::Up);
    test("-135", options, "-1.4e2");
    options.set_rounding_mode(RoundingMode::Ceiling);
    test("-135", options, "-1.3e2");

    options.set_rounding_mode(RoundingMode::Exact);
    test("-140", options, "-1.4e2");

    options.set_rounding_mode(RoundingMode::Nearest);
    test("-999", options, "-1e3");
    options.set_rounding_mode(RoundingMode::Down);
    test("-999", options, "-9.9e2");
    options.set_rounding_mode(RoundingMode::Floor);
    test("-999", options, "-1e3");
    options.set_rounding_mode(RoundingMode::Up);
    test("-999", options, "-1e3");
    options.set_rounding_mode(RoundingMode::Ceiling);
    test("-999", options, "-9.9e2");
}

#[should_panic]
#[test]
pub fn to_sci_with_options_fail() {
    let mut options = ToSciOptions::default();
    options.set_rounding_mode(RoundingMode::Exact);
    options.set_precision(2);
    Integer::from(123).to_sci_with_options(options).to_string();
}

#[test]
fn to_sci_properties() {
    let mut powers_of_10 = HashMap::new();
    let ten = Integer::from(10);
    let u_ten = Natural::from(10u8);
    let default_p = 16;
    integer_gen().test_properties(|x| {
        assert!(x.fmt_sci_valid(ToSciOptions::default()));
        let s = x.to_sci().to_string();
        assert_eq!(
            x.to_sci_with_options(ToSciOptions::default()).to_string(),
            s
        );
        assert!(string_is_subset(&s, DECIMAL_SCI_STRING_CHARS));
        assert!(!s.starts_with('+'));
        assert!(!s.starts_with('.'));
        assert!(!s.ends_with('+'));
        assert!(!s.ends_with('-'));
        assert!(!s.ends_with('.'));
        assert!(!s.contains('E'));
        assert!(!s.contains("++"));
        assert!(!s.contains("+-"));
        assert!(!s.contains("-+"));
        assert!(!s.contains("--"));
        assert!(!s.contains("-+"));
        assert!(!s.contains("+."));
        assert!(!s.contains("-."));
        assert!(!s.contains("e-"));
        let x_from = Integer::from_sci_string(&s).unwrap();
        if x == 0u32 {
            assert_eq!(x_from, 0u32);
        } else {
            let log = x.unsigned_abs_ref().floor_log_base(&u_ten);
            if log < default_p {
                assert_eq!(x_from, x);
            } else {
                let pow = powers_of_10
                    .entry(log - default_p + 1)
                    .or_insert_with_key(|&p| (&ten).pow(p));
                assert_eq!(x.round_to_multiple(&*pow, RoundingMode::Nearest), x_from);
            }
        }
    });

    signed_gen::<SignedLimb>().test_properties(|x| {
        assert_eq!(
            x.to_sci().to_string(),
            Integer::from(x).to_sci().to_string()
        );
    });
}

#[test]
fn to_sci_with_options_properties() {
    let mut powers = HashMap::new();
    let mut chars = HashMap::new();
    integer_to_sci_options_pair_gen_var_1().test_properties(|(x, options)| {
        assert!(x.fmt_sci_valid(options));
        let s = x.to_sci_with_options(options).to_string();
        let cs: &mut String = chars.entry(options.get_base()).or_insert_with_key(|&base| {
            let mut cs = "+-.0123456789".to_string();
            if base > 10 {
                let limit = usize::from(base - 10);
                for c in ('a'..='z').take(limit) {
                    cs.push(c);
                }
                for c in ('A'..='Z').take(limit) {
                    cs.push(c);
                }
            }
            if base < 15 {
                cs.push('e');
                cs.push('E');
            }
            cs
        });
        assert!(string_is_subset(&s, cs));
        assert!(!s.starts_with('+'));
        assert!(!s.starts_with('.'));
        assert!(!s.ends_with('+'));
        assert!(!s.ends_with('-'));
        assert!(!s.ends_with('.'));
        assert!(!s.contains("++"));
        assert!(!s.contains("+-"));
        assert!(!s.contains("-+"));
        assert!(!s.contains("--"));
        assert!(!s.contains("-+"));
        assert!(!s.contains("+."));
        assert!(!s.contains("-."));
        assert!(!s.contains("e-"));
        assert!(!s.contains("E-"));
        assert!(s.chars().filter(|&c| c == '.').count() <= 1);
        assert!(s.chars().filter(|&c| c == '-').count() <= 1);
        assert!(s.chars().filter(|&c| c == '+').count() <= 1);
        let mut from_options = FromSciStringOptions::default();
        from_options.set_base(options.get_base());
        let x_from = Integer::from_sci_string_with_options(&s, from_options).unwrap();
        if x == 0u32 {
            assert_eq!(x_from, 0u32);
        } else {
            let base = Integer::from(options.get_base());
            let u_base = Natural::from(options.get_base());
            let scale = match options.get_size_options() {
                SciSizeOptions::Complete | SciSizeOptions::Scale(_) => None,
                SciSizeOptions::Precision(p) => {
                    let log = x.unsigned_abs_ref().floor_log_base(&u_base);
                    if log >= p {
                        Some(log - p + 1)
                    } else {
                        None
                    }
                }
            };
            if let Some(scale) = scale {
                let pow = powers
                    .entry((base.clone(), scale))
                    .or_insert_with(|| base.pow(scale));
                assert_eq!(
                    x.round_to_multiple(&*pow, options.get_rounding_mode()),
                    x_from
                );
            } else {
                assert_eq!(x_from, x);
            }
        }
    });

    signed_to_sci_options_pair_gen_var_1::<SignedLimb>().test_properties(|(x, options)| {
        assert_eq!(
            x.to_sci_with_options(options).to_string(),
            Integer::from(x).to_sci_with_options(options).to_string()
        );
    });
}
