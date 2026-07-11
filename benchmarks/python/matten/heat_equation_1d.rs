use matten::Tensor;

fn row_stochastic(n: usize) -> Tensor {
    let mut data = vec![0.0; n * n];
    for r in 0..n {
        let mut row_sum = 0.0;
        for c in 0..n {
            let w = ((r + c) % 5) as f64 + 1.0;
            data[r * n + c] = w;
            row_sum += w;
        }
        for c in 0..n {
            data[r * n + c] /= row_sum;
        }
    }
    Tensor::new(data, &[n, n])
}

fn solve() -> Tensor {
    let n = 64;
    let operator = row_stochastic(n);
    let u = Tensor::from_vec((0..n).map(|i| i as f64).collect());
    operator.matmul(&u)
}
