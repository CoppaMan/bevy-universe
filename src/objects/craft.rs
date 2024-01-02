use {
    bevy::{
        app::{App, Plugin, Startup, Update},
        asset::{AssetServer, Assets, Handle},
        core_pipeline::core_3d::Camera3d,
        ecs::{
            bundle::Bundle,
            component::Component,
            query::{With, Without},
            system::{Commands, Query, Res, ResMut},
        },
        hierarchy::BuildChildren,
        log::info,
        math::{DVec3, Vec2, Vec3},
        pbr::MaterialMeshBundle,
        pbr::{AlphaMode, PbrBundle, StandardMaterial},
        render::{
            color::Color,
            mesh::{shape::Quad, Mesh},
            prelude::SpatialBundle,
        },
        transform::components::Transform,
        utils::Duration,
    },
    directories::ProjectDirs,
    serde::{Deserialize, Serialize},
    std::fs::{create_dir_all, read_dir, read_to_string},
};

use bevy::{
    ecs::schedule::IntoSystemConfigs, time::common_conditions::on_timer,
    transform::components::GlobalTransform,
};

use crate::{
    objects::components::Focusable,
    physics::components::{Acceleration, NBodyEffector, Velocity},
    renderer::line::{LineMaterial, LineStrip, OrbitHistoryMesh},
    utils::{self, vectors::f32_3_to_vec3},
};

pub struct SpawnCraftPlugin;

impl Plugin for SpawnCraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_crafts).add_systems(
            Update,
            (
                orient_labels,
                update_orbit_history.run_if(on_timer(Duration::from_millis(100))),
            ),
        );
    }
}

fn spawn_crafts(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials_line: ResMut<Assets<LineMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let proj_dir = ProjectDirs::from("com", "CoppaCom", "BeviPoc")
        .expect("")
        .data_dir()
        .join("crafts");

    create_dir_all(&proj_dir).expect("");
    let craft_file_paths = read_dir(&proj_dir).expect("Unable to read craft files");

    for craft_file in craft_file_paths {
        let craft_file_path = &craft_file.expect("").path();

        let craft_string = read_to_string(craft_file_path).expect("");

        let quad_width = 1.0;
        let quad_handle = meshes.add(Mesh::from(Quad::new(Vec2::new(quad_width, quad_width))));
        let texture_handle = asset_server.load("textures/craft.png");

        // this material renders the texture normally
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..Default::default()
        });

        let line_mesh = meshes.add(Mesh::from(LineStrip {
            points: vec![
                //Vec3::new(0., 5000000., 0.),
                //Vec3::new(0., 5000000., 100000.),
            ],
        }));

        let mesh_id = line_mesh.id();
        let craft_id = commands
            .spawn(CraftBundle::from_json(&craft_string))
            .with_children(|parent| {
                parent.spawn(CraftLabelBundle::new(quad_handle, material_handle));
            })
            .id();
        commands.spawn((
            MaterialMeshBundle {
                mesh: line_mesh,
                material: materials_line.add(LineMaterial {
                    color: Color::LIME_GREEN,
                }),
                ..Default::default()
            },
            OrbitHistoryMesh {
                orbit_mesh: mesh_id,
                craft: craft_id,
            },
        ));

        info!(
            "Spawned craft {}",
            craft_file_path.as_os_str().to_str().expect("")
        );
    }
}

#[derive(Component)]
pub struct Craft;

#[derive(Bundle)]
struct CraftBundle {
    nbody: NBodyEffector,
    entity_type: Craft,
    velocity: Velocity,
    acceleration: Acceleration,
    focusable: Focusable,

    #[bundle()]
    spatial: SpatialBundle,
}

impl CraftBundle {
    fn new(position: DVec3, velocity: DVec3) -> Self {
        Self {
            nbody: NBodyEffector,
            entity_type: Craft,
            velocity: Velocity(velocity),
            acceleration: Acceleration(DVec3::ZERO),
            focusable: Focusable {
                focus_min_distance: 10.,
                focus_sphere_radius: 100000.,
            },
            spatial: SpatialBundle {
                transform: Transform::from_translation(position.as_vec3()),
                ..Default::default()
            },
        }
    }

    fn from_json(data: &str) -> Self {
        let c: CraftParser = serde_json::from_str(data).expect("");

        CraftBundle::new(
            utils::vectors::vec_to_dvec3(&c.position),
            utils::vectors::vec_to_dvec3(&c.velocity),
        )
    }
}

#[derive(Serialize, Deserialize)]
struct CraftParser {
    position: Vec<f64>,
    velocity: Vec<f64>,
}

#[derive(Component)]
pub struct CraftLabel;

#[derive(Bundle)]
struct CraftLabelBundle {
    entity_type: CraftLabel,

    #[bundle()]
    label_plane: PbrBundle,
}

impl CraftLabelBundle {
    fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self {
            entity_type: CraftLabel,
            label_plane: PbrBundle {
                mesh: mesh,
                material: material,
                ..Default::default()
            },
        }
    }
}

fn orient_labels(
    mut labels_mut: Query<
        (&mut Transform, &GlobalTransform),
        (With<CraftLabel>, Without<Camera3d>, Without<Craft>),
    >,
    camera: Query<&GlobalTransform, (With<Camera3d>, Without<CraftLabel>, Without<Craft>)>,
) {
    for (mut label_transform, label_global) in labels_mut.iter_mut() {
        let camera_transform = camera
            .get_single()
            .expect("There should be exactly on camera");

        let look_at = label_global.translation() - camera_transform.translation();
        label_transform.look_at(look_at, Vec3::Z);
        label_transform.scale = Vec3::ONE * look_at.length() * 4e-2;
    }
}

fn update_orbit_history(
    mut mesh_asset_mut: ResMut<Assets<Mesh>>,
    orbits: Query<&OrbitHistoryMesh>,
    crafts: Query<&Transform, With<Craft>>,
) {
    for orbit in orbits.iter() {
        let mesh_mut = mesh_asset_mut.get_mut(orbit.orbit_mesh).expect("");
        let craft_pos = crafts.get(orbit.craft).expect("").translation;

        let points_attr_id = mesh_mut.attributes_mut().last().expect("").0;
        let mut points: Vec<Vec3> = mesh_mut
            .attribute(points_attr_id)
            .expect("")
            .as_float3()
            .expect("")
            .iter()
            .map(|x| f32_3_to_vec3(x))
            .collect();
        //info!("Points: {:?}", points);

        // Append new entry
        points.push(craft_pos);
        mesh_mut.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
    }
}
