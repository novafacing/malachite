use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::{nrm_pairs_of_naturals, pairs_of_naturals, rm_pairs_of_naturals};
use malachite_base::num::SignificantBits;
use std::cmp::max;

pub fn demo_natural_add_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x += y.clone();
        println!("x := {}; x += {}; x = {}", x_old, y, x);
    }
}

pub fn demo_natural_add_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x += &y;
        println!("x := {}; x += &{}; x = {}", x_old, y, x);
    }
}

pub fn demo_natural_add(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} + {} = {}", x_old, y_old, x + y);
    }
}

pub fn demo_natural_add_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        println!("{} + &{} = {}", x_old, y, x + &y);
    }
}

pub fn demo_natural_add_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} + {} = {}", x, y_old, &x + y);
    }
}

pub fn demo_natural_add_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        println!("&{} + &{} = {}", x, y, &x + &y);
    }
}

pub fn benchmark_natural_add_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural += Natural",
        BenchmarkType::Ordinary,
        rm_pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &[
            ("malachite", &mut (|(_, (mut x, y))| x += y)),
            ("rug", &mut (|((mut x, y), _)| x += y)),
        ],
    );
}

pub fn benchmark_natural_add_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural += Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &[
            ("Natural += Natural", &mut (|(mut x, y)| no_out!(x += y))),
            ("Natural += &Natural", &mut (|(mut x, y)| no_out!(x += &y))),
        ],
    );
}

pub fn benchmark_natural_add(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural + Natural",
        BenchmarkType::Ordinary,
        nrm_pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &[
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x + y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x + y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x + y))),
        ],
    );
}

pub fn benchmark_natural_add_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural + Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &[
            ("Natural + Natural", &mut (|(x, y)| no_out!(x + y))),
            ("Natural + &Natural", &mut (|(x, y)| no_out!(x + &y))),
            ("&Natural + Natural", &mut (|(x, y)| no_out!(&x + y))),
            ("&Natural + &Natural", &mut (|(x, y)| no_out!(&x + &y))),
        ],
    );
}
