use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn bench_replace(c: &mut Criterion) {
    let s = "hello world".repeat(100_000);
    
    c.bench_function("replace", |b| {
        b.iter(|| {
            let mut s = black_box(s.clone());
            for _ in 0..100 {
                s = s.replace("hello", "hi");
            }
            s
        })
    });
}

criterion_group!(benches, bench_replace);
criterion_main!(benches);