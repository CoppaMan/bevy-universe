use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        query::{Changed, With, Without},
        system::{Commands, Query, Res, ResMut},
    },
    log::info,
    text::Text,
    text::TextStyle,
    ui::{node_bundles::TextBundle, FlexDirection, Interaction, Style, UiRect, Val},
};

use crate::physics::resources::PhysicsTimeScale;

use super::{
    button::{UiButtonBuilder, UiButtonStyle},
    container::UiContainerBuilder,
    window::UiWindowBuilder,
};

#[derive(Component)]
struct SimSpeedChange(u16);

#[derive(Component)]
struct SimSpeedDisplay;

pub struct UiSimSpeedPlugin;

impl Plugin for UiSimSpeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_ui)
            .add_systems(Update, change_speed);
    }
}

pub fn build_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_1 = UiButtonBuilder::build(
        &mut commands,
        &asset_server,
        SimSpeedChange(1),
        "+1x".into(),
        UiButtonStyle::default(),
    );

    let button_10 = UiButtonBuilder::build(
        &mut commands,
        &asset_server,
        SimSpeedChange(10),
        "+10x".into(),
        UiButtonStyle::default(),
    );

    let button_100 = UiButtonBuilder::build(
        &mut commands,
        &asset_server,
        SimSpeedChange(100),
        "+100x".into(),
        UiButtonStyle::default(),
    );

    let button_container = UiContainerBuilder::build(
        &mut commands,
        FlexDirection::Row,
        &[button_1, button_10, button_100],
    );

    let speed = commands
        .spawn((
            TextBundle::from_section(
                "1x",
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
            SimSpeedDisplay,
        ))
        .id();

    let container = UiContainerBuilder::build(
        &mut commands,
        FlexDirection::Row,
        &[speed, button_container],
    );

    UiWindowBuilder::build(
        &mut commands,
        &asset_server,
        "Simulation speed".into(),
        container,
        (10.0, 10.0),
    );
}

fn change_speed(
    interaction_query: Query<
        (&Interaction, &SimSpeedChange),
        (Changed<Interaction>, With<SimSpeedChange>),
    >,
    mut display: Query<&mut Text, (With<SimSpeedDisplay>, Without<SimSpeedChange>)>,
    mut speed_scale: ResMut<PhysicsTimeScale>,
) {
    for (interaction, speed_change) in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                info!("Increase timescale by {}", speed_change.0);
                speed_scale.0 += speed_change.0;
                display.get_single_mut().expect("").sections[0].value =
                    speed_scale.0.to_string() + "x";
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
