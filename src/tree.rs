use bevy::{
    math::Vec3,
    prelude::{Commands, Component, Res, Transform, Entity},
    sprite::SpriteBundle,
};

use crate::{
    loading::TextureAssets,
    movement::{hack_3d_position_to_2d, Position},
    resources::{BreaksIntoResources, ResourceChunk, ResourceKind},
};

#[derive(Component)]
pub struct Tree;

#[derive(Component)]
pub struct SimpleDestructible {
    pub current_health: f32,
    pub max_health: f32,
}

pub fn spawn_tree(commands: &mut Commands, textures: &Res<TextureAssets>, position: Vec3) -> Entity {
    commands
        .spawn()
        .insert(Tree)
        .insert(SimpleDestructible {
            current_health: 1000.0,
            max_health: 1000.0,
        })
        .insert(Position(position))
        .insert(BreaksIntoResources(vec![ResourceChunk {
            kind: ResourceKind::Wood,
            quantity: 2,
        }]))
        .insert_bundle(SpriteBundle {
            texture: textures.tree2.clone(),
            transform: Transform {
                translation: hack_3d_position_to_2d(position),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Transform::default()
            },
            ..Default::default()
        })
        .id()
}
