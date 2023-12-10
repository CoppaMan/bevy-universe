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
        math::{DVec3, Vec2, Vec3},
        pbr::{AlphaMode, PbrBundle, StandardMaterial},
        render::{
            mesh::{shape::Quad, Mesh},
            prelude::SpatialBundle,
        },
        transform::components::Transform,
    },
    directories::ProjectDirs,
    serde::{Deserialize, Serialize},
    std::fs::{create_dir_all, read_dir, read_to_string},
};

use bevy::log::info;

use crate::{
    physics::components::{Acceleration, NBodyEffector, Velocity},
    utils,
};

use super::components::FocusSphere;

pub struct SpawnCraftPlugin;

impl Plugin for SpawnCraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_crafts)
            .add_systems(Update, orient_labels);
    }
}

fn spawn_crafts(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

        commands
            .spawn(CraftBundle::from_json(&craft_string))
            .with_children(|parent| {
                parent.spawn(CraftLabelBundle::new(quad_handle, material_handle));
            });

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
    focusable: FocusSphere,

    #[bundle()]
    label_plane: PbrBundle,
}

impl CraftLabelBundle {
    fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self {
            entity_type: CraftLabel,
            focusable: FocusSphere(10.),
            label_plane: PbrBundle {
                mesh: mesh,
                material: material,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
        }
    }
}

fn orient_labels(
    mut labels_mut: Query<&mut Transform, (With<CraftLabel>, Without<Camera3d>, Without<Craft>)>,
    camera: Query<&Transform, (With<Camera3d>, Without<CraftLabel>, Without<Craft>)>,
) {
    for mut label_transform in labels_mut.iter_mut() {
        let camera_transform = camera
            .get_single()
            .expect("There should be exactly on camera");

        let look_at = label_transform.translation - camera_transform.translation;
        label_transform.look_at(look_at, Vec3::Z);
        label_transform.scale = Vec3::ONE * look_at.length() * 2e-2;
    }
}
