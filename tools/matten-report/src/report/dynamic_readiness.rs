use std::error::Error;

use matten::{Element, NumericPolicy, Tensor};

#[derive(Debug)]
pub(crate) struct DynamicReadinessReportData {
    pub(crate) shape: Vec<usize>,
    pub(crate) values: Vec<DynamicValueData>,
    pub(crate) schema_summary: Vec<DynamicSchemaSummaryRow>,
    pub(crate) none_mask_values: Vec<f64>,
    pub(crate) numeric_mask_values: Vec<f64>,
    pub(crate) strict_numeric_ready: bool,
    pub(crate) strict_conversion_result: &'static str,
    pub(crate) explicit_policy: &'static str,
    pub(crate) converted_shape: Vec<usize>,
    pub(crate) converted_values: Vec<f64>,
}

#[derive(Debug)]
pub(crate) struct DynamicValueData {
    pub(crate) row: usize,
    pub(crate) column: usize,
    pub(crate) element: String,
}

#[derive(Debug)]
pub(crate) struct DynamicSchemaSummaryRow {
    pub(crate) label: &'static str,
    pub(crate) count: usize,
}

pub(crate) fn build() -> Result<DynamicReadinessReportData, Box<dyn Error>> {
    let dynamic = Tensor::from_elements(
        vec![
            Element::Float(1.0),
            Element::text("2.5"),
            Element::None,
            Element::Int(4),
            Element::text("6.0"),
            Element::Float(8.0),
        ],
        &[2, 3],
    );
    let none_mask = dynamic.none_mask();
    let numeric_mask = dynamic.numeric_mask();
    let converted = dynamic
        .try_numeric_with(NumericPolicy::default().none_as(0.0).allow_text_parse())
        .map_err(Box::<dyn Error>::from)?;

    if dynamic.try_numeric().is_ok() {
        return Err("strict dynamic conversion unexpectedly succeeded".into());
    }

    let shape = dynamic.shape().to_vec();
    let columns = shape.get(1).copied().unwrap_or(1);
    let values = dynamic
        .to_elements()
        .iter()
        .enumerate()
        .map(|(index, element)| DynamicValueData {
            row: index / columns,
            column: index % columns,
            element: format_dynamic_element(element),
        })
        .collect();

    Ok(DynamicReadinessReportData {
        shape,
        values,
        schema_summary: dynamic_schema_summary_rows(&dynamic),
        none_mask_values: none_mask.as_slice().to_vec(),
        numeric_mask_values: numeric_mask.as_slice().to_vec(),
        strict_numeric_ready: dynamic.is_numeric_convertible(),
        strict_conversion_result: "error: strict conversion rejects Text and None values",
        explicit_policy: "none_as(0.0) + allow_text_parse()",
        converted_shape: converted.shape().to_vec(),
        converted_values: converted.as_slice().to_vec(),
    })
}

fn dynamic_schema_summary_rows(tensor: &Tensor) -> Vec<DynamicSchemaSummaryRow> {
    let mut floats = 0;
    let mut ints = 0;
    let mut texts = 0;
    let mut bools = 0;
    let mut none = 0;

    for element in tensor.to_elements() {
        match element {
            Element::Float(_) => floats += 1,
            Element::Int(_) => ints += 1,
            Element::Text(_) => texts += 1,
            Element::Bool(_) => bools += 1,
            Element::None => none += 1,
        }
    }

    let mut rows = vec![
        DynamicSchemaSummaryRow {
            label: "Float",
            count: floats,
        },
        DynamicSchemaSummaryRow {
            label: "Int",
            count: ints,
        },
        DynamicSchemaSummaryRow {
            label: "Text",
            count: texts,
        },
    ];
    if bools > 0 {
        rows.push(DynamicSchemaSummaryRow {
            label: "Bool",
            count: bools,
        });
    }
    rows.push(DynamicSchemaSummaryRow {
        label: "None",
        count: none,
    });
    rows
}

fn format_dynamic_element(element: &Element) -> String {
    match element {
        Element::Float(value) => format!("Float({value:?})"),
        Element::Int(value) => format!("Int({value})"),
        Element::Text(value) => format!("Text({value:?})"),
        Element::Bool(value) => format!("Bool({value})"),
        Element::None => "None".to_string(),
    }
}
