use horrid_vec::HorridVec;
use std::fmt::Write;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn regular_nice_rust_vec_iter(cap: usize) {
    let mut vec = Vec::with_capacity(cap);
    for i in 0..cap {
        vec.push(black_box(i));
    }

    for val in vec.into_iter() {}
}

fn bench_horrid_vec_iter(cap: usize) {
    let mut vec = HorridVec::with_capacity(cap);
    for i in 0..cap {
        vec.push(black_box(i));
    }

    for val in vec.into_iter() {}
}

fn bench_vecs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec");
    let caps = [100usize];
    for i in &caps {
        group.bench_with_input(BenchmarkId::new("Regular vec", i), i, |b, cap| {
            b.iter(|| regular_nice_rust_vec_iter(*cap))
        });
        group.bench_with_input(BenchmarkId::new("Horrid vec", i), i, |b, cap| {
            b.iter(|| bench_horrid_vec_iter(*cap))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_vecs);
criterion_main!(benches);
