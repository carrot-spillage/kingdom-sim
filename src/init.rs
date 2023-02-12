use std::collections::{HashMap, VecDeque};

use bevy::{
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    prelude::{
        debug, App, Bundle, Camera2dBundle, Commands, Component, Entity, Plugin, Query, Res,
        ResMut, Resource, State, SystemSet, Transform, With,
    },
    sprite::{Sprite, SpriteBundle},
    utils::tracing::field::debug,
};
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};
use rand::Rng;

use crate::{
    building::{
        get_construction_site_texture, spawn_construction_site, BuildingPrefab, BuildingTextureSet,
    },
    items::{CarrierInventory, ItemPrefabId},
    loading::{FontAssets, TextureAssets},
    movement::{hack_3d_position_to_2d, Position, Walker},
    planting::logic::PlantPrefabMap,
    plants::{
        bundle::PlantPrefabId, spawn_plant, IntrinsicPlantResourceGrower, PlantMaturityStage,
        PlantResourceProducer,
    },
    skills::{SkillType, Skilled},
    stockpile::spawn_stockpile,
    tasks::{IdlingWorker, WorkerTask, WorkerTasks},
    worker_job_tooltip::{create_tooltip_bundle, WorkerJobTooltip},
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
    //     let pos = get_random_pos(Vec2::ZERO, world_params.size / 2.0);
    //     let worker_id = spawn_worker(&mut commands, &textures, &fonts, pos);

    //     let work_id = MonkeyPlanner::plan_house(
    //         &mut commands,
    //         &textures,
    //         get_random_pos(Vec2::ZERO, world_params.size / 4.0),
    //     );

    //     MonkeyPlanner::temp_recruit_workers(
    //         &mut commands,
    //         work_id,
    //         vec![worker_id],
    //         BUILDING_JOB_NAME,
    //     )
    // }

    {
        let pos = get_random_pos(Vec2::ZERO, world_params.size / 2.0);
        spawn_stockpile(&mut commands, pos, Vec2::new(100.0, 100.0));
    }

    // for _ in 0..1 {
    //     let pos = get_random_pos(Vec2::ZERO, world_params.size / 2.0);
    //     let worker_id = spawn_worker(&mut commands, &textures, &fonts, pos);

    //     let work_id = plan_farm_field(
    //         &mut commands,
    //         get_random_pos(Vec2::ZERO, world_params.size / 4.0),
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
    //     let worker_pos = get_random_pos(Vec2::ZERO, world_params.size / 3.0);
    //     let worker_id = spawn_worker(&mut commands, &textures, &fonts, worker_pos);
    //     let tree_pos = get_random_pos(Vec2::ZERO, world_params.size / 3.0);

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
    //     let worker_pos = get_random_pos(Vec2::ZERO, world_params.size / 3.0);
    //     let worker_id = spawn_worker(&mut commands, &textures, &fonts, worker_pos);
    //     plan_resource_gathering(&mut commands, worker_id);
    // }

    let house_textures = BuildingTextureSet {
        in_progress: vec![textures.house_in_progress.clone()],
        completed: textures.house.clone(),
        scale: 0.03,
    };

    // CONSTRUCTION SITES
    for _ in 0..5 {
        let pos = get_random_pos(Vec2::ZERO, world_params.size / 4.0);

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
    //     let position = get_random_pos(Vec2::ZERO, world_params.size / 2.0);
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
        let tree_pos = get_random_pos(Vec2::ZERO, world_params.size / 2.0);
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
        let bush_pos = get_random_pos(Vec2::ZERO, world_params.size / 3.0);
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
        let worker_pos = get_random_pos(Vec2::ZERO, world_params.size / 2.0);
        let worker_id = spawn_worker(
            &mut commands,
            &mut global_rng,
            &textures,
            &fonts,
            worker_pos,
        );
    }

    game_state.overwrite_set(GameState::Playing).unwrap();
}

fn run_dummy_commands(
    mut commands: Commands,
    mut workers: Query<(Entity, &mut RngComponent), With<Worker>>,
    trees: Query<Entity, With<IntrinsicPlantResourceGrower>>,
    bushes: Query<Entity, With<PlantResourceProducer>>,
) {
    println!("run dummy commands");
    let mut trees_iter = trees.iter();
    let mut bushes_iter = bushes.iter();
    for (worker_id, mut rng) in &mut workers {
        if rng.bool() {
            let tree_id = trees_iter.next().unwrap();
            commands
                .entity(worker_id)
                .insert(WorkerTasks(VecDeque::from(vec![
                    WorkerTask::MoveToTarget { target_id: tree_id },
                    WorkerTask::CutTree { target_id: tree_id },
                ])));
        } else {
            let bush_id = bushes_iter.next().unwrap();
            commands
                .entity(worker_id)
                .insert(WorkerTasks(VecDeque::from(vec![
                    WorkerTask::MoveToTarget { target_id: bush_id },
                    WorkerTask::Harvest { target_id: bush_id },
                ])));
        }
    }
}

pub fn get_random_pos(origin: Vec2, range: Vec2) -> Vec3 {
    let mut rng = rand::thread_rng();
    (Vec2::new(
        rng.gen_range(-range.x..range.x),
        rng.gen_range(-range.y..range.y),
    ) + origin)
        .extend(0.0)
}

#[derive(Component)]
pub struct Worker;

fn spawn_worker(
    commands: &mut Commands,
    global_rng: &mut ResMut<GlobalRng>,
    textures: &Res<TextureAssets>,
    fonts: &Res<FontAssets>,
    position: Vec3,
) -> Entity {
    let bundle = WorkerBundle {
        worker: Worker,
        skilled: Skilled {
            skills: HashMap::from([
                (SkillType::Building, 0.5),
                (SkillType::GrowingPlants, 0.5),
                (SkillType::None, 0.5),
            ]),
        },
        inventory: CarrierInventory {
            items: vec![],
            max_weight: 50,
        },
        walker: Walker {
            max_speed: 2.0,
            current_speed: 0.0,
            acceleration: 0.5,
        },
        position: Position(position),
        sprite: SpriteBundle {
            texture: textures.peasant.clone(),
            transform: Transform {
                translation: hack_3d_position_to_2d(position),
                ..Transform::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(8.0, 12.25)),
                ..Sprite::default()
            },
            ..Default::default()
        },
    };

    let mut id = None::<Entity>;
    commands
        .spawn(bundle)
        .with_children(|parent| {
            id = Some(parent.spawn(create_tooltip_bundle(13.0, &fonts)).id());
        })
        .insert((
            Position(position),
            RngComponent::from(global_rng),
            IdlingWorker,
            WorkerJobTooltip {
                title: "".to_string(),
                child: id.unwrap(),
            },
        ))
        .id()
}

#[derive(Bundle)]

struct WorkerBundle {
    worker: Worker,
    skilled: Skilled,
    walker: Walker,
    position: Position,
    sprite: SpriteBundle,
    inventory: CarrierInventory,
}
