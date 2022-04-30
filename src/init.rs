use std::collections::HashMap;

use bevy::{
    ecs::bundle,
    math::{Vec2, Vec3},
    prelude::{
        App, AssetServer, Bundle, Commands, Component, OrthographicCameraBundle, Plugin, Res,
        ResMut, SystemSet, Transform,
    },
    sprite::SpriteBundle,
};
use rand::Rng;

use crate::{
    activity::{Job, WorkProcess, Working},
    movement::{Position, Walker},
    work_process::{SkillType, Skilled, WorkProcessState},
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
    mut asset_server: ResMut<AssetServer>,
    jobs: Res<Vec<Job>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    for _ in 0..20 {
        spawn_worker(
            &mut commands,
            get_random_pos_in_world(&world_params),
            &mut asset_server,
            &jobs,
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

fn spawn_worker(
    commands: &mut Commands,
    position: Position,
    asset_server: &mut ResMut<AssetServer>,
    jobs: &Res<Vec<Job>>,
) {
    let planting_crops_job_id = jobs[0].id;

    let work_process = WorkProcess {
        units_of_work: 2.0,
        job_id: planting_crops_job_id,
        max_workers: 1,
        state: WorkProcessState::IncompleteWorkProcessState {
            units_of_work_left: 0.0,
            quality_counter: crate::work_process::QualityCounter {
                points: 0.0,
                instances: 0,
            },
            work_chunks: vec![],
        },
        worker_ids: vec![],
        tentative_worker_ids: vec![],
    };

    let work_process_id = commands.spawn().insert(work_process).id();

    let bundle = WorkerBundle {
        skilled: Skilled {
            skills: HashMap::from([(SkillType::PlantingCrops, 0.5)]),
        },
        walker: Walker {
            max_speed: 2.0,
            current_speed: 0.0,
            acceleration: 0.5,
        },
        position: position,
        sprite: SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform {
                translation: position.0,
                ..Transform::default()
            },
            ..Default::default()
        },
    };

    commands
        .spawn_bundle(bundle)
        .insert(Working { work_process_id })
        .insert(position);
}

#[derive(Component, Bundle)]

struct WorkerBundle {
    skilled: Skilled,
    walker: Walker,
    position: Position,

    #[bundle]
    sprite: SpriteBundle,
}
