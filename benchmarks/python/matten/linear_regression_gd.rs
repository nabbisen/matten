use matten::Tensor;

fn solve() -> Tensor {
    let m = 256;
    let mut design = Vec::with_capacity(m * 2);
    for i in 0..m {
        design.push(1.0);
        design.push(i as f64);
    }
    let x = Tensor::new(design, &[m, 2]);
    let theta = Tensor::from_vec(vec![0.0, 0.0]);
    let y: Vec<f64> = (0..m).map(|i| 2.0 * i as f64 + 1.0).collect();
    let pred = x.matmul(&theta);
    let residual: Vec<f64> = pred.as_slice().iter().zip(&y).map(|(p, t)| p - t).collect();
    let grad = x.transpose().matmul(&Tensor::from_vec(residual));
    let updated: Vec<f64> = theta
        .as_slice()
        .iter()
        .zip(grad.as_slice())
        .map(|(w, g)| w - 0.0001 * g)
        .collect();
    Tensor::from_vec(updated)
}
