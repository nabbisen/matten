use std::error::Error;

use matten::Tensor;
use matten_mlprep::standardize_columns;

pub(crate) struct MlprepStandardizationReportData {
    pub(crate) input_shape: Vec<usize>,
    pub(crate) input_values: Vec<f64>,
    pub(crate) before_mean: Vec<f64>,
    pub(crate) before_std: Vec<f64>,
    pub(crate) output_shape: Vec<usize>,
    pub(crate) output_values: Vec<f64>,
    pub(crate) after_mean: Vec<f64>,
    pub(crate) after_std: Vec<f64>,
}

pub(crate) fn build() -> Result<MlprepStandardizationReportData, Box<dyn Error>> {
    let input = Tensor::new(vec![8.0, 80.0, 10.0, 100.0, 12.0, 120.0], &[3, 2]);
    let standardized = standardize_columns(&input).map_err(Box::<dyn Error>::from)?;
    let before_mean = input.mean_axis(0);
    let before_std = input.std_axis(0);
    let after_mean = standardized.mean_axis(0);
    let after_std = standardized.std_axis(0);

    Ok(MlprepStandardizationReportData {
        input_shape: input.shape().to_vec(),
        input_values: input.as_slice().to_vec(),
        before_mean: before_mean.as_slice().to_vec(),
        before_std: before_std.as_slice().to_vec(),
        output_shape: standardized.shape().to_vec(),
        output_values: standardized.as_slice().to_vec(),
        after_mean: after_mean.as_slice().to_vec(),
        after_std: after_std.as_slice().to_vec(),
    })
}
