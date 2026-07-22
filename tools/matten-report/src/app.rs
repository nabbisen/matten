use std::env;
use std::error::Error;

use matten_data::Table;

use crate::cli::{self, Action};
use crate::output;
use crate::render;
use crate::report;
use crate::request::{Config, Input, OutputFormat, ReportKind};

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    let config = match cli::parse_args(env::args().skip(1))? {
        Action::Run(config) => config,
        Action::Help => {
            println!("{}", cli::usage());
            return Ok(());
        }
    };
    let report = render_report(&config)?;
    output::write(&report, config.output.as_deref())
}

fn render_report(config: &Config) -> Result<String, Box<dyn Error>> {
    match (&config.input, config.kind) {
        (Input::Demo, ReportKind::ShapeFlow) => {
            let data = report::shape_flow::build();
            match config.format {
                OutputFormat::Markdown => render::markdown::shape_flow::render(&data),
                OutputFormat::Html => render::render_shape_flow_html_report(&data),
                OutputFormat::Json => render::render_shape_flow_json_report(&data),
            }
        }
        (Input::Demo, ReportKind::DynamicReadiness) => {
            let data = report::dynamic_readiness::build()?;
            match config.format {
                OutputFormat::Markdown => render::markdown::dynamic_readiness::render(&data),
                OutputFormat::Html => render::render_dynamic_readiness_html_report(&data),
                OutputFormat::Json => render::render_dynamic_readiness_json_report(&data),
            }
        }
        (Input::Demo, ReportKind::MlprepStandardization) => {
            let data = report::mlprep_standardization::build()?;
            match config.format {
                OutputFormat::Markdown => render::markdown::mlprep_standardization::render(&data),
                OutputFormat::Html => render::render_mlprep_standardization_html_report(&data),
                OutputFormat::Json => render::render_mlprep_standardization_json_report(&data),
            }
        }
        (Input::Demo, ReportKind::EducationalPath) => {
            let data = report::educational_path::build()?;
            match config.format {
                OutputFormat::Markdown => render::markdown::educational_path::render(&data),
                OutputFormat::Html => render::render_educational_path_html_report(&data),
                OutputFormat::Json => render::render_educational_path_json_report(&data),
            }
        }
        (Input::Demo, ReportKind::DataReadiness) => {
            let data = report::data_readiness::build_demo()?;
            match config.format {
                OutputFormat::Markdown => render::markdown::data_readiness::render(&data),
                OutputFormat::Html => render::render_data_readiness_html_report(&data),
                OutputFormat::Json => render::render_data_readiness_json_report(&data),
            }
        }
        (Input::CsvPath { path }, ReportKind::DataReadiness) => {
            if config.format == OutputFormat::Json {
                return Err("--format json is not supported for --input yet".into());
            }
            let table = Table::from_csv_path(path).map_err(Box::<dyn Error>::from)?;
            let data = report::data_readiness::build(
                &format!("path: {}", path.display()),
                &table,
                &config.select,
            )?;
            match config.format {
                OutputFormat::Markdown => render::markdown::data_readiness::render(&data),
                OutputFormat::Html => render::render_input_data_readiness_html_report(&data),
                OutputFormat::Json => Err("--format json is not supported for --input yet".into()),
            }
        }
        (Input::CsvPath { .. }, kind) => Err(format!(
            "unsupported report kind {:?}; expected {:?}",
            kind.as_str(),
            ReportKind::DataReadiness.as_str()
        )
        .into()),
    }
}

#[cfg(test)]
mod tests;
