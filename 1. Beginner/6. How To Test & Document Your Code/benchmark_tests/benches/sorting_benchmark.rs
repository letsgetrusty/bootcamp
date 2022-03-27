use benchmark_tests::sort_arr;
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

fn sort_arr_benchmark(c: &mut Criterion) {
    let mut arr = black_box(
        [6, 2, 4, 1, 9, -2, 5]
    );

    c.bench_function(
        "sorting algorithm", 
        |b| b.iter(|| sort_arr(&mut arr))
    );
}

criterion_group!(benches, sort_arr_benchmark);
criterion_main!(benches);