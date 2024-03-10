use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Planet {
    pub axial_tilt: f64,
    pub spin_velocity: f64,
    pub spin_position: f64,
    pub name: String,
}
