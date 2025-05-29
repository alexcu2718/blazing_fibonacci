use blazing_fibonacci::fast_double;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_fast_double(c: &mut Criterion) {
    c.bench_function("fast_double 1_000_000", |b| {
        b.iter(|| fast_double(black_box(1_000_000)))
    });
}

criterion_group!(benches, bench_fast_double);
criterion_main!(benches);
