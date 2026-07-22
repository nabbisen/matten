use std::error::Error;
use std::fmt::Write as _;

use crate::report::shape_flow::ShapeFlowReportData;
use crate::request::KIND_SHAPE_FLOW;

pub(crate) fn render(data: &ShapeFlowReportData) -> Result<String, Box<dyn Error>> {
    let mut report = String::new();
    writeln!(report, "# matten shape-flow report")?;
    writeln!(report)?;

    writeln!(report, "## Input")?;
    writeln!(report, "demo: {KIND_SHAPE_FLOW}")?;
    writeln!(
        report,
        "note: fixed demo report, not automatic expression tracing"
    )?;
    writeln!(report)?;

    writeln!(report, "## Broadcast add")?;
    writeln!(report, "input a: shape {:?}", data.broadcast.input_a_shape)?;
    writeln!(report, "input b: shape {:?}", data.broadcast.input_b_shape)?;
    writeln!(report, "operation: {}", data.broadcast.operation)?;
    writeln!(
        report,
        "shape flow: {:?} + {:?} -> {:?}",
        data.broadcast.input_a_shape, data.broadcast.input_b_shape, data.broadcast.result_shape
    )?;
    writeln!(report, "result values: {:?}", data.broadcast.result_values)?;
    writeln!(report)?;

    writeln!(report, "## Reshape")?;
    writeln!(report, "input: shape {:?}", data.reshape.input_shape)?;
    writeln!(report, "operation: {}", data.reshape.operation)?;
    writeln!(
        report,
        "shape flow: {:?} -> {:?}",
        data.reshape.input_shape, data.reshape.result_shape
    )?;
    writeln!(report, "result values: {:?}", data.reshape.result_values)?;
    writeln!(report)?;

    writeln!(report, "## Axis reductions")?;
    writeln!(report, "input: shape {:?}", data.axis.input_shape)?;
    writeln!(
        report,
        "mean_axis(0): {:?} -> {:?}",
        data.axis.input_shape, data.axis.mean_axis_0_shape
    )?;
    writeln!(
        report,
        "mean_axis(0) values: {:?}",
        data.axis.mean_axis_0_values
    )?;
    writeln!(
        report,
        "mean_axis(1): {:?} -> {:?}",
        data.axis.input_shape, data.axis.mean_axis_1_shape
    )?;
    writeln!(
        report,
        "mean_axis(1) values: {:?}",
        data.axis.mean_axis_1_values
    )?;
    writeln!(report)?;

    writeln!(report, "## Matrix multiplication")?;
    writeln!(report, "left: shape {:?}", data.matmul.left_shape)?;
    writeln!(report, "right: shape {:?}", data.matmul.right_shape)?;
    writeln!(report, "operation: {}", data.matmul.operation)?;
    writeln!(
        report,
        "shape flow: {:?} @ {:?} -> {:?}",
        data.matmul.left_shape, data.matmul.right_shape, data.matmul.result_shape
    )?;
    writeln!(report, "result values: {:?}", data.matmul.result_values)?;

    Ok(report)
}

#[cfg(test)]
mod tests;
