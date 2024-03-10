use bevy::{
    ecs::{bundle::Bundle, entity::Entity},
    math::{DVec3, Quat, Vec3},
    transform::components::Transform,
};

use crate::{
    floatingorigin::bundles::FloatingOriginWithHistoryBundle, objects::components::Focusable,
    orbits::history::OrbitHistoryEntity, physics::bundles::NBodyActiveBundle, utils,
};

use super::{super::components::FocusType, components::Planet, parsers::PlanetParser};

#[derive(Bundle)]
pub struct PlanetBundle {
    entity_type: Planet,
    focusable: Focusable,
    transform: Transform,
    orbit_history: OrbitHistoryEntity,
    floating_origin: FloatingOriginWithHistoryBundle,
    nbody: NBodyActiveBundle,
}
impl PlanetBundle {
    fn new(
        name: String,
        position: DVec3,
        velocity: DVec3,
        mass: f64,
        radius: f64,
        axial_tilt: f64,
        angular_velocity: f64,
        orbit_history: Entity,
    ) -> Self {
        Self {
            entity_type: Planet {
                name: name,
                axial_tilt: axial_tilt,
                spin_velocity: angular_velocity,
                spin_position: 0.0,
            },

            focusable: Focusable {
                focus_min_distance: radius * 1.006,
                focus_sphere_radius: radius,
                focus_type: FocusType::Fixed,
            },
            transform: Transform {
                translation: position.as_vec3(),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            orbit_history: OrbitHistoryEntity(orbit_history),
            floating_origin: FloatingOriginWithHistoryBundle::new(&position),
            nbody: NBodyActiveBundle::new(&velocity, mass),
        }
    }
    pub fn from_parser(parser: PlanetParser, orbit_history: Entity) -> Self {
        PlanetBundle::new(
            parser.name,
            utils::vectors::vec_to_dvec3(&parser.position),
            utils::vectors::vec_to_dvec3(&parser.velocity),
            parser.mass,
            parser.radius,
            parser.axial_tilt,
            parser.angular_velocity,
            orbit_history,
        )
    }
}
