use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::{signed_bit_bucketer, unsigned_bit_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use num::BigInt;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_integer_from_unsigned);
    register_signed_demos!(runner, demo_integer_from_signed);

    register_unsigned_benches!(runner, benchmark_integer_from_unsigned);
    register_signed_benches!(runner, benchmark_integer_from_signed);
    register_bench!(runner, benchmark_integer_from_u32_library_comparison);
    register_bench!(runner, benchmark_integer_from_u64_library_comparison);
    register_bench!(runner, benchmark_integer_from_i32_library_comparison);
    register_bench!(runner, benchmark_integer_from_i64_library_comparison);
}

fn demo_integer_from_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Integer: From<T>,
{
    for u in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("Integer::from({}) = {}", u, Integer::from(u));
    }
}

fn demo_integer_from_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Integer: From<T>,
{
    for i in signed_gen::<T>().get(gm, config).take(limit) {
        println!("Integer::from({}) = {}", i, Integer::from(i));
    }
}

#[allow(unused_must_use)]
fn benchmark_integer_from_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: From<T>,
{
    run_benchmark(
        &format!("Integer::from({})", T::NAME),
        BenchmarkType::Single,
        unsigned_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(Integer::from(u)))],
    );
}

#[allow(unused_must_use)]
fn benchmark_integer_from_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: From<T>,
{
    run_benchmark(
        &format!("Integer::from({})", T::NAME),
        BenchmarkType::Single,
        signed_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(Integer::from(u)))],
    );
}

#[allow(unused_must_use)]
fn benchmark_integer_from_u32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from(u32)",
        BenchmarkType::LibraryComparison,
        unsigned_gen::<u32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("Malachite", &mut |u| no_out!(Integer::from(u))),
            ("num", &mut |u| no_out!(BigInt::from(u))),
            ("rug", &mut |u| no_out!(rug::Integer::from(u))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_integer_from_u64_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from(u64)",
        BenchmarkType::LibraryComparison,
        unsigned_gen::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("Malachite", &mut |u| no_out!(Integer::from(u))),
            ("num", &mut |u| no_out!(BigInt::from(u))),
            ("rug", &mut |u| no_out!(rug::Integer::from(u))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_integer_from_i32_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from(i32)",
        BenchmarkType::LibraryComparison,
        signed_gen::<i32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [
            ("Malachite", &mut |i| no_out!(Integer::from(i))),
            ("num", &mut |i| no_out!(BigInt::from(i))),
            ("rug", &mut |i| no_out!(rug::Integer::from(i))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_integer_from_i64_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from(i64)",
        BenchmarkType::LibraryComparison,
        signed_gen::<i64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [
            ("Malachite", &mut |i| no_out!(Integer::from(i))),
            ("num", &mut |i| no_out!(BigInt::from(i))),
            ("rug", &mut |i| no_out!(rug::Integer::from(i))),
        ],
    );
}
