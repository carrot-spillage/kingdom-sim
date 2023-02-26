use std::f32::consts::PI;

use bevy::{
    math::Vec3,
    prelude::{
        App, Changed, Commands, Component, Entity, Plugin, Query, SystemSet, Transform, Vec2,
    },
};

use crate::{
    tasks::{CreatureTask, IdlingCreature},
    GameState,
};

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
    let result = isometric(position.truncate());
    result.extend(1000.0 - result.y)
}

fn isometric(vec: Vec2) -> Vec2 {
    Vec2::from_angle(-PI * 0.25).rotate(vec) * Vec2::new(1.0, 0.5)
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ArrivedToPositionEvent>()
            .add_event::<ArrivedToEntityEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_to_position)
                    .with_system(move_to_entity)
                    .with_system(isometrify_from_position),
            );
    }
}

fn isometrify_from_position(mut positions: Query<(&mut Transform, &Position), Changed<Position>>) {
    for (mut transform, position) in &mut positions {
        transform.translation = hack_3d_position_to_2d(position.0);
    }
}

fn move_to_position(
    mut moving: Query<(Entity, &mut Walker, &MovingToPosition)>,
    mut positions: Query<&mut Position>,
    mut commands: Commands,
) {
    for (entity_id, mut walker, moving_to_position) in moving.iter_mut() {
        let mut this_pos_res = positions.get_mut(entity_id).unwrap();

        let distance = this_pos_res.0.distance(moving_to_position.position);
        if distance > moving_to_position.sufficient_range {
            this_pos_res.0 = this_pos_res
                .0
                .lerp(moving_to_position.position, walker.current_speed / distance);
            walker.walk();
        } else {
            walker.stop();
            commands
                .entity(entity_id)
                .remove::<(CreatureTask, MovingToPosition)>()
                .insert(IdlingCreature);
        }
    }
}

fn move_to_entity(
    mut moving: Query<(Entity, &mut Walker, &MovingToEntity)>,
    mut positions: Query<&mut Position>,
    mut commands: Commands,
) {
    for (entity_id, mut walker, moving) in moving.iter_mut() {
        let maybe_destination_position = positions
            .get(moving.destination_entity)
            .map(|x| x.0.clone());
        if let Ok(destination_position) = maybe_destination_position {
            let mut this_pos_res = positions.get_mut(entity_id).unwrap();
            let distance = this_pos_res.0.distance(destination_position);
            if distance > moving.sufficient_range {
                this_pos_res.0 = this_pos_res
                    .0
                    .lerp(destination_position, walker.current_speed / distance);
                walker.walk();
            } else {
                println!("Stopped {:?}", entity_id);
                walker.stop();
                commands
                    .entity(entity_id)
                    .remove::<(CreatureTask, MovingToEntity)>()
                    .insert(IdlingCreature);
            }
        }
    }
}
