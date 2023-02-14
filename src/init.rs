use std::collections::VecDeque;

use bevy::{
    math::{Vec2, Vec3},
    prelude::{
        App, Camera2dBundle, Commands, Component, Entity, Plugin, Query, Res, ResMut, Resource,
        State, SystemSet, With, Without,
    },
};
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};

use crate::{
    building::{
        get_construction_site_texture, spawn_construction_site, BuildingPrefab, BuildingTextureSet,
    },
    items::ItemPrefabId,
    loading::{FontAssets, TextureAssets},
    movement::Position,
    planting::logic::{PlantPrefabMap, Planting},
    plants::{
        bundle::PlantPrefabId, spawn_plant, IntrinsicPlantResourceGrower, PlantMaturityStage,
        PlantResourceProducer,
    },
    tasks::{CreatureTask, CreatureTasks},
    creature::{spawn_creature, Creature},
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

    // for _ in 0..1 {
    //     let pos = get_random_pos_2(&mut global_rng, Vec2::ZERO, world_params.size / 2.0);
    //     let worker_id = spawn_worker(&mut commands, &textures, &fonts, pos);

    //     let work_id = MonkeyPlanner::plan_house(
    //         &mut commands,
    //         &textures,
    //         get_random_pos_2(&mut global_rng, Vec2::ZERO, world_params.size / 4.0),
    //     );

    //     MonkeyPlanner::temp_recruit_workers(
    //         &mut commands,
    //         work_id,
    //         vec![worker_id],
    //         BUILDING_JOB_NAME,
    //     )
    // }

    // for _ in 0..1 {
    //     let pos = get_random_pos_2(&mut global_rng, Vec2::ZERO, world_params.size / 2.0);
    //     let worker_id = spawn_worker(&mut commands, &textures, &fonts, pos);

    //     let work_id = plan_farm_field(
    //         &mut commands,
    //         get_random_pos_2(&mut global_rng, Vec2::ZERO, world_params.size / 4.0),
    //     );

    //     MonkeyPlanner::temp_recruit_workers(
    //         &mut commands,
    //         work_id,
    //         vec![worker_id],
    //         PLANTING_JOB_NAME,
    //     )
    // }

    // TODO: new tree cutting
    // for _ in 0..2 {
    //     let worker_pos = get_random_pos_2(&mut global_rng, Vec2::ZERO, world_params.size / 3.0);
    //     let worker_id = spawn_worker(&mut commands, &textures, &fonts, worker_pos);
    //     let tree_pos = get_random_pos_2(&mut global_rng, Vec2::ZERO, world_params.size / 3.0);

    //     let tree_id = spawn_tree(&mut commands, &textures, tree_pos);
    //     let work_id = plan_tree_cutting(&mut commands, tree_id);

    //     // MonkeyPlanner::temp_recruit_workers(
    //     //     &mut commands,
    //     //     work_id,
    //     //     vec![worker_id],
    //     //     TREE_CUTTING_JOB_NAME,
    //     // )
    // }

    // for _ in 0..1 {
    //     let worker_pos = get_random_pos_2(&mut global_rng, Vec2::ZERO, world_params.size / 3.0);
    //     let worker_id = spawn_worker(&mut commands, &textures, &fonts, worker_pos);
    //     plan_resource_gathering(&mut commands, worker_id);
    // }

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

        // convert_construction_site_to_building(construction_site_id, &mut commands, &house_textures);
    }

    // MOVE TO THE WORK ENTITY
    // Use move_to_work()

    // for _ in 0..20 {
    //     let position = get_random_pos_2(&mut global_rng, Vec2::ZERO, world_params.size / 2.0);
    //     let (prefab, texture) = plants.0.get(&PlantPrefabId(1)).unwrap();
    //     spawn_plant(
    //         &mut commands,
    //         prefab,
    //         texture.clone(),
    //         position,
    //         &PlantMaturityStage::FullyGrown,
    //     );
    // }

    for _ in 0..30 {
        let tree_pos = get_random_pos(&mut global_rng, Vec2::ZERO, world_params.size / 2.0);
        let (prefab, texture) = plants.0.get(&PlantPrefabId(1)).unwrap();
        let tree_id = spawn_plant(
            &mut commands,
            &mut global_rng,
            prefab,
            texture.clone(),
            tree_pos,
            &PlantMaturityStage::FullyGrown,
        );
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
    (Vec2::new(
        global_rng.f32_normalized() * range.x,
        global_rng.f32_normalized() * range.y,
    ) + origin)
        .extend(0.0)
}
