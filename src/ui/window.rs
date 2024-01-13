use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::Entity,
        query::{Changed, With},
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, Parent},
    log::info,
    math::Vec2,
    render::color::Color,
    text::TextStyle,
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        AlignItems, FlexDirection, Interaction, JustifyContent, Style, UiRect, Val,
    },
    window::{PrimaryWindow, Window},
};

use crate::objects::systemsets::CameraSets;

use super::{
    button::{UiButtonBuilder, UiButtonStyle},
    resources::UiClicked,
    systemsets::UiSets,
};

pub struct UiWindowPlugin;
impl Plugin for UiWindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiClicked(false)).add_systems(
            Update,
            (move_window, set_ui_click)
                .in_set(UiSets::UiAll)
                .before(CameraSets::CameraAll),
        );
    }
}

#[derive(Component)]
pub struct UiWindowFrame {
    move_offset: Vec2,
}

#[derive(Component)]
pub struct UiWindowHeader;

#[derive(Component)]
pub struct HideUiWindow;

pub struct UiWindowBuilder;
impl UiWindowBuilder {
    pub fn build(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        title: String,
        content: Entity,
        start: (f32, f32),
    ) -> Entity {
        let hide_button = UiButtonBuilder::build(
            commands,
            asset_server,
            HideUiWindow,
            "_".into(),
            UiButtonStyle {
                fixed_size: Some(Vec2::new(20.0, 20.0)),
                button_background_color: Color::DARK_GRAY,
                ..Default::default()
            },
        );

        let mut header = Entity::PLACEHOLDER;
        let mut content_holder = Entity::PLACEHOLDER;
        let frame = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        left: Val::Px(start.0),
                        top: Val::Px(start.1),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    background_color: Color::rgb_u8(191, 113, 17).into(),
                    ..Default::default()
                },
                UiWindowFrame {
                    move_offset: Vec2::ZERO,
                },
                Interaction::None,
            ))
            .with_children(|frame_parent| {
                header = frame_parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                flex_grow: 2.0,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        Interaction::None,
                        UiWindowHeader,
                    ))
                    .with_children(|header_parent| {
                        header_parent.spawn(
                            TextBundle::from_section(
                                title,
                                TextStyle {
                                    font: asset_server.load("fonts/Consolas.ttf"),
                                    font_size: 15.0,
                                    ..Default::default()
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::right(Val::Px(10.0)),
                                ..Default::default()
                            }),
                        );
                    })
                    .id();
                content_holder = frame_parent
                    .spawn(NodeBundle {
                        style: Style {
                            margin: UiRect::new(
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(0.0),
                                Val::Px(2.0),
                            ),
                            ..Default::default()
                        },
                        background_color: Color::rgb_u8(0, 113, 17).into(),
                        ..Default::default()
                    })
                    .id();
            })
            .id();

        commands.entity(header).push_children(&[hide_button]);
        commands.entity(content_holder).push_children(&[content]);

        frame
    }
}

fn set_ui_click(
    interaction_query: Query<&Interaction, With<UiWindowFrame>>,
    mut clicked: ResMut<UiClicked>,
) {
    for interaction in interaction_query.iter() {
        info!("Interaction: {:?}", interaction);
        match *interaction {
            Interaction::Pressed => {
                clicked.0 = true;
                return;
            }
            Interaction::Hovered => {
                clicked.0 = true;
                return;
            }
            Interaction::None => {}
        }
    }
    clicked.0 = false;
}

fn move_window(
    mut interaction_query: Query<(&Parent, &Interaction), With<UiWindowHeader>>,
    mut interaction_changed: Query<
        (&Parent, &Interaction),
        (With<UiWindowHeader>, Changed<Interaction>),
    >,
    mut window_q: Query<(&mut Style, &mut UiWindowFrame)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    for (parent_frame, interaction) in interaction_changed.iter_mut() {
        let (frame, mut win) = window_q.get_mut(parent_frame.get()).expect("");

        match *interaction {
            Interaction::Pressed => {
                if let Some(position) = q_windows.single().cursor_position() {
                    let left = frame.left.resolve(0.0, Vec2::ZERO).expect("");
                    let top = frame.top.resolve(0.0, Vec2::ZERO).expect("");
                    win.move_offset = Vec2::new(position.x - left, position.y - top);
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }

    for (parent_frame, interaction) in interaction_query.iter_mut() {
        let (mut frame, win) = window_q.get_mut(parent_frame.get()).expect("");

        match *interaction {
            Interaction::Pressed => {
                if let Some(position) = q_windows.single().cursor_position() {
                    frame.left = Val::Px(position.x - win.move_offset.x);
                    frame.top = Val::Px(position.y - win.move_offset.y);
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
