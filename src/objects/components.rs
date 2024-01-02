use bevy::ecs::component::Component;

///
#[derive(Component)]
pub struct Focusable {
    pub focus_min_distance: f64,
    pub focus_sphere_radius: f64,
}
