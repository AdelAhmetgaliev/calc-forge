import os
import random

import numpy as np

from . import DATA_DIR


def generate_data(
    func, x_min: float, x_max: float, num_points: int, filename: str
) -> None:
    filepath = os.path.join(DATA_DIR, filename)
    x_values = np.linspace(x_min, x_max, num_points)

    y_values = np.zeros(100)

    for _ in range(5):
        y_values += generate_normal_distribution_points()

    np.savetxt(
        filepath,
        np.column_stack((x_values, y_values)),
        fmt="%.6f",
        delimiter="\t",
    )


def function(x: np.ndarray) -> np.ndarray:
    return 1 / (1 + 25 * x**2)


def generate_normal_distribution_points(num_points=100):
    mean = random.uniform(-10, 10)
    std_dev = random.uniform(1, 5)
    amplitude = random.uniform(0.5, 2)

    points = np.random.normal(loc=mean, scale=std_dev, size=num_points)

    points *= amplitude

    return points


if __name__ == "__main__":
    filename = "input_data_normal.dat"
    x_min, x_max = -1.0, 1.0
    num_points = 100

    generate_data(function, x_min, x_max, num_points, filename)
