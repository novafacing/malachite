use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz_test_util::bench::bucketers::pair_1_vec_natural_sum_bits_bucketer;
use malachite_nz_test_util::generators::natural_vec_integer_pair_gen_var_1;
use malachite_q::Rational;
use malachite_q_test_util::conversion::continued_fraction::from_continued_fraction::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_from_continued_fraction);
    register_demo!(runner, demo_rational_from_continued_fraction_ref);
    register_bench!(
        runner,
        benchmark_rational_from_continued_fraction_algorithms
    );
    register_bench!(
        runner,
        benchmark_rational_from_continued_fraction_evaluation_strategy
    );
}

fn demo_rational_from_continued_fraction(gm: GenMode, config: GenConfig, limit: usize) {
    for (xs, floor) in natural_vec_integer_pair_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "from_continued_fraction({}, {:?}) = {}",
            floor.clone(),
            xs.clone(),
            Rational::from_continued_fraction(floor, xs)
        );
    }
}

fn demo_rational_from_continued_fraction_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (xs, floor) in natural_vec_integer_pair_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "from_continued_fraction_ref({}, {:?}) = {}",
            floor,
            xs,
            Rational::from_continued_fraction_ref(&floor, &xs)
        );
    }
}

fn benchmark_rational_from_continued_fraction_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_continued_fraction(&Integer, &[Natural])",
        BenchmarkType::Algorithms,
        natural_vec_integer_pair_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_natural_sum_bits_bucketer(),
        &mut [
            ("default", &mut |(xs, floor)| {
                no_out!(Rational::from_continued_fraction(floor, xs))
            }),
            ("alt", &mut |(xs, floor)| {
                no_out!(from_continued_fraction_alt(floor, xs))
            }),
        ],
    );
}

fn benchmark_rational_from_continued_fraction_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_continued_fraction(&Integer, &[Natural])",
        BenchmarkType::EvaluationStrategy,
        natural_vec_integer_pair_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_natural_sum_bits_bucketer(),
        &mut [
            (
                "Rational::from_continued_fraction(Integer, Vec<Natural>)",
                &mut |(xs, floor)| no_out!(Rational::from_continued_fraction(floor, xs)),
            ),
            (
                "Rational::from_continued_fraction_ref(&Integer, &[Natural])",
                &mut |(xs, floor)| no_out!(Rational::from_continued_fraction_ref(&floor, &xs)),
            ),
        ],
    );
}
