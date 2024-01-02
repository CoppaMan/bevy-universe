use bevy::{
    app::{App, Startup},
    core_pipeline::clear_color::ClearColor,
    ecs::system::Commands,
    math::Vec3,
    pbr::{AmbientLight, CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle},
    render::color::Color,
    transform::components::Transform,
    DefaultPlugins,
};

mod objects;
mod physics;
mod renderer;
mod ui;
mod utils;

use crate::{
    objects::LoadObjectsPlugins, physics::PhysicPlugin, renderer::RendererPlugin, ui::UiPlugins,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RendererPlugin,
            UiPlugins,
            LoadObjectsPlugins,
            PhysicPlugin,
        ))
        .add_systems(Startup, spawn_light)
        .run();
}

fn spawn_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::BLACK,
        brightness: 0.,
    });

    commands.insert_resource(ClearColor(Color::BLACK));

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
