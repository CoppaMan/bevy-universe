use bevy::{
    ecs::system::{Query, Res},
    math::DVec3,
    time::Time,
    transform::components::Transform,
};

use crate::physics::components::*;

use super::PhysicsStepScale;

///
/// Uses a symplectic integrator to apply the sum of accelerations to the velocity and position at the of a timestep timestep
///
pub fn integrate_time(
    mut bodys_mut: Query<(&mut Transform, &mut Velocity, &mut Acceleration)>,
    step_scale: Res<PhysicsStepScale>,
    time: Res<Time>,
) {
    for (mut transform, mut vel, mut acc) in bodys_mut.iter_mut() {
        // Scale timestep
        let final_step = time.delta_seconds_f64() * step_scale.0 as f64;

        // Semi-implicit Euler method
        vel.0 += acc.0 * final_step;
        transform.translation += (vel.0 * final_step).as_vec3();

        // Reset acceleration sum
        acc.0 = DVec3::ZERO;
    }
}
