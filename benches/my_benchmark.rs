use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

// mod algebrars;
use algebrars::{
    function::{
        fast_function::{FastFunction, VariableVal},
        function::Function,
    },
    math_tree::MathTree,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut fx =
        FastFunction::from(&Function::from(MathTree::parse("x^x").unwrap()).unwrap()).unwrap();

    let val = vec![VariableVal::new("x".to_string(), 5.5)];
    c.bench_function("fastfn xpx", 
    |b| b.iter(|| fx.evaluate_float(val.clone())));

    // let mut group = c.benchmark_group("criterion_benchmark");
    // for i in 0..1000 {
    //     group.bench_with_input(BenchmarkId::from_parameter(&i), &i, |b, i| {
    //         let val = vec![VariableVal::new("x".to_string(), *i as f64)];
    //         b.iter(|| fx.evaluate_float(val.clone()))
    //     });
    // }
    // group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
