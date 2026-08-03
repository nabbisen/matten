# matten educational-path report

## Input
demo: educational-path
note: fixed educational demo report, not automatic expression tracing

## How to read shapes first
1. ask what shape each input has
2. ask which axes align, disappear, or remain
3. read the output shape before reading values
4. convert dynamic data before numeric computation

## Broadcasting
shape flow: [3, 1] + [1, 4] -> [3, 4]
axis 1: left repeats across 4 columns
axis 0: right repeats across 3 rows
result values:
```text
11.0 21.0 31.0 41.0
12.0 22.0 32.0 42.0
13.0 23.0 33.0 43.0
```

## Reshape and transpose
reshape: [2, 3] -> [3, 2]
reshape values:
```text
1.0 2.0
3.0 4.0
5.0 6.0
```
transpose: [2, 3] -> [3, 2]
transpose values:
```text
1.0 4.0
2.0 5.0
3.0 6.0
```
meaning: reshape changes grouping; transpose changes coordinate meaning

## Axis reductions
mean_axis(0): [2, 3] -> [3]
mean_axis(0) keeps columns: [2.5, 3.5, 4.5]
mean_axis(1): [2, 3] -> [2]
mean_axis(1) keeps rows: [2.0, 5.0]

## Matrix multiplication
shape flow: [2, 3] @ [3, 4] -> [2, 4]
shared inner dimension: 3
result values:
```text
38.0 44.0  50.0  56.0
83.0 98.0 113.0 128.0
```

## Dynamic readiness
dynamic shape: [2, 3]
none mask:
```text
0.0 0.0 1.0
0.0 0.0 0.0
```
numeric mask: strict policy readiness
```text
1.0 0.0 0.0
1.0 0.0 1.0
```
Text values are not numeric-ready under the strict mask
next step: clean values, then call try_numeric()

## Standardization
operation: standardize_columns(input)
shape flow: [3, 2] -> [3, 2]
before column mean: [10.000, 100.000]
before column population std: [1.633, 16.330]
after column mean: [0.000, 0.000]
after column population std: [1.000, 1.000]

## What this report is not
- not a public API
- not source scanning
- not a renderer
- not model-quality analysis
