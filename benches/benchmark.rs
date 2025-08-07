use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_replace(c: &mut Criterion) {
    let long_str = "hello world".repeat(100_000);
    c.bench_function("long_str_100x_replace", |b| {
        b.iter(|| {
            let mut s = black_box(long_str.clone());
            for _ in 0..100 {
                s = s.replace("hello", "hi");
            }
            s
        })
    });

    let short_str = "a".repeat(40);
    c.bench_function("short_str_100x_replace", |b| {
        b.iter(|| {
            let mut s = black_box(short_str.clone());
            for _ in 0..100 {
                s = s.replace("a", "b");
            }
            s
        })
    });
}

criterion_group!(benches, bench_replace);
criterion_main!(benches);
