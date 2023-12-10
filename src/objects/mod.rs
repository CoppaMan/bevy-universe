use self::{camera::SpawnCameraPlugin, craft::SpawnCraftPlugin, planet::SpawnPlanetsPlugin};

pub mod components;

mod camera;
mod craft;
mod planet;

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct LoadObjectsPlugins;

impl PluginGroup for LoadObjectsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SpawnCraftPlugin)
            .add(SpawnPlanetsPlugin)
            .add(SpawnCameraPlugin)
    }
}
