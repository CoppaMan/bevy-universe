use bevy::ecs::entity::Entity;

use {
    bevy::{
        app::{App, Plugin, Startup, Update},
        asset::{AssetServer, Assets, Handle},
        core_pipeline::core_3d::Camera3d,
        ecs::{
            bundle::Bundle,
            query::{With, Without},
            schedule::IntoSystemConfigs,
            system::{Commands, Query, Res, ResMut},
        },
        hierarchy::BuildChildren,
        log::info,
        math::{DVec3, Vec2, Vec3},
        pbr::{AlphaMode, PbrBundle, StandardMaterial},
        render::{
            mesh::{shape::Quad, Mesh},
            prelude::SpatialBundle,
        },
        transform::components::{GlobalTransform, Transform},
    },
    serde::{Deserialize, Serialize},
    std::fs::{create_dir_all, read_dir, read_to_string},
};

use crate::{
    floatingorigin::components::FloatingOriginPosition,
    objects::{components::Focusable, systemsets::ObjectSets},
    orbits::history::{OrbitHistoryBundle, OrbitHistoryEntity},
    physics::components::{NBodyAcceleration, NBodyEffector, NBodyVelocity},
    renderer::line::LineMaterial,
    utils::{
        self,
        data::{get_data_dir, DataDir},
    },
};

use super::components::{Craft, CraftLabel, FocusType};

#[derive(Bundle)]
struct CraftBundle {
    nbody: NBodyEffector,
    entity_type: Craft,
    position: FloatingOriginPosition,
    velocity: NBodyVelocity,
    acceleration: NBodyAcceleration,
    focusable: Focusable,
    orbit_history: OrbitHistoryEntity,
    spatial: SpatialBundle,
}

#[derive(Bundle)]
struct CraftLabelBundle {
    entity_type: CraftLabel,

    #[bundle()]
    label_plane: PbrBundle,
}

pub struct SpawnCraftPlugin;

impl Plugin for SpawnCraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_crafts.in_set(ObjectSets::SpawnCraft))
            .add_systems(Update, orient_labels);
    }
}

fn spawn_crafts(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials_line: ResMut<Assets<LineMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let proj_dir = get_data_dir(DataDir::Crafts);

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

        let hist_id = OrbitHistoryBundle::spawn(&mut commands, &mut meshes, &mut materials_line);
        let _ = commands
            .spawn(CraftBundle::from_json(&craft_string, hist_id))
            .with_children(|parent| {
                parent.spawn(CraftLabelBundle::new(quad_handle, material_handle));
            })
            .id();

        info!(
            "Spawned craft {}",
            craft_file_path.as_os_str().to_str().expect("")
        );
    }
}

impl CraftBundle {
    fn new(position: DVec3, velocity: DVec3, orbit_history: Entity) -> Self {
        Self {
            nbody: NBodyEffector,
            entity_type: Craft,
            position: FloatingOriginPosition(position),
            velocity: NBodyVelocity(velocity),
            acceleration: NBodyAcceleration(DVec3::ZERO),
            focusable: Focusable {
                focus_min_distance: 1000.,
                focus_sphere_radius: 0.5,
                focus_type: FocusType::Scale,
            },
            orbit_history: OrbitHistoryEntity(orbit_history),
            spatial: SpatialBundle {
                transform: Transform::from_translation(position.as_vec3()),
                ..Default::default()
            },
        }
    }

    fn from_json(data: &str, orbit_history: Entity) -> Self {
        let c: CraftParser = serde_json::from_str(data).expect("");

        CraftBundle::new(
            utils::vectors::vec_to_dvec3(&c.position),
            utils::vectors::vec_to_dvec3(&c.velocity),
            orbit_history,
        )
    }
}

#[derive(Serialize, Deserialize)]
struct CraftParser {
    position: Vec<f64>,
    velocity: Vec<f64>,
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
