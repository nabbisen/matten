use super::*;

mod html;
mod json;

fn educational_path_data() -> crate::report::educational_path::EducationalPathReportData {
    crate::report::educational_path::build().expect("educational-path data should build")
}
