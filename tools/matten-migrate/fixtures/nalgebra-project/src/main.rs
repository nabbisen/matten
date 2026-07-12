use matten::Tensor;
use nalgebra::DMatrix;

fn main() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let _m = DMatrix::from_row_slice(2, 2, t.as_slice());
}
