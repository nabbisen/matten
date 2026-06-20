//! Unary operators (RFC-006).

use crate::Tensor;
use std::ops::Neg;

impl Neg for &Tensor {
    type Output = Tensor;
    /// Negates every element.
    ///
    /// ```
    /// use matten::Tensor;
    /// let t = Tensor::new(vec![1.0, -2.0, 3.0], &[3]);
    /// let r = -&t;
    /// assert_eq!(r.as_slice(), &[-1.0, 2.0, -3.0]);
    /// ```
    fn neg(self) -> Tensor {
        Tensor {
            data: self.data.iter().map(|&v| -v).collect(),
            shape: self.shape.clone(),
            #[cfg(feature = "dynamic")]
            dynamic: None,
        }
    }
}
