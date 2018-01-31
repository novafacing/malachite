use common::{integer_to_bigint, GenerationMode};
use inputs::integer::pairs_of_integer_and_unsigned;
use malachite_base::num::SignificantBits;
use malachite_base::num::Assign;
use malachite_nz::integer::Integer;
use num::BigInt;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};

pub fn num_assign_u64(x: &mut BigInt, u: u64) {
    *x = BigInt::from(u);
}

pub fn demo_integer_assign_u64(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_unsigned::<u64>(gm).take(limit) {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

pub fn benchmark_integer_assign_u64(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.assign(u64)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integer_and_unsigned::<u64>(gm),
        function_f: &(|(mut n, u): (Integer, u64)| n.assign(u)),
        function_g: &(|(mut n, u): (BigInt, u64)| num_assign_u64(&mut n, u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (integer_to_bigint(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        title: "Integer.assign(u64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
