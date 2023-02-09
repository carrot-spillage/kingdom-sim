use std::ops::{Range, RangeBounds};

use bevy::prelude::{Component, Entity};
use bevy_turborand::prelude::*;

#[derive(Component)]
pub struct CreationProgress(pub f32);

#[derive(Component)]
pub struct NeedsDestroying;

/**
 * Means player's workers are NOT allowed to claim this entity for any other task.
 */
#[derive(Component)]
pub struct ClaimedBy(pub Entity);

#[derive(Component, Clone, Copy, Debug)]
pub struct SimpleDestructible {
    pub current_health: f32,
    pub max_health: f32,
}

#[derive(Clone, Copy, Debug)]

pub struct Countdown {
    initial_value: usize,
    current_value: usize,
}

#[derive(Clone, Debug)]
pub struct VariableCountdown {
    range: Range<usize>,
    current_value: usize,
    pristine: bool,
}

impl VariableCountdown {
    pub fn new(range: Range<usize>) -> Self {
        if range.start < 1 {
            panic!("Countdown range must have values above zero");
        }

        Self {
            range,
            current_value: 0,
            pristine: true,
        }
    }

    pub fn tick_yield(&mut self, rng: &mut RngComponent) -> bool {
        if self.current_value > 0 {
            self.current_value -= 1;
            return false;
        } else {
            let initial_value = rng.usize(self.range.clone());
            self.current_value = initial_value;
            if self.pristine {
                self.pristine = false;
                self.current_value -= 1;
                return false;
            }
            return true;
        }
    }
}

impl Countdown {
    pub fn new(initial_value: usize) -> Self {
        Self {
            initial_value,
            current_value: initial_value,
        }
    }

    pub fn tick(&mut self) {
        if self.is_done() {
            self.current_value = self.initial_value
        } else {
            self.current_value -= 1
        }
    }

    pub fn is_done(&self) -> bool {
        self.current_value == 0
    }
}
