use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct UiClicked(pub bool);
