use rand::Rng;
use rand_chacha::ChaCha20Rng;

use num::arithmetic::traits::Parity;
use num::random::geometric::SimpleRational;
use num::random::{random_unsigneds_less_than, RandomUnsignedsLessThan};
use random::Seed;

/// Uniformly generates random `bool`s.
///
/// This `struct` is created by the `random_bools` function. See its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomBools {
    rng: ChaCha20Rng,
    x: u32,
    bits_left: u8,
}

impl Iterator for RandomBools {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        if self.bits_left == 0 {
            self.x = self.rng.gen();
            self.bits_left = 31;
        } else {
            self.x >>= 1;
            self.bits_left -= 1;
        }
        Some(self.x.odd())
    }
}

/// Uniformly generates random `bool`s.
///
/// $P(\text{false}) = P(\text{true}) = \frac{1}{2}$.
///
/// The output length is infinite.
///
/// # Complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::bools::random::random_bools;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     random_bools(EXAMPLE_SEED).take(10).collect::<Vec<_>>(),
///     &[true, false, false, false, true, true, true, false, true, true]
/// )
/// ```
///
/// # Implementation notes
/// The resulting iterator uses every random bit generated by the PRNG, unlike some implementations
/// which only use one bit out of 32 or 64.
#[inline]
pub fn random_bools(seed: Seed) -> RandomBools {
    RandomBools {
        rng: seed.get_rng(),
        x: 0,
        bits_left: 0,
    }
}

/// Generates random `bool`s, with a fixed probability of generating `true`.
///
/// This `struct` is created by the `weighted_random_bools` function. See its documentation for
/// more.
#[derive(Clone, Debug)]
pub struct WeightedRandomBools {
    numerator: u64,
    xs: RandomUnsignedsLessThan<u64>,
}

impl Iterator for WeightedRandomBools {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        Some(self.xs.next().unwrap() < self.numerator)
    }
}

/// Generates random `bool`s, with a fixed probability of generating `true`.
///
/// The relative probabilities of generating `true` and `false` are specified by a weight $w$ =
/// `w_numerator` / `w_denominator`, with `true` being $w$ times more likely to appear than `false`.
/// So when $w=1$ the probabilities are equal, when $w>1$ `true` is more likely, and when $w<1$
/// `false` is more likely.
///
/// $P(\text{true}) = \frac{w}{w+1}$
///
/// $P(\text{false}) = \frac{1}{w+1}$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::bools::random::weighted_random_bools;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     weighted_random_bools(EXAMPLE_SEED, 3, 1).take(10).collect::<Vec<_>>(),
///     &[true, true, false, true, false, false, true, false, true, true]
/// )
/// ```
pub fn weighted_random_bools(
    seed: Seed,
    w_numerator: u64,
    w_denominator: u64,
) -> WeightedRandomBools {
    let w = SimpleRational::new(w_numerator, w_denominator)
        .inverse()
        .add_u64(1)
        .inverse();
    WeightedRandomBools {
        numerator: w.n,
        xs: random_unsigneds_less_than(seed, w.d),
    }
}
