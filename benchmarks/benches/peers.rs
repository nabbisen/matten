//! Criterion bench target for the RFC-049 Phase 2 Rust peer comparison.
//!
//! Compiles only under `--features peers`. For each comparable task it builds the
//! same logical data once, converts it to each library's native type, and benches
//! `matten`, `ndarray`, and `nalgebra` side by side. This is a peer comparison, not a
//! ranking: same problem, same sizes, library-natural representations.

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use matten::Tensor;
use matten_benchmarks::workloads::peers::{nalgebra_tasks as na, ndarray_tasks as nd};
use matten_benchmarks::workloads::{core as mcore, scenarios as mscen};
use nalgebra::{DMatrix, DVector};
use ndarray::{Array1, Array2};

fn vec_data(n: usize) -> Vec<f64> {
    (0..n).map(|i| i as f64).collect()
}
fn spread_data(n: usize) -> Vec<f64> {
    (0..n).map(|i| (i % 7) as f64 + 1.0).collect()
}
fn matrix_data(r: usize, c: usize) -> Vec<f64> {
    (0..r * c).map(|i| i as f64).collect()
}
fn row_stochastic_data(n: usize) -> Vec<f64> {
    let mut d = vec![0.0; n * n];
    for r in 0..n {
        let mut s = 0.0;
        for c in 0..n {
            let w = ((r + c) % 5) as f64 + 1.0;
            d[r * n + c] = w;
            s += w;
        }
        for c in 0..n {
            d[r * n + c] /= s;
        }
    }
    d
}

fn bench_peers(c: &mut Criterion) {
    // cosine similarity — vectors of length 512
    {
        let d = spread_data(512);
        let (mt_a, mt_b) = (Tensor::from_vec(d.clone()), Tensor::from_vec(d.clone()));
        let (nd_a, nd_b) = (Array1::from_vec(d.clone()), Array1::from_vec(d.clone()));
        let (na_a, na_b) = (DVector::from_vec(d.clone()), DVector::from_vec(d.clone()));
        let mut g = c.benchmark_group("peers/cosine_similarity");
        g.bench_function("matten", |b| {
            b.iter(|| black_box(mscen::cosine_similarity(black_box(&mt_a), black_box(&mt_b))))
        });
        g.bench_function("ndarray", |b| {
            b.iter(|| black_box(nd::cosine_similarity(black_box(&nd_a), black_box(&nd_b))))
        });
        g.bench_function("nalgebra", |b| {
            b.iter(|| black_box(na::cosine_similarity(black_box(&na_a), black_box(&na_b))))
        });
        g.finish();
    }

    // small matrix multiplication — 64×64
    {
        let d = matrix_data(64, 64);
        let (mt_a, mt_b) = (
            Tensor::new(d.clone(), &[64, 64]),
            Tensor::new(d.clone(), &[64, 64]),
        );
        let nd_a = Array2::from_shape_vec((64, 64), d.clone()).unwrap();
        let nd_b = nd_a.clone();
        let na_a = DMatrix::from_row_slice(64, 64, &d);
        let na_b = na_a.clone();
        let mut g = c.benchmark_group("peers/matmul");
        g.bench_function("matten", |b| {
            b.iter(|| black_box(mcore::matmul(black_box(&mt_a), black_box(&mt_b))))
        });
        g.bench_function("ndarray", |b| {
            b.iter(|| black_box(nd::matmul(black_box(&nd_a), black_box(&nd_b))))
        });
        g.bench_function("nalgebra", |b| {
            b.iter(|| black_box(na::matmul(black_box(&na_a), black_box(&na_b))))
        });
        g.finish();
    }

    // Markov step — n = 64
    {
        let n = 64;
        let (dist, p) = (vec_data(n), row_stochastic_data(n));
        let (mt_d, mt_p) = (
            Tensor::from_vec(dist.clone()),
            Tensor::new(p.clone(), &[n, n]),
        );
        let nd_d = Array1::from_vec(dist.clone());
        let nd_p = Array2::from_shape_vec((n, n), p.clone()).unwrap();
        let (na_d, na_p) = (
            DVector::from_vec(dist.clone()),
            DMatrix::from_row_slice(n, n, &p),
        );
        let mut g = c.benchmark_group("peers/markov_step");
        g.bench_function("matten", |b| {
            b.iter(|| black_box(mscen::markov_step(black_box(&mt_d), black_box(&mt_p))))
        });
        g.bench_function("ndarray", |b| {
            b.iter(|| black_box(nd::markov_step(black_box(&nd_d), black_box(&nd_p))))
        });
        g.bench_function("nalgebra", |b| {
            b.iter(|| black_box(na::markov_step(black_box(&na_d), black_box(&na_p))))
        });
        g.finish();
    }

    // PageRank step — n = 64
    {
        let n = 64;
        let (link, rank) = (row_stochastic_data(n), vec_data(n));
        let (mt_m, mt_r) = (
            Tensor::new(link.clone(), &[n, n]),
            Tensor::from_vec(rank.clone()),
        );
        let nd_m = Array2::from_shape_vec((n, n), link.clone()).unwrap();
        let nd_r = Array1::from_vec(rank.clone());
        let (na_m, na_r) = (
            DMatrix::from_row_slice(n, n, &link),
            DVector::from_vec(rank.clone()),
        );
        let mut g = c.benchmark_group("peers/pagerank_step");
        g.bench_function("matten", |b| {
            b.iter(|| {
                black_box(mscen::pagerank_step(
                    black_box(&mt_m),
                    black_box(&mt_r),
                    0.85,
                ))
            })
        });
        g.bench_function("ndarray", |b| {
            b.iter(|| black_box(nd::pagerank_step(black_box(&nd_m), black_box(&nd_r), 0.85)))
        });
        g.bench_function("nalgebra", |b| {
            b.iter(|| black_box(na::pagerank_step(black_box(&na_m), black_box(&na_r), 0.85)))
        });
        g.finish();
    }

    // linear-regression gradient-descent step — m = 256
    {
        let m = 256;
        let mut xd = Vec::with_capacity(m * 2);
        for i in 0..m {
            xd.push(1.0);
            xd.push(i as f64);
        }
        let yd: Vec<f64> = (0..m).map(|i| 2.0 * i as f64 + 1.0).collect();
        let theta = vec![0.0, 0.0];
        let mt_x = Tensor::new(xd.clone(), &[m, 2]);
        let mt_xt = mt_x.transpose();
        let mt_th = Tensor::from_vec(theta.clone());
        let nd_x = Array2::from_shape_vec((m, 2), xd.clone()).unwrap();
        let nd_xt = nd_x.t().to_owned();
        let nd_th = Array1::from_vec(theta.clone());
        let nd_y = Array1::from_vec(yd.clone());
        let na_x = DMatrix::from_row_slice(m, 2, &xd);
        let na_xt = na_x.transpose();
        let na_th = DVector::from_vec(theta.clone());
        let na_y = DVector::from_vec(yd.clone());
        let mut g = c.benchmark_group("peers/linreg_gd_step");
        g.bench_function("matten", |b| {
            b.iter(|| {
                black_box(mscen::linreg_gd_step(
                    black_box(&mt_x),
                    black_box(&mt_xt),
                    black_box(&mt_th),
                    black_box(yd.as_slice()),
                    0.0001,
                ))
            })
        });
        g.bench_function("ndarray", |b| {
            b.iter(|| {
                black_box(nd::linreg_gd_step(
                    black_box(&nd_x),
                    black_box(&nd_xt),
                    black_box(&nd_th),
                    black_box(&nd_y),
                    0.0001,
                ))
            })
        });
        g.bench_function("nalgebra", |b| {
            b.iter(|| {
                black_box(na::linreg_gd_step(
                    black_box(&na_x),
                    black_box(&na_xt),
                    black_box(&na_th),
                    black_box(&na_y),
                    0.0001,
                ))
            })
        });
        g.finish();
    }

    // 1-D heat-equation step — n = 64
    {
        let n = 64;
        let (op, u) = (row_stochastic_data(n), vec_data(n));
        let (mt_op, mt_u) = (
            Tensor::new(op.clone(), &[n, n]),
            Tensor::from_vec(u.clone()),
        );
        let nd_op = Array2::from_shape_vec((n, n), op.clone()).unwrap();
        let nd_u = Array1::from_vec(u.clone());
        let (na_op, na_u) = (
            DMatrix::from_row_slice(n, n, &op),
            DVector::from_vec(u.clone()),
        );
        let mut g = c.benchmark_group("peers/heat_step");
        g.bench_function("matten", |b| {
            b.iter(|| black_box(mscen::heat_step(black_box(&mt_op), black_box(&mt_u))))
        });
        g.bench_function("ndarray", |b| {
            b.iter(|| black_box(nd::heat_step(black_box(&nd_op), black_box(&nd_u))))
        });
        g.bench_function("nalgebra", |b| {
            b.iter(|| black_box(na::heat_step(black_box(&na_op), black_box(&na_u))))
        });
        g.finish();
    }
}

criterion_group!(peer_benches, bench_peers);
criterion_main!(peer_benches);
