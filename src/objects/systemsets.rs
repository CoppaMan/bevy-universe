use bevy::ecs::schedule::SystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ObjectSets {
    SpawnPlanet,
    SpawnCraft,
    SpawnCamera,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CameraSets {
    CameraAll,
    MoveCamera,
    TrackFocus,
}
