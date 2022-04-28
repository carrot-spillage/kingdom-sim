use bevy::{
    math::Vec3,
    prelude::{App, Commands, Component, Entity, EventWriter, Plugin, Query, SystemSet, Transform},
};

use crate::GameState;

#[derive(Clone, Copy)]
pub enum ArraivalHandlerData {
    Job(Entity),
}

#[derive(Component)]
pub struct MovingToPosition {
    pub position: Vec3,
    pub sufficient_range: f32,
    pub arrival_handler_data: ArraivalHandlerData,
}

#[derive(Component)]
pub struct Walker {
    pub max_speed: f32,
    pub current_speed: f32,
    pub acceleration: f32,
}

impl Walker {
    pub fn walk(&mut self) {
        if self.current_speed < self.max_speed {
            self.current_speed += self.acceleration;
        }
    }

    pub fn stop(&mut self) {
        self.current_speed = 0.0
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Position(pub Vec3);

pub struct MovementPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ArraivalHandlerData>()
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_to_position));
    }
}

fn move_to_position(
    mut moving: Query<(Entity, &mut Walker, &MovingToPosition)>,
    mut positions: Query<(&mut Position, &mut Transform)>,
    mut commands: Commands,
    mut ev_arrival: EventWriter<ArraivalHandlerData>,
) {
    for (entity_id, mut walker, moving_to_position) in moving.iter_mut() {
        let (mut this_pos_res, mut this_transform) = positions.get_mut(entity_id).unwrap();

        let distance = this_pos_res.0.distance(moving_to_position.position);
        if distance > moving_to_position.sufficient_range {
            this_pos_res.0 = this_pos_res
                .0
                .lerp(moving_to_position.position, walker.current_speed / distance);
            this_transform.translation = this_pos_res.0;
            walker.walk();
        } else {
            commands.entity(entity_id).remove::<MovingToPosition>();
            walker.stop();
            ev_arrival.send(moving_to_position.arrival_handler_data);
        }
    }
}
