use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;  // 使用标准库的 black_box

fn bench_replace(c: &mut Criterion) {
    let s = "hello world".repeat(100); // 测试长字符串
    
    c.bench_function("replace", |b| {
        b.iter(|| black_box(s.replace("hello", "hi")))
    });
}

criterion_group!(benches, bench_replace);
criterion_main!(benches);