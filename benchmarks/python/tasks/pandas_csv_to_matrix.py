from io import StringIO

import pandas as pd


def solve():
    csv = """region,sales,cost
north,100,40
south,150,
east,120,55"""
    frame = pd.read_csv(StringIO(csv))
    selected = frame[["sales", "cost"]].fillna(0.0)
    return selected.to_numpy(dtype="float64")


if __name__ == "__main__":
    print(solve())
