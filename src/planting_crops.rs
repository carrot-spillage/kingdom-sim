use bevy::{
    math::{Vec2, Vec3},
    prelude::{
        App, Commands, Component, Entity, EventWriter, Image, Plugin, Query, Res,
        SystemSet, Transform,
    },
    asset::Handle,
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    loading::TextureAssets,
    movement::{hack_3d_position_to_2d, Position},
    planned_work::{PlannedWork, WorkerCompletedWorkEvent},
    skills::{SkillType, Skilled},
    work_progress::{advance_work_process_state, WorkProgress, WorkProgressUpdate},
    GameState,
};

pub struct PlantingCropsPlugin;

#[derive(Component)]
pub struct OccupiedArea(pub Vec2);

#[derive(Component)]
struct FarmFieldMaturity(pub f32);

#[derive(Component)]
struct MatureCrops;

static JOB_NAME: &'static str = "PlantingCrops";

impl Plugin for PlantingCropsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(handle_work_process)
                .with_system(grow),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn handle_work_process(
    mut commands: Commands,
    mut farm_fields: Query<(Entity, &PlannedWork, &Position, &mut WorkProgress)>,
    mut textured: Query<&mut Handle<Image>>,
    workers: Query<&Skilled>,
    mut worker_completion_events: EventWriter<WorkerCompletedWorkEvent>,
    textures: Res<TextureAssets>,
) {
    for (work_id, work, position, mut work_progress) in farm_fields.iter_mut() {
        if work.job_id != JOB_NAME {
            continue;
        }
        let farm_field_id = work_id;
        let workers: Vec<&Skilled> = work
            .worker_ids
            .iter()
            .map(|worker_id| workers.get(*worker_id).unwrap())
            .collect();

        if workers.is_empty() {
            continue;
        }

        if work_progress.units_of_work_left == work.units_of_work {
            spawn_farm_field_for_sowing(&mut commands, position.0, farm_field_id, &textures);
        }

        match advance_work_process_state(workers, &work_progress, SkillType::GrowingPlants) {
            WorkProgressUpdate::Complete { .. } => {
                for worker_id in work
                    .worker_ids
                    .iter()
                    .chain(work.tentative_worker_ids.iter())
                {
                    remove_work(&mut commands, work_id);

                    commands.entity(work_id).insert(FarmFieldMaturity(0.0));

                    worker_completion_events.send(WorkerCompletedWorkEvent {
                        worker_id: *worker_id,
                    })
                }
            }
            WorkProgressUpdate::Incomplete { progress, .. } => {
                if let Ok(mut texture) = textured.get_mut(work_id) {
                    (*texture) = get_farm_field_texture_based_on_sowing_progress(
                        1.0 - progress.units_of_work_left / work.units_of_work,
                        &textures,
                    );
                    // TODO: need some simpler visual representation of the progress then different textures
                }

                *work_progress = progress;
            }
        }
    }
}

fn grow(mut commands: Commands, mut crops: Query<(Entity, &mut FarmFieldMaturity)>) {
    for (entity, mut crop) in crops.iter_mut() {
        crop.0 += 0.01;
        if crop.0 >= 1.0 {
            commands
                .entity(entity)
                .remove::<FarmFieldMaturity>()
                .insert(MatureCrops);
        }
    }
}

pub fn plan_farm_field(commands: &mut Commands, position: Vec3) -> Entity {
    let units_of_work = 70.0;
    let size = Vec2::new(160.0, 160.0);

    commands
        .spawn_empty()
        .insert(PlannedWork::new(JOB_NAME, units_of_work, 1))
        .insert(WorkProgress::new(units_of_work))
        .insert(Position(position))
        .insert(OccupiedArea(size))
        .id()
}

fn remove_work(commands: &mut Commands, work_id: Entity) {
    commands
        .entity(work_id)
        .remove::<WorkProgress>()
        .remove::<PlannedWork>();
}

fn spawn_farm_field_for_sowing(
    commands: &mut Commands,
    position: Vec3,
    farm_field_id: Entity,
    textures: &Res<TextureAssets>,
) {
    commands.entity(farm_field_id).insert(SpriteBundle {
        texture: textures.farm_field_sowing_1.clone(),
        transform: Transform {
            translation: hack_3d_position_to_2d(position),
            ..Transform::default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..Sprite::default()
        },
        ..Default::default()
    });
}

fn get_farm_field_texture_based_on_sowing_progress(
    progress: f32,
    textures: &Res<TextureAssets>,
) -> Handle<Image> {
    if progress < 0.5 {
        return textures.farm_field_sowing_1.clone();
    } else if progress < 0.75 {
        return textures.farm_field_sowing_2.clone();
    } else {
        return textures.farm_field_sowing_3.clone();
    }
}
