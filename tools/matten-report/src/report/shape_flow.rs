use matten::Tensor;

pub(crate) struct ShapeFlowReportData {
    pub(crate) broadcast: ShapeFlowBroadcastData,
    pub(crate) reshape: ShapeFlowReshapeData,
    pub(crate) axis: ShapeFlowAxisData,
    pub(crate) matmul: ShapeFlowMatmulData,
}

pub(crate) struct ShapeFlowBroadcastData {
    pub(crate) input_a_shape: Vec<usize>,
    pub(crate) input_b_shape: Vec<usize>,
    pub(crate) result_shape: Vec<usize>,
    pub(crate) operation: &'static str,
    pub(crate) result_values: Vec<f64>,
}

pub(crate) struct ShapeFlowReshapeData {
    pub(crate) input_shape: Vec<usize>,
    pub(crate) result_shape: Vec<usize>,
    pub(crate) operation: &'static str,
    pub(crate) result_values: Vec<f64>,
}

pub(crate) struct ShapeFlowAxisData {
    pub(crate) input_shape: Vec<usize>,
    pub(crate) mean_axis_0_shape: Vec<usize>,
    pub(crate) mean_axis_0_values: Vec<f64>,
    pub(crate) mean_axis_1_shape: Vec<usize>,
    pub(crate) mean_axis_1_values: Vec<f64>,
}

pub(crate) struct ShapeFlowMatmulData {
    pub(crate) left_shape: Vec<usize>,
    pub(crate) right_shape: Vec<usize>,
    pub(crate) result_shape: Vec<usize>,
    pub(crate) operation: &'static str,
    pub(crate) result_values: Vec<f64>,
}

pub(crate) fn build() -> ShapeFlowReportData {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let b = Tensor::new(vec![10.0, 20.0, 30.0], &[3]);
    let broadcast = &a + &b;
    let reshaped = a.reshape(&[3, 2]);
    let mean_axis_0 = a.mean_axis(0);
    let mean_axis_1 = a.mean_axis(1);
    let left = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let right = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[3, 2]);
    let product = left.matmul(&right);

    ShapeFlowReportData {
        broadcast: ShapeFlowBroadcastData {
            input_a_shape: a.shape().to_vec(),
            input_b_shape: b.shape().to_vec(),
            result_shape: broadcast.shape().to_vec(),
            operation: "a + b",
            result_values: broadcast.as_slice().to_vec(),
        },
        reshape: ShapeFlowReshapeData {
            input_shape: a.shape().to_vec(),
            result_shape: reshaped.shape().to_vec(),
            operation: "reshape([3, 2])",
            result_values: reshaped.as_slice().to_vec(),
        },
        axis: ShapeFlowAxisData {
            input_shape: a.shape().to_vec(),
            mean_axis_0_shape: mean_axis_0.shape().to_vec(),
            mean_axis_0_values: mean_axis_0.as_slice().to_vec(),
            mean_axis_1_shape: mean_axis_1.shape().to_vec(),
            mean_axis_1_values: mean_axis_1.as_slice().to_vec(),
        },
        matmul: ShapeFlowMatmulData {
            left_shape: left.shape().to_vec(),
            right_shape: right.shape().to_vec(),
            result_shape: product.shape().to_vec(),
            operation: "left.matmul(right)",
            result_values: product.as_slice().to_vec(),
        },
    }
}
