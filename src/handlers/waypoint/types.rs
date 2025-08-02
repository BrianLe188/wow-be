use serde::Deserialize;

#[derive(Deserialize)]
pub struct OptimizeWaypointPayload {
    pub origin: [f64; 2],
    pub waypoints: Vec<Vec<[f64; 2]>>,
}
