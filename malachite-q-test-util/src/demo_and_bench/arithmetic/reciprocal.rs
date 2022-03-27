use malachite_base::num::arithmetic::traits::{Reciprocal, ReciprocalAssign};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_q_test_util::bench::bucketers::{
    rational_bit_bucketer, triple_3_rational_bit_bucketer,
};
use malachite_q_test_util::generators::{rational_gen_var_1, rational_gen_var_1_nrm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_reciprocal);
    register_demo!(runner, demo_rational_reciprocal_ref);
    register_demo!(runner, demo_rational_reciprocal_assign);

    register_bench!(runner, benchmark_rational_reciprocal_library_comparison);
    register_bench!(runner, benchmark_rational_reciprocal_evaluation_strategy);
    register_bench!(runner, benchmark_rational_reciprocal_assign);
}

fn demo_rational_reciprocal(gm: GenMode, config: GenConfig, limit: usize) {
    for n in rational_gen_var_1().get(gm, &config).take(limit) {
        println!("reciprocal({}) = {}", n.clone(), n.reciprocal());
    }
}

fn demo_rational_reciprocal_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for n in rational_gen_var_1().get(gm, &config).take(limit) {
        println!("reciprocal(&{}) = {}", n, (&n).reciprocal());
    }
}

fn demo_rational_reciprocal_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for mut n in rational_gen_var_1().get(gm, &config).take(limit) {
        let n_old = n.clone();
        n.reciprocal_assign();
        println!("n := {}; n.reciprocal_assign(); n = {}", n_old, n);
    }
}

#[allow(unused_must_use)]
fn benchmark_rational_reciprocal_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.reciprocal()",
        BenchmarkType::LibraryComparison,
        rational_gen_var_1_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_rational_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, n)| no_out!(n.reciprocal())),
            ("num", &mut |(n, _, _)| no_out!(n.recip())),
            ("rug", &mut |(_, n, _)| no_out!(n.recip())),
        ],
    );
}

fn benchmark_rational_reciprocal_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.reciprocal()",
        BenchmarkType::EvaluationStrategy,
        rational_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            ("Rational.reciprocal()", &mut |n| no_out!(n.reciprocal())),
            ("(&Rational).reciprocal()", &mut |n| {
                no_out!((&n).reciprocal())
            }),
        ],
    );
}

fn benchmark_rational_reciprocal_assign(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.reciprocal_assign()",
        BenchmarkType::Single,
        rational_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut n| n.reciprocal_assign())],
    );
}
