use {
    bevy::{
        app::{App, Plugin, Startup},
        asset::{AssetServer, Assets},
        ecs::{
            bundle::Bundle,
            component::Component,
            schedule::{apply_deferred, IntoSystemConfigs},
            system::{Commands, Res, ResMut},
        },
        log::info,
        math::{DVec3, Quat, Vec3},
        pbr::{PbrBundle, StandardMaterial},
        render::mesh::{shape::UVSphere, Mesh},
        transform::components::Transform,
    },
    physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION,
    serde::{Deserialize, Serialize},
    std::fs::{create_dir_all, read_dir, read_to_string},
};

use crate::{
    floatingorigin::components::FloatingOriginPosition,
    objects::{components::Focusable, systemsets::ObjectSets},
    physics::components::{MassG, NBodyAcceleration, NBodyEffector, NBodyVelocity},
    utils::{
        self,
        data::{get_data_dir, DataDir},
    },
};

pub struct SpawnPlanetsPlugin;
impl Plugin for SpawnPlanetsPlugin {
    fn build(&self, app: &mut App) {
        // Apply deferred to ensure planets have been created
        app.add_systems(
            Startup,
            (spawn_planets, apply_deferred)
                .chain()
                .in_set(ObjectSets::SpawnPlanet),
        );
    }
}

fn spawn_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let proj_dir = get_data_dir(DataDir::Planets);

    create_dir_all(&proj_dir).expect("");
    let planet_file_paths = read_dir(&proj_dir).expect("Unable to read craft files");

    for planet_file in planet_file_paths {
        let planet_file_path = &planet_file.expect("").path();
        let planet_string = read_to_string(planet_file_path).expect("");
        let parser: PlanetParser = serde_json::from_str(&planet_string).expect("");
        let planet_name = planet_file_path.file_stem().expect("").to_str().expect("");

        let mesh_handle = meshes.add(Mesh::from(UVSphere {
            radius: parser.radius as f32,
            sectors: 64,
            stacks: 64,
        }));
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load(format!("{}/base_color.jpg", planet_name))),
            /*depth_map: Some(asset_server.load(format!("{}/elevation_surface.jpg", planet_name))),
            parallax_mapping_method: ParallaxMappingMethod::Relief { max_steps: 4 },
            emissive_texture: Some(asset_server.load(format!("{}/emissive.jpg", planet_name))),
            emissive: Color::hsl(0.0, 0.0, 0.5),
            metallic_roughness_texture: Some(
                asset_server.load(format!("{}/metallic_roughness.png", planet_name)),
            ),
            normal_map_texture: Some(asset_server.load(format!("{}/normal_map.jpg", planet_name))),*/
            ..Default::default()
        });

        commands
            .spawn(PbrBundle {
                mesh: mesh_handle,
                material: material_handle,
                ..Default::default()
            })
            .insert(PlanetBundle::from_parser(&parser));
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
    radius: f64,
}

#[derive(Bundle)]

struct PlanetBundle {
    entity_type: Planet,
    nbody: NBodyEffector,
    focusable: Focusable,
    transform: Transform,
    position: FloatingOriginPosition,
    velocity: NBodyVelocity,
    acceleration: NBodyAcceleration,
    mass_g: MassG,
}

impl PlanetBundle {
    fn new(position: DVec3, velocity: DVec3, mass: f64, radius: f64) -> Self {
        Self {
            entity_type: Planet,
            nbody: NBodyEffector,
            focusable: Focusable {
                focus_min_distance: radius * 1.006,
                focus_sphere_radius: radius,
            },
            transform: Transform {
                translation: position.as_vec3(),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            position: FloatingOriginPosition(position),
            velocity: NBodyVelocity(velocity),
            acceleration: NBodyAcceleration(DVec3::ZERO),
            mass_g: MassG(mass * NEWTONIAN_CONSTANT_OF_GRAVITATION),
        }
    }

    fn from_parser(parser: &PlanetParser) -> Self {
        PlanetBundle::new(
            utils::vectors::vec_to_dvec3(&parser.position),
            utils::vectors::vec_to_dvec3(&parser.velocity),
            parser.mass,
            parser.radius,
        )
    }
}
