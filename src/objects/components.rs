use bevy::{
    ecs::{component::Component, entity::Entity},
    math::DVec3,
};

///
#[derive(Component)]
pub struct Focusable {
    pub focus_min_distance: f64,
    pub focus_sphere_radius: f64,
}

#[derive(Component)]
pub struct CraftLabel;

#[derive(Component)]
pub struct Craft;

#[derive(Component)]
pub struct FocusTarget {
    pub target: Entity,
    pub distance: DVec3,
}
