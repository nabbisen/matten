use std::path::PathBuf;

pub(crate) const KIND_DATA_READINESS: &str = "data-readiness";
pub(crate) const KIND_SHAPE_FLOW: &str = "shape-flow";
pub(crate) const KIND_DYNAMIC_READINESS: &str = "dynamic-readiness";
pub(crate) const KIND_MLPREP_STANDARDIZATION: &str = "mlprep-standardization";
pub(crate) const KIND_EDUCATIONAL_PATH: &str = "educational-path";
pub(crate) const SUPPORTED_DEMOS: &str = "\"data-readiness\", \"shape-flow\", \"dynamic-readiness\", \"mlprep-standardization\", or \"educational-path\"";

#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) input: Input,
    pub(crate) kind: ReportKind,
    pub(crate) select: Vec<String>,
    pub(crate) output: Option<PathBuf>,
    pub(crate) format: OutputFormat,
}

#[derive(Debug)]
pub(crate) enum Input {
    Demo,
    CsvPath { path: PathBuf },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OutputFormat {
    Markdown,
    Html,
    Json,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ReportKind {
    DataReadiness,
    ShapeFlow,
    DynamicReadiness,
    MlprepStandardization,
    EducationalPath,
}

impl ReportKind {
    pub(crate) fn from_label(label: &str) -> Option<Self> {
        match label {
            KIND_DATA_READINESS => Some(Self::DataReadiness),
            KIND_SHAPE_FLOW => Some(Self::ShapeFlow),
            KIND_DYNAMIC_READINESS => Some(Self::DynamicReadiness),
            KIND_MLPREP_STANDARDIZATION => Some(Self::MlprepStandardization),
            KIND_EDUCATIONAL_PATH => Some(Self::EducationalPath),
            _ => None,
        }
    }

    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::DataReadiness => KIND_DATA_READINESS,
            Self::ShapeFlow => KIND_SHAPE_FLOW,
            Self::DynamicReadiness => KIND_DYNAMIC_READINESS,
            Self::MlprepStandardization => KIND_MLPREP_STANDARDIZATION,
            Self::EducationalPath => KIND_EDUCATIONAL_PATH,
        }
    }
}
