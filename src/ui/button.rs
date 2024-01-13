use std::marker::PhantomData;

use bevy::{
    asset::AssetServer,
    ecs::{
        bundle::Bundle,
        entity::Entity,
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    math::Vec2,
    render::color::Color,
    text::TextStyle,
    ui::{
        node_bundles::{ButtonBundle, TextBundle},
        AlignItems, JustifyContent, Style, Val,
    },
};

pub struct UiButtonStyle {
    pub button_background_color: Color,
    pub fixed_size: Option<Vec2>,
}
impl Default for UiButtonStyle {
    fn default() -> Self {
        UiButtonStyle {
            button_background_color: Color::rgb(0.5, 0.5, 0.5),
            fixed_size: None,
        }
    }
}

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
        style: UiButtonStyle,
    ) -> Entity {
        let mut button_style = Style {
            //width: Val::Px(40.0),
            //height: Val::Px(20.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        };

        if style.fixed_size.is_some() {
            let size = style.fixed_size.expect("");
            button_style.width = Val::Px(size.x);
            button_style.height = Val::Px(size.y);
        }

        let button = commands
            .spawn((
                ButtonBundle {
                    style: button_style,
                    background_color: style.button_background_color.into(),
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
