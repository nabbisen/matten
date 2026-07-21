use std::error::Error;
use std::fs;
use std::path::Path;

pub(crate) fn write(report: &str, path: Option<&Path>) -> Result<(), Box<dyn Error>> {
    if let Some(path) = path {
        fs::write(path, report)?;
    } else {
        print!("{report}");
    }
    Ok(())
}
