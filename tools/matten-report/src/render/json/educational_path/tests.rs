use super::render;

fn educational_path_data() -> crate::report::educational_path::EducationalPathReportData {
    crate::report::educational_path::build().expect("educational-path data should build")
}

#[test]
fn fixed_demo_json_is_deterministic() {
    let first_data = educational_path_data();
    let first = render(&first_data).expect("educational-path JSON should render");
    let second_data = educational_path_data();
    let second = render(&second_data).expect("educational-path JSON should render again");

    assert_eq!(first, second);
}

#[test]
fn educational_path_json_report_matches_expected_snapshot() {
    let data = educational_path_data();
    let report = render(&data).expect("educational-path JSON should render");

    assert_eq!(
        report,
        r#"{
  "schema_version": 0,
  "schema_status": "private-local",
  "tool": "matten-report",
  "report_kind": "educational-path",
  "input_mode": "demo",
  "data": {
    "reading_steps": [
      "ask what shape each input has",
      "ask which axes align, disappear, or remain",
      "read the output shape before reading values",
      "convert dynamic data before numeric computation"
    ],
    "broadcasting": {
      "left_shape": [
        3,
        1
      ],
      "right_shape": [
        1,
        4
      ],
      "result": {
        "shape": [
          3,
          4
        ],
        "values": [
          11.0,
          21.0,
          31.0,
          41.0,
          12.0,
          22.0,
          32.0,
          42.0,
          13.0,
          23.0,
          33.0,
          43.0
        ],
        "truncated": false,
        "shown_values": 12,
        "total_values": 12,
        "limit": 12
      },
      "axis_1_meaning": "left repeats across 4 columns",
      "axis_0_meaning": "right repeats across 3 rows"
    },
    "reshape_and_transpose": {
      "input_shape": [
        2,
        3
      ],
      "reshape": {
        "shape": [
          3,
          2
        ],
        "values": [
          1.0,
          2.0,
          3.0,
          4.0,
          5.0,
          6.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      },
      "transpose": {
        "shape": [
          3,
          2
        ],
        "values": [
          1.0,
          4.0,
          2.0,
          5.0,
          3.0,
          6.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      },
      "meaning": "reshape changes grouping; transpose changes coordinate meaning"
    },
    "axis_reductions": {
      "input_shape": [
        2,
        3
      ],
      "mean_axis_0": {
        "shape": [
          3
        ],
        "values": [
          2.5,
          3.5,
          4.5
        ],
        "truncated": false,
        "shown_values": 3,
        "total_values": 3,
        "limit": 12
      },
      "mean_axis_1": {
        "shape": [
          2
        ],
        "values": [
          2.0,
          5.0
        ],
        "truncated": false,
        "shown_values": 2,
        "total_values": 2,
        "limit": 12
      }
    },
    "matmul": {
      "left_shape": [
        2,
        3
      ],
      "right_shape": [
        3,
        4
      ],
      "shared_inner_dimension": 3,
      "result": {
        "shape": [
          2,
          4
        ],
        "values": [
          38.0,
          44.0,
          50.0,
          56.0,
          83.0,
          98.0,
          113.0,
          128.0
        ],
        "truncated": false,
        "shown_values": 8,
        "total_values": 8,
        "limit": 12
      }
    },
    "dynamic_readiness": {
      "shape": [
        2,
        3
      ],
      "none_mask": {
        "shape": [
          2,
          3
        ],
        "values": [
          0.0,
          0.0,
          1.0,
          0.0,
          0.0,
          0.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      },
      "numeric_mask": {
        "shape": [
          2,
          3
        ],
        "values": [
          1.0,
          0.0,
          0.0,
          1.0,
          0.0,
          1.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      },
      "note": "Text values are not numeric-ready under the strict mask",
      "next_step": "clean values, then call try_numeric()"
    },
    "standardization": {
      "operation": "standardize_columns(input)",
      "input_shape": [
        3,
        2
      ],
      "output_shape": [
        3,
        2
      ],
      "before_mean": [
        10.0,
        100.0
      ],
      "before_population_std": [
        1.632993161855452,
        16.32993161855452
      ],
      "after_mean": [
        0.0,
        0.0
      ],
      "after_population_std": [
        0.9999999999999999,
        0.9999999999999999
      ]
    },
    "non_goals": [
      "not a public API",
      "not source scanning",
      "not a renderer",
      "not model-quality analysis"
    ]
  }
}
"#
    );
}
