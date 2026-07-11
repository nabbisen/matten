use matten::Tensor;

fn main() -> Result<(), matten::TensorError> {
    let a = Tensor::new(vec![1.0, 2.0, 3.0], vec![3])?;
    let b = Tensor::new(vec![4.0, 5.0, 6.0], vec![3])?;
    let _sum = a.sum(0)?;
    let _mean = a.mean(0)?;
    let _dot = a.dot(&b)?;
    Ok(())
}
