use std::f64::consts::PI;

use crate::physics::resources::*;
use bevy::{
    ecs::system::{Query, Res},
    math::Quat,
    time::Time,
    transform::components::Transform,
};

use super::components::Planet;

pub fn rotate_planets(
    time: Res<Time>,
    time_scale: Res<PhysicsTimeScale>,
    step_scale: Res<PhysicsStepScale>,
    mut planets_q: Query<(&mut Transform, &mut Planet)>,
) {
    for (mut planet_transform, mut planet) in planets_q.iter_mut() {
        // Compute rotation and condition
        let speed = time_scale.0 as f64 * step_scale.0 as f64 * time.delta_seconds_f64();
        planet.spin_position += planet.spin_velocity * speed;
        let over = (planet.spin_position / (2.0 * PI)).floor();
        planet.spin_position -= planet.spin_position * over;

        let tilt = Quat::from_rotation_x(planet.axial_tilt as f32);
        let spin = Quat::from_rotation_z(planet.spin_position as f32);
        planet_transform.rotation = tilt * spin;
    }
}
