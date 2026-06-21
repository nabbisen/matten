//! Numeric conversion policy for dynamic-to-numeric tensor conversion (RFC-017).
//!
//! [`NumericPolicy`] controls how [`Element`](super::Element) values are
//! coerced to `f64` during [`Tensor::try_numeric_with`].
//!
//! # Default (strict) policy
//!
//! | Element | Default behaviour |
//! |---|---|
//! | `Float(f)` | accepted as-is |
//! | `Int(i)` | converted to `f64` (precision caveat for large values) |
//! | `Bool(_)` | rejected |
//! | `Text(_)` | rejected |
//! | `None` | rejected |
//!
//! # Examples
//!
//! ```rust
//! # #[cfg(feature = "dynamic")] {
//! use matten::{Element, NumericPolicy, Tensor};
//!
//! let t = Tensor::from_elements(
//!     vec![Element::Float(1.0), Element::None, Element::Float(3.0)],
//!     &[3],
//! );
//!
//! // Default strict policy — None causes error
//! assert!(t.try_numeric().is_err());
//!
//! // Permissive: treat None as 0.0
//! let x = t.try_numeric_with(NumericPolicy::default().none_as(0.0)).unwrap();
//! assert_eq!(x.as_slice(), &[1.0, 0.0, 3.0]);
//! # }
//! ```

/// Controls how [`Element`](super::Element) variants are coerced to `f64`
/// during [`Tensor::try_numeric_with`].
#[derive(Debug, Clone)]
pub struct NumericPolicy {
    /// How to handle `Bool(b)`. `None` = reject (default).
    pub(crate) bool_as: Option<fn(bool) -> f64>,
    /// How to handle `Text(_)`. `None` = reject (default).
    pub(crate) text_parse: bool,
    /// How to handle `None`. `None` = reject (default).
    pub(crate) none_value: Option<f64>,
}

impl Default for NumericPolicy {
    /// Returns the strict default policy: only `Float` and `Int` are
    /// accepted; `Bool`, `Text`, and `None` are rejected.
    fn default() -> Self {
        Self {
            bool_as: None,
            text_parse: false,
            none_value: None,
        }
    }
}

impl NumericPolicy {
    /// Strict policy (same as default): only `Float` and `Int` accepted.
    pub fn strict() -> Self {
        Self::default()
    }

    /// Permissive policy: `Bool` as 0.0/1.0, `Text` parsed as f64, `None` as 0.0.
    pub fn permissive() -> Self {
        Self {
            bool_as: Some(|b| if b { 1.0 } else { 0.0 }),
            text_parse: true,
            none_value: Some(0.0),
        }
    }

    /// Accept `Bool` values: `true → 1.0`, `false → 0.0`.
    pub fn allow_bool(mut self) -> Self {
        self.bool_as = Some(|b| if b { 1.0 } else { 0.0 });
        self
    }

    /// Try to parse `Text` values as f64. Rejects on parse failure.
    pub fn allow_text_parse(mut self) -> Self {
        self.text_parse = true;
        self
    }

    /// Treat `None` as a fixed f64 value instead of rejecting.
    pub fn none_as(mut self, value: f64) -> Self {
        self.none_value = Some(value);
        self
    }

    /// Treat `None` as `f64::NAN`.
    pub fn none_as_nan(self) -> Self {
        self.none_as(f64::NAN)
    }

    /// Apply this policy to a single [`Element`], returning the coerced f64
    /// or an error string describing the rejection.
    pub(crate) fn coerce(&self, elem: &super::Element, position: usize) -> Result<f64, String> {
        use super::Element;
        match elem {
            Element::Float(v) => Ok(*v),
            Element::Int(i) => Ok(*i as f64),
            Element::Bool(b) => match self.bool_as {
                Some(f) => Ok(f(*b)),
                None => Err(format!(
                    "element at position {position} is Bool({b}) and cannot be coerced \
                     to f64 under the current NumericPolicy; use allow_bool() or \
                     fill_none / explicit conversion first"
                )),
            },
            Element::Text(t) => {
                if self.text_parse {
                    t.parse::<f64>().map_err(|_| {
                        format!(
                            "element at position {position} is Text({t:?}) and could \
                             not be parsed as f64 under allow_text_parse()"
                        )
                    })
                } else {
                    Err(format!(
                        "element at position {position} is Text({t:?}) and cannot be \
                         coerced to f64 under the current NumericPolicy; use \
                         allow_text_parse() or fill_none first"
                    ))
                }
            }
            Element::None => match self.none_value {
                Some(v) => Ok(v),
                None => Err(format!(
                    "element at position {position} is None and cannot be coerced to \
                     f64 under the current NumericPolicy; use none_as(value) or \
                     fill_none first"
                )),
            },
        }
    }
}
