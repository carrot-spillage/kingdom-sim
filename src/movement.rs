use bevy::{
    math::Vec3,
    prelude::{
        App, Changed, Commands, Component, Entity, IntoSystemConfigs, Mat2, OnUpdate, Plugin,
        Query, Res, Transform, Vec2,
    },
};

use crate::{
    init::WorldParams,
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

pub fn isometrify_position(position: Vec3, world_params: &Res<WorldParams>) -> Vec3 {
    let mut result = isometric(position.truncate());
    let z = world_params.half_max_isometric_z - result.y;
    if position.z > 0.0 {
        result.y += position.z;
    }

    result.extend(z)
}

static ISO_MAT: Mat2 = Mat2::from_cols(Vec2::new(1.0, -0.5), Vec2::new(1.0, 0.5));

pub fn isometric(vec: Vec2) -> Vec2 {
    ISO_MAT * vec
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ArrivedToPositionEvent>()
            .add_event::<ArrivedToEntityEvent>()
            .add_systems(
                (move_to_position, move_to_entity, isometrify_from_position)
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

fn isometrify_from_position(
    mut positions: Query<(&mut Transform, &Position), Changed<Position>>,
    world_params: Res<WorldParams>,
) {
    for (mut transform, position) in &mut positions {
        transform.translation = isometrify_position(position.0, &world_params);
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
