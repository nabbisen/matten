# matten mlprep-standardization report

## Input
demo: mlprep-standardization
note: fixed demo report, not automatic model-quality analysis

## Operation
operation: standardize_columns(input)
meaning: each column is centered to mean 0 and population standard deviation 1

## Before
shape: [3, 2]
row-major values:
```text
 8.000  80.000
10.000 100.000
12.000 120.000
```
column mean: [10.000, 100.000]
column population std: [1.633, 16.330]

## After
shape: [3, 2]
row-major values:
```text
-1.225 -1.225
 0.000  0.000
 1.225  1.225
```
column mean: [0.000, 0.000]
column population std: [1.000, 1.000]

## Shape meaning
shape flow: [3, 2] -> [3, 2]
rows: samples unchanged
columns: features unchanged
