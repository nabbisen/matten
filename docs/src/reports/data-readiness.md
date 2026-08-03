# matten data-readiness report

## Input
demo: data-readiness

## Source columns
- region
- sales
- cost
- note

## Selected columns
- sales
- cost

## Columns left out
- region
- note

## Missing values
| column | missing |
|---|---:|
| sales | 0 |
| cost | 0 |

## Numeric conversion
strict conversion: success

## Tensor preview
shape: [3, 2]
row-major values:
```text
100.0 40.0
150.0 45.0
120.0 55.0
```
