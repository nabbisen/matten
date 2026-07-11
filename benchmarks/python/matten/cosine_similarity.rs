use matten::Tensor;

fn l2_norm(v: &Tensor) -> f64 {
    v.as_slice().iter().map(|x| x * x).sum::<f64>().sqrt()
}

fn solve() -> f64 {
    let a = Tensor::from_vec((0..512).map(|i| ((i % 7) as f64) + 1.0).collect());
    let b = Tensor::from_vec((0..512).map(|i| ((i % 7) as f64) + 1.0).collect());
    let dot = a.dot(&b).as_slice()[0];
    dot / (l2_norm(&a) * l2_norm(&b))
}
