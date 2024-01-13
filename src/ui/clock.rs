use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        system::{Commands, Query, Res},
    },
    text::Text,
    text::TextStyle,
    time::Time,
    ui::{node_bundles::TextBundle, FlexDirection, Style, UiRect, Val},
};

use crate::physics::resources::{PhysicsStepScale, PhysicsTimeScale};

use super::{container::UiContainerBuilder, window::UiWindowBuilder};

const SEC_PER_DAY: f64 = 86400.0;
const SEC_PER_HOUR: f64 = 3600.0;
const SEC_PER_MIN: f64 = 60.0;

#[derive(Component)]
struct SimSpeedChange(u16);

#[derive(Component)]
struct Clock(f64);

pub struct UiClockPlugin;

impl Plugin for UiClockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_ui)
            .add_systems(Update, update_time);
    }
}

pub fn build_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let time_display = commands
        .spawn((
            TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/Consolas.ttf"),
                    font_size: 20.0,
                    ..Default::default()
                },
            )
            .with_style(Style {
                margin: UiRect::right(Val::Px(5.)),
                ..Default::default()
            }),
            Clock(0.0),
        ))
        .id();

    let container = UiContainerBuilder::build(&mut commands, FlexDirection::Row, &[time_display]);

    UiWindowBuilder::build(
        &mut commands,
        &asset_server,
        "Clock".into(),
        container,
        (10.0, 300.0),
    );
}

fn update_time(
    mut display: Query<(&mut Clock, &mut Text)>,
    speed_scale: Res<PhysicsTimeScale>,
    step_scale: Res<PhysicsStepScale>,
    time: Res<Time>,
) {
    let time_passed = time.delta_seconds_f64() * (speed_scale.0 * step_scale.0) as f64;
    let (mut clock, mut text) = display.get_single_mut().expect("");
    clock.0 += time_passed;

    let mut secs = clock.0;
    let days = (secs / SEC_PER_DAY).floor();
    secs -= days * SEC_PER_DAY;

    let hours = (secs / SEC_PER_HOUR).floor();
    secs -= hours * SEC_PER_HOUR;

    let mins = (secs / SEC_PER_MIN).floor();
    secs -= mins * SEC_PER_MIN;

    secs = secs.floor();

    text.sections[0].value = format!("{days}:{hours}:{mins}:{secs}");
}
