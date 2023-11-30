pub mod bundle;
mod destruction;
mod intrinsic_resource;
mod resource_producer;

use std::f32::consts::PI;

use bevy::{
    ecs::event::EventReader,
    math::Vec2,
    prelude::{
        in_state, App, Commands, Entity, EventWriter, Handle, Image, IntoSystemConfigs, Plugin,
        Query, Rect, Res, ResMut, Transform, Update, Vec3,
    },
    sprite::{Sprite, SpriteBundle},
};
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};

use crate::{
    create_world::{AreaOccupiedEvent, WorldParams},
    movement::{isometrify_position, Position},
    planting::logic::PlantPrefabMap,
    quad_tree::QuadTree,
    timer_plugin::ElapsedEvent,
    GameState,
};

use self::{
    bundle::{Germinator, GerminatorParams, Growing, PlantPrefab, PlantPrefabId},
    destruction::break_into_resources,
    intrinsic_resource::grow_elapsed,
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
    prefab: &PlantPrefab<Handle<Image>>,
    position: Vec3,
    maturity_state: &PlantMaturityStage,
) -> Entity {
    let (plant_bundle, maybe_resource_grower, maybe_producer, maybe_growing, maybe_germinator) =
        prefab.to_plant_components(maturity_state, global_rng);
    let maybe_maturity_based_producer =
        maybe_producer.map(|producer: PlantResourceProducer| match maturity_state {
            PlantMaturityStage::Germ => producer,
            PlantMaturityStage::FullyGrown => {
                let mut maxed_producer = producer.clone();
                maxed_producer.current.quantity = maxed_producer.max_quantity;
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

    let mut sub_commands = commands.spawn((
        plant_bundle,
        Position(position),
        SpriteBundle {
            texture: prefab.textures.default.clone(),
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
    ));

    if let Some(component) = maybe_maturity_based_grower {
        sub_commands.insert(component);
    }

    if let Some(component) = maybe_maturity_based_producer {
        sub_commands.insert(component);
    }

    if let Some(component) = maybe_growing {
        sub_commands.insert(component);
    }

    if let Some(component) = maybe_germinator {
        sub_commands.insert(component);
    }

    sub_commands.id()
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
    mut elapsed_germinators: EventReader<ElapsedEvent<Germinator>>,
    mut germinator_params_query: Query<(
        &PlantPrefabId,
        &Position,
        &GerminatorParams,
        &mut RngComponent,
    )>,
    mut quad_tree: ResMut<QuadTree<Entity>>,
    mut area_occupied_events: EventWriter<AreaOccupiedEvent>,
) {
    for germinator_event in &mut elapsed_germinators {
        let (plant_prefab_id, position, germinator_params, mut rng) = germinator_params_query
            .get_mut(germinator_event.entity)
            .unwrap();
        let rand_offset_x = rng.f32_normalized() * germinator_params.radius as f32;
        let rand_offset_y = (rng.f32_normalized() * PI).sin() * germinator_params.radius as f32;

        let germ_offset = Vec2::new(rand_offset_x as f32, rand_offset_y as f32);

        let germ_position = position.0 + germ_offset.extend(0.0);
        let prefab = plant_prefab_map.0.get(plant_prefab_id).unwrap();
        let germ_rect = Rect::from_center_size(germ_position.truncate(), prefab.collision_box);
        quad_tree.try_occupy_rect(germ_rect, || {
            area_occupied_events.send(AreaOccupiedEvent { area: germ_rect });
            return spawn_plant(
                &mut commands,
                &mut global_rng,
                &world_params,
                prefab,
                germ_rect.center().extend(germ_position.z),
                &PlantMaturityStage::Germ,
            );
        });
    }
}

pub struct PlantsPlugin;
impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                grow,
                germinate,
                grow_elapsed,
                produce_resources,
                break_into_resources,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
