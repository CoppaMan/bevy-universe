use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        query::{Changed, With, Without},
        system::{Commands, Query, Res, ResMut},
    },
    log::info,
    prelude::BuildChildren,
    render::color::Color,
    text::Text,
    text::TextStyle,
    ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        AlignItems, FlexDirection, Interaction, JustifyContent, Style, UiRect, Val,
    },
};

use crate::physics::PhysicsTimeScale;

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
    let frame = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(200.),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(3.)),
                ..Default::default()
            },
            background_color: Color::rgb_u8(191, 113, 17).into(),
            ..Default::default()
        })
        .id();

    let window_title = commands
        .spawn(
            TextBundle::from_section(
                "Simulation speed",
                TextStyle {
                    font: asset_server.load("fonts/Consolas.ttf"),
                    font_size: 15.0,
                    ..Default::default()
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(3.)),
                ..Default::default()
            }),
        )
        .id();

    let content = commands
        .spawn(NodeBundle {
            style: Style {
                flex_grow: 2.,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexStart,
                padding: UiRect::all(Val::Px(5.0)),
                ..Default::default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..Default::default()
        })
        .id();

    commands
        .entity(frame)
        .push_children(&[window_title, content]);

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

    let buttons = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            background_color: Color::rgb(0.05, 0.05, 0.05).into(),
            ..Default::default()
        })
        .id();

    commands.entity(content).push_children(&[speed, buttons]);

    let one_times = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(40.0),
                    height: Val::Px(20.0),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    margin: UiRect::right(Val::Px(5.)),
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::rgb(0.5, 0.5, 0.5).into(),
                ..Default::default()
            },
            SimSpeedChange(1),
        ))
        .id();

    let one_times_text = commands
        .spawn(TextBundle::from_section(
            "+1x",
            TextStyle {
                font: asset_server.load("fonts/Consolas.ttf"),
                font_size: 15.0,
                ..Default::default()
            },
        ))
        .id();
    commands.entity(one_times).push_children(&[one_times_text]);

    let ten_times = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(40.0),
                    height: Val::Px(20.0),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    margin: UiRect::right(Val::Px(5.)),
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::rgb(0.5, 0.5, 0.5).into(),
                ..Default::default()
            },
            SimSpeedChange(10),
        ))
        .id();

    let ten_times_text = commands
        .spawn(TextBundle::from_section(
            "+10x",
            TextStyle {
                font: asset_server.load("fonts/Consolas.ttf"),
                font_size: 15.0,
                ..Default::default()
            },
        ))
        .id();
    commands.entity(ten_times).push_children(&[ten_times_text]);

    let hundred_times = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(40.0),
                    height: Val::Px(20.0),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::rgb(0.5, 0.5, 0.5).into(),
                ..Default::default()
            },
            SimSpeedChange(100),
        ))
        .id();

    let hundred_times_text = commands
        .spawn(TextBundle::from_section(
            "+100x",
            TextStyle {
                font: asset_server.load("fonts/Consolas.ttf"),
                font_size: 15.0,
                ..Default::default()
            },
        ))
        .id();
    commands
        .entity(hundred_times)
        .push_children(&[hundred_times_text]);

    commands
        .entity(buttons)
        .push_children(&[one_times, ten_times, hundred_times]);
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
