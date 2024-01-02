use bevy::{ecs::component::Component, math::DVec3};

/// Marks entities as being affected by the n-body calculations.
#[derive(Component)]
pub struct NBodyEffector;

/// The velocity of the body
#[derive(Component)]
pub struct Velocity(pub DVec3);

/// The acceleration of the entity, used for summing small accelerations during physics interactions.
/// Acceleration gets applied and reset during timestep integration.
#[derive(Component)]
pub struct Acceleration(pub DVec3);

/// Mass multiplied by the gravitional constant G.
/// Used as a gravity source during the n-body calculations.
#[derive(Component)]
pub struct MassG(pub f64);
