//! Criterion bench target for the scenario workloads (one step each).

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use matten::Tensor;
use matten_benchmarks::common;
use matten_benchmarks::workloads::scenarios;

fn bench_scenarios(c: &mut Criterion) {
    // Cosine similarity: two bounded-magnitude vectors.
    let cos_a = common::unit_spread_vector(512);
    let cos_b = common::unit_spread_vector(512);
    c.bench_function("scenario/cosine_similarity", |b| {
        b.iter(|| {
            black_box(scenarios::cosine_similarity(
                black_box(&cos_a),
                black_box(&cos_b),
            ))
        })
    });

    // Markov chain: state vector [n] x transition [n, n].
    let n = 64;
    let dist = common::vector(n);
    let transition = common::row_stochastic(n);
    c.bench_function("scenario/markov_step", |b| {
        b.iter(|| {
            black_box(scenarios::markov_step(
                black_box(&dist),
                black_box(&transition),
            ))
        })
    });

    // PageRank: link matrix [n, n] x rank vector [n].
    let link = common::row_stochastic(n);
    let rank = common::vector(n);
    c.bench_function("scenario/pagerank_step", |b| {
        b.iter(|| {
            black_box(scenarios::pagerank_step(
                black_box(&link),
                black_box(&rank),
                0.85,
            ))
        })
    });

    // Linear regression GD step: design matrix [m, 2], reused transpose.
    let m = 256;
    let mut design = Vec::with_capacity(m * 2);
    for i in 0..m {
        design.push(1.0);
        design.push(i as f64);
    }
    let x = Tensor::new(design, &[m, 2]);
    let xt = x.transpose();
    let theta = Tensor::from_vec(vec![0.0, 0.0]);
    let y: Vec<f64> = (0..m).map(|i| 2.0 * i as f64 + 1.0).collect();
    c.bench_function("scenario/linreg_gd_step", |b| {
        b.iter(|| {
            black_box(scenarios::linreg_gd_step(
                black_box(&x),
                black_box(&xt),
                black_box(&theta),
                black_box(&y),
                0.0001,
            ))
        })
    });

    // Heat equation: precomputed [n, n] operator x state [n].
    let operator = common::row_stochastic(n);
    let u = common::vector(n);
    c.bench_function("scenario/heat_step", |b| {
        b.iter(|| black_box(scenarios::heat_step(black_box(&operator), black_box(&u))))
    });
}

criterion_group!(scenario_benches, bench_scenarios);
criterion_main!(scenario_benches);
