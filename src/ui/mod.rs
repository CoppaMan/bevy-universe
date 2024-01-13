use bevy::{
    app::{App, Plugin, PluginGroup, PluginGroupBuilder, Update},
    ecs::{
        schedule::{IntoSystemConfigs, IntoSystemSetConfigs},
        system::ResMut,
    },
};

use simspeed::UiSimSpeedPlugin;

use crate::objects::systemsets::CameraSets;

use self::{
    button::set_button_ui_click,
    clock::UiClockPlugin,
    resources::UiClicked,
    systemsets::UiSets,
    window::{move_window, set_window_ui_click, toggle_hide_window},
};

pub mod resources;
pub mod systemsets;

mod button;
mod clock;
mod container;
mod simspeed;
mod window;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiClicked(false))
            .configure_sets(Update, UiSets::UiAll.before(CameraSets::CameraAll))
            .add_systems(
                Update,
                (
                    null_clicked,
                    (set_window_ui_click, set_button_ui_click, move_window),
                    toggle_hide_window,
                )
                    .chain()
                    .in_set(UiSets::UiAll),
            );
    }
}

pub struct UiPlugins;
impl PluginGroup for UiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(UiPlugin)
            .add(UiSimSpeedPlugin)
            .add(UiClockPlugin)
    }
}

fn null_clicked(mut clicked: ResMut<UiClicked>) {
    clicked.0 = false;
}
