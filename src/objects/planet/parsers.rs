use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlanetParser {
    pub name: String,
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub mass: f64,
    pub radius: f64,
    pub axial_tilt: f64,
    pub angular_velocity: f64,
}
