pub mod bundle;
mod destruction;
mod intrinsic_resource;
mod resource_producer;

use bevy::{
    prelude::{
        App, Commands, Entity, EventWriter, Handle, Image, Plugin, Query, Rect, Res, ResMut,
        SystemSet, Transform, Vec3,
    },
    sprite::{Sprite, SpriteBundle},
};
use bevy_turborand::{GlobalRng, RngComponent};
use conditional_commands::ConditionalInsertBundleExt;

use crate::{
    init::{AreaOccupiedEvent, WorldParams},
    movement::{isometrify_position, Position},
    planting::logic::PlantPrefabMap,
    quad_tree::QuadTree,
    GameState,
};

use self::{
    bundle::{Germinator, GerminatorParams, Growing, PlantPrefab, PlantPrefabId},
    destruction::break_into_resources,
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
    global_rng: &mut ResMut<GlobalRng>,
    world_params: &Res<WorldParams>,
    prefab: &PlantPrefab,
    texture: Handle<Image>,
    position: Vec3,
    maturity_state: &PlantMaturityStage,
) -> Entity {
    let (plant_bundle, maybe_resource_grower, maybe_producer, maybe_growing, maybe_germinator) =
        prefab.to_plant_components(maturity_state, global_rng);
    let maybe_maturity_based_producer = maybe_producer.map(|producer| match maturity_state {
        PlantMaturityStage::Germ => producer,
        PlantMaturityStage::FullyGrown => {
            let mut maxed_producer = producer.clone();
            maxed_producer.max_out();
            maxed_producer
        }
    });
    let maybe_maturity_based_grower = maybe_resource_grower.map(|grower| match maturity_state {
        PlantMaturityStage::Germ => grower,
        PlantMaturityStage::FullyGrown => {
            let mut maxed_grower = grower.clone();
            maxed_grower.max_out();
            maxed_grower
        }
    });
    commands
        .spawn((
            plant_bundle,
            Position(position),
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: isometrify_position(position, &world_params),
                    scale: match maturity_state {
                        PlantMaturityStage::Germ => Vec3::new(0.0, 0.0, 1.0),
                        PlantMaturityStage::FullyGrown => Vec3::new(1.0, 1.0, 1.0),
                    },
                    ..Transform::default()
                },
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::BottomCenter,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .insert_if(maybe_maturity_based_grower.is_some(), || {
            maybe_maturity_based_grower.unwrap()
        })
        .insert_if(maybe_maturity_based_producer.is_some(), || {
            maybe_maturity_based_producer.unwrap()
        })
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
    mut global_rng: ResMut<GlobalRng>,
    plant_prefab_map: Res<PlantPrefabMap>,
    world_params: Res<WorldParams>,
    mut germinator_params_query: Query<(
        &PlantPrefabId,
        &Position,
        &mut Germinator,
        &mut RngComponent,
    )>,
    mut quad_tree: ResMut<QuadTree<Entity>>,
    mut area_occupied_events: EventWriter<AreaOccupiedEvent>,
) {
    for (plant_prefab_id, position, mut germinator, mut rng) in &mut germinator_params_query {
        if let Some(germ_offset) = germinator.try_produce(&mut rng) {
            let germ_position = position.0 + germ_offset.extend(0.0);
            let (prefab, texture) = plant_prefab_map.0.get(plant_prefab_id).unwrap();
            let germ_rect =
                Rect::from_center_size(germ_position.truncate(), prefab.collision_box.to_vec());
            let occupant_id = commands.spawn_empty().id();
            if quad_tree.try_occupy_rect(germ_rect, occupant_id) {
                println!("Sends AreaOccupiedEvent");
                area_occupied_events.send(AreaOccupiedEvent { area: germ_rect });
                spawn_plant(
                    &mut commands,
                    &mut global_rng,
                    &world_params,
                    prefab,
                    texture.clone(),
                    germ_rect.center().extend(germ_position.z),
                    &PlantMaturityStage::Germ,
                );
            }
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
                .with_system(produce_resources)
                .with_system(break_into_resources),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
