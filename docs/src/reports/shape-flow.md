# matten shape-flow report

## Input
demo: shape-flow
note: fixed demo report, not automatic expression tracing

## Broadcast add
input a: shape [2, 3]
input b: shape [3]
operation: a + b
shape flow: [2, 3] + [3] -> [2, 3]
result values:
```text
11.0 22.0 33.0
14.0 25.0 36.0
```

## Reshape
input: shape [2, 3]
operation: reshape([3, 2])
shape flow: [2, 3] -> [3, 2]
result values:
```text
1.0 2.0
3.0 4.0
5.0 6.0
```

## Axis reductions
input: shape [2, 3]
mean_axis(0): [2, 3] -> [3]
mean_axis(0) values: [2.5, 3.5, 4.5]
mean_axis(1): [2, 3] -> [2]
mean_axis(1) values: [2.0, 5.0]

## Matrix multiplication
left: shape [2, 3]
right: shape [3, 2]
operation: left.matmul(right)
shape flow: [2, 3] @ [3, 2] -> [2, 2]
result values:
```text
22.0 28.0
49.0 64.0
```
