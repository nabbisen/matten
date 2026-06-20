//! Serde integration for [`Tensor`](crate::Tensor) (RFC-009, `serde` feature).
//!
//! The canonical on-wire representation is a JSON object with two fields:
//!
//! ```json
//! { "shape": [2, 2], "data": [1.0, 2.0, 3.0, 4.0] }
//! ```
//!
//! This form is unambiguous for any rank, avoids deeply nested JSON for higher-
//! rank tensors, and maps 1:1 to the internal flat row-major storage.
//!
//! Deserialization validates shape consistency with `Tensor::try_new`, so a
//! malformed payload (shape/data length mismatch, zero-sized dim, overflow,
//! rank too large) yields a serde error rather than a panic.

#[cfg(feature = "serde")]
use crate::Tensor;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
/// Private mirror used for serde round-trips.
#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
struct TensorSerde {
    shape: Vec<usize>,
    data: Vec<f64>,
}

#[cfg(feature = "serde")]
impl Serialize for Tensor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "dynamic")]
        if self.is_dynamic() {
            return Err(serde::ser::Error::custom(
                "matten: dynamic tensors cannot be serialized with the default serde                  implementation; call try_numeric() first to convert to a numeric tensor, or use to_elements() to handle Element values manually",
            ));
        }
        TensorSerde {
            shape: self.shape().to_vec(),
            data: self.to_vec(),
        }
        .serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Tensor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let ts = TensorSerde::deserialize(deserializer)?;
        Tensor::try_new(ts.data, &ts.shape).map_err(serde::de::Error::custom)
    }
}
