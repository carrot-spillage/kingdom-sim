use std::collections::VecDeque;

use bevy::{
    asset::Assets,
    math::{Vec2, Vec3},
    prelude::{
        App, Commands, Component, Entity, Event, EventWriter, NextState, OnEnter, Plugin, Query,
        Rect, Res, ResMut, Resource, Transform, With, Without,
    },
    render::texture::Image,
    sprite::SpriteBundle,
};

use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};

use crate::{
    building::{
        get_construction_site_texture, spawn_construction_site, BuildingPrefabId,
        BuildingPrefabMap, ConstructionSite,
    },
    items::{spawn_item_batch, ItemBatch, ItemPrefabId, ItemPrefabMap},
    land_tilemap::create_land_tilemap,
    planting::logic::Planting,
    quad_tree::QuadTree,
};

use crate::{
    creature::{spawn_creature, Creature},
    loading::{FontAssets, TextureAssets},
    movement::{isometrify_position, Position},
    planting::logic::PlantPrefabMap,
    plants::{
        bundle::PlantPrefabId, spawn_plant, IntrinsicPlantResourceGrower, PlantMaturityStage,
        PlantResourceProducer,
    },
    tasks::{CreatureTask, CreatureTasks},
    GameState,
};

pub struct CreateWorldPlugin;

impl Plugin for CreateWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CreatingWorld), create_world)
            .add_systems(OnEnter(GameState::Playing), run_dummy_commands);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

#[derive(Resource, Debug)]
pub struct WorldParams {
    pub side: f32,
    pub size: Vec2,
    pub half_max_isometric_z: f32,
    pub tile_side: f32,
}

fn create_world(
    world_params: Res<WorldParams>,
    mut commands: Commands,
    mut global_rng: ResMut<GlobalRng>,
    textures: Res<TextureAssets>,
    fonts: Res<FontAssets>,
    plants: Res<PlantPrefabMap>,
    items: Res<ItemPrefabMap>,
    buildings: Res<BuildingPrefabMap>,
    mut assets: ResMut<Assets<Image>>,

    mut quad_tree: ResMut<QuadTree<Entity>>,
    mut area_occupied_events: EventWriter<AreaOccupiedEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    create_land_tilemap(&mut commands, &world_params, &mut assets);
    let house_prefab = buildings.0.get(&BuildingPrefabId(1)).unwrap();

    let campfire_pos = get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 4.0);
    commands.spawn((
        Campfire,
        Position(campfire_pos),
        SpriteBundle {
            transform: Transform {
                translation: isometrify_position(campfire_pos, &world_params),
                ..Default::default()
            },
            texture: textures.campfire.clone(),
            ..Default::default()
        },
    ));

    // CONSTRUCTION SITES
    for _ in 0..1 {
        let pos = get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 4.0);

        let construction_site_id = commands.spawn_empty().id();
        spawn_construction_site(
            &mut commands,
            construction_site_id,
            pos,
            &house_prefab,
            &world_params,
        );
        let building_prefab = buildings.0.get(&BuildingPrefabId(1)).unwrap();
        if let Some(new_texture) = get_construction_site_texture(0.0, 0.1, &building_prefab) {
            commands.entity(construction_site_id).insert(new_texture);
        }

        let wood_prefab_id = ItemPrefabId(3);
        let wood_prefab = items.0.get(&wood_prefab_id).unwrap();
        for _ in 0..5 {
            let worker_pos = get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 2.0);
            spawn_creature(
                &mut commands,
                &mut global_rng,
                &textures,
                &fonts,
                &world_params,
                worker_pos,
            );

            let resource_pos = get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 2.0);

            spawn_item_batch(
                &mut commands,
                wood_prefab.textures.dropped.clone(),
                ItemBatch {
                    prefab_id: wood_prefab_id,
                    quantity: 5,
                },
                resource_pos,
                &world_params,
            );
        }
    }

    println!("Creating trees {:?}", world_params.side as usize / 2);
    for _ in 0..world_params.side as usize / 2 {
        let prefab = plants.0.get(&PlantPrefabId(1)).unwrap();
        let tree_pos = get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 2.0);
        let tree_rect = Rect::from_center_size(tree_pos.truncate(), prefab.collision_box);

        quad_tree.try_occupy_rect(tree_rect, || {
            area_occupied_events.send(AreaOccupiedEvent { area: tree_rect });

            return spawn_plant(
                &mut commands,
                &mut global_rng,
                &world_params,
                prefab,
                tree_rect.center().extend(tree_pos.z),
                //Vec2::new(0.0, i as f32 * 20.0).extend(10.0),
                &PlantMaturityStage::FullyGrown,
            );
        });
    }

    println!("Trees created");
    for _ in 0..5 {
        let bush_pos = get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 3.0);
        let prefab = plants.0.get(&PlantPrefabId(2)).unwrap();
        spawn_plant(
            &mut commands,
            &mut global_rng,
            &world_params,
            prefab,
            bush_pos,
            &PlantMaturityStage::FullyGrown,
        );
    }

    for _ in 0..5 {
        let worker_pos = get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 2.0);
        spawn_creature(
            &mut commands,
            &mut global_rng,
            &textures,
            &fonts,
            &world_params,
            worker_pos,
        );
    }
    next_state.set(GameState::Playing);
}

#[derive(Event)]
pub struct AreaOccupiedEvent {
    pub area: Rect,
}

#[derive(Component)]
struct Campfire;

fn run_dummy_commands(
    mut global_rng: ResMut<GlobalRng>,
    mut commands: Commands,
    campfires: Query<&Position, With<Campfire>>,
    mut workers: Query<(Entity, &mut RngComponent), With<Creature>>,
    item_batches: Query<Entity, With<ItemBatch>>,
    world_params: Res<WorldParams>,
    costruction_sites: Query<Entity, With<ConstructionSite>>,
    trees: Query<
        Entity,
        (
            With<IntrinsicPlantResourceGrower>,
            Without<PlantResourceProducer>,
        ),
    >,
    bushes: Query<Entity, With<PlantResourceProducer>>,
) {
    let mut trees_iter = trees.iter();
    let mut costruction_sites_iter = costruction_sites.iter();

    let mut bushes_iter = bushes.iter();
    let mut item_batches_iter = item_batches.iter();
    let campfire_pos = campfires.single().0.clone();

    for (worker_id, mut rng) in &mut workers.iter_mut() {
        let val = rng.f32();
        if val < 0.3 {
            if let Some(construction_site_id) = costruction_sites_iter.next() {
                let item_batch_id = item_batches_iter.next().unwrap();
                commands
                    .entity(worker_id)
                    .insert(CreatureTasks(VecDeque::from(vec![
                        CreatureTask::MoveToTarget {
                            target_id: item_batch_id,
                        },
                        CreatureTask::CollectItems {
                            target_id: item_batch_id,
                        },
                        CreatureTask::MoveToTarget {
                            target_id: construction_site_id,
                        },
                        CreatureTask::TransferItems {
                            target_id: construction_site_id,
                        },
                        CreatureTask::Build {
                            target_id: construction_site_id,
                        },
                    ])));
            }
        } else if val < 0.5 {
            let tree_id = trees_iter.next().unwrap();

            commands
                .entity(worker_id)
                .insert(CreatureTasks(VecDeque::from(vec![
                    CreatureTask::MoveToTarget { target_id: tree_id },
                    CreatureTask::CutTree { target_id: tree_id },
                ])));
        } else if val < 0.8 {
            let bush_id = bushes_iter.next().unwrap();
            let drop_pos = get_random_pos(
                &mut global_rng,
                campfire_pos.truncate(),
                Vec2::new(20.0, 20.0),
            );
            commands
                .entity(worker_id)
                .insert(CreatureTasks(VecDeque::from(vec![
                    CreatureTask::MoveToTarget { target_id: bush_id },
                    CreatureTask::Harvest { target_id: bush_id },
                    CreatureTask::MoveToPosition { position: drop_pos },
                    CreatureTask::DropItems,
                ])));
        } else {
            let new_plant_pos =
                get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 2.0);

            commands
                .entity(worker_id)
                .insert(CreatureTasks(VecDeque::from(vec![
                    CreatureTask::MoveToPosition {
                        position: new_plant_pos,
                    },
                    CreatureTask::Plant {
                        planting: Planting {
                            plant_prefab_id: PlantPrefabId(1),
                            position: new_plant_pos,
                        },
                    },
                ])));
        }
    }
}

pub fn get_random_pos(
    global_rng: &mut ResMut<GlobalRng>,
    origin: Vec2,
    //entity_box: Vec2,
    range: Vec2,
) -> Vec3 {
    let entity_box = Vec2::new(24.0, 24.0);
    let pos = Vec2::new(global_rng.f32_normalized(), global_rng.f32_normalized())
        * (range - entity_box)
        + origin;

    pos.extend(0.0)
}
