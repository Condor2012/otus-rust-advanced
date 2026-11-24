use criterion::{Criterion, criterion_group, criterion_main};
use logging_debugging_profiling::sum_to;
use std::hint::black_box;

fn bench_sum(c: &mut Criterion) {
    c.bench_function("sum_to 1000", |bencher| {
        bencher.iter(|| sum_to(black_box(1_000)));
    });
}

criterion_group!(benches, bench_sum);
criterion_main!(benches);
