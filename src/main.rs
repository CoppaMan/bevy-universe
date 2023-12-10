use bevy::{
    app::{App, Startup},
    ecs::system::Commands,
    math::Vec3,
    pbr::{CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle},
    transform::components::Transform,
    DefaultPlugins,
};

mod objects;
mod physics;
mod ui;
mod utils;

use crate::{objects::LoadObjectsPlugins, physics::PhysicPlugin, ui::UiPlugins};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugins)
        .add_plugins(LoadObjectsPlugins)
        .add_plugins(PhysicPlugin)
        .add_systems(Startup, spawn_light)
        .run();
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 25000.,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(-20000000.0, 0.0, 0.0)
            .looking_at(Vec3::new(1.0, 0.0, 0.0), Vec3::Z),
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    });
}
