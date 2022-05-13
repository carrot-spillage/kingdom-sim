use std::collections::HashMap;

use bevy::{
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    prelude::{
        default, App, AssetServer, Bundle, Commands, Component, OrthographicCameraBundle, Plugin,
        Res, SystemSet, Transform,
    },
    sprite::{Sprite, SpriteBundle},
    text::Text2dBundle,
};
use rand::Rng;

use crate::{
    activity_info::create_activity_bundle,
    jobs::work_process::{SkillType, Skilled},
    movement::{Position, Walker},
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

fn init(world_params: Res<WorldParams>, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    for _ in 0..20 {
        spawn_worker(
            &mut commands,
            get_random_pos_in_world(&world_params),
            &asset_server,
        );
    }
}

fn get_random_pos_in_world(world_params: &WorldParams) -> Position {
    let mut rng = rand::thread_rng();
    let world_half = world_params.size / 2.0;
    Position(Vec3::new(
        rng.gen_range(-world_half.x..world_half.x),
        rng.gen_range(-world_half.y..world_half.y),
        0.0,
    ))
}

fn spawn_worker(commands: &mut Commands, position: Position, asset_server: &Res<AssetServer>) {
    let bundle = WorkerBundle {
        skilled: Skilled {
            skills: HashMap::from([(SkillType::PlantingCrops, 0.5)]),
        },
        walker: Walker {
            max_speed: 2.0,
            current_speed: 0.0,
            acceleration: 0.5,
        },
        position,
        sprite: SpriteBundle {
            texture: asset_server.load("textures/peasant.png"),
            transform: Transform {
                translation: position.0,
                ..Transform::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(12.0, 16.0)),
                ..Sprite::default()
            },
            ..Default::default()
        },
    };

    commands
        .spawn_bundle(bundle)
        .insert(position)
        .with_children(|parent| {
            parent.spawn_bundle(create_activity_bundle(13.0, &asset_server));
        });
}

#[derive(Component, Bundle)]

struct WorkerBundle {
    skilled: Skilled,
    walker: Walker,
    position: Position,

    #[bundle]
    sprite: SpriteBundle,
}