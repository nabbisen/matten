use std::error::Error;

use super::model::{
    JsonAxisReductions, JsonBroadcastOperation, JsonMatmulOperation, JsonReshapeOperation,
    JsonShapeFlowPayload, json_tensor_preview, render_json_envelope,
};
use crate::report::shape_flow::ShapeFlowReportData;
use crate::request::KIND_SHAPE_FLOW;

pub(crate) fn render(data: &ShapeFlowReportData) -> Result<String, Box<dyn Error>> {
    render_json_envelope(KIND_SHAPE_FLOW, payload(data)?)
}

fn payload(data: &ShapeFlowReportData) -> Result<JsonShapeFlowPayload, Box<dyn Error>> {
    Ok(JsonShapeFlowPayload {
        broadcast: JsonBroadcastOperation {
            operation: data.broadcast.operation,
            input_a_shape: data.broadcast.input_a_shape.clone(),
            input_b_shape: data.broadcast.input_b_shape.clone(),
            result: json_tensor_preview(
                &data.broadcast.result_shape,
                &data.broadcast.result_values,
            )?,
        },
        reshape: JsonReshapeOperation {
            operation: data.reshape.operation,
            input_shape: data.reshape.input_shape.clone(),
            result: json_tensor_preview(&data.reshape.result_shape, &data.reshape.result_values)?,
        },
        axis_reductions: JsonAxisReductions {
            input_shape: data.axis.input_shape.clone(),
            mean_axis_0: json_tensor_preview(
                &data.axis.mean_axis_0_shape,
                &data.axis.mean_axis_0_values,
            )?,
            mean_axis_1: json_tensor_preview(
                &data.axis.mean_axis_1_shape,
                &data.axis.mean_axis_1_values,
            )?,
        },
        matmul: JsonMatmulOperation {
            operation: data.matmul.operation,
            left_shape: data.matmul.left_shape.clone(),
            right_shape: data.matmul.right_shape.clone(),
            result: json_tensor_preview(&data.matmul.result_shape, &data.matmul.result_values)?,
        },
    })
}

#[cfg(test)]
mod tests;
