pub mod components;
pub mod systemsets;

mod camera;
mod craft;
mod planet;

use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::objects::{
    camera::SpawnCameraPlugin, craft::SpawnCraftPlugin, planet::SpawnPlanetsPlugin,
};

pub struct LoadObjectsPlugins;
impl PluginGroup for LoadObjectsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SpawnPlanetsPlugin)
            .add(SpawnCameraPlugin)
            .add(SpawnCraftPlugin)
    }
}
