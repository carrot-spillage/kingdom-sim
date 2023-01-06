pub mod bundle;

use bevy::{
    prelude::{Commands, Entity, Vec3, Transform, Res},
    sprite::SpriteBundle,
};

use crate::{movement::{Position, hack_3d_position_to_2d}, loading::TextureAssets, tree::SimpleDestructible};

use self::bundle::PlantBundle;

pub fn init_plant(commands: &mut Commands, plant_bundle: PlantBundle, textures: &Res<TextureAssets>, position: Vec3) -> Entity {
    commands
        .spawn((
            plant_bundle,
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
        ))
        .id()
}
