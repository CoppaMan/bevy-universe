use bevy::{
    app::{App, Plugin, Update},
    asset::Assets,
    ecs::{
        query::With,
        schedule::IntoSystemConfigs,
        system::{Query, ResMut},
    },
    math::Vec3,
    render::mesh::Mesh,
};

use crate::{
    floatingorigin::{components::FloatingOriginPosition, systemsets::FloatingOriginSet},
    objects::components::Craft,
    orbits::systemsets::OrbitSets,
    physics::systemsets::PhysicsSet,
    renderer::line::OrbitHistoryMesh,
    utils::vectors::f32_3_to_vec3,
};

pub struct OrbitHistoryPlugin;
impl Plugin for OrbitHistoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_orbit_history
                .in_set(OrbitSets::DrawHistory)
                .after(PhysicsSet::All)
                .before(FloatingOriginSet::ApplyTransform),
        );
    }
}

fn update_orbit_history(
    mut mesh_asset_mut: ResMut<Assets<Mesh>>,
    orbits: Query<&OrbitHistoryMesh>,
    crafts: Query<&FloatingOriginPosition, With<Craft>>,
) {
    for orbit in orbits.iter() {
        let mesh_mut = mesh_asset_mut.get_mut(orbit.orbit_mesh).expect("");
        let craft_pos = crafts.get(orbit.craft).expect("").0;

        let points_attr_id = mesh_mut.attributes_mut().last().expect("").0;
        let mut points: Vec<Vec3> = mesh_mut
            .attribute(points_attr_id)
            .expect("")
            .as_float3()
            .expect("")
            .iter()
            .map(|x| f32_3_to_vec3(x))
            .collect();

        // Append new entry
        points.push(craft_pos.as_vec3());
        mesh_mut.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
    }
}
