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

    for (mut pan_orbit, mut transform, _) in query.iter_mut() {
        let up = transform.rotation * Vec3::Z;

        let mut any = false;
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
