use bevy::{
    math::{Vec2, Vec3},
    prelude::{Commands, Component, Res, Transform},
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    loading::TextureAssets,
    movement::{hack_3d_position_to_2d, Position},
    resources::{BreaksIntoResources, ResourceChunk, ResourceKind},
};

#[derive(Component)]
pub struct Tree;

pub fn spawn_tree(commands: &mut Commands, textures: &Res<TextureAssets>, position: Vec3) {
    commands
        .spawn()
        .insert(Tree)
        .insert(BreaksIntoResources(vec![ResourceChunk {
            kind: ResourceKind::Wood,
            quantity: 2.0,
        }]))
        .insert_bundle(SpriteBundle {
            texture: textures.tree2.clone(),
            transform: Transform {
                translation: hack_3d_position_to_2d(position),
                ..Transform::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..Sprite::default()
            },
            ..Default::default()
        });
}
