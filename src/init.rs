use std::collections::VecDeque;

use bevy::{
    math::{Vec2, Vec3},
    prelude::{
        App, Camera2dBundle, Commands, Component, Entity, Plugin, Query, Res, ResMut, Resource,
        State, SystemSet, With, Without,
    },
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
        get_construction_site_texture, spawn_construction_site, BuildingPrefab, BuildingTextureSet,
    },
    creature::{spawn_creature, Creature},
    items::ItemPrefabId,
    loading::{FontAssets, TextureAssets},
    movement::Position,
    planting::logic::{PlantPrefabMap, Planting},
    plants::{
        bundle::PlantPrefabId, spawn_plant, IntrinsicPlantResourceGrower, PlantMaturityStage,
        PlantResourceProducer,
    },
    tasks::{CreatureTask, CreatureTasks},
    GameState,
};

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::CreatingWorld).with_system(init))
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(run_dummy_commands),
            );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

#[derive(Resource)]
pub struct WorldParams {
    pub size: Vec2,
}

fn init(
    mut game_state: ResMut<State<GameState>>,
    world_params: Res<WorldParams>,
    mut commands: Commands,
    mut global_rng: ResMut<GlobalRng>,
    textures: Res<TextureAssets>,
    fonts: Res<FontAssets>,
    plants: Res<PlantPrefabMap>,
) {
    commands.spawn(Camera2dBundle::default());

    create_tilemap(&mut commands, &world_params, &textures);
    let house_textures = BuildingTextureSet {
        in_progress: vec![textures.house_in_progress.clone()],
        completed: textures.house.clone(),
        scale: 0.03,
    };

    let campfire_pos = get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 4.0);
    commands.spawn((Campfire, Position(campfire_pos)));
    // CONSTRUCTION SITES
    for _ in 0..5 {
        let pos = get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 4.0);

        let construction_site_id = commands.spawn_empty().id();
        spawn_construction_site(&mut commands, construction_site_id, pos, &house_textures);
        let building_prefab = BuildingPrefab {
            name: "House",
            max_hp: 2000.0,
            units_of_work: 100.0,
            texture_set: BuildingTextureSet {
                in_progress: vec![
                    textures.house_in_progress.clone(),
                    textures.house_in_progress.clone(),
                ],
                completed: textures.house.clone(),
                scale: 0.03,
            },
            max_workers: 2,
            required_resources: vec![(ItemPrefabId(3), 4)],
        };
        if let Some(new_texture) = get_construction_site_texture(0.0, 0.1, &building_prefab) {
            commands.entity(construction_site_id).insert(new_texture);
        }
    }

    // putting them all in line
    let tree_pos = get_random_pos(&mut global_rng, Vec2::ZERO, Vec2::new(10.0, 10.0));
    for i in 0..10 {
        let (prefab, texture) = plants.0.get(&PlantPrefabId(1)).unwrap();
        let tree_id = spawn_plant(
            &mut commands,
            &mut global_rng,
            prefab,
            texture.clone(),
            Vec3::new(100.0, 100. - (i as f32) * 20.0, 0.),
            &PlantMaturityStage::FullyGrown,
        );
        println!("spawns a tree");
    }

    for i in 0..10 {
        let (prefab, texture) = plants.0.get(&PlantPrefabId(1)).unwrap();
        let tree_id = spawn_plant(
            &mut commands,
            &mut global_rng,
            prefab,
            textures.logs.clone(),
            Vec3::new(-300.0, 100. - (i as f32) * 20.0, 0.),
            &PlantMaturityStage::FullyGrown,
        );
        println!("spawns a log");
    }

    for _ in 0..5 {
        let bush_pos = get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 3.0);
        let (prefab, texture) = plants.0.get(&PlantPrefabId(2)).unwrap();
        let bush_id = spawn_plant(
            &mut commands,
            &mut global_rng,
            prefab,
            texture.clone(),
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
            worker_pos,
        );
    }

    game_state.overwrite_set(GameState::Playing).unwrap();
}

fn create_tilemap(
    commands: &mut Commands,
    world_params: &Res<WorldParams>,
    textures: &Res<TextureAssets>,
) {
    let tile_size = TilemapTileSize { x: 19.0, y: 11.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    let tilemap_entity = commands.spawn_empty().id();
    let side = (world_params.size.x / tile_size.x) as u32;
    let map_size = TilemapSize { x: side, y: side };
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
    world_params: Res<WorldParams>,
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
    let mut bushes_iter = bushes.iter();
    let campfire_pos = campfires.single().0.clone();

    for (worker_id, mut rng) in &mut workers {
        let val = rng.f32();
        if val < 0.3 {
            let tree_id = trees_iter.next().unwrap();
            let drop_pos = get_random_pos(
                &mut global_rng,
                campfire_pos.truncate(),
                Vec2::new(20.0, 20.0),
            );

            commands
                .entity(worker_id)
                .insert(CreatureTasks(VecDeque::from(vec![
                    CreatureTask::MoveToTarget { target_id: tree_id },
                    CreatureTask::CutTree { target_id: tree_id },
                ])));
        } else if val < 0.6 {
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

pub fn get_random_pos(global_rng: &mut ResMut<GlobalRng>, origin: Vec2, range: Vec2) -> Vec3 {
    let pos = (Vec2::new(
        global_rng.f32_normalized() * range.x,
        global_rng.f32_normalized() * range.y,
    ) + origin);

    pos.extend(1000.0 - pos.y)
}
