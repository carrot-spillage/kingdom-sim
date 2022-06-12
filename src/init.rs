use std::collections::HashMap;

use bevy::{
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    prelude::{
        App, Bundle, Commands, Component, Entity, OrthographicCameraBundle, Plugin, Res, SystemSet,
        Transform,
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
    skills::{SkillType, Skilled},
    tree::spawn_tree,
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

pub struct WorldParams {
    pub size: Vec2,
}

fn init(
    world_params: Res<WorldParams>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
    fonts: Res<FontAssets>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

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

    let house_textures = BuildingTextureSet {
        in_progress: vec![textures.house_in_progress.clone()],
        completed: textures.house.clone(),
        scale: 0.03,
    };

    for _ in 0..5 {
        let pos = get_random_pos(Vec2::ZERO, world_params.size / 2.0);

        let construction_site_id = commands.spawn().id();
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
            skills: HashMap::from([(SkillType::Building, 0.5), (SkillType::None, 0.5)]),
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
        .spawn_bundle(bundle)
        .insert(Position(position))
        .with_children(|parent| {
            id = Some(
                parent
                    .spawn_bundle(create_activity_bundle(13.0, &fonts))
                    .id(),
            );
        })
        .insert(ActivityInfo {
            title: "".to_string(),
            child: id.unwrap(),
        })
        .id()
}

#[derive(Component, Bundle)]

struct WorkerBundle {
    skilled: Skilled,
    walker: Walker,
    position: Position,

    #[bundle]
    sprite: SpriteBundle,
}
