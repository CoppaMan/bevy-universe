use {
    bevy::{
        asset::{AssetServer, Assets},
        ecs::system::{Commands, Res, ResMut},
        log::info,
        math::Quat,
        pbr::{PbrBundle, StandardMaterial},
        render::mesh::{shape::UVSphere, Mesh},
        transform::components::Transform,
    },
    std::fs::{create_dir_all, read_dir, read_to_string},
};

use crate::{
    orbits::history::{OrbitHistoryBundle, OrbitHistoryEntity},
    renderer::line::LineMaterial,
    utils::data::{get_data_dir, DataDir},
};

use super::{bundles::PlanetBundle, parsers::PlanetParser};

pub fn spawn_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials_line: ResMut<Assets<LineMaterial>>,
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

        let hist_id = OrbitHistoryBundle::spawn(&mut commands, &mut meshes, &mut materials_line);
        let _ = commands
            .spawn(PbrBundle {
                mesh: mesh_handle,
                material: material_handle,
                transform: Transform::from_rotation(Quat::from_rotation_x(
                    parser.axial_tilt as f32,
                )),
                ..Default::default()
            })
            .insert(OrbitHistoryEntity(hist_id))
            .insert(PlanetBundle::from_parser(parser, hist_id))
            .id();

        info!(
            "Spawned planet {}",
            planet_file_path.as_os_str().to_str().expect("")
        );
    }
}
