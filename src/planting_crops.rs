use bevy::prelude::{App, Commands, Component, Entity, EventReader, Plugin, Query, SystemSet};

use crate::common::TargetOrPosition;
use crate::jobs::helpers::register_job;
use crate::jobs::systems::{Job, WorkCompletedEvent, WorkProgressedEvent, WorkScheduledEvent};
use crate::jobs::work_process::SkillType;
use crate::GameState;

pub struct PlantingCropsPlugin;

static JOB_NAME: &'static str = "PlantingCrops";

#[derive(Component)]
pub struct FarmFieldReference(pub Entity);

#[derive(Component)]
pub struct FarmFieldHarvestable;

#[derive(Component)]
pub struct FarmField {
    readiness: f32,
}

impl Plugin for PlantingCropsPlugin {
    fn build(&self, app: &mut App) {
        register_job(app, Job::new(JOB_NAME, SkillType::None));

        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(handle_work_scheduled)
                .with_system(handle_work_progressed)
                .with_system(handle_work_completed),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn handle_work_scheduled(mut commands: Commands, mut events: EventReader<WorkScheduledEvent>) {
    for scheduled_event in events.iter().filter(|e| e.job_id == JOB_NAME) {
        let farm_field_id = match scheduled_event.target {
            TargetOrPosition::Target(tree_id) => tree_id,
            _ => panic!("Must have a target"),
        };
        commands
            .entity(scheduled_event.work_process_id)
            .insert(FarmFieldReference(farm_field_id));
    }
}

fn handle_work_progressed(
    mut events: EventReader<WorkProgressedEvent>,
    farm_field_references: Query<&FarmFieldReference>,
    mut farm_fields: Query<&mut FarmField>,
) {
    for progress_event in events.iter().filter(|e| e.job_id == JOB_NAME) {
        let farm_field_id = farm_field_references
            .get(progress_event.work_process_id)
            .unwrap()
            .0;

        let mut farm_field = farm_fields.get_mut(farm_field_id).unwrap();
        farm_field.readiness = progress_event.units_of_work_left / progress_event.units_of_work;
    }
}

fn handle_work_completed(
    mut commands: Commands,
    mut events: EventReader<WorkCompletedEvent>,
    target_references: Query<&FarmFieldReference>,
) {
    for event in events.iter().filter(|e| e.job_id == JOB_NAME) {
        let target_id = target_references.get(event.work_process_id).unwrap().0;
        commands.entity(target_id).insert(FarmFieldHarvestable);
    }
}

// #[derive(Component)]
// struct AxeSwing {
//     timer: Timer,
// }

// #[derive(Component)]
// pub struct Cutting(Entity);

// struct Damage(f32);

// fn advance_strikes(
//     mut commands: Commands,
//     mut q: Query<(Entity, &mut AxeSwing, &Cutting)>,
//     time: Res<Time>,
//     mut trees: Query<&mut SimpleDestructible, (With<Tree>, With<BreaksIntoResources>)>,
//     mut breakages: EventWriter<BreaksIntoResourcesEvent>,
// ) {
//     for (entity, mut swing, Cutting(tree_id)) in q.iter_mut() {
//         swing.timer.tick(time.delta());

//         if swing.timer.finished() {
//             commands.entity(entity).despawn();
//             let mut simple_destructible = trees.get_mut(*tree_id).unwrap();

//             let probability = 0.95;
//             let damage = 10.0;

//             (*simple_destructible).0 .0 = (simple_destructible.0 .0 - damage).max(0.0);

//             if simple_destructible.0 .0 <= 0.0 {
//                 commands.entity(*tree_id).despawn();
//                 breakages.send(BreaksIntoResourcesEvent(*tree_id));
//             }
//         }
//     }
// }
