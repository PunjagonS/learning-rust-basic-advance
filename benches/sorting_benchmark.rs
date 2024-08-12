use basic_advance::modules::benchmark_testing::{sorting_algo_1, sorting_algo_2};
use criterion::{criterion_group, criterion_main, Criterion};

fn sort_benchmark(c: &mut Criterion) {
    let mut numbers = vec![
        1,1,1,1,2,2,2,23,3,3,33,4,4,4,5,5,687,78
    ];

    c.bench_function("Sorting Algorithm", |b| {
        // b.iter( || sorting_algo_1(&mut numbers))
        b.iter( || sorting_algo_1(&mut numbers))
    });
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);
