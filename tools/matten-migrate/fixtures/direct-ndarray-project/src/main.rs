use matten::Tensor;
use ndarray::{ArrayD, IxDyn};

fn main() {
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    let _arr = ArrayD::from_shape_vec(IxDyn(t.shape()), t.to_vec()).unwrap();
}
