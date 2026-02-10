from . import DATA_DIR

import os

import numpy as np


def generate_data(
    func, x_min: float, x_max: float, num_points: int, filename: str
) -> None:
    filepath = os.path.join(DATA_DIR, filename)
    x_values = np.linspace(x_min, x_max, num_points)
    y_values = func(x_values)

    np.savetxt(
        filepath,
        np.column_stack((x_values, y_values)),
        fmt="%.6f",
        delimiter="\t",
    )


def function(x: np.ndarray) -> np.ndarray:
    return 1 / (1 + 25 * x**2)


if __name__ == "__main__":
    filename = "input_data.dat"
    x_min, x_max = -1.0, 1.0
    num_points = 100

    generate_data(function, x_min, x_max, num_points, filename)
