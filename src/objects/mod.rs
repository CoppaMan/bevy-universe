pub mod components;
pub mod systemsets;

pub mod planet;

mod camera;
mod craft;

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
