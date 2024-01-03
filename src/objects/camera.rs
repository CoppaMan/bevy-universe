use bevy::{
    app::{App, Plugin, Startup, Update},
    core_pipeline::{clear_color::ClearColorConfig, core_3d::Camera3dBundle},
    ecs::{
        event::*,
        system::{Commands, Query, Res},
    },
    input::{mouse::*, Input},
    math::{Mat3, Quat, Vec2, Vec3},
    prelude::*,
    transform::components::Transform,
};

use crate::objects::{components::Focusable, planet::Planet, systemsets::ObjectSets};

pub struct SpawnCameraPlugin;

impl Plugin for SpawnCameraPlugin {
    fn build(&self, app: &mut App) {
        // Only spawn the camera after the planets have been spawned
        app.configure_sets(
            Startup,
            ObjectSets::SpawnCamera.after(ObjectSets::SpawnPlanet),
        )
        .add_systems(Startup, spawn_camera.in_set(ObjectSets::SpawnCamera))
        .add_systems(Update, (pan_orbit_camera, change_camera_focus));
    }
}

fn spawn_camera(mut commands: Commands, planets: Query<Entity, With<Planet>>) {
    info!("Spawning camera");
    let camera_start_pos = Vec3::new(-10000000.0, 0.0, 0.0);

    let camera = commands
        .spawn((Camera3dBundle {
            transform: Transform::from_translation(camera_start_pos)
                .looking_at(Vec3::new(1., 0., 0.), Vec3::Z),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 1.0472,
                ..Default::default()
            }),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..Default::default()
            },
            ..Default::default()
        },))
        .id();

    for planet in planets.iter() {
        commands
            .get_entity(planet)
            .unwrap()
            .push_children(&[camera]);
        info!("Attached camera to planet {:?}", planet);
        break;
    }
}

fn change_camera_focus(
    win_q: Query<&Window>,
    input_mouse: Res<Input<MouseButton>>,
    focus: Query<(Entity, &GlobalTransform, &Focusable), With<Focusable>>,
    mut camera_q: Query<(Entity, &GlobalTransform, &Camera, &mut Transform), With<Camera3d>>,
    mut commands: Commands,
) {
    if input_mouse.just_released(MouseButton::Left) {
        info!("Checking for new focus for camera");
        let window = win_q.get_single().expect("");
        let (camera_id, camera_transform, camera, mut cam_transform_mut) =
            camera_q.get_single_mut().expect("");

        let mut closest_id = Entity::PLACEHOLDER;
        let mut closest_distance = f32::MAX;
        let mut closest_transform = GlobalTransform::IDENTITY;
        for (entity_id, focus_transform, focus_foc) in focus.iter() {
            info!("Test for intersection with {:?}", entity_id);
            let ray = camera
                .viewport_to_world(camera_transform, window.cursor_position().expect(""))
                .expect("");

            info!("{:?} Ray is: {:?}", entity_id, ray);

            // Compute intersection
            let m = ray.origin - focus_transform.translation();
            let b = m.dot(ray.direction);
            let c =
                m.dot(m) - (focus_foc.focus_sphere_radius * focus_foc.focus_sphere_radius) as f32;

            // Exit if r’s origin outside s (c > 0) and r pointing away from s (b > 0)
            if c > 0.0 && b > 0.0 {
                // No intersection possible
                info!("{:?} Pointing away", entity_id);
                continue;
            }

            let discr = (b * b) - c;
            info!("{:?} Discriminant: {}", entity_id, discr);

            // A negative discriminant corresponds to ray missing sphere
            if discr < 0.0 {
                info!("{:?} Missing sphere", entity_id);
                continue;
            }

            // Ray now found to intersect sphere, compute smallest t value of intersection
            let t = -b - discr.sqrt();
            if t <= closest_distance {
                info!("{:?} Found intersection at {}", entity_id, t);
                closest_distance = t;
                closest_id = entity_id;
                closest_transform = *focus_transform;
            } else {
                info!(
                    "{:?} Is further away then {:?}: {} vs {}",
                    entity_id, closest_id, t, closest_distance
                );
            }
        }

        // Change camera focus when new focus point was selected
        if closest_id != Entity::PLACEHOLDER {
            info!("{:?} is closest", closest_id);

            commands
                .get_entity(closest_id)
                .unwrap()
                .push_children(&[camera_id]);

            cam_transform_mut.translation =
                camera_transform.translation() - closest_transform.translation();

            let new_dir = -cam_transform_mut.translation;
            cam_transform_mut.look_at(new_dir, Vec3::Z);
        } else {
            info!("No intersection found")
        }

        info!(
            "Camera transform {:?}\nFocus transform {:?}",
            camera_transform, closest_transform
        )
    }
}

fn pan_orbit_camera(
    win_q: Query<&Window>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&Parent, &mut Transform), With<Camera>>,
    center_object_q: Query<&Focusable>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;

    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;

    let win = win_q.get_single().expect("");

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.read() {
            rotation_move += ev.delta;
        }
    }
    for ev in ev_scroll.read() {
        scroll += ev.y;
    }

    for (center, mut transform) in query.iter_mut() {
        let up: Vec3 = transform.rotation * Vec3::Z;
        let mut distance = transform.translation.length();

        let mut any = false;

        // Panning
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(win);
            let delta_z = rotation_move.x / window.x * std::f32::consts::PI;
            let delta_x = {
                let delta = rotation_move.y / window.y * std::f32::consts::PI;
                if (-0.95 > up.z && delta < 0.0) || (up.z > 0.95 && delta > 0.0) {
                    0.0
                } else {
                    delta
                }
            };
            let roll = Quat::from_rotation_z(-delta_z);
            let pitch = Quat::from_rotation_x(-delta_x);
            transform.rotation = roll * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis

        // Zooming
        } else if scroll.abs() > 0.0 {
            any = true;

            // New distance can only be larger than minimum focus distance
            let center_object = center_object_q.get(center.get()).expect("");
            let new_distance = distance - scroll * distance * 0.2;
            if new_distance > center_object.focus_min_distance as f32 {
                distance = new_distance;
            }
            info!("Zoom distance: {}", distance);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, distance));
        }
    }

    // consume any remaining events, so they don't pile up if we don't need them
    // (and also to avoid Bevy warning us about not checking events every frame update)
    ev_motion.clear();
}

fn get_primary_window_size(window: &Window) -> Vec2 {
    Vec2::new(
        window.resolution.physical_width() as f32,
        window.resolution.physical_height() as f32,
    )
}
