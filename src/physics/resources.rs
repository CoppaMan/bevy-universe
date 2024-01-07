use bevy::ecs::system::Resource;

/// Determines how many times the physics schedules are run per frame
#[derive(Resource)]
pub struct PhysicsTimeScale(pub u16);

/// Scales the timestep duration by integer multiple.
/// Too large values might cause instabilities during the integration step.
#[derive(Resource)]
pub struct PhysicsStepScale(pub u16);
