use malachite_base_test_util::generators::{signed_gen, unsigned_gen};

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

#[test]
fn test_checked_square() {
    fn test<T: PrimitiveInteger>(x: T, out: Option<T>) {
        assert_eq!(x.checked_square(), out);
    };
    test::<u8>(0, Some(0));
    test::<i16>(1, Some(1));
    test::<u32>(2, Some(4));
    test::<i64>(3, Some(9));
    test::<u128>(10, Some(100));
    test::<isize>(123, Some(15_129));
    test::<u32>(1_000, Some(1_000_000));

    test::<i16>(-1, Some(1));
    test::<i32>(-2, Some(4));
    test::<i64>(-3, Some(9));
    test::<i128>(-10, Some(100));
    test::<isize>(-123, Some(15_129));
    test::<i32>(-1_000, Some(1_000_000));

    test::<u16>(1_000, None);
    test::<i16>(-1_000, None);
}

fn unsigned_checked_square_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        let square = x.checked_square();
        assert_eq!(square, x.checked_pow(2));
        if let Some(square) = square {
            assert_eq!(x.square(), square);
        }
    });
}

fn signed_checked_square_properties_helper<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|x| {
        let square = x.checked_square();
        assert_eq!(square, x.checked_pow(2));
        if let Some(square) = square {
            assert_eq!(x.square(), square);
        }
    });
}

#[test]
fn checked_square_properties() {
    apply_fn_to_unsigneds!(unsigned_checked_square_properties_helper);
    apply_fn_to_signeds!(signed_checked_square_properties_helper);
}
