use std::collections::VecDeque;

use bevy::{
    math::{Vec2, Vec3},
    prelude::{
        App, Camera2dBundle, Color, Commands, Component, Entity, EventReader, EventWriter, Plugin,
        Query, Rect, Res, ResMut, Resource, State, SystemSet, Transform, With, Without,
    },
    sprite::SpriteBundle,
};
use bevy_ecs_tilemap::{
    prelude::{
        get_tilemap_center_transform, IsoCoordSystem, TilemapId, TilemapSize, TilemapTexture,
        TilemapTileSize, TilemapType,
    },
    tiles::{TileBundle, TileColor, TilePos, TileStorage},
    TilemapBundle,
};
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};

use crate::quad_tree::QuadTree;

use crate::{
    building::{
        get_construction_site_texture, spawn_construction_site, BuildingPrefab, BuildingTextureSet,
    },
    creature::{spawn_creature, Creature},
    loading::{FontAssets, TextureAssets},
    movement::{isometrify_position, Position},
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

#[derive(Resource, Debug)]
pub struct WorldParams {
    pub side: f32,
    pub size: Vec2,
    pub half_max_isometric_z: f32,
    pub tile_side: f32,
}

fn init(
    mut game_state: ResMut<State<GameState>>,
    world_params: Res<WorldParams>,
    mut commands: Commands,
    mut global_rng: ResMut<GlobalRng>,
    textures: Res<TextureAssets>,
    fonts: Res<FontAssets>,
    plants: Res<PlantPrefabMap>,
    mut quad_tree: ResMut<QuadTree<Entity>>,
    mut area_occupied_events: EventWriter<AreaOccupiedEvent>,
) {
    commands.spawn(Camera2dBundle::new_with_far(
        world_params.half_max_isometric_z * 2.0,
    ));

    create_tilemap(&mut commands, &world_params, &textures);
    let house_textures = BuildingTextureSet {
        in_progress: vec![textures.house_in_progress.clone()],
        completed: textures.house.clone(),
        scale: 0.03,
    };

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
    // for _ in 0..5 {
    //     let pos = get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 4.0);

    //     let construction_site_id = commands.spawn_empty().id();
    //     spawn_construction_site(&mut commands, construction_site_id, pos, &house_textures);
    //     let building_prefab = BuildingPrefab {
    //         name: "House",
    //         max_hp: 2000.0,
    //         units_of_work: 100.0,
    //         texture_set: BuildingTextureSet {
    //             in_progress: vec![
    //                 textures.house_in_progress.clone(),
    //                 textures.house_in_progress.clone(),
    //             ],
    //             completed: textures.house.clone(),
    //             scale: 0.03,
    //         },
    //         max_workers: 2,
    //         required_resources: vec![(ItemPrefabId(3), 4)],
    //     };
    //     if let Some(new_texture) = get_construction_site_texture(0.0, 0.1, &building_prefab) {
    //         commands.entity(construction_site_id).insert(new_texture);
    //     }
    // }

    for _ in 0..50 {
        let (prefab, texture) = plants.0.get(&PlantPrefabId(1)).unwrap();
        let tree_pos = get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 2.0);
        let tree_rect = Rect::from_center_size(tree_pos.truncate(), prefab.collision_box.to_vec());
        let tenant_id = commands.spawn_empty().id();
        if quad_tree.try_occupy_rect(tree_rect, tenant_id) {
            println!(
                "Occupied: tree_pos {:?} tree_rect {:?}",
                tree_pos, tree_rect
            );
            area_occupied_events.send(AreaOccupiedEvent { area: tree_rect });

            let tree_id = spawn_plant(
                &mut commands,
                &mut global_rng,
                &world_params,
                prefab,
                texture.clone(),
                tree_rect.center().extend(tree_pos.z),
                //Vec2::new(0.0, i as f32 * 20.0).extend(10.0),
                &PlantMaturityStage::FullyGrown,
            );
        } else {
            println!("Failed to occupy");
        }
    }

    for _ in 0..5 {
        let bush_pos = get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 3.0);
        let (prefab, texture) = plants.0.get(&PlantPrefabId(2)).unwrap();
        let bush_id = spawn_plant(
            &mut commands,
            &mut global_rng,
            &world_params,
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
            &world_params,
            worker_pos,
        );
    }

    game_state.overwrite_set(GameState::Playing).unwrap();
}

pub struct AreaOccupiedEvent {
    pub area: Rect,
}

pub struct OccupyTilesPlugin;

impl Plugin for OccupyTilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AreaOccupiedEvent>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(mark_tiles_in_area_as_occupied),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn mark_tiles_in_area_as_occupied(
    mut commands: Commands,
    grids: Query<&TileStorage>,
    mut events: EventReader<AreaOccupiedEvent>,
    world_params: Res<WorldParams>,
) {
    if events.is_empty() {
        return;
    }

    let tile_storage = grids.single();
    for AreaOccupiedEvent { area } in events.iter() {
        let world_offset = world_params.size / 2.0;
        let offset_area = Rect {
            min: area.min + world_offset,
            max: area.max + world_offset,
        };

        let start_grid_x = (offset_area.min.x / world_params.tile_side).floor() as u32;
        let end_grid_x = (offset_area.max.x / world_params.tile_side).floor() as u32;

        let start_grid_y = (offset_area.min.y / world_params.tile_side).floor() as u32;
        let end_grid_y = (offset_area.max.y / world_params.tile_side).floor() as u32;

        println!(
            "Occupying area {:?} {:?} start xy {:?} {:?} end xy {:?} {:?}",
            area, offset_area, start_grid_x, start_grid_y, end_grid_x, end_grid_y
        );

        for x in start_grid_x..=end_grid_x {
            for y in start_grid_y..=end_grid_y {
                let tile = tile_storage.get(&TilePos { x, y }).unwrap();
                commands.entity(tile).insert(TileColor(Color::ORANGE_RED));
            }
        }
    }
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
