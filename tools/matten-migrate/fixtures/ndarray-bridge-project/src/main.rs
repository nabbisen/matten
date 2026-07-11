use matten::Tensor;
use matten_ndarray::{from_arrayd, to_arrayd};

fn main() {
    let x = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let array = to_arrayd(&x).unwrap();
    let _roundtrip = from_arrayd(array).unwrap();
}
