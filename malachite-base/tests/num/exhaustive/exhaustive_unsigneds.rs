use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::exhaustive::exhaustive_unsigneds;

fn exhaustive_unsigneds_helper<T: PrimitiveUnsigned>()
where
    u8: ExactFrom<T>,
{
    assert_eq!(
        exhaustive_unsigneds::<T>()
            .map(u8::exact_from)
            .take(20)
            .collect::<Vec<u8>>(),
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
    )
}

fn exhaustive_unsigneds_long_helper<T: PrimitiveUnsigned>(last_20: &[T]) {
    let expected_len = usize::power_of_two(T::WIDTH);
    let xs = exhaustive_unsigneds::<T>();
    assert_eq!(xs.clone().count(), expected_len);
    assert_eq!(xs.skip(expected_len - 20).collect::<Vec<T>>(), last_20)
}

#[test]
fn test_exhaustive_unsigneds() {
    exhaustive_unsigneds_helper::<u8>();
    exhaustive_unsigneds_helper::<u16>();
    exhaustive_unsigneds_helper::<u32>();
    exhaustive_unsigneds_helper::<u64>();
    exhaustive_unsigneds_helper::<u128>();
    exhaustive_unsigneds_helper::<usize>();

    exhaustive_unsigneds_long_helper::<u8>(&[
        236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253,
        254, 255,
    ]);
    exhaustive_unsigneds_long_helper::<u16>(&[
        65_516,
        65_517,
        65_518,
        65_519,
        0xfff0,
        0xfff1,
        0xfff2,
        0xfff3,
        0xfff4,
        0xfff5,
        0xfff6,
        0xfff7,
        0xfff8,
        0xfff9,
        0xfffa,
        0xfffb,
        0xfffc,
        0xfffd,
        u16::MAX - 1,
        u16::MAX,
    ]);
}
