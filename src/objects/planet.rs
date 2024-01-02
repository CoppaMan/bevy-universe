use {
    bevy::{
        app::{App, Plugin, Startup},
        asset::{AssetServer, Assets},
        ecs::{
            bundle::Bundle,
            component::Component,
            system::{Commands, Res, ResMut},
        },
        math::{DVec3, Quat, Vec3},
        pbr::{ParallaxMappingMethod, PbrBundle, StandardMaterial},
        render::mesh::{shape::UVSphere, Mesh},
        transform::components::Transform,
    },
    directories::ProjectDirs,
    physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION,
    serde::{Deserialize, Serialize},
    std::fs::{create_dir_all, read_dir, read_to_string},
};

use bevy::{log::info, render::color::Color};

use crate::{
    physics::components::{Acceleration, MassG, Velocity},
    utils,
};

use super::components::Focusable;

pub struct SpawnPlanetsPlugin;
impl Plugin for SpawnPlanetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_planets);
    }
}

fn spawn_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let proj_dir = ProjectDirs::from("com", "CoppaCom", "BeviPoc")
        .expect("")
        .data_dir()
        .join("planets");

    create_dir_all(&proj_dir).expect("");
    let planet_file_paths = read_dir(&proj_dir).expect("Unable to read craft files");

    for planet_file in planet_file_paths {
        let planet_file_path = &planet_file.expect("").path();
        let planet_string = read_to_string(planet_file_path).expect("");

        let mesh_handle = meshes.add(Mesh::from(UVSphere {
            radius: 6371000.,
            sectors: 64,
            stacks: 64,
        }));
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("earth/base_color.jpg")),
            depth_map: Some(asset_server.load("earth/elevation_surface.jpg")),
            parallax_mapping_method: ParallaxMappingMethod::Relief { max_steps: 4 },
            emissive_texture: Some(asset_server.load("earth/emissive.jpg")),
            emissive: Color::hsl(0.0, 0.0, 0.5),
            metallic_roughness_texture: Some(asset_server.load("earth/metallic_roughness.png")),
            normal_map_texture: Some(asset_server.load("earth/normal_map.jpg")),
            ..Default::default()
        });

        commands
            .spawn(PbrBundle {
                mesh: mesh_handle,
                material: material_handle,
                ..Default::default()
            })
            .insert(PlanetBundle::from_json(&planet_string));
        info!(
            "Spawned planet {}",
            planet_file_path.as_os_str().to_str().expect("")
        );
    }
}

#[derive(Component)]
pub struct Planet;

#[derive(Serialize, Deserialize)]
struct PlanetParser {
    position: Vec<f64>,
    velocity: Vec<f64>,
    mass: f64,
}

#[derive(Bundle)]

struct PlanetBundle {
    entity_type: Planet,
    focusable: Focusable,
    transform: Transform,
    velocity: Velocity,
    acceleration: Acceleration,
    mass_g: MassG,
}

impl PlanetBundle {
    fn new(position: DVec3, velocity: DVec3, mass: f64) -> Self {
        Self {
            entity_type: Planet,
            focusable: Focusable {
                focus_min_distance: 7000000.,
                focus_sphere_radius: 6371000.,
            },
            transform: Transform {
                translation: position.as_vec3(),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            velocity: Velocity(velocity),
            acceleration: Acceleration(DVec3::ZERO),
            mass_g: MassG(mass * NEWTONIAN_CONSTANT_OF_GRAVITATION),
        }
    }

    fn from_json(data: &str) -> Self {
        let c: PlanetParser = serde_json::from_str(data).expect("");

        PlanetBundle::new(
            utils::vectors::vec_to_dvec3(&c.position),
            utils::vectors::vec_to_dvec3(&c.velocity),
            c.mass,
        )
    }
}
