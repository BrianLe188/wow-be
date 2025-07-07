use std::f64;
const R: f64 = 6371.0;

fn to_radians(degree: f64) -> f64 {
    degree * f64::consts::PI / 180.0
}

/// Calculates the great-circle distance between two geographic coordinates using the Haversine formula.
///
/// # Parameters
/// - `coord1`: The first coordinate as `[latitude, longitude]` in degrees.
/// - `coord2`: The second coordinate as `[latitude, longitude]` in degrees.
///
/// # Returns
/// The distance between the two coordinates in meters (`f64`).
///
/// # Example
/// ```rust
/// let d = haversine([10.0, 106.0], [10.1, 106.1]);
/// println!("distance: {} meters", d);
/// ```
///
/// # Remarks
/// This function expects latitude and longitude in degrees and uses the Earth's radius (`R`) in meters for calculations.
///
fn haversine(coord1: [f64; 2], coord2: [f64; 2]) -> f64 {
    let [lat1, lng1] = coord1;
    let [lat2, lng2] = coord2;

    let phi1 = to_radians(lat1);
    let phi2 = to_radians(lat2);
    let delta_phi = to_radians(lat2 - lat1);
    let delta_lambda = to_radians(lng2 - lng1);

    let a = (f64::sin(delta_phi / 2.0)).powi(2)
        + f64::cos(phi1) * f64::cos(phi2) * (f64::sin(delta_lambda / 2.0)).powi(2);
    let c = 2.0 * f64::atan2(f64::sqrt(a), f64::sqrt(1.0 - a));

    R * c
}

/// Finds the point in `points` that is closest to the `origin` using the Haversine distance.
///
/// # Parameters
/// - `origin`: The reference point as `[latitude, longitude]` in degrees.
/// - `points`: A slice of points (`[[latitude, longitude]; N]`) to compare.
///
/// # Returns
/// A reference to the point in `points` that is nearest to `origin` (by Haversine distance).
///
/// # Example
/// ```rust
/// let origin = [10.0, 106.0];
/// let points = [[10.1, 106.2], [10.2, 106.3]];
/// let nearest = min_point(origin, &points);
/// println!("Nearest point: {:?}", nearest);
///
fn min_point(origin: [f64; 2], points: &[[f64; 2]]) -> &[f64; 2] {
    let mut point = points.first().unwrap_or(&[0.0, 0.0]);
    let mut min_distance = haversine(origin, *point);
    let mut index = 1;

    while index < points.len() {
        let distance = haversine(origin, points[index]);
        if distance < min_distance {
            min_distance = distance;
            point = &points[index];
        }
        index += 1;
    }

    point
}

/// Solves the Traveling Salesman Problem approximately using the Nearest Neighbor heuristic.
///
/// # Parameters
/// - `origin`: Starting point as `[latitude, longitude]` in degrees.
/// - `points`: List of waypoints as `Vec<[latitude, longitude]>` in degrees.
///
/// # Returns
/// A list (`Vec<[f64; 2]>`) representing the order in which to visit waypoints to minimize travel distance,
/// starting from `origin`.
///
/// # Example
/// ```rust
/// let origin = [10.0, 106.0];
/// let waypoints = vec![[10.2, 106.1], [10.1, 106.2]];
/// let path = nearest_neighbor(origin, waypoints);
/// println!("Optimized path: {:?}", path);
///
pub fn nearest_neighbor(origin: [f64; 2], points: Vec<[f64; 2]>) -> Vec<[f64; 2]> {
    let mut path: Vec<[f64; 2]> = vec![];
    let mut unvisited = points.clone();
    let mut current_point = origin;

    while !unvisited.is_empty() {
        let next_point = *min_point(current_point, &unvisited);
        path.push(next_point);
        if let Some(pos) = unvisited.iter().position(|&p| p == next_point) {
            unvisited.remove(pos);
        }
        current_point = next_point;
    }

    path
}
