use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::{signed_bit_bucketer, unsigned_bit_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_base::test_util::runner::Runner;
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_rational_from_unsigned);
    register_signed_demos!(runner, demo_rational_from_signed);

    register_unsigned_benches!(runner, benchmark_rational_from_unsigned);
    register_signed_benches!(runner, benchmark_rational_from_signed);
    register_bench!(runner, benchmark_rational_from_u32_library_comparison);
    register_bench!(runner, benchmark_rational_from_u64_library_comparison);
    register_bench!(runner, benchmark_rational_from_i32_library_comparison);
    register_bench!(runner, benchmark_rational_from_i64_library_comparison);
}

fn demo_rational_from_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Rational: From<T>,
{
    for u in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("Rational::from({}) = {}", u, Rational::from(u));
    }
}

fn demo_rational_from_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Rational: From<T>,
{
    for i in signed_gen::<T>().get(gm, config).take(limit) {
        println!("Rational::from({}) = {}", i, Rational::from(i));
    }
}

#[allow(unused_must_use)]
fn benchmark_rational_from_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: From<T>,
{
    run_benchmark(
        &format!("Rational::from({})", T::NAME),
        BenchmarkType::Single,
        unsigned_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(Rational::from(u)))],
    );
}

#[allow(unused_must_use)]
fn benchmark_rational_from_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: From<T>,
{
    run_benchmark(
        &format!("Rational::from({})", T::NAME),
        BenchmarkType::Single,
        signed_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(Rational::from(u)))],
    );
}

#[allow(unused_must_use)]
fn benchmark_rational_from_u32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from(u32)",
        BenchmarkType::LibraryComparison,
        unsigned_gen::<u32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("Malachite", &mut |u| no_out!(Rational::from(u))),
            ("rug", &mut |u| no_out!(rug::Rational::from(u))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_rational_from_u64_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from(u64)",
        BenchmarkType::LibraryComparison,
        unsigned_gen::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("Malachite", &mut |u| no_out!(Rational::from(u))),
            ("rug", &mut |u| no_out!(rug::Rational::from(u))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_rational_from_i32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from(i32)",
        BenchmarkType::LibraryComparison,
        signed_gen::<i32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [
            ("Malachite", &mut |i| no_out!(Rational::from(i))),
            ("rug", &mut |i| no_out!(rug::Rational::from(i))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_rational_from_i64_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from(i64)",
        BenchmarkType::LibraryComparison,
        signed_gen::<i64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [
            ("Malachite", &mut |i| no_out!(Rational::from(i))),
            ("rug", &mut |i| no_out!(rug::Rational::from(i))),
        ],
    );
}
