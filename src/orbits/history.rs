use std::collections::VecDeque;

use bevy::{
    app::{App, FixedUpdate, Plugin},
    asset::Assets,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    log::info,
    math::{DVec3, Vec3},
    pbr::MaterialMeshBundle,
    render::{color::Color, mesh::Mesh, view::NoFrustumCulling},
    time::{Fixed, Time},
};

use crate::{
    floatingorigin::{components::FloatingOriginPosition, systemsets::FloatingOriginSet},
    orbits::systemsets::OrbitSets,
    physics::systemsets::PhysicsSet,
    renderer::line::{LineMaterial, LineStrip, OrbitHistoryMesh},
};

#[derive(Resource)]
pub struct SelectedReferenceFrame {
    pub target: Entity,
}

#[derive(Resource)]
pub struct OrbitHistoryMaxSize(pub usize);

#[derive(Resource)]
pub struct OrbitHistoryUpdateInterval {
    pub since_last: f32,
    pub max_interval: f32,
}

#[derive(Component)]
pub struct OrbitHistoryEntity(pub Entity);

#[derive(Bundle)]
pub struct OrbitHistoryBundle {
    origin: FloatingOriginPosition,
    mesh: MaterialMeshBundle<LineMaterial>,
    history: OrbitHistoryMesh,
    culling: NoFrustumCulling,
}
impl OrbitHistoryBundle {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials_line: &mut ResMut<Assets<LineMaterial>>,
    ) -> Entity {
        let line_mesh = meshes.add(Mesh::from(LineStrip { points: vec![] }));
        let line_mesh_id = line_mesh.id();
        commands
            .spawn(OrbitHistoryBundle {
                origin: FloatingOriginPosition(DVec3::ZERO),
                mesh: MaterialMeshBundle {
                    mesh: line_mesh,
                    material: materials_line.add(LineMaterial { color: Color::GRAY }),
                    ..Default::default()
                },
                history: OrbitHistoryMesh {
                    orbit_mesh: line_mesh_id,
                    history: VecDeque::new(),
                },
                culling: NoFrustumCulling,
            })
            .id()
    }
}

pub struct OrbitHistoryPlugin;
impl Plugin for OrbitHistoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OrbitHistoryUpdateInterval {
            since_last: 0.,
            max_interval: 1.,
        })
        .insert_resource(OrbitHistoryMaxSize(1000000))
        .insert_resource(SelectedReferenceFrame {
            // Show orbit history relative to this body
            target: Entity::PLACEHOLDER,
        })
        .add_systems(
            FixedUpdate,
            update_orbit_history
                .in_set(OrbitSets::DrawHistory)
                .after(PhysicsSet::All)
                .before(FloatingOriginSet::ApplyTransform),
        );
    }
}

fn update_orbit_history(
    mut mesh_asset_mut: ResMut<Assets<Mesh>>,
    history_objects: Query<(&OrbitHistoryEntity, &FloatingOriginPosition)>,
    mut histories: Query<&mut OrbitHistoryMesh>,
    reference: Res<SelectedReferenceFrame>,
    max_length: Res<OrbitHistoryMaxSize>,
    mut last_update: ResMut<OrbitHistoryUpdateInterval>,
    time: Res<Time<Fixed>>,
) {
    last_update.since_last += time.delta_seconds();
    if last_update.since_last < last_update.max_interval {
        return;
    } else {
        last_update.since_last -= last_update.max_interval;
    }

    // Add current position to our history
    history_objects.iter().for_each(|(history, origin)| {
        let mut history = histories.get_mut(history.0).expect("");
        history.history.push_back(origin.0);
        if history.history.len() > max_length.0 {
            info!("Pruning front: {} {}", history.history.len(), max_length.0);
            history.history.pop_front();
        }
    });

    // Get reference frame history
    let ref_history = match histories.get(reference.target) {
        Ok(o) => Some(o.history.to_owned()),
        Err(_) => None,
    };

    for (object_orbit, _) in history_objects.iter() {
        let history = histories.get(object_orbit.0).expect("");

        //for mut history in histories.iter_mut() {
        // Transform
        let transformed: Vec<Vec3> = match ref_history {
            Some(ref h) => h
                .iter()
                .zip(history.history.iter())
                .map(|(reference, own)| (*own - *reference + *h.back().expect("")).as_vec3())
                .collect(),
            None => history.history.iter().map(|v| v.as_vec3()).collect(),
        };

        // Apply translation
        mesh_asset_mut
            .get_mut(history.orbit_mesh)
            .expect("")
            .insert_attribute(Mesh::ATTRIBUTE_POSITION, transformed);
    }
}
