use bevy::{ecs::bundle::Bundle, math::DVec3};
use physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;

use super::components::{MassG, NBodyAcceleration, NBodyEffector, NBodyVelocity};

#[derive(Bundle)]
pub struct NBodyPassiveBundle {
    effector: NBodyEffector,
    velocity: NBodyVelocity,
    acceleration: NBodyAcceleration,
}
impl NBodyPassiveBundle {
    pub fn new(start_velocity: &DVec3) -> NBodyPassiveBundle {
        NBodyPassiveBundle {
            effector: NBodyEffector,
            velocity: NBodyVelocity(*start_velocity),
            acceleration: NBodyAcceleration(DVec3::ZERO),
        }
    }
}

#[derive(Bundle)]
pub struct NBodyActiveBundle {
    mass: MassG,
    passive: NBodyPassiveBundle,
}
impl NBodyActiveBundle {
    pub fn new(start_velocity: &DVec3, mass: f64) -> NBodyActiveBundle {
        NBodyActiveBundle {
            mass: MassG(mass * NEWTONIAN_CONSTANT_OF_GRAVITATION),
            passive: NBodyPassiveBundle::new(start_velocity),
        }
    }
}
