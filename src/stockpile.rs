use bevy::{
    math::{Vec2, Vec3},
    prelude::{Color, Commands, Component, Entity, Transform},
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    movement::{hack_3d_position_to_2d, Position},
    planting_crops::OccupiedArea,
};

#[derive(Component)]
pub struct Stockpile;

#[derive(Component)]
pub struct InStockpile;

pub fn spawn_stockpile(commands: &mut Commands, position: Vec3, size: Vec2) -> Entity {
    let sprite = Sprite {
        color: Color::rgba(0.1, 0.1, 0.5, 0.15),
        flip_x: false,
        flip_y: false,
        custom_size: Some(size),
        ..Default::default()
    };
    commands
        .spawn(SpriteBundle {
            sprite,
            transform: Transform {
                translation: hack_3d_position_to_2d(position),
                ..Transform::default()
            },
            ..Default::default()
        })
        .insert(Position(position))
        .insert(OccupiedArea(size))
        .insert(Stockpile)
        .id()
}

pub fn drop_in_stockpile(commands: &mut Commands, worker_id: Entity, stockpile_id: Entity) {}
