use std::collections::HashMap;

use bevy::{
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    prelude::{
        App, Bundle, Camera2dBundle, Commands, Entity, Plugin, Res, Resource, SystemSet, Transform,
    },
    sprite::{Sprite, SpriteBundle},
};
use rand::Rng;

use crate::{
    building::{
        get_construction_site_texture, spawn_construction_site, BuildingPrefab, BuildingTextureSet,
    },
    loading::{FontAssets, TextureAssets},
    movement::{hack_3d_position_to_2d, Position, Walker},
    planting::logic::PlantPrefabMap,
    plants::{spawn_plant, PlantMaturityState},
    resources::{spawn_resource, ResourceCarrier, ResourceChunk, ResourceKind},
    skills::{SkillType, Skilled},
    stockpile::spawn_stockpile,
    tree::spawn_tree,
    worker_job_tooltip::{create_tooltip_bundle, WorkerJobTooltip},
    GameState,
};

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(init));
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
    world_params: Res<WorldParams>,
    mut commands: Commands,
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

    // RESOURCES
    for _ in 0..10 {
        let position = get_random_pos(Vec2::ZERO, world_params.size / 3.0);
        spawn_resource(
            &mut commands,
            &textures,
            ResourceChunk {
                kind: crate::resources::ResourceKind::Wood,
                quantity: 5,
            },
            position,
            false,
        );
    }

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
            required_resources: vec![(ResourceKind::Wood, 4)],
        };
        if let Some(new_texture) = get_construction_site_texture(0.0, 0.1, &building_prefab) {
            commands.entity(construction_site_id).insert(new_texture);
        }

        // convert_construction_site_to_building(construction_site_id, &mut commands, &house_textures);
    }

    // MOVE TO THE WORK ENTITY
    // Use move_to_work()

    for _ in 0..40 {
        let pos = get_random_pos(Vec2::ZERO, world_params.size / 3.0);
        spawn_tree(&mut commands, &textures, pos);
    }

    for _ in 0..40 {
        let position = get_random_pos(Vec2::ZERO, world_params.size / 3.0);
        let (prefab, texture) = plants.0.values().next().unwrap();
        spawn_plant(
            &mut commands,
            prefab,
            texture.clone(),
            position,
            &PlantMaturityState::FullyGrown,
        );
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

fn spawn_worker(
    commands: &mut Commands,
    textures: &Res<TextureAssets>,
    fonts: &Res<FontAssets>,
    position: Vec3,
) -> Entity {
    let bundle = WorkerBundle {
        skilled: Skilled {
            skills: HashMap::from([
                (SkillType::Building, 0.5),
                (SkillType::GrowingPlants, 0.5),
                (SkillType::None, 0.5),
            ]),
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
        .insert(Position(position))
        .insert(ResourceCarrier { max_volume: 120 })
        .with_children(|parent| {
            id = Some(parent.spawn(create_tooltip_bundle(13.0, &fonts)).id());
        })
        .insert(WorkerJobTooltip {
            title: "".to_string(),
            child: id.unwrap(),
        })
        .id()
}

#[derive(Bundle)]

struct WorkerBundle {
    skilled: Skilled,
    walker: Walker,
    position: Position,
    sprite: SpriteBundle,
}
