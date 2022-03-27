use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::float::NiceFloat;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_q::Rational;
use malachite_q_test_util::bench::bucketers::pair_1_rational_bit_bucketer;
use malachite_q_test_util::generators::rational_primitive_float_pair_gen;
use std::cmp::Ordering;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_rational_partial_cmp_abs_float);
    register_primitive_float_demos!(runner, demo_float_partial_cmp_abs_rational);

    register_primitive_float_benches!(runner, benchmark_rational_partial_cmp_abs_float);
    register_primitive_float_benches!(runner, benchmark_float_partial_cmp_abs_rational);
}

fn demo_rational_partial_cmp_abs_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    Rational: PartialOrdAbs<T>,
{
    for (n, f) in rational_primitive_float_pair_gen::<T>()
        .get(gm, &config)
        .take(limit)
    {
        match n.partial_cmp_abs(&f) {
            None => println!("{} is not comparable with {}", n, NiceFloat(f)),
            Some(Ordering::Less) => println!("|{}| < |{}|", n, NiceFloat(f)),
            Some(Ordering::Equal) => println!("|{}| = |{}|", n, NiceFloat(f)),
            Some(Ordering::Greater) => println!("|{}| > |{}|", n, NiceFloat(f)),
        }
    }
}

fn demo_float_partial_cmp_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, f) in rational_primitive_float_pair_gen::<T>()
        .get(gm, &config)
        .take(limit)
    {
        match f.partial_cmp_abs(&n) {
            None => println!("{} is not comparable with {}", NiceFloat(f), n),
            Some(Ordering::Less) => println!("|{}| < |{}|", NiceFloat(f), n),
            Some(Ordering::Equal) => println!("|{}| = |{}|", NiceFloat(f), n),
            Some(Ordering::Greater) => println!("|{}| > |{}|", NiceFloat(f), n),
        }
    }
}

fn benchmark_rational_partial_cmp_abs_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Rational.partial_cmp_abs(&{})", T::NAME),
        BenchmarkType::Single,
        rational_primitive_float_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.partial_cmp_abs(&y)))],
    );
}

fn benchmark_float_partial_cmp_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.partial_cmp_abs(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_primitive_float_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.partial_cmp_abs(&x)))],
    );
}
