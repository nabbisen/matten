import numpy as np


def solve():
    m = 256
    x = np.column_stack((np.ones(m, dtype=np.float64), np.arange(m, dtype=np.float64)))
    theta = np.array([0.0, 0.0], dtype=np.float64)
    y = 2.0 * np.arange(m, dtype=np.float64) + 1.0
    residual = x @ theta - y
    grad = x.T @ residual
    return theta - 0.0001 * grad


if __name__ == "__main__":
    print(solve())
