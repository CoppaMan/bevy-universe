use bevy::{
    ecs::{entity::Entity, system::Commands},
    hierarchy::BuildChildren,
    render::color::Color,
    ui::{node_bundles::NodeBundle, AlignItems, FlexDirection, JustifyContent, Style, UiRect, Val},
};

pub struct UiContainerBuilder;
impl UiContainerBuilder {
    pub fn build(commands: &mut Commands, direction: FlexDirection, contents: &[Entity]) -> Entity {
        let style = Style {
            flex_grow: 2.,
            flex_direction: direction,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::FlexStart,
            padding: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        };

        if [FlexDirection::Column, FlexDirection::ColumnReverse].contains(&direction) {
        } else if [FlexDirection::Row, FlexDirection::RowReverse].contains(&direction) {
        }

        let content = commands
            .spawn(NodeBundle {
                style: style,
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..Default::default()
            })
            .id();

        commands.entity(content).push_children(contents);

        content
    }
}
