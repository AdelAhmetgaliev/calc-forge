pub fn select_nearest_points(points: &[(f64, f64)], degree: usize, x: f64) -> Vec<(f64, f64)> {
    let mut distances: Vec<(f64, usize)> = points
        .iter()
        .enumerate()
        .map(|(idx, &(val, _))| (f64::abs(val - x), idx))
        .collect();
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let selected_points: Vec<(f64, f64)> = distances
        .iter()
        .take(degree + 1)
        .map(|&(_, idx)| points[idx])
        .collect();

    selected_points
}
