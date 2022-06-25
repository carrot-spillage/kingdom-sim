use bevy::prelude::{
    App, Commands, Entity, EventWriter, Plugin, Query, SystemSet, With,
};

use crate::planned_work::{PlannedWork, WorkerCompletedWorkEvent};
use crate::resources::BreaksIntoResourcesEvent;
use crate::skills::{SkillType, Skilled};
use crate::tree::{SimpleDestructible, Tree};
use crate::work_progress::{advance_work_process_state, WorkProgress, WorkProgressUpdate};
use crate::GameState;
pub struct TreeCuttingJobPlugin;

static JOB_NAME: &'static str = "TreeCutting";

impl Plugin for TreeCuttingJobPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(handle_work));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn update_destructable(
    tree_id: Entity,
    delta: f32,
    trees: &mut Query<&mut SimpleDestructible, With<Tree>>,
) {
    let mut simple_destructible = trees.get_mut(tree_id).unwrap();
    let progress_factor = 2.0;
    (*simple_destructible).current_health =
        (simple_destructible.current_health - progress_factor * delta).max(0.0);
}

fn handle_work(
    mut commands: Commands,
    mut work_query: Query<(Entity, &PlannedWork, &mut WorkProgress)>,
    workers: Query<&Skilled>,
    mut worker_completion_events: EventWriter<WorkerCompletedWorkEvent>,
    mut breakages: EventWriter<BreaksIntoResourcesEvent>,
    mut trees: Query<&mut SimpleDestructible, With<Tree>>,
) {
    for (work_id, work, mut work_progress) in work_query.iter_mut() {
        let tree_id = work_id;
        let workers: Vec<&Skilled> = work
            .worker_ids
            .iter()
            .map(|worker_id| workers.get(*worker_id).unwrap())
            .collect();

        if workers.is_empty() {
            continue;
        }

        match advance_work_process_state(workers, &work_progress, SkillType::None) {
            WorkProgressUpdate::Complete { .. } => {
                for worker_id in work
                    .worker_ids
                    .iter()
                    .chain(work.tentative_worker_ids.iter())
                {
                    remove_work(&mut commands, work_id);

                    breakages.send(BreaksIntoResourcesEvent(tree_id));

                    worker_completion_events.send(WorkerCompletedWorkEvent {
                        worker_id: *worker_id,
                    })
                }
            }
            WorkProgressUpdate::Incomplete { progress, delta } => {
                update_destructable(work_id, delta, &mut trees);

                *work_progress = progress;
            }
        }
    }
}

pub fn plan_tree_cutting(commands: &mut Commands, tree_id: Entity) -> Entity {
    let units_of_work = 20.0;

    commands
        .entity(tree_id)
        .insert(PlannedWork::new(JOB_NAME, units_of_work, 1))
        .insert(WorkProgress::new(units_of_work))
        .id()
}

fn remove_work(commands: &mut Commands, work_id: Entity) {
    commands
        .entity(work_id)
        .remove::<WorkProgress>()
        .remove::<PlannedWork>();
}
