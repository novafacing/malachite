use malachite_base::num::conversion::traits::FromSciString;
use malachite_base_test_util::bench::bucketers::{pair_1_string_len_bucketer, string_len_bucketer};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    string_from_sci_string_options_pair_gen_var_2, string_from_sci_string_options_pair_gen_var_3,
    string_gen_var_14, string_gen_var_15,
};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::Natural;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_from_sci_string);
    register_demo!(runner, demo_natural_from_sci_string_targeted);
    register_demo!(runner, demo_natural_from_sci_string_with_options);
    register_demo!(runner, demo_natural_from_sci_string_with_options_targeted);

    register_bench!(runner, benchmark_natural_from_sci_string);
    register_bench!(runner, benchmark_natural_from_sci_string_with_options);
}

fn demo_natural_from_sci_string(gm: GenMode, config: GenConfig, limit: usize) {
    for s in string_gen_var_14().get(gm, &config).take(limit) {
        println!(
            "Natural::from_sci_string({}) = {:?}",
            s,
            Natural::from_sci_string(&s)
        );
    }
}

fn demo_natural_from_sci_string_targeted(gm: GenMode, config: GenConfig, limit: usize) {
    for s in string_gen_var_15().get(gm, &config).take(limit) {
        println!(
            "Natural::from_sci_string({}) = {:?}",
            s,
            Natural::from_sci_string(&s)
        );
    }
}

fn demo_natural_from_sci_string_with_options(gm: GenMode, config: GenConfig, limit: usize) {
    for (s, options) in string_from_sci_string_options_pair_gen_var_2()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "Natural::from_sci_string_with_options({}, {:?}) = {:?}",
            s,
            options,
            Natural::from_sci_string_with_options(&s, options)
        );
    }
}

fn demo_natural_from_sci_string_with_options_targeted(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (s, options) in string_from_sci_string_options_pair_gen_var_3()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "Natural::from_sci_string_with_options({}, {:?}) = {:?}",
            s,
            options,
            Natural::from_sci_string_with_options(&s, options)
        );
    }
}

fn benchmark_natural_from_sci_string(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::from_sci_string(&str)",
        BenchmarkType::Single,
        string_gen_var_15().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [("Malachite", &mut |s| no_out!(Natural::from_sci_string(&s)))],
    );
}

fn benchmark_natural_from_sci_string_with_options(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::from_sci_string_with_options(&str, FromSciStringOptions)",
        BenchmarkType::Single,
        string_from_sci_string_options_pair_gen_var_3().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_string_len_bucketer("s"),
        &mut [("Malachite", &mut |(s, options)| {
            no_out!(Natural::from_sci_string_with_options(&s, options))
        })],
    );
}
