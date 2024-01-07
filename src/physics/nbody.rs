use bevy::{
    ecs::{
        entity::Entity,
        query::{With, Without},
        system::Query,
    },
    math::DVec3,
    transform::components::Transform,
};

use crate::physics::components::*;

///
/// Computes the interactions on bodies with NBodyEffector caused by large bodies with MassG.
/// Converts the gravitational force into a sum of accelerations.
///
pub fn nbody_accelerate(
    mut crafts_mut: Query<
        (&mut NBodyAcceleration, &Transform),
        (With<NBodyEffector>, Without<MassG>),
    >,
    mut planets_mut: Query<(Entity, &mut NBodyAcceleration, &Transform, &MassG), With<MassG>>,
) {
    //info!("nbody_accelerate");
    fn gravity_acc(own_pos: DVec3, other_pos: DVec3, other_mass: f64) -> DVec3 {
        let dist_vec = other_pos - own_pos;
        let dist = dist_vec.length();
        (other_mass / dist.powi(3)) * dist_vec
    }

    // Calculate and apply acceleration on crafts
    for (mut acc, craft_transform) in crafts_mut.iter_mut() {
        //info!("Computing forces for craft");
        for (_, _, planet_transform, planet_mass) in planets_mut.iter() {
            acc.0 += gravity_acc(
                craft_transform.translation.as_dvec3(),
                planet_transform.translation.as_dvec3(),
                planet_mass.0,
            )
        }
        //info!("Craft acceleration is {:?}", acc.0);
    }

    // Calculate acceleration on planets
    let mut planets_acc_change: Vec<DVec3> = Vec::new();
    for (id_dst, _, transform_dst, _) in planets_mut.iter() {
        //info!("Computing forces for planet");
        let mut planet_acc_change = DVec3::ZERO;
        for (id_src, _, transform_src, mass) in planets_mut.iter() {
            if id_dst == id_src {
                continue;
            }
            planet_acc_change += gravity_acc(
                transform_src.translation.as_dvec3(),
                transform_dst.translation.as_dvec3(),
                mass.0,
            )
        }
        planets_acc_change.push(planet_acc_change);
    }

    // Apply acceleration on planets
    for (_, mut acc, _, _) in planets_mut.iter_mut() {
        acc.0 += planets_acc_change.pop().expect("");
    }
}
