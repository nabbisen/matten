//! Dynamic `impl Tensor` methods (M8 / RFC-011 + RFC-012).
//! This file is `#[cfg(feature = "dynamic")]` — compiled only under that feature.

use crate::{MattenError, Tensor};

// ---- Dynamic tensor (M8 / RFC-011 + RFC-012) ----------------------------
//
// Dynamic tensors store Element values in a DynamicTensor (Arc + view).
// The public Tensor struct is unchanged; dynamic storage lives alongside
// the Phase 1 Vec<f64> fields (which are empty for pure dynamic tensors).

#[cfg(feature = "dynamic")]
impl Tensor {
    /// Creates a dynamic tensor from a `Vec<Element>` and a shape.
    ///
    /// Panics on invalid shape. Use [`try_from_elements`](Tensor::try_from_elements)
    /// for recoverable construction.
    pub fn from_elements(data: Vec<crate::dynamic::Element>, shape: &[usize]) -> Tensor {
        Self::try_from_elements(data, shape).unwrap_or_else(|e| panic!("{e}"))
    }

    /// Creates a dynamic tensor from a `Vec<Element>` and a shape, returning
    /// an error instead of panicking.
    ///
    /// # Errors
    ///
    /// Returns [`MattenError::Shape`] or [`MattenError::Allocation`] for
    /// invalid shapes.
    pub fn try_from_elements(
        data: Vec<crate::dynamic::Element>,
        shape: &[usize],
    ) -> Result<Tensor, MattenError> {
        let expected = crate::shape::validate_shape(shape, "try_from_elements")?;
        if data.len() != expected {
            return Err(MattenError::Shape {
                operation: "try_from_elements",
                message: format!(
                    "data length {} does not match shape {shape:?}, which requires {expected} elements",
                    data.len()
                ),
            });
        }
        let dyn_tensor = crate::dynamic::storage::DynamicTensor::from_vec(data, shape.to_vec());
        Ok(Tensor {
            data: Vec::new(),
            shape: shape.to_vec(),
            dynamic: Some(Box::new(dyn_tensor)),
        })
    }

    /// Returns the element at the multidimensional coordinate, or `None`.
    ///
    /// Only meaningful on dynamic tensors.
    pub fn get_element(&self, coord: &[usize]) -> Option<crate::dynamic::Element> {
        let flat = crate::shape::coord_to_flat(coord, &self.shape)?;
        self.dynamic.as_ref()?.get_flat(flat).cloned()
    }

    /// Returns `true` if this tensor uses dynamic (`Element`) storage.
    pub fn is_dynamic(&self) -> bool {
        self.dynamic.is_some()
    }

    /// Parses a JSON string into a dynamic `Tensor`, mapping JSON values to
    /// `Element` variants.
    ///
    /// Accepts canonical `{"shape":[…],"data":[…]}` form and nested arrays.
    ///
    /// # Errors
    ///
    /// Returns [`MattenError::Parse`] for any parse or structure error.
    #[cfg(feature = "json")]
    pub fn from_json_dynamic(input: &str) -> Result<Tensor, MattenError> {
        crate::dynamic::parse::json::from_json_dynamic(input)
    }

    /// Parses a CSV string into a dynamic `Tensor`, inferring `Element` per
    /// field (int → `Int`, float → `Float`, empty → `None`, etc.).
    ///
    /// # Errors
    ///
    /// Returns [`MattenError::Parse`] for ragged rows or I/O errors.
    #[cfg(feature = "csv")]
    pub fn from_csv_dynamic(input: &str) -> Result<Tensor, MattenError> {
        crate::dynamic::parse::csv::from_csv_dynamic(input)
    }

    /// Returns all elements in logical row-major order.
    ///
    /// # Panics
    ///
    /// Panics if called on a non-dynamic (Phase 1 numeric) tensor.
    pub fn to_elements(&self) -> Vec<crate::dynamic::Element> {
        self.dynamic
            .as_ref()
            .expect("to_elements called on a non-dynamic tensor")
            .to_vec()
    }

    /// Returns a new tensor replacing all `Element::None` values with `value`.
    ///
    /// # Panics
    ///
    /// Panics if called on a non-dynamic tensor.
    pub fn fill_none(&self, value: impl Into<crate::dynamic::Element>) -> Tensor {
        let fill = value.into();
        let dyn_t = self
            .dynamic
            .as_ref()
            .expect("fill_none called on a non-dynamic tensor");
        let new_data: Vec<crate::dynamic::Element> = dyn_t
            .to_vec()
            .into_iter()
            .map(|e| {
                if e == crate::dynamic::Element::None {
                    fill.clone()
                } else {
                    e
                }
            })
            .collect();
        let new_dyn =
            crate::dynamic::storage::DynamicTensor::from_vec(new_data, dyn_t.shape.clone());
        Tensor {
            data: Vec::new(),
            shape: dyn_t.shape.clone(),
            #[cfg(feature = "dynamic")]
            dynamic: Some(Box::new(new_dyn)),
        }
    }

    /// Converts a dynamic tensor containing only numeric elements to a Phase 1
    /// `f64` tensor.
    ///
    /// # Errors
    ///
    /// Returns [`MattenError::Unsupported`] if any element is not numeric.
    ///
    /// # Panics
    ///
    /// Panics if called on a non-dynamic tensor.
    pub fn try_numeric(&self) -> Result<Tensor, MattenError> {
        let dyn_t = self
            .dynamic
            .as_ref()
            .expect("try_numeric called on a non-dynamic tensor");
        let mut floats = Vec::with_capacity(dyn_t.len);
        for i in 0..dyn_t.len {
            let elem = dyn_t.get_flat(i).unwrap_or(&crate::dynamic::Element::None);
            match elem.try_as_f64() {
                Some(v) => floats.push(v),
                None => {
                    return Err(MattenError::Unsupported {
                        operation: "try_numeric",
                        message: format!(
                            "element at position {i} is {elem:?} and cannot be coerced to f64; \
                         use fill_none or explicit conversion first"
                        ),
                    });
                }
            }
        }
        Tensor::try_new(floats, &dyn_t.shape)
    }
}

#[cfg(feature = "dynamic")]
impl Tensor {
    // ── Additional missing-value and numeric utilities (M9/M10) ──────────

    /// Returns a Phase 1 boolean-like tensor (`1.0` = None, `0.0` = not None)
    /// indicating which elements are `Element::None`.
    ///
    /// # Panics
    ///
    /// Panics if called on a non-dynamic tensor.
    #[cfg(feature = "dynamic")]
    pub fn none_mask(&self) -> Tensor {
        let dyn_t = self
            .dynamic
            .as_ref()
            .expect("none_mask called on a non-dynamic tensor");
        let data: Vec<f64> = dyn_t
            .to_vec()
            .iter()
            .map(|e| if e.is_none() { 1.0 } else { 0.0 })
            .collect();
        Tensor::new(data, &dyn_t.shape)
    }

    /// Alias for [`none_mask`](Tensor::none_mask) — returns a Phase 1 `f64` tensor
    /// where `1.0` marks `Element::None` positions and `0.0` marks all others.
    ///
    /// This name matches the RFC-011 §10 specified API `is_none(&self) -> Tensor`.
    ///
    /// # Panics
    ///
    /// Panics if called on a non-dynamic tensor.
    #[cfg(feature = "dynamic")]
    pub fn is_none_mask(&self) -> Tensor {
        self.none_mask()
    }

    /// Counts the number of `Element::None` values in the tensor.
    ///
    /// # Panics
    ///
    /// Panics if called on a non-dynamic tensor.
    #[cfg(feature = "dynamic")]
    pub fn count_none(&self) -> usize {
        let dyn_t = self
            .dynamic
            .as_ref()
            .expect("count_none called on a non-dynamic tensor");
        dyn_t.to_vec().iter().filter(|e| e.is_none()).count()
    }

    /// Replaces `Element::None` values by carrying the last non-None value
    /// forward along the flat (row-major) order. Leading None values that
    /// have no predecessor are replaced with `fallback`.
    ///
    /// # Panics
    ///
    /// Panics if called on a non-dynamic tensor.
    #[cfg(feature = "dynamic")]
    pub fn forward_fill_none(&self, fallback: impl Into<crate::dynamic::Element>) -> Tensor {
        use crate::dynamic::Element;
        let fallback = fallback.into();
        let dyn_t = self
            .dynamic
            .as_ref()
            .expect("forward_fill_none called on a non-dynamic tensor");
        let mut last: Element = fallback;
        let new_data: Vec<Element> = dyn_t
            .to_vec()
            .into_iter()
            .map(|e| {
                if e == Element::None {
                    last.clone()
                } else {
                    last = e.clone();
                    e
                }
            })
            .collect();
        let shape = dyn_t.shape.clone();
        let new_dyn = crate::dynamic::storage::DynamicTensor::from_vec(new_data, shape.clone());
        Tensor {
            data: Vec::new(),
            shape,
            dynamic: Some(Box::new(new_dyn)),
        }
    }

    /// Sums the numeric elements, skipping `None` values.
    ///
    /// Returns `0.0` if all elements are `None`. Panics on non-numeric,
    /// non-None elements (call `fill_none` or `try_numeric` first).
    ///
    /// # Panics
    ///
    /// Panics if called on a non-dynamic tensor or if any element is
    /// non-numeric and non-None.
    #[cfg(feature = "dynamic")]
    pub fn sum_skip_none(&self) -> f64 {
        let dyn_t = self
            .dynamic
            .as_ref()
            .expect("sum_skip_none called on a non-dynamic tensor");
        let mut acc = 0.0f64;
        for e in dyn_t.to_vec() {
            if e.is_none() {
                continue;
            }
            acc += e.try_as_f64().unwrap_or_else(|| {
                panic!("sum_skip_none: non-numeric element {e:?}; use fill_none first")
            });
        }
        acc
    }
}
