use std::collections::VecDeque;

use bevy::{ecs::bundle::Bundle, math::DVec3};

use super::components::{FloatingOriginHistory, FloatingOriginPosition};

#[derive(Bundle)]
pub struct FloatingOriginBundle {
    position: FloatingOriginPosition,
}
impl FloatingOriginBundle {
    pub fn new(starting_position: &DVec3) -> FloatingOriginBundle {
        FloatingOriginBundle {
            position: FloatingOriginPosition(*starting_position),
        }
    }
}

#[derive(Bundle)]
pub struct FloatingOriginWithHistoryBundle {
    history: FloatingOriginHistory,
    position: FloatingOriginBundle,
}
impl FloatingOriginWithHistoryBundle {
    pub fn new(starting_position: &DVec3) -> FloatingOriginWithHistoryBundle {
        FloatingOriginWithHistoryBundle {
            history: FloatingOriginHistory(VecDeque::from([*starting_position])),
            position: FloatingOriginBundle::new(starting_position),
        }
    }
}
