//! Criterion bench target for the core micro-workloads.
//!
//! Inputs are built once, outside the timed closure; the measured body calls a
//! single workload and hands the result to `black_box`.

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use matten_benchmarks::common;
use matten_benchmarks::workloads::core;

fn bench_core(c: &mut Criterion) {
    let n = 4096;
    let side = 64; // 64 * 64 == 4096
    let vec_a = common::vector(n);
    let vec_b = common::vector(n);
    let mat = common::matrix(side, side);
    let row = common::vector(side);
    let lhs = common::matrix(side, side);
    let rhs = common::matrix(side, side);

    c.bench_function("core/construction", |b| {
        b.iter(|| black_box(core::construction(black_box(n))))
    });
    c.bench_function("core/reshape_flatten", |b| {
        b.iter(|| black_box(core::reshape_flatten(black_box(&vec_a), side)))
    });
    c.bench_function("core/elementwise_add", |b| {
        b.iter(|| black_box(core::elementwise_add(black_box(&vec_a), black_box(&vec_b))))
    });
    c.bench_function("core/elementwise_mul", |b| {
        b.iter(|| black_box(core::elementwise_mul(black_box(&vec_a), black_box(&vec_b))))
    });
    c.bench_function("core/broadcasting", |b| {
        b.iter(|| black_box(core::broadcasting(black_box(&mat), black_box(&row))))
    });
    c.bench_function("core/sum_mean", |b| {
        b.iter(|| black_box(core::sum_mean(black_box(&vec_a))))
    });
    c.bench_function("core/sum_mean_axis", |b| {
        b.iter(|| black_box(core::sum_mean_axis(black_box(&mat))))
    });
    c.bench_function("core/matmul", |b| {
        b.iter(|| black_box(core::matmul(black_box(&lhs), black_box(&rhs))))
    });
    c.bench_function("core/slice_rows", |b| {
        b.iter(|| black_box(core::slice_rows(black_box(&mat), 8)))
    });

    #[cfg(feature = "dynamic")]
    {
        use matten::Element;
        let elements: Vec<Element> = (0..n).map(|i| Element::Float(i as f64)).collect();
        c.bench_function("core/dynamic_try_numeric", |b| {
            b.iter(|| black_box(core::dynamic_try_numeric(black_box(&elements), &[n])))
        });
    }
}

criterion_group!(core_benches, bench_core);
criterion_main!(core_benches);
