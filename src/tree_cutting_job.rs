use bevy::core::{Time, Timer};
use bevy::prelude::{
    App, AssetServer, Commands, Component, Entity, EventReader, EventWriter, Handle, Image, Plugin,
    Query, Res, SystemSet, With,
};

use crate::common::TargetOrPosition;
use crate::jobs::helpers::register_job;
use crate::jobs::systems::{Job, WorkCompletedEvent, WorkProgressedEvent, WorkScheduledEvent};
use crate::jobs::work_process::SkillType;
use crate::loading::TextureAssets;
use crate::resources::{BreaksIntoResources, BreaksIntoResourcesEvent};
use crate::tree::{SimpleDestructible, Tree};
use crate::GameState;

pub struct TreeCuttingJobPlugin;

#[derive(Component)]
pub struct TreeReference(pub Entity);

static JOB_NAME: &'static str = "TreeCutting";

impl Plugin for TreeCuttingJobPlugin {
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

fn handle_work_scheduled(
    mut commands: Commands,
    mut events: EventReader<WorkScheduledEvent>,
    textures: Res<TextureAssets>,
) {
    for scheduled_event in events.iter().filter(|e| e.job_id == JOB_NAME) {
        let tree_id = match scheduled_event.target {
            TargetOrPosition::Target(tree_id) => tree_id,
            _ => panic!("Must have a target"),
        };
        commands
            .entity(scheduled_event.work_process_id)
            .insert(TreeReference(tree_id));
    }
}

fn handle_work_progressed(
    mut events: EventReader<WorkProgressedEvent>,
    tree_references: Query<&TreeReference>,
    textures: Res<TextureAssets>,
    mut trees: Query<&mut SimpleDestructible, With<Tree>>,
) {
    for progress_event in events.iter().filter(|e| e.job_id == JOB_NAME) {
        let tree_id = tree_references
            .get(progress_event.work_process_id)
            .unwrap()
            .0;
        let mut simple_destructible = trees.get_mut(tree_id).unwrap();

        let progress_percentage = progress_event.units_of_work_left / progress_event.units_of_work;
        (*simple_destructible).current_health =
            (simple_destructible.max_health * progress_percentage).max(0.0);
    }
}

fn handle_work_completed(
    mut commands: Commands,
    mut events: EventReader<WorkCompletedEvent>,
    tree_references: Query<&TreeReference>,
    textures: Res<TextureAssets>,
    mut breakages: EventWriter<BreaksIntoResourcesEvent>,
) {
    for event in events.iter().filter(|e| e.job_id == JOB_NAME) {
        let tree_id = tree_references.get(event.work_process_id).unwrap().0;
        commands.entity(tree_id).despawn();
        println!("Despawning tree {:?}", tree_id);
        breakages.send(BreaksIntoResourcesEvent(tree_id));
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
