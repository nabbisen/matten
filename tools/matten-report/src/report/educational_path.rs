use std::error::Error;

use matten::{Element, Tensor};
use matten_mlprep::standardize_columns;

pub(crate) struct EducationalPathReportData {
    pub(crate) reading_steps: [&'static str; 4],
    pub(crate) broadcast: EducationalBroadcastData,
    pub(crate) reshape_transpose: EducationalReshapeTransposeData,
    pub(crate) axis_reductions: EducationalAxisReductionData,
    pub(crate) matmul: EducationalMatmulData,
    pub(crate) dynamic_readiness: EducationalDynamicReadinessData,
    pub(crate) standardization: EducationalStandardizationData,
    pub(crate) non_goals: [&'static str; 4],
}

pub(crate) struct EducationalBroadcastData {
    pub(crate) left_shape: Vec<usize>,
    pub(crate) right_shape: Vec<usize>,
    pub(crate) result_shape: Vec<usize>,
    pub(crate) result_values: Vec<f64>,
}

pub(crate) struct EducationalReshapeTransposeData {
    pub(crate) input_shape: Vec<usize>,
    pub(crate) reshape_shape: Vec<usize>,
    pub(crate) reshape_values: Vec<f64>,
    pub(crate) transpose_shape: Vec<usize>,
    pub(crate) transpose_values: Vec<f64>,
}

pub(crate) struct EducationalAxisReductionData {
    pub(crate) input_shape: Vec<usize>,
    pub(crate) mean_axis_0_shape: Vec<usize>,
    pub(crate) mean_axis_0_values: Vec<f64>,
    pub(crate) mean_axis_1_shape: Vec<usize>,
    pub(crate) mean_axis_1_values: Vec<f64>,
}

pub(crate) struct EducationalMatmulData {
    pub(crate) left_shape: Vec<usize>,
    pub(crate) right_shape: Vec<usize>,
    pub(crate) result_shape: Vec<usize>,
    pub(crate) shared_inner_dimension: usize,
    pub(crate) result_values: Vec<f64>,
}

pub(crate) struct EducationalDynamicReadinessData {
    pub(crate) shape: Vec<usize>,
    pub(crate) none_mask_values: Vec<f64>,
    pub(crate) numeric_mask_values: Vec<f64>,
}

pub(crate) struct EducationalStandardizationData {
    pub(crate) input_shape: Vec<usize>,
    pub(crate) output_shape: Vec<usize>,
    pub(crate) before_mean: Vec<f64>,
    pub(crate) before_std: Vec<f64>,
    pub(crate) after_mean: Vec<f64>,
    pub(crate) after_std: Vec<f64>,
}

pub(crate) fn build() -> Result<EducationalPathReportData, Box<dyn Error>> {
    let broadcast_left = Tensor::new(vec![1.0, 2.0, 3.0], &[3, 1]);
    let broadcast_right = Tensor::new(vec![10.0, 20.0, 30.0, 40.0], &[1, 4]);
    let broadcast = &broadcast_left + &broadcast_right;

    let shape_input = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let reshaped = shape_input.reshape(&[3, 2]);
    let transposed = shape_input.transpose();
    let mean_axis_0 = shape_input.mean_axis(0);
    let mean_axis_1 = shape_input.mean_axis(1);

    let matmul_left = Tensor::new((1..=6).map(|value| value as f64).collect(), &[2, 3]);
    let matmul_right = Tensor::new((1..=12).map(|value| value as f64).collect(), &[3, 4]);
    let matmul = matmul_left.matmul(&matmul_right);

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

    let standardization_input = Tensor::new(vec![8.0, 80.0, 10.0, 100.0, 12.0, 120.0], &[3, 2]);
    let standardized =
        standardize_columns(&standardization_input).map_err(Box::<dyn Error>::from)?;
    let before_mean = standardization_input.mean_axis(0);
    let before_std = standardization_input.std_axis(0);
    let after_mean = standardized.mean_axis(0);
    let after_std = standardized.std_axis(0);

    Ok(EducationalPathReportData {
        reading_steps: [
            "ask what shape each input has",
            "ask which axes align, disappear, or remain",
            "read the output shape before reading values",
            "convert dynamic data before numeric computation",
        ],
        broadcast: EducationalBroadcastData {
            left_shape: broadcast_left.shape().to_vec(),
            right_shape: broadcast_right.shape().to_vec(),
            result_shape: broadcast.shape().to_vec(),
            result_values: broadcast.as_slice().to_vec(),
        },
        reshape_transpose: EducationalReshapeTransposeData {
            input_shape: shape_input.shape().to_vec(),
            reshape_shape: reshaped.shape().to_vec(),
            reshape_values: reshaped.as_slice().to_vec(),
            transpose_shape: transposed.shape().to_vec(),
            transpose_values: transposed.as_slice().to_vec(),
        },
        axis_reductions: EducationalAxisReductionData {
            input_shape: shape_input.shape().to_vec(),
            mean_axis_0_shape: mean_axis_0.shape().to_vec(),
            mean_axis_0_values: mean_axis_0.as_slice().to_vec(),
            mean_axis_1_shape: mean_axis_1.shape().to_vec(),
            mean_axis_1_values: mean_axis_1.as_slice().to_vec(),
        },
        matmul: EducationalMatmulData {
            left_shape: matmul_left.shape().to_vec(),
            right_shape: matmul_right.shape().to_vec(),
            result_shape: matmul.shape().to_vec(),
            shared_inner_dimension: matmul_left.shape()[1],
            result_values: matmul.as_slice().to_vec(),
        },
        dynamic_readiness: EducationalDynamicReadinessData {
            shape: dynamic.shape().to_vec(),
            none_mask_values: none_mask.as_slice().to_vec(),
            numeric_mask_values: numeric_mask.as_slice().to_vec(),
        },
        standardization: EducationalStandardizationData {
            input_shape: standardization_input.shape().to_vec(),
            output_shape: standardized.shape().to_vec(),
            before_mean: before_mean.as_slice().to_vec(),
            before_std: before_std.as_slice().to_vec(),
            after_mean: after_mean.as_slice().to_vec(),
            after_std: after_std.as_slice().to_vec(),
        },
        non_goals: [
            "not a public API",
            "not source scanning",
            "not a renderer",
            "not model-quality analysis",
        ],
    })
}
