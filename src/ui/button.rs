use std::marker::PhantomData;

use bevy::{
    asset::AssetServer,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::BuildChildren,
    math::Vec2,
    render::color::Color,
    text::TextStyle,
    ui::{
        node_bundles::{ButtonBundle, TextBundle},
        AlignItems, Interaction, JustifyContent, Style, Val,
    },
};

use super::resources::UiClicked;

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

#[derive(Component)]
pub struct UiButton;

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
                UiButton,
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

pub fn set_button_ui_click(
    interaction_query: Query<&Interaction, With<UiButton>>,
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
