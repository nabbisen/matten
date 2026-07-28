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
                OutputFormat::Html => render::html::shape_flow::render(&data),
                OutputFormat::Json => render::json::shape_flow::render(&data),
            }
        }
        (Input::Demo, ReportKind::DynamicReadiness) => {
            let data = report::dynamic_readiness::build()?;
            match config.format {
                OutputFormat::Markdown => render::markdown::dynamic_readiness::render(&data),
                OutputFormat::Html => render::html::dynamic_readiness::render(&data),
                OutputFormat::Json => render::json::dynamic_readiness::render(&data),
            }
        }
        (Input::Demo, ReportKind::MlprepStandardization) => {
            let data = report::mlprep_standardization::build()?;
            match config.format {
                OutputFormat::Markdown => render::markdown::mlprep_standardization::render(&data),
                OutputFormat::Html => render::html::mlprep_standardization::render(&data),
                OutputFormat::Json => render::json::mlprep_standardization::render(&data),
            }
        }
        (Input::Demo, ReportKind::EducationalPath) => {
            let data = report::educational_path::build()?;
            match config.format {
                OutputFormat::Markdown => render::markdown::educational_path::render(&data),
                OutputFormat::Html => render::html::educational_path::render(&data),
                OutputFormat::Json => render::json::educational_path::render(&data),
            }
        }
        (Input::Demo, ReportKind::DataReadiness) => {
            let data = report::data_readiness::build_demo()?;
            match config.format {
                OutputFormat::Markdown => render::markdown::data_readiness::render(&data),
                OutputFormat::Html => render::html::data_readiness::render_demo(&data),
                OutputFormat::Json => render::json::data_readiness::render(&data),
            }
        }
        (Input::CsvPath { path }, ReportKind::DataReadiness) => {
            let table = Table::from_csv_path(path).map_err(Box::<dyn Error>::from)?;
            let data = report::data_readiness::build(
                &format!("path: {}", path.display()),
                &table,
                &config.select,
            )?;
            match config.format {
                OutputFormat::Markdown => render::markdown::data_readiness::render(&data),
                OutputFormat::Html => render::html::data_readiness::render_input(&data),
                OutputFormat::Json => render::json::data_readiness::input::render(&data),
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
