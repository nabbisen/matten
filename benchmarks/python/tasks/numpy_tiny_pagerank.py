import numpy as np


def row_stochastic(n):
    rows = np.fromfunction(lambda r, c: ((r + c) % 5) + 1.0, (n, n), dtype=np.float64)
    return rows / rows.sum(axis=1, keepdims=True)


def solve():
    n = 64
    damping = 0.85
    links = row_stochastic(n)
    rank = np.arange(n, dtype=np.float64)
    base = (1.0 - damping) / n
    return (links @ rank) * damping + base


if __name__ == "__main__":
    print(solve()[:5])
