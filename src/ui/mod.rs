use bevy::app::{PluginGroup, PluginGroupBuilder};

use simspeed::UiSimSpeedPlugin;

mod simspeed;

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(UiSimSpeedPlugin)
    }
}
