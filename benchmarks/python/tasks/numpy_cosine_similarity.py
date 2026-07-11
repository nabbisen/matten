import numpy as np


def solve():
    a = (np.arange(512, dtype=np.float64) % 7.0) + 1.0
    b = (np.arange(512, dtype=np.float64) % 7.0) + 1.0
    dot = np.dot(a, b)
    magnitude = np.linalg.norm(a) * np.linalg.norm(b)
    return float(dot / magnitude)


if __name__ == "__main__":
    print(solve())
