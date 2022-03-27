use bools::random::{random_bools, RandomBools};
use num::conversion::string::options::{FromSciStringOptions, SciSizeOptions, ToSciOptions};
use num::random::geometric::{
    geometric_random_negative_signeds, geometric_random_unsigneds, GeometricRandomNaturalValues,
    GeometricRandomNegativeSigneds,
};
use num::random::{random_unsigned_inclusive_range, RandomUnsignedInclusiveRange};
use random::Seed;
use rounding_modes::random::{random_rounding_modes, RandomRoundingModes};

/// Generates random `SciSizeOptions`s.
pub struct RandomSciSizeOptions {
    bs: RandomBools,
    xs: GeometricRandomNaturalValues<u64>,
}

impl Iterator for RandomSciSizeOptions {
    type Item = SciSizeOptions;

    fn next(&mut self) -> Option<SciSizeOptions> {
        let x = self.xs.next().unwrap();
        Some(if self.bs.next().unwrap() {
            if x == 0 {
                SciSizeOptions::Complete
            } else {
                SciSizeOptions::Precision(x)
            }
        } else {
            SciSizeOptions::Scale(x)
        })
    }
}

/// Generates random `SciSizeOptions`s.
///
/// The scales and precisions are chosen from a geometric distribution whose mean is the
/// ratio `m_size_numerator / m_size_denominator`.
///
/// The output length is infinite.
pub fn random_sci_size_options(
    seed: Seed,
    m_size_numerator: u64,
    m_size_denominator: u64,
) -> RandomSciSizeOptions {
    RandomSciSizeOptions {
        bs: random_bools(seed.fork("bs")),
        xs: geometric_random_unsigneds(seed.fork("xs"), m_size_numerator, m_size_denominator),
    }
}

/// Generates random `ToSciOptions`s.
pub struct RandomToSciOptions {
    us: RandomUnsignedInclusiveRange<u8>,
    rms: RandomRoundingModes,
    sos: RandomSciSizeOptions,
    is: GeometricRandomNegativeSigneds<i64>,
    bs: RandomBools,
}

impl Iterator for RandomToSciOptions {
    type Item = ToSciOptions;

    fn next(&mut self) -> Option<ToSciOptions> {
        Some(ToSciOptions {
            base: self.us.next().unwrap(),
            rounding_mode: self.rms.next().unwrap(),
            size_options: self.sos.next().unwrap(),
            neg_exp_threshold: self.is.next().unwrap(),
            lowercase: self.bs.next().unwrap(),
            e_lowercase: self.bs.next().unwrap(),
            force_exponent_plus_sign: self.bs.next().unwrap(),
            include_trailing_zeros: self.bs.next().unwrap(),
        })
    }
}

/// Generates random `ToSciOptions`s.
///
/// The scales, precisions, and the negative of the negative exponenet threshold are chosen from a
/// geometric distribution whose mean is the ratio `m_small_numerator / m_small_denominator`.
///
/// The output length is infinite.
pub fn random_to_sci_options(
    seed: Seed,
    m_small_numerator: u64,
    m_small_denominator: u64,
) -> RandomToSciOptions {
    RandomToSciOptions {
        us: random_unsigned_inclusive_range(seed.fork("us"), 2, 36),
        rms: random_rounding_modes(seed.fork("rms")),
        sos: random_sci_size_options(seed.fork("sos"), m_small_numerator, m_small_denominator),
        is: geometric_random_negative_signeds(
            seed.fork("is"),
            m_small_numerator,
            m_small_denominator,
        ),
        bs: random_bools(seed.fork("bs")),
    }
}

/// Generates random `FromSciStringOptions`s.
pub struct RandomFromSciStringOptions {
    us: RandomUnsignedInclusiveRange<u8>,
    rms: RandomRoundingModes,
}

impl Iterator for RandomFromSciStringOptions {
    type Item = FromSciStringOptions;

    fn next(&mut self) -> Option<FromSciStringOptions> {
        Some(FromSciStringOptions {
            base: self.us.next().unwrap(),
            rounding_mode: self.rms.next().unwrap(),
        })
    }
}

/// Generates random `FromSciStringOptions`s.
///
/// The output length is infinite.
pub fn random_from_sci_string_options(seed: Seed) -> RandomFromSciStringOptions {
    RandomFromSciStringOptions {
        us: random_unsigned_inclusive_range(seed.fork("us"), 2, 36),
        rms: random_rounding_modes(seed.fork("rms")),
    }
}
