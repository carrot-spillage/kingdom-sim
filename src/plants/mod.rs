use bevy::{
    prelude::{Commands, Component, Entity, Vec3, Transform, Res},
    sprite::SpriteBundle,
};

use crate::{movement::{Position, hack_3d_position_to_2d}, loading::TextureAssets, tree::SimpleDestructible};

#[derive(Component)]
pub struct Plant {
    class_id: usize,
}

#[derive(Component)]
pub struct Germinating;

#[derive(Component)]
pub struct Growing;

pub fn init_plant(commands: &mut Commands, textures: &Res<TextureAssets>, class_id: usize, position: Vec3) -> Entity {
    commands
        .spawn((
            Plant { class_id },
            Growing,
            Germinating,
            Position(position),
            SpriteBundle {
                texture: textures.tree2.clone(),
                transform: Transform {
                    translation: hack_3d_position_to_2d(position),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..Transform::default()
                },
                ..Default::default()
            },
            SimpleDestructible {
                current_health: 1000.0,
                max_health: 1000.0,
            },
        ))
        .id()
}
