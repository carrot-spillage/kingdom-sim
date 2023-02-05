use bevy::{
    math::Vec3,
    prelude::{App, Commands, Component, Entity, EventWriter, Plugin, Query, SystemSet, Transform},
};

use crate::{tasks::IdlingWorker, GameState};

#[derive(Component)]
pub struct MovingToPosition {
    pub position: Vec3,
    pub sufficient_range: f32,
}

#[derive(Component)]
pub struct MovingToEntity {
    pub destination_entity: Entity,
    pub sufficient_range: f32,
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

pub struct ArrivedToPositionEvent(pub Entity);

pub struct ArrivedToEntityEvent {
    pub moving_entity: Entity,
    pub destination_entity: Entity,
}

pub struct MovementPlugin;

pub fn hack_3d_position_to_2d(position: Vec3) -> Vec3 {
    Vec3::new(position.x, position.y, 500.0 + position.y) // z cannot be negative so adding 1000.0 just to be sure
}

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ArrivedToPositionEvent>()
            .add_event::<ArrivedToEntityEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_to_position)
                    .with_system(move_to_entity),
            );
    }
}
// TODO: deprecate ArrivedToPositionEvent and use IdlingWorker instead?
fn move_to_position(
    mut moving: Query<(Entity, &mut Walker, &MovingToPosition)>,
    mut positions: Query<(&mut Position, &mut Transform)>,
    mut commands: Commands,
    mut arrivals: EventWriter<ArrivedToPositionEvent>,
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
            println!("Stopped {:?}", entity_id);
            walker.stop();
            commands.entity(entity_id).remove::<MovingToPosition>();

            arrivals.send(ArrivedToPositionEvent(entity_id))
        }
    }
}

fn move_to_entity(
    mut moving: Query<(Entity, &mut Walker, &MovingToEntity)>,
    mut positions_and_transforms: Query<(&mut Position, Option<&mut Transform>)>,
    mut commands: Commands,
) {
    for (entity_id, mut walker, moving) in moving.iter_mut() {
        let destination_position = positions_and_transforms
            .get(moving.destination_entity)
            .unwrap()
            .0
             .0;
        let (mut this_pos_res, this_transform) =
            positions_and_transforms.get_mut(entity_id).unwrap();
        let distance = this_pos_res.0.distance(destination_position);
        if distance > moving.sufficient_range {
            this_pos_res.0 = this_pos_res
                .0
                .lerp(destination_position, walker.current_speed / distance);
            this_transform.unwrap().translation = this_pos_res.0;
            walker.walk();
        } else {
            println!("Stopped {:?}", entity_id);
            walker.stop();
            commands
                .entity(entity_id)
                .remove::<MovingToEntity>()
                .insert(IdlingWorker);
        }
    }
}
