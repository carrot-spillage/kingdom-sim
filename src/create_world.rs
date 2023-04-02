use std::collections::VecDeque;

use bevy::{
    math::{Vec2, Vec3},
    prelude::{
        App, Camera2dBundle, Commands, Component, Entity, EventWriter, IntoSystemAppConfig,
        NextState, OnEnter, Plugin, Query, Rect, Res, ResMut, Resource, Transform, With, Without,
    },
    sprite::SpriteBundle,
};
use bevy_ecs_tilemap::{
    prelude::{
        get_tilemap_center_transform, IsoCoordSystem, TilemapId, TilemapSize, TilemapTexture,
        TilemapTileSize, TilemapType,
    },
    tiles::{TileBundle, TilePos, TileStorage},
    TilemapBundle,
};
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};

use crate::{
    building::{
        get_construction_site_texture, spawn_construction_site, BuildingPrefabId,
        BuildingPrefabMap, ConstructionSite,
    },
    items::{spawn_item_batch, ItemBatch, ItemPrefabId, ItemPrefabMap},
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
    tasks::{CreatureTaskType, CreatureTasks},
    GameState,
};

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_world.in_schedule(OnEnter(GameState::CreatingWorld)))
            .add_system(run_dummy_commands.in_schedule(OnEnter(GameState::Playing)));
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

    mut quad_tree: ResMut<QuadTree<Entity>>,
    mut area_occupied_events: EventWriter<AreaOccupiedEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // commands.spawn(Window {
    //     resolution: WindowResolution::new(800., 600.),
    //     title: "kingdom_sim".to_string(),
    //     ..default()
    // });

    commands.spawn(Camera2dBundle::new_with_far(
        world_params.half_max_isometric_z * 2.0,
    ));

    create_tilemap(&mut commands, &world_params, &textures);
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
            let worker_id = spawn_creature(
                &mut commands,
                &mut global_rng,
                &textures,
                &fonts,
                &world_params,
                worker_pos,
            );

            let resource_pos = get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 2.0);

            let item_batch_id = spawn_item_batch(
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

    for _ in 0..50 {
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

    for _ in 0..5 {
        let bush_pos = get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 3.0);
        let prefab = plants.0.get(&PlantPrefabId(2)).unwrap();
        let bush_id = spawn_plant(
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
        let worker_id = spawn_creature(
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

pub struct AreaOccupiedEvent {
    pub area: Rect,
}

fn create_tilemap(
    commands: &mut Commands,
    world_params: &Res<WorldParams>,
    textures: &Res<TextureAssets>,
) {
    let tile_size = TilemapTileSize {
        x: world_params.tile_side * 2.0,
        y: world_params.tile_side,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    let tilemap_entity = commands.spawn_empty().id();
    let map_size = TilemapSize {
        x: (world_params.size.x / world_params.tile_side) as u32,
        y: (world_params.size.y / world_params.tile_side) as u32,
    };
    println!("Map size {:?}", map_size);
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(textures.tile.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

// fn swap_texture_or_hide(
//     asset_server: Res<AssetServer>,
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<(&mut TilemapTexture, &mut Visibility)>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::Space) {
//         let texture_a = TilemapTexture::Single(asset_server.load("tiles.png"));
//         let texture_b = TilemapTexture::Single(asset_server.load("tiles2.png"));
//         for (mut tilemap_tex, _) in &mut query {
//             if *tilemap_tex == texture_a {
//                 *tilemap_tex = texture_b.clone();
//             } else {
//                 *tilemap_tex = texture_a.clone();
//             }
//         }
//     }
//     if keyboard_input.just_pressed(KeyCode::H) {
//         for (_, mut visibility) in &mut query {
//             if visibility.is_visible {
//                 visibility.is_visible = false;
//             } else {
//                 visibility.is_visible = true;
//             }
//         }
//     }
// }

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

    for (worker_id, mut rng) in &mut workers.iter_mut().take(1) {
        let val = rng.f32();
        // if val < 0.3 {
        let construction_site_id = costruction_sites_iter.next().unwrap();

        let item_batch_id = item_batches_iter.next().unwrap();
        commands
            .entity(worker_id)
            .insert(CreatureTasks(VecDeque::from(vec![
                CreatureTaskType::MoveToTarget {
                    target_id: item_batch_id,
                },
                CreatureTaskType::CollectItems {
                    target_id: item_batch_id,
                },
                CreatureTaskType::MoveToTarget {
                    target_id: construction_site_id,
                },
                CreatureTaskType::TransferItems {
                    target_id: construction_site_id,
                },
            ])));
        // } else if val < 0.5 {
        //     let tree_id = trees_iter.next().unwrap();
        //     let drop_pos = get_random_pos(
        //         &mut global_rng,
        //         campfire_pos.truncate(),
        //         Vec2::new(20.0, 20.0),
        //     );

        //     commands
        //         .entity(worker_id)
        //         .insert(CreatureTasks(VecDeque::from(vec![
        //             CreatureTask::MoveToTarget { target_id: tree_id },
        //             CreatureTask::CutTree { target_id: tree_id },
        //         ])));
        // } else if val < 0.8 {
        //     let bush_id = bushes_iter.next().unwrap();
        //     let drop_pos = get_random_pos(
        //         &mut global_rng,
        //         campfire_pos.truncate(),
        //         Vec2::new(20.0, 20.0),
        //     );
        //     commands
        //         .entity(worker_id)
        //         .insert(CreatureTasks(VecDeque::from(vec![
        //             CreatureTask::MoveToTarget { target_id: bush_id },
        //             CreatureTask::Harvest { target_id: bush_id },
        //             CreatureTask::MoveToPosition { position: drop_pos },
        //             CreatureTask::DropItems,
        //         ])));
        // } else {
        //     let new_plant_pos =
        //         get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 2.0);

        //     commands
        //         .entity(worker_id)
        //         .insert(CreatureTasks(VecDeque::from(vec![
        //             CreatureTask::MoveToPosition {
        //                 position: new_plant_pos,
        //             },
        //             CreatureTask::Plant {
        //                 planting: Planting {
        //                     plant_prefab_id: PlantPrefabId(1),
        //                     position: new_plant_pos,
        //                 },
        //             },
        //         ])));
        // }
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
