use bevy::{
    app::{App, Plugin},
    pbr::MaterialPlugin,
};

pub mod line;

use crate::renderer::line::LineMaterial;

pub struct RendererPlugin;
impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<LineMaterial>::default());
    }
}
