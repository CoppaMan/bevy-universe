use bevy::app::{PluginGroup, PluginGroupBuilder};

use self::history::OrbitHistoryPlugin;

pub mod components;
pub mod systemsets;

mod history;

pub struct OrbitsPlugins;
impl PluginGroup for OrbitsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(OrbitHistoryPlugin)
    }
}
