use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::Entity,
        query::{Changed, With},
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, Parent},
    math::Vec2,
    render::color::Color,
    text::TextStyle,
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        AlignItems, Display, FlexDirection, Interaction, JustifyContent, Style, UiRect, Val,
    },
    window::{PrimaryWindow, Window},
};

use super::{
    button::{UiButtonBuilder, UiButtonStyle},
    resources::UiClicked,
};

#[derive(Component)]
pub struct UiWindowFrame {
    move_offset: Vec2,
}

#[derive(Component)]
pub struct UiWindowHeader;

#[derive(Component)]
pub struct UiWindowContent;

#[derive(Component)]
pub struct ToggleUiWindow {
    content: Entity,
}

pub struct UiWindowBuilder;
impl UiWindowBuilder {
    pub fn build(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        title: String,
        content: Entity,
        start: (f32, f32),
    ) -> Entity {
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
                    .spawn((
                        NodeBundle {
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
                        },
                        UiWindowContent,
                    ))
                    .id();
            })
            .id();

        let hide_button = UiButtonBuilder::build(
            commands,
            asset_server,
            ToggleUiWindow {
                content: content_holder,
            },
            "_".into(),
            UiButtonStyle {
                fixed_size: Some(Vec2::new(20.0, 20.0)),
                button_background_color: Color::DARK_GRAY,
                ..Default::default()
            },
        );

        commands.entity(header).push_children(&[hide_button]);
        commands.entity(content_holder).push_children(&[content]);

        frame
    }
}

pub fn set_window_ui_click(
    interaction_query: Query<&Interaction, With<UiWindowFrame>>,
    mut clicked: ResMut<UiClicked>,
) {
    if clicked.0 {
        return;
    }

    for interaction in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed | Interaction::Hovered => {
                clicked.0 = true;
                return;
            }
            _ => {}
        }
    }
    clicked.0 = false;
}

pub fn move_window(
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
            _ => {}
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
            _ => {}
        }
    }
}

pub fn toggle_hide_window(
    interaction_query: Query<
        (&Interaction, &ToggleUiWindow),
        (Changed<Interaction>, With<ToggleUiWindow>),
    >,
    mut windows_content: Query<&mut Style, With<UiWindowContent>>,
) {
    for (interaction, hide_window) in interaction_query.iter() {
        let window_entity = match *interaction {
            Interaction::Pressed => hide_window.content,
            _ => {
                continue;
            }
        };
        let mut content = windows_content.get_mut(window_entity).expect("");
        if content.display == Display::None {
            content.display = Display::Flex;
        } else {
            content.display = Display::None;
        }
    }
}
