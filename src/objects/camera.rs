use bevy::{
    app::{App, Plugin, Startup, Update},
    core_pipeline::{clear_color::ClearColorConfig, core_3d::Camera3dBundle},
    ecs::{
        event::*,
        system::{Commands, Query, Res},
    },
    input::{mouse::*, Input},
    math::{DVec3, Mat3, Quat, Vec2, Vec3},
    prelude::*,
    transform::components::Transform,
};

use crate::{
    floatingorigin::{components::FloatingOriginPosition, systemsets::FloatingOriginSet},
    objects::{
        components::{FocusTarget, Focusable},
        planet::Planet,
        systemsets::{CameraSets, ObjectSets},
    },
    physics::systemsets::PhysicsSet,
};

pub struct SpawnCameraPlugin;

impl Plugin for SpawnCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            spawn_camera
                .in_set(ObjectSets::SpawnCamera)
                .after(ObjectSets::SpawnPlanet),
        )
        .add_systems(
            Update,
            (
                (pan_orbit_camera, change_camera_focus)
                    .in_set(CameraSets::MoveCamera)
                    .before(PhysicsSet::All),
                track_camera_focus
                    .in_set(CameraSets::TrackFocus)
                    .after(PhysicsSet::All)
                    .before(FloatingOriginSet::ApplyTransform),
            )
                .in_set(CameraSets::CameraAll),
        );
    }
}

fn spawn_camera(mut commands: Commands, planets: Query<Entity, With<Planet>>) {
    info!("Spawning camera");
    let camera_start_pos = DVec3::new(-100000000.0, 0.0, 0.0);

    // Focus on first parent
    let mut focus = Entity::PLACEHOLDER;
    for planet in planets.iter() {
        info!("Attached camera to planet {:?}", planet);
        focus = planet;
        break;
    }

    commands.spawn((
        (Camera3dBundle {
            transform: Transform::IDENTITY.looking_at(Vec3::new(1., 0., 0.), Vec3::Z),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 1.0472,
                ..Default::default()
            }),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..Default::default()
            },
            ..Default::default()
        },),
        FloatingOriginPosition(camera_start_pos),
        FocusTarget {
            target: focus,
            distance: camera_start_pos,
        },
    ));
}

fn track_camera_focus(
    mut camera_q: Query<(&mut FloatingOriginPosition, &FocusTarget), With<Camera>>,
    focus_targets: Query<&FloatingOriginPosition, (With<Focusable>, Without<Camera>)>,
) {
    let (mut camera_origin, camera_target) = camera_q.single_mut();
    let target_origin = focus_targets.get(camera_target.target).expect("");
    camera_origin.0 = target_origin.0 + camera_target.distance;
}

fn change_camera_focus(
    win_q: Query<&Window>,
    input_mouse: Res<Input<MouseButton>>,
    focus: Query<
        (
            Entity,
            &GlobalTransform,
            &Focusable,
            &FloatingOriginPosition,
        ),
        With<Focusable>,
    >,
    mut camera_q: Query<
        (
            &GlobalTransform,
            &Camera,
            &mut Transform,
            &mut FocusTarget,
            &FloatingOriginPosition,
        ),
        With<Camera3d>,
    >,
) {
    if input_mouse.just_released(MouseButton::Left) {
        info!("Checking for new focus for camera");
        let window = win_q.get_single().expect("");
        let (camera_transform, camera, mut cam_transform_mut, mut focus_entity, camera_origin) =
            camera_q.get_single_mut().expect("");

        let mut closest_id = Entity::PLACEHOLDER;
        let mut closest_distance = f32::MAX;
        let mut closest_transform = GlobalTransform::IDENTITY;
        let mut closest_origin = DVec3::ZERO;
        for (entity_id, focus_transform, focus_foc, target_origin) in focus.iter() {
            //info!("Test for intersection with {:?}", entity_id);
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
                closest_origin = target_origin.0;
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

            let new_distance = camera_origin.0 - closest_origin;
            focus_entity.target = closest_id;
            focus_entity.distance = new_distance;
            cam_transform_mut.look_at(-new_distance.as_vec3(), Vec3::Z);
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
    mut camera_q: Query<
        (
            &mut Transform,
            &mut FloatingOriginPosition,
            &mut FocusTarget,
        ),
        With<Camera>,
    >,
    focusable_q: Query<(&Focusable, &FloatingOriginPosition), Without<Camera>>,
) {
    //info!("pan_orbit_camera");

    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;

    // Get panning vector
    let mut rotation_move = Vec2::ZERO;
    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.read() {
            rotation_move += ev.delta;
        }
    }

    // Get scroll amount
    let mut scroll_move = 0.0;
    for ev in ev_scroll.read() {
        scroll_move += ev.y as f64;
    }

    for (mut camera_transform, mut camera_origin, mut focus_target) in camera_q.iter_mut() {
        let up: Vec3 = camera_transform.rotation * Vec3::Z;
        let (_, focus_origin) = focusable_q.get(focus_target.target).expect("");

        let mut distance = (focus_origin.0 - camera_origin.0).length();
        let mut any = false;

        // Panning
        if rotation_move.length_squared() > 0.0 {
            any = true;

            let win = win_q.get_single().expect("");
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
            camera_transform.rotation = roll * camera_transform.rotation; // rotate around global y axis
            camera_transform.rotation = camera_transform.rotation * pitch; // rotate around local x axis

        // Zooming
        } else if scroll_move.abs() > 0.0 {
            any = true;
            distance = distance - scroll_move * distance * 0.2;
            info!("Zoom distance: {}", distance);
        }

        if any {
            // Compute new origin position after rotation
            let rot_matrix = Mat3::from_quat(camera_transform.rotation).as_dmat3();
            let new_pos_rel = rot_matrix.mul_vec3(DVec3::new(0.0, 0.0, distance));
            let new_pos_abs = new_pos_rel + focus_origin.0;

            // Check if the new position lies within any minimum focus radius
            let mut collides = false;
            for (candidate_focus, candidate_origin) in focusable_q.iter() {
                if (new_pos_abs - candidate_origin.0).length() < candidate_focus.focus_min_distance
                {
                    collides = true;
                }
            }

            if !collides {
                camera_origin.0 = new_pos_abs;
                focus_target.distance = new_pos_rel;
            }
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
