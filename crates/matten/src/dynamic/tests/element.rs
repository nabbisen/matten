//! Element-level tests.

//! Tests for the `dynamic` feature: Element model, storage, CoW, and parsers.

#[cfg(feature = "dynamic")]
mod element_tests {
    use crate::dynamic::Element;

    #[test]
    fn element_size_is_24_bytes() {
        // RFC-011 §12.1: measure and document.
        // Arc<str> chosen; all text representations give 24 bytes on 64-bit.
        assert_eq!(std::mem::size_of::<Element>(), 24);
    }

    #[test]
    fn is_none_and_is_numeric() {
        assert!(Element::None.is_none());
        assert!(!Element::Float(1.0).is_none());
        assert!(Element::Float(1.0).is_numeric());
        assert!(Element::Int(42).is_numeric());
        assert!(!Element::Bool(true).is_numeric());
        assert!(!Element::text("hi").is_numeric());
        assert!(!Element::None.is_numeric());
    }

    #[test]
    fn try_as_f64_coercion_policy() {
        // Float and Int are coercible; everything else is not.
        assert_eq!(Element::Float(1.5).try_as_f64(), Some(1.5));
        assert_eq!(Element::Int(42).try_as_f64(), Some(42.0));
        assert_eq!(Element::None.try_as_f64(), None);
        assert_eq!(Element::Bool(true).try_as_f64(), None); // no silent bool coercion
        assert_eq!(Element::text("3.14").try_as_f64(), None); // no silent text coercion
    }

    #[test]
    fn text_constructor_and_accessor() {
        let e = Element::text("hello");
        assert_eq!(e.as_text(), Some("hello"));
        assert!(Element::Float(1.0).as_text().is_none());
    }

    #[test]
    fn from_conversions() {
        assert_eq!(Element::from(1.5f64), Element::Float(1.5));
        assert_eq!(Element::from(42i64), Element::Int(42));
        assert_eq!(Element::from(7i32), Element::Int(7));
        assert_eq!(Element::from(true), Element::Bool(true));
        assert_eq!(Element::from("text"), Element::text("text"));
    }

    #[test]
    fn display_formatting() {
        assert_eq!(Element::Float(1.5).to_string(), "1.5");
        assert_eq!(Element::Int(42).to_string(), "42");
        assert_eq!(Element::Bool(true).to_string(), "true");
        assert_eq!(Element::text("hi").to_string(), "hi");
        assert_eq!(Element::None.to_string(), "None");
    }
}
