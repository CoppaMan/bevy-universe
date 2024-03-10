pub mod bundles;
pub mod components;
pub mod spawn;

mod parsers;
mod rotation;

use bevy::app::Update;

use bevy::{
    app::{App, Plugin, Startup},
    ecs::schedule::{apply_deferred, IntoSystemConfigs},
};

use self::{rotation::rotate_planets, spawn::spawn_planets};

use super::systemsets::ObjectSets;

pub struct SpawnPlanetsPlugin;
impl Plugin for SpawnPlanetsPlugin {
    fn build(&self, app: &mut App) {
        // Apply deferred to ensure planets have been created
        app.add_systems(
            Startup,
            (spawn_planets, apply_deferred)
                .chain()
                .in_set(ObjectSets::SpawnPlanet),
        )
        .add_systems(Update, rotate_planets);
    }
}
