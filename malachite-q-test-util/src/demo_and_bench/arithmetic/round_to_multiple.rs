use malachite_base::num::arithmetic::traits::{RoundToMultiple, RoundToMultipleAssign};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_q_test_util::bench::bucketers::triple_1_rational_bit_bucketer;
use malachite_q_test_util::generators::rational_rational_rounding_mode_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_round_to_multiple_assign);
    register_demo!(runner, demo_rational_round_to_multiple_assign_ref);
    register_demo!(runner, demo_rational_round_to_multiple);
    register_demo!(runner, demo_rational_round_to_multiple_val_ref);
    register_demo!(runner, demo_rational_round_to_multiple_ref_val);
    register_demo!(runner, demo_rational_round_to_multiple_ref_ref);

    register_bench!(
        runner,
        benchmark_rational_round_to_multiple_assign_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_rational_round_to_multiple_evaluation_strategy
    );
}

fn demo_rational_round_to_multiple_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, y, rm) in rational_rational_rounding_mode_triple_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        x.round_to_multiple_assign(y, rm);
        println!(
            "x := {}; x.round_to_multiple_assign({}, {}); x = {}",
            x_old, y_old, rm, x
        );
    }
}

fn demo_rational_round_to_multiple_assign_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, y, rm) in rational_rational_rounding_mode_triple_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        let x_old = x.clone();
        x.round_to_multiple_assign(&y, rm);
        println!(
            "x := {}; x.round_to_multiple_assign(&{}, {}); x = {}",
            x_old, y, rm, x
        );
    }
}

fn demo_rational_round_to_multiple(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y, rm) in rational_rational_rounding_mode_triple_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).round_to_multiple({}, {}) = {}",
            x_old,
            y_old,
            rm,
            x.round_to_multiple(y, rm)
        );
    }
}

fn demo_rational_round_to_multiple_val_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y, rm) in rational_rational_rounding_mode_triple_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).round_to_multiple(&{}, {}) = {}",
            x_old,
            y,
            rm,
            x.round_to_multiple(&y, rm)
        );
    }
}

fn demo_rational_round_to_multiple_ref_val(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y, rm) in rational_rational_rounding_mode_triple_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).round_to_multiple({}, {}) = {}",
            x,
            y_old,
            rm,
            (&x).round_to_multiple(y, rm)
        );
    }
}

fn demo_rational_round_to_multiple_ref_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y, rm) in rational_rational_rounding_mode_triple_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "(&{}).round_to_multiple(&{}, {}) = {}",
            x,
            y,
            rm,
            (&x).round_to_multiple(&y, rm)
        );
    }
}

fn benchmark_rational_round_to_multiple_assign_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.round_to_multiple_assign(Rational, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_rational_rounding_mode_triple_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_bit_bucketer("n"),
        &mut [
            (
                "Rational.round_to_multiple_assign(Rational, RoundingMode)",
                &mut |(mut x, y, rm)| x.round_to_multiple_assign(y, rm),
            ),
            (
                "Rational.round_to_multiple_assign(&Rational, RoundingMode)",
                &mut |(mut x, y, rm)| x.round_to_multiple_assign(&y, rm),
            ),
        ],
    );
}

fn benchmark_rational_round_to_multiple_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.round_to_multiple(Rational, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_rational_rounding_mode_triple_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_bit_bucketer("n"),
        &mut [
            (
                "Rational.round_to_multiple(Rational, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.round_to_multiple(y, rm)),
            ),
            (
                "Rational.round_to_multiple(&Rational, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.round_to_multiple(&y, rm)),
            ),
            (
                "(&Rational).round_to_multiple(Rational, RoundingMode)",
                &mut |(x, y, rm)| no_out!((&x).round_to_multiple(y, rm)),
            ),
            (
                "(&Rational).round_to_multiple(&Rational, RoundingMode)",
                &mut |(x, y, rm)| no_out!((&x).round_to_multiple(&y, rm)),
            ),
        ],
    );
}
