use bevy::{
    asset::AssetServer,
    ecs::{
        entity::Entity,
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    render::color::Color,
    text::TextStyle,
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        FlexDirection, Style, UiRect, Val,
    },
};

pub struct UiWindowBuilder;
impl UiWindowBuilder {
    pub fn build(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        title: String,
        content: Entity,
    ) -> Entity {
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
                    title,
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

        commands
            .entity(frame)
            .push_children(&[window_title, content]);

        frame
    }
}
