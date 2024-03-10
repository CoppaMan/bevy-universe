use std::collections::VecDeque;

use bevy::{ecs::component::Component, math::DVec3};

/// The nbody position
#[derive(Component)]
pub struct FloatingOriginPosition(pub DVec3);

#[derive(Component)]
pub struct FloatingOriginHistory(pub VecDeque<DVec3>);
