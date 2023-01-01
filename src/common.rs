use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct CreationProgress(pub f32);

/**
 * Means player's workers are NOT allowed to claim this entity for any other task.
 */
#[derive(Component)]
pub struct ClaimedBy(pub Entity);

#[derive(Clone, Copy)]

pub struct Countdown {
    pub initial_value: usize,
    current_value: usize,
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
