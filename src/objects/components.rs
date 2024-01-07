use bevy::{
    ecs::{component::Component, entity::Entity},
    math::DVec3,
};

#[derive(Eq, PartialEq, Hash)]
pub enum FocusType {
    Fixed,
    Scale,
}

#[derive(Component)]
pub struct Focusable {
    pub focus_min_distance: f64,
    pub focus_sphere_radius: f64,
    pub focus_type: FocusType,
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
