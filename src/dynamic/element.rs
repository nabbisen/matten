//! The `Element` type: one value in a dynamic (heterogeneous) tensor (RFC-011).
//!
//! # Variant semantics
//!
//! | Variant | Meaning |
//! |---|---|
//! | `Float(f64)` | IEEE 754 double |
//! | `Int(i64)` | Signed 64-bit integer |
//! | `Text(Arc<str>)` | UTF-8 string (immutable, cheap clone) |
//! | `Bool(bool)` | Boolean |
//! | `None` | Missing / null value |
//!
//! # Coercion policy (RFC-011 §11)
//!
//! Allowed silently:
//! - `Int(i64)` → `f64` when a numeric operation requires it.
//!
//! NOT allowed silently:
//! - `Text` → number
//! - `Bool` → number
//! - `None` → any value
//!
//! Users must call explicit helpers (`fill_none`, future conversion APIs) to
//! clean data before arithmetic.

use std::fmt;
use std::sync::Arc;

/// A single value in a dynamic (heterogeneous) tensor.
///
/// `size_of::<Element>() == 24` on 64-bit targets for all text representation
/// choices; `Arc<str>` was selected for cheap clone under CoW slice semantics.
#[cfg(feature = "dynamic")]
#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    /// IEEE 754 double-precision float.
    Float(f64),
    /// Signed 64-bit integer.
    Int(i64),
    /// UTF-8 text. Stored as `Arc<str>` for cheap clone in CoW tensors.
    Text(Arc<str>),
    /// Boolean.
    Bool(bool),
    /// Missing / null value.
    None,
}

#[cfg(feature = "dynamic")]
impl Element {
    // ── Constructors ──────────────────────────────────────────────────────

    /// Creates a `Text` element from any `&str` or `String`.
    ///
    /// ```
    /// use matten::Element;
    /// let e = Element::text("hello");
    /// assert_eq!(e.as_text(), Some("hello"));
    /// ```
    pub fn text(s: impl AsRef<str>) -> Self {
        Element::Text(Arc::from(s.as_ref()))
    }

    // ── Predicates ────────────────────────────────────────────────────────

    /// Returns `true` for `Element::None`.
    ///
    /// ```
    /// use matten::Element;
    /// assert!(Element::None.is_none());
    /// assert!(!Element::Float(1.0).is_none());
    /// ```
    pub fn is_none(&self) -> bool {
        matches!(self, Element::None)
    }

    /// Returns `true` for `Float` or `Int` (numerically coercible) variants.
    ///
    /// ```
    /// use matten::Element;
    /// assert!(Element::Float(1.0).is_numeric());
    /// assert!(Element::Int(42).is_numeric());
    /// assert!(!Element::Bool(true).is_numeric());
    /// assert!(!Element::Text("3".into()).is_numeric());
    /// assert!(!Element::None.is_numeric());
    /// ```
    pub fn is_numeric(&self) -> bool {
        matches!(self, Element::Float(_) | Element::Int(_))
    }

    // ── Coercion helpers ──────────────────────────────────────────────────

    /// Returns the value as `f64` if this element is numeric, or `None`
    /// otherwise.
    ///
    /// `Int(i64)` is losslessly cast to `f64` unless the value cannot be
    /// exactly represented (values with magnitude > 2⁵³ may lose precision).
    ///
    /// ```
    /// use matten::Element;
    /// assert_eq!(Element::Float(1.5).try_as_f64(), Some(1.5));
    /// assert_eq!(Element::Int(42).try_as_f64(), Some(42.0));
    /// assert_eq!(Element::None.try_as_f64(), None);
    /// assert_eq!(Element::Bool(true).try_as_f64(), None); // no silent bool coercion
    /// ```
    pub fn try_as_f64(&self) -> Option<f64> {
        match self {
            Element::Float(v) => Some(*v),
            Element::Int(v) => Some(*v as f64),
            _ => None,
        }
    }

    /// Returns the text content if this element is `Text`, otherwise `None`.
    ///
    /// ```
    /// use matten::Element;
    /// assert_eq!(Element::text("hello").as_text(), Some("hello"));
    /// assert_eq!(Element::Float(1.0).as_text(), None);
    /// ```
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Element::Text(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the boolean value if this element is `Bool`, otherwise `None`.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Element::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

#[cfg(feature = "dynamic")]
impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Float(v) => write!(f, "{v}"),
            Element::Int(v) => write!(f, "{v}"),
            Element::Text(s) => write!(f, "{s}"),
            Element::Bool(b) => write!(f, "{b}"),
            Element::None => f.write_str("None"),
        }
    }
}

// ── From conversions ──────────────────────────────────────────────────────

#[cfg(feature = "dynamic")]
impl From<f64> for Element {
    fn from(v: f64) -> Self {
        Element::Float(v)
    }
}

#[cfg(feature = "dynamic")]
impl From<i64> for Element {
    fn from(v: i64) -> Self {
        Element::Int(v)
    }
}

#[cfg(feature = "dynamic")]
impl From<i32> for Element {
    fn from(v: i32) -> Self {
        Element::Int(v as i64)
    }
}

#[cfg(feature = "dynamic")]
impl From<bool> for Element {
    fn from(v: bool) -> Self {
        Element::Bool(v)
    }
}

#[cfg(feature = "dynamic")]
impl From<String> for Element {
    fn from(s: String) -> Self {
        Element::Text(Arc::from(s.as_str()))
    }
}

#[cfg(feature = "dynamic")]
impl From<&str> for Element {
    fn from(s: &str) -> Self {
        Element::Text(Arc::from(s))
    }
}

#[cfg(feature = "dynamic")]
impl From<Arc<str>> for Element {
    fn from(s: Arc<str>) -> Self {
        Element::Text(s)
    }
}
