pub mod bundle;

use bevy::{
    prelude::{Commands, Entity, Query, Res, Transform, Vec3, SystemSet, Plugin, App},
    sprite::SpriteBundle,
};

use crate::{
    loading::TextureAssets,
    movement::{hack_3d_position_to_2d, Position},
    GameState,
};

use self::bundle::{Growing, PlantBundle};

pub fn init_plant(
    commands: &mut Commands,
    plant_bundle: PlantBundle,
    textures: &Res<TextureAssets>,
    position: Vec3,
) -> Entity {
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

pub fn grow(mut commands: Commands, mut growing_query: Query<(Entity, &mut Growing, &mut Transform)>) {
    for (tree_id, mut growing, mut transform) in &mut growing_query {
        growing.maturity = (growing.maturity + growing.speed).min(1.0);
        transform.scale = (transform.scale.truncate() * (1.0 + growing.maturity)).extend(1.0);
        if growing.maturity == 0.0 {
            commands.entity(tree_id).remove::<Growing>();
        }
    }
}

pub struct PlantsPlugin;
impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(grow),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}