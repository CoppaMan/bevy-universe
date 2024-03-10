pub mod bundles;
pub mod components;
pub mod systemsets;

use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        query::With,
        schedule::{IntoSystemConfigs, IntoSystemSetConfigs},
        system::Query,
    },
    render::camera::Camera,
    transform::components::Transform,
};

use crate::physics::systemsets::PhysicsSet;

use self::{components::FloatingOriginPosition, systemsets::FloatingOriginSet};

pub struct FloatingOriginPlugin;
impl Plugin for FloatingOriginPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            floating_origin_transform.in_set(FloatingOriginSet::ApplyTransform),
        )
        .configure_sets(
            Update,
            FloatingOriginSet::ApplyTransform.after(PhysicsSet::All),
        );
    }
}

pub fn floating_origin_transform(
    camera: Query<&FloatingOriginPosition, With<Camera>>,
    mut bodies: Query<(&mut Transform, &FloatingOriginPosition)>,
) {
    //info!("floating_origin_transform");
    let camera_position = camera.get_single().expect("");
    for (mut body_transform, body_position) in bodies.iter_mut() {
        body_transform.translation = (body_position.0 - camera_position.0).as_vec3();
    }
}
