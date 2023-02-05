use bevy::prelude::{App, Commands, Component, Entity, Plugin, Query, SystemSet, With};

use crate::{
    cutting_tree::start_cutting_tree,
    harvesting::start_harvesting,
    planting::logic::{start_planting, Planting},
    GameState,
};

pub struct TaskPlugin;

impl Plugin for TaskPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(proceed_to_next_task),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub enum WorkerTask {
    CutTree { target_id: Entity },
    Plant { planting: Planting },
    Harvest { target_id: Entity },
    MoveTo { target_id: Entity },
}

#[derive(Component)]
pub struct WorkerTasks(Vec<WorkerTask>);

#[derive(Component)]
pub struct IdlingWorker;

fn proceed_to_next_task(
    mut commands: Commands,
    mut idling_workers: Query<(Entity, &mut WorkerTasks), With<IdlingWorker>>,
) {
    for (worker_id, mut tasks) in &mut idling_workers {
        let next_task = tasks.0.pop().unwrap();
        arrange_next_task(&mut commands, worker_id, next_task);
        if tasks.0.is_empty() {
            commands.entity(worker_id).remove::<WorkerTasks>();
        }
    }
}

fn arrange_next_task(commands: &mut Commands, worker_id: Entity, next_task: WorkerTask) {
    match next_task {
        WorkerTask::CutTree { target_id } => {
            start_cutting_tree(commands, worker_id, target_id, 1.0)
        }
        WorkerTask::Harvest { target_id } => start_harvesting(commands, worker_id, target_id, 1.0),
        WorkerTask::Plant { planting } => start_planting(commands, planting, worker_id, 1.0),
        WorkerTask::MoveTo { target_id } => todo!(),
    }
}
