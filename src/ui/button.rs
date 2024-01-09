use std::marker::PhantomData;

use bevy::{
    asset::AssetServer,
    ecs::{
        bundle::Bundle,
        entity::Entity,
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    render::color::Color,
    text::TextStyle,
    ui::{
        node_bundles::{ButtonBundle, TextBundle},
        AlignItems, JustifyContent, Style, Val,
    },
};

pub struct UiButtonBuilder<BundleTrait> {
    phantom: PhantomData<BundleTrait>,
}
impl<BundleTrait> UiButtonBuilder<BundleTrait>
where
    BundleTrait: Bundle,
{
    pub fn build(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        action: BundleTrait,
        label: String,
    ) -> Entity {
        let button = commands
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
                action,
            ))
            .id();

        let label = commands
            .spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: asset_server.load("fonts/Consolas.ttf"),
                    font_size: 15.0,
                    ..Default::default()
                },
            ))
            .id();
        commands.entity(button).push_children(&[label]);

        button
    }
}
