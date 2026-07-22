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
    if matches!(config.input, Input::Demo) && config.kind == ReportKind::ShapeFlow {
        let data = report::shape_flow::build();
        return match config.format {
            OutputFormat::Markdown => render::render_shape_flow_report(&data),
            OutputFormat::Html => render::render_shape_flow_html_report(&data),
            OutputFormat::Json => render::render_shape_flow_json_report(&data),
        };
    }

    if matches!(config.input, Input::Demo) && config.kind == ReportKind::DynamicReadiness {
        let data = report::dynamic_readiness::build()?;
        return match config.format {
            OutputFormat::Markdown => render::render_dynamic_readiness_report(&data),
            OutputFormat::Html => render::render_dynamic_readiness_html_report(&data),
            OutputFormat::Json => render::render_dynamic_readiness_json_report(&data),
        };
    }

    if matches!(config.input, Input::Demo) && config.kind == ReportKind::MlprepStandardization {
        let data = report::mlprep_standardization::build()?;
        return match config.format {
            OutputFormat::Markdown => render::render_mlprep_standardization_report(&data),
            OutputFormat::Html => render::render_mlprep_standardization_html_report(&data),
            OutputFormat::Json => render::render_mlprep_standardization_json_report(&data),
        };
    }

    if matches!(config.input, Input::Demo) && config.kind == ReportKind::EducationalPath {
        let data = report::educational_path::build()?;
        return match config.format {
            OutputFormat::Markdown => render::render_educational_path_report(&data),
            OutputFormat::Html => render::render_educational_path_html_report(&data),
            OutputFormat::Json => render::render_educational_path_json_report(&data),
        };
    }

    if config.format == OutputFormat::Json {
        return match config.input {
            Input::Demo => render::render_fixed_demo_json_report(config.kind.as_str()),
            Input::CsvPath { .. } => Err("--format json is not supported for --input yet".into()),
        };
    }

    if config.format == OutputFormat::Html {
        return match (&config.input, config.kind) {
            (Input::Demo, ReportKind::DataReadiness) => render::render_data_readiness_html_report(),
            (Input::Demo, ReportKind::EducationalPath) => {
                unreachable!("educational-path is dispatched with prebuilt report data")
            }
            (Input::Demo, ReportKind::ShapeFlow) => {
                unreachable!("shape-flow is dispatched with prebuilt report data")
            }
            (Input::Demo, ReportKind::DynamicReadiness) => {
                unreachable!("dynamic-readiness is dispatched with prebuilt report data")
            }
            (Input::Demo, ReportKind::MlprepStandardization) => {
                unreachable!("mlprep-standardization is dispatched with prebuilt report data")
            }
            (Input::CsvPath { path }, ReportKind::DataReadiness) => {
                let table = Table::from_csv_path(path).map_err(Box::<dyn Error>::from)?;
                render::render_input_data_readiness_html_report(
                    &format!("path: {}", path.display()),
                    &table,
                    &config.select,
                )
            }
            (Input::CsvPath { .. }, kind) => Err(format!(
                "unsupported report kind {:?}; expected {:?}",
                kind.as_str(),
                ReportKind::DataReadiness.as_str()
            )
            .into()),
        };
    }

    match (&config.input, config.kind) {
        (Input::Demo, ReportKind::ShapeFlow) => {
            unreachable!("shape-flow is dispatched with prebuilt report data")
        }
        (Input::Demo, ReportKind::DynamicReadiness) => {
            unreachable!("dynamic-readiness is dispatched with prebuilt report data")
        }
        (Input::Demo, ReportKind::MlprepStandardization) => {
            unreachable!("mlprep-standardization is dispatched with prebuilt report data")
        }
        (Input::Demo, ReportKind::EducationalPath) => {
            unreachable!("educational-path is dispatched with prebuilt report data")
        }
        (Input::Demo, ReportKind::DataReadiness) => {
            render::render_data_readiness_markdown_report(&config.select)
        }
        (Input::CsvPath { path }, ReportKind::DataReadiness) => {
            let table = Table::from_csv_path(path).map_err(Box::<dyn Error>::from)?;
            render::render_table_report(
                &format!("path: {}", path.display()),
                &table,
                &config.select,
            )
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
