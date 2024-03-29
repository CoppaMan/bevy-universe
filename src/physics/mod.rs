use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        schedule::{IntoSystemConfigs, IntoSystemSetConfigs, Schedule, ScheduleLabel},
        world::World,
    },
};

use crate::physics::{
    integrator::integrate_time,
    nbody::nbody_accelerate,
    resources::{PhysicsStepScale, PhysicsTimeScale},
    systemsets::PhysicsSet,
};

// Only expose components to world for queries
pub mod bundles;
pub mod components;
pub mod resources;
pub mod systemsets;

// Keep the rest in module only
mod integrator;
mod nbody;

/// Schedule contining all physics related systems
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct PhysicsSchedule;

/// Plugin initializing the physics systems.
/// PhysicsTimeScale and PhysicsStepScale are both initialized to 1.
pub struct PhysicPlugin;
impl Plugin for PhysicPlugin {
    fn build(&self, app: &mut App) {
        // Create the Physics schedule containing all of our physics systems
        let mut schedule = Schedule::new(PhysicsSchedule);
        schedule
            .add_systems((
                nbody_accelerate.in_set(PhysicsSet::Forces),
                integrate_time.in_set(PhysicsSet::Integration),
            ))
            .configure_sets(PhysicsSet::Integration.after(PhysicsSet::Forces));

        // Run the Physics schedule
        fn run_physics_schedule(world: &mut World) {
            let iterations = match world.get_resource::<PhysicsTimeScale>() {
                None => 1 as u16,
                Some(s) => s.0,
            };

            // Repeat to advance simulation more than once per frame
            for _ in 0..iterations {
                world.run_schedule(PhysicsSchedule);
            }
        }

        // Build plugin
        app.insert_resource(PhysicsTimeScale(1))
            .insert_resource(PhysicsStepScale(1))
            .add_schedule(schedule)
            .add_systems(Update, run_physics_schedule.in_set(PhysicsSet::All));
    }
}
