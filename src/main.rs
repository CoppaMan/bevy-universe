use bevy::{
    app::{App, Startup},
    core_pipeline::clear_color::ClearColor,
    ecs::system::Commands,
    log::info,
    math::Vec3,
    pbr::{AmbientLight, DirectionalLight, DirectionalLightBundle},
    render::color::Color,
    transform::components::Transform,
    DefaultPlugins,
};

mod objects;
mod physics;
mod renderer;
mod ui;
mod utils;

use utils::{arguments::parse_arguments, data::create_data};

use crate::{
    objects::LoadObjectsPlugins, physics::PhysicPlugin, renderer::RendererPlugin, ui::UiPlugins,
};

fn main() {
    // Create the data directory with example bodies
    let args = parse_arguments();
    if args.create_data {
        info!("Creating data dir");
        create_data("data".into());
    }

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
        ..Default::default()
    });
}
