use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_q::Rational;
use malachite_q_test_util::bench::bucketers::pair_2_pair_1_rational_bit_bucketer;
use malachite_q_test_util::generators::{
    rational_primitive_float_pair_gen, rational_primitive_float_pair_gen_rm,
};
use std::cmp::Ordering;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_rational_partial_cmp_float);
    register_primitive_float_demos!(runner, demo_float_partial_cmp_rational);

    register_primitive_float_benches!(
        runner,
        benchmark_rational_partial_cmp_float_library_comparison
    );
    register_primitive_float_benches!(
        runner,
        benchmark_float_partial_cmp_rational_library_comparison
    );
}

fn demo_rational_partial_cmp_float<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize)
where
    Rational: PartialOrd<T>,
{
    for (n, f) in rational_primitive_float_pair_gen::<T>()
        .get(gm, &config)
        .take(limit)
    {
        match n.partial_cmp(&f) {
            None => println!("{} is not comparable with {}", n, NiceFloat(f)),
            Some(Ordering::Less) => println!("{} < {}", n, NiceFloat(f)),
            Some(Ordering::Equal) => println!("{} = {}", n, NiceFloat(f)),
            Some(Ordering::Greater) => println!("{} > {}", n, NiceFloat(f)),
        }
    }
}

fn demo_float_partial_cmp_rational<T: PartialOrd<Rational> + PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, f) in rational_primitive_float_pair_gen::<T>()
        .get(gm, &config)
        .take(limit)
    {
        match f.partial_cmp(&n) {
            None => println!("{} is not comparable with {}", NiceFloat(f), n),
            Some(Ordering::Less) => println!("{} < {}", NiceFloat(f), n),
            Some(Ordering::Equal) => println!("{} = {}", NiceFloat(f), n),
            Some(Ordering::Greater) => println!("{} > {}", NiceFloat(f), n),
        }
    }
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_rational_partial_cmp_float_library_comparison<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: PartialOrd<T>,
    rug::Rational: PartialOrd<T>,
{
    run_benchmark(
        &format!("Rational.partial_cmp(&{})", T::NAME),
        BenchmarkType::LibraryComparison,
        rational_primitive_float_pair_gen_rm::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_rational_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.partial_cmp(&y))),
            ("rug", &mut |((x, y), _)| no_out!(x.partial_cmp(&y))),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_float_partial_cmp_rational_library_comparison<
    T: PartialOrd<Rational> + PartialOrd<rug::Rational> + PrimitiveFloat,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.partial_cmp(&Rational)", T::NAME),
        BenchmarkType::LibraryComparison,
        rational_primitive_float_pair_gen_rm::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_rational_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(y.partial_cmp(&x))),
            ("rug", &mut |((x, y), _)| no_out!(y.partial_cmp(&x))),
        ],
    );
}
