use std::collections::HashMap;

use bevy::{
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    prelude::{
        App, Bundle, Commands, Entity, Plugin, Res, SystemSet,
        Transform, Camera2dBundle, Resource,
    },
    sprite::{Sprite, SpriteBundle},
};
use rand::Rng;

use crate::{
    activity_info::{create_activity_bundle, ActivityInfo},
    building::{
        convert_construction_site_to_building, spawn_construction_site, BuildingTextureSet,
    },
    loading::{FontAssets, TextureAssets},
    monkey_planner::MonkeyPlanner,
    movement::{hack_3d_position_to_2d, Position, Walker},
    planting_crops::plan_farm_field,
    resource_gathering::plan_resource_gathering,
    resources::ResourceCarrier,
    skills::{SkillType, Skilled},
    stockpile::spawn_stockpile,
    tree::spawn_tree,
    tree_cutting_job::plan_tree_cutting,
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
) {
    commands.spawn(Camera2dBundle::default());

    for _ in 0..1 {
        let pos = get_random_pos(Vec2::ZERO, world_params.size / 2.0);
        let worker_id = spawn_worker(&mut commands, &textures, &fonts, pos);

        let work_id = MonkeyPlanner::plan_house(
            &mut commands,
            &textures,
            get_random_pos(Vec2::ZERO, world_params.size / 4.0),
        );

        MonkeyPlanner::temp_recruit_workers(&mut commands, work_id, vec![worker_id])
    }

    {
        let pos = get_random_pos(Vec2::ZERO, world_params.size / 2.0);
        spawn_stockpile(&mut commands, pos, Vec2::new(100.0, 100.0));
        
    }

    for _ in 0..1 {
        let pos = get_random_pos(Vec2::ZERO, world_params.size / 2.0);
        let worker_id = spawn_worker(&mut commands, &textures, &fonts, pos);

        let work_id = plan_farm_field(
            &mut commands,
            get_random_pos(Vec2::ZERO, world_params.size / 4.0),
        );

        MonkeyPlanner::temp_recruit_workers(&mut commands, work_id, vec![worker_id])
    }

    for _ in 0..2 {
        let worker_pos = get_random_pos(Vec2::ZERO, world_params.size / 3.0);
        let worker_id = spawn_worker(&mut commands, &textures, &fonts, worker_pos);
        let tree_pos = get_random_pos(Vec2::ZERO, world_params.size / 3.0);

        let tree_id = spawn_tree(&mut commands, &textures, tree_pos);
        let work_id = plan_tree_cutting(&mut commands, tree_id);

        MonkeyPlanner::temp_recruit_workers(&mut commands, work_id, vec![worker_id])
    }

    for _ in 0..1 {
        let worker_pos = get_random_pos(Vec2::ZERO, world_params.size / 3.0);
        let worker_id = spawn_worker(&mut commands, &textures, &fonts, worker_pos);
        plan_resource_gathering(&mut commands, worker_id);
    }

    let house_textures = BuildingTextureSet {
        in_progress: vec![textures.house_in_progress.clone()],
        completed: textures.house.clone(),
        scale: 0.03,
    };

    for _ in 0..5 {
        let pos = get_random_pos(Vec2::ZERO, world_params.size / 2.0);

        let construction_site_id = commands.spawn_empty().id();
        spawn_construction_site(&mut commands, construction_site_id, pos, &house_textures);
        convert_construction_site_to_building(construction_site_id, &mut commands, &house_textures);
    }

    for _ in 0..40 {
        let pos = get_random_pos(Vec2::ZERO, world_params.size / 3.0);
        spawn_tree(&mut commands, &textures, pos);
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
            id = Some(
                parent
                    .spawn(create_activity_bundle(13.0, &fonts))
                    .id(),
            );
        })
        .insert(ActivityInfo {
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
