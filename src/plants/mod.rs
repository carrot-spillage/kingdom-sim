pub mod bundle;
mod intrinsic_resource;
mod resource_producer;

use bevy::{
    prelude::{
        App, Commands, Entity, Handle, Image, Plugin, Query, Rect, Res, SystemSet, Transform, Vec2,
        Vec3,
    },
    sprite::{Sprite, SpriteBundle},
};
use conditional_commands::ConditionalInsertBundleExt;

use crate::{
    movement::{hack_3d_position_to_2d, Position},
    planting::logic::PlantPrefabMap,
    GameState,
};

use self::{
    bundle::{Germinator, GerminatorParams, Growing, PlantPrefab, PlantPrefabId},
    intrinsic_resource::grow_resource,
    resource_producer::produce_resources,
};

pub use self::{
    intrinsic_resource::IntrinsicPlantResourceGrower, resource_producer::PlantResourceProducer,
};

pub enum PlantMaturityStage {
    Germ,
    FullyGrown,
}

pub fn spawn_plant(
    commands: &mut Commands,
    prefab: &PlantPrefab,
    texture: Handle<Image>,
    position: Vec3,
    maturity_state: &PlantMaturityStage,
) -> Entity {
    let (plant_bundle, maybe_resource_grower, maybe_producer, maybe_growing, maybe_germinator) =
        prefab.to_plant_components(maturity_state);
    commands
        .spawn((
            plant_bundle,
            Position(position),
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: hack_3d_position_to_2d(position),
                    scale: Vec3::new(0.0, 0.0, 1.0),
                    ..Transform::default()
                },
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::BottomCenter,
                    custom_size: Some(Vec2::new(24.0, 24.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .insert_if(maybe_resource_grower.is_some(), || {
            maybe_resource_grower.unwrap()
        })
        .insert_if(maybe_producer.is_some(), || maybe_producer.unwrap())
        .insert_if(maybe_growing.is_some(), || maybe_growing.unwrap())
        .insert_if(maybe_germinator.is_some(), || maybe_germinator.unwrap())
        .id()
}

pub fn grow(
    mut commands: Commands,
    mut growing_query: Query<(Entity, &mut Growing, &mut Transform, &GerminatorParams)>,
) {
    for (tree_id, mut growing, mut transform, germinator_params) in &mut growing_query {
        growing.maturity = (growing.maturity + growing.rate).min(1.0);
        transform.scale = Vec3::new(growing.maturity, growing.maturity, 1.0);
        if growing.maturity == 1.0 {
            commands
                .entity(tree_id)
                .remove::<Growing>()
                .insert(Germinator::new(germinator_params.clone()));
        }
    }
}

pub fn germinate(
    mut commands: Commands,
    plant_prefab_map: Res<PlantPrefabMap>,
    mut germinator_params_query: Query<(&PlantPrefabId, &Position, &mut Germinator)>,
) {
    for (plant_prefab_id, position, mut germinator) in &mut germinator_params_query {
        if let Some(germ_offset) = germinator.try_produce() {
            let germ_position = position.0 + germ_offset.extend(0.0);
            let (prefab, texture) = plant_prefab_map.0.get(plant_prefab_id).unwrap();
            spawn_plant(
                &mut commands,
                prefab,
                texture.clone(),
                germ_position,
                &PlantMaturityStage::Germ,
            );
        }
    }
}

pub struct PlantsPlugin;
impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(grow)
                .with_system(germinate)
                .with_system(grow_resource)
                .with_system(produce_resources),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
