use bevy::{
    app::{App, Plugin, Startup, Update},
    core_pipeline::clear_color::ClearColorConfig,
    core_pipeline::core_3d::Camera3dBundle,
    ecs::{
        event::*,
        system::{Commands, Query, Res},
    },
    input::{mouse::*, Input},
    math::Mat3,
    math::{Quat, Vec2, Vec3},
    prelude::*,
    transform::components::Transform,
};

use super::components::FocusSphere;

pub struct SpawnCameraPlugin;

impl Plugin for SpawnCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, pan_orbit_camera);
    }
}

#[derive(Component)]
struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

fn spawn_camera(mut commands: Commands) {
    let camera_start_pos = Vec3::new(-20000000.0, 0.0, 0.0);

    commands.spawn((
        Camera3dBundle {
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
        },
        PanOrbitCamera {
            radius: camera_start_pos.length(),
            focus: Vec3::ZERO,
            upside_down: false,
        },
    ));
}

/*
fn focus_object(
    win_q: Query<&Window>,
    input_mouse: Res<Input<MouseButton>>,
    focus: Query<&Transform, With<FocusSphere>>,
    mut camera_q: Query<(&mut PanOrbitCamera, &GlobalTransform, &Camera), With<Camera3d>>,
) {
    if input_mouse.pressed(MouseButton::Left) {
        let window = win_q.get_single().expect("");
        let (mut camera_orbit, camera_transform, camera) = camera_q.get_single().expect("");
        for mut focus_transform in focus.iter() {
            let ray = camera
                .viewport_to_world(camera_transform, window.cursor_position().expect(""))
                .expect("");
        }
    }
}
*/

fn pan_orbit_camera(
    win_q: Query<&Window>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Middle;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;
    let win = win_q.get_single().expect("");

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.read() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.read() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.read() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, _) in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Z;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(win);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_z(delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            /*
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(win);
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
            */
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
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
