use super::render;

fn shape_flow_data() -> crate::report::shape_flow::ShapeFlowReportData {
    crate::report::shape_flow::build()
}

#[test]
fn shape_flow_json_report_matches_expected_snapshot() {
    let report = render(&shape_flow_data()).expect("shape-flow JSON should render");

    assert_eq!(
        report,
        r#"{
  "schema_version": 0,
  "schema_status": "private-local",
  "tool": "matten-report",
  "report_kind": "shape-flow",
  "input_mode": "demo",
  "data": {
    "broadcast": {
      "operation": "a + b",
      "input_a_shape": [
        2,
        3
      ],
      "input_b_shape": [
        3
      ],
      "result": {
        "shape": [
          2,
          3
        ],
        "values": [
          11.0,
          22.0,
          33.0,
          14.0,
          25.0,
          36.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      }
    },
    "reshape": {
      "operation": "reshape([3, 2])",
      "input_shape": [
        2,
        3
      ],
      "result": {
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
      }
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
      "operation": "left.matmul(right)",
      "left_shape": [
        2,
        3
      ],
      "right_shape": [
        3,
        2
      ],
      "result": {
        "shape": [
          2,
          2
        ],
        "values": [
          22.0,
          28.0,
          49.0,
          64.0
        ],
        "truncated": false,
        "shown_values": 4,
        "total_values": 4,
        "limit": 12
      }
    }
  }
}
"#
    );
}
