use bevy::{
    app::{App, Plugin, PreUpdate},
    ecs::{
        schedule::{IntoSystemConfigs, Schedule, ScheduleLabel},
        system::Resource,
        world::World,
    },
    //log::{debug, info},
};

use crate::physics::{integrator::integrate_time, nbody::nbody_accelerate};

pub mod components;
mod integrator;
mod nbody;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct PhysicsSchedule;

// Decides how often to advance the simulation per frame
#[derive(Resource)]
pub struct PhysicsTimeScale(pub u16);

// Scales the time steps
#[derive(Resource)]
pub struct PhysicsStepScale(pub u16);

pub struct PhysicPlugin;
impl Plugin for PhysicPlugin {
    fn build(&self, app: &mut App) {
        let mut schedule = Schedule::new(PhysicsSchedule);
        schedule.add_systems((nbody_accelerate, integrate_time).chain());

        fn run_physics_schedule(world: &mut World) {
            let iterations = match world.get_resource::<PhysicsTimeScale>() {
                None => 1 as u16,
                Some(s) => s.0,
            };

            for _ in 0..iterations {
                world.run_schedule(PhysicsSchedule);
            }
        }

        app.insert_resource(PhysicsTimeScale(1))
            .insert_resource(PhysicsStepScale(1))
            .add_schedule(schedule)
            .add_systems(PreUpdate, run_physics_schedule);
    }
}
