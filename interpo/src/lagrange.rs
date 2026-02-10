use crate::{InterpolationError, select_nearest_points};

pub fn lagrange_interp(points: &[(f64, f64)], x: f64) -> Result<f64, InterpolationError> {
    if points.is_empty() {
        return Err(InterpolationError::with_message(
            "Points array cannot be empty.",
        ));
    }

    let mut result: f64 = 0.0;
    for (idx, &(xi, yi)) in points.iter().enumerate() {
        let mut numerator = 1.0;
        let mut denominator = 1.0;

        for (k, &(xj, _)) in points.iter().enumerate() {
            if k != idx {
                numerator *= x - xj;
                denominator *= xi - xj;
            }
        }

        result += yi * numerator / denominator;
    }

    Ok(result)
}

pub fn lagrange_interp_nearest(
    points: &[(f64, f64)],
    degree: usize,
    x: f64,
) -> Result<f64, InterpolationError> {
    if points.len() < degree + 1 {
        return Err(InterpolationError::with_message(
            "Degree must be less than the number of points.",
        ));
    }

    let selected_points = select_nearest_points(points, degree, x);
    lagrange_interp(&selected_points, x)
}
