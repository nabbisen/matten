# matten dynamic-readiness report

## Input
demo: dynamic-readiness
note: fixed demo report, not automatic data profiling

## Dynamic values
shape: [2, 3]
row-major values:
- [0, 0] Float(1.0)
- [0, 1] Text("2.5")
- [0, 2] None
- [1, 0] Int(4)
- [1, 1] Text("6.0")
- [1, 2] Float(8.0)
schema summary:
- Float: 2
- Int: 1
- Text: 2
- None: 1

## Readiness masks
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
strict numeric-ready: false

## Strict conversion
result: error: strict conversion rejects Text and None values

## Explicit policy conversion
policy: none_as(0.0) + allow_text_parse()
converted shape: [2, 3]
converted row-major values:
```text
1.0 2.5 0.0
4.0 6.0 8.0
```
