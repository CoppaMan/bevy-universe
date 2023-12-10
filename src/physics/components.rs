use bevy::{ecs::component::Component, math::DVec3};

#[derive(Component)]
pub struct NBodyEffector;

#[derive(Component)]
pub struct Velocity(pub DVec3);

#[derive(Component)]
pub struct Acceleration(pub DVec3);

#[derive(Component)]
pub struct MassG(pub f64);
