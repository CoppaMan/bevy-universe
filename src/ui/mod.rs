use bevy::app::{PluginGroup, PluginGroupBuilder};

use simspeed::UiSimSpeedPlugin;

use self::{clock::UiClockPlugin, window::UiWindowPlugin};

pub mod resources;
pub mod systemsets;

mod button;
mod clock;
mod container;
mod simspeed;
mod window;

pub struct UiPlugins;
impl PluginGroup for UiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(UiWindowPlugin)
            .add(UiSimSpeedPlugin)
            .add(UiClockPlugin)
    }
}
