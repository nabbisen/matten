use matten::Tensor;

fn main() {
    let x = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let reshaped = x.reshape(&[4]);
    let _mean = reshaped.mean_axis(0);
}
