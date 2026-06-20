use crate::{DataFormat, MattenError};

#[test]
fn error_display_and_matching() {
    let e = MattenError::Parse {
        format: DataFormat::Csv,
        message: "row 3, column 2".into(),
    };
    assert!(matches!(
        e,
        MattenError::Parse {
            format: DataFormat::Csv,
            ..
        }
    ));
    assert_eq!(e.to_string(), "matten csv parse error: row 3, column 2");
}

#[test]
fn data_format_is_copy_eq_display() {
    assert_eq!(DataFormat::Json, DataFormat::Json);
    assert_ne!(DataFormat::Json, DataFormat::Csv);
    assert_eq!(DataFormat::Json.to_string(), "json");
}
