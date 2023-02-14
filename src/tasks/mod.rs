mod tooltip;

use crate::{
    cutting_tree::start_cutting_tree,
    harvesting::start_harvesting,
    movement::{MovingToEntity, MovingToPosition},
    planting::logic::{start_planting, Planting},
    worker::schedule_dropping_items,
    GameState,
};
use bevy::prelude::{App, Commands, Component, Entity, Plugin, Query, SystemSet, Vec3, With};
use std::collections::VecDeque;

pub use self::tooltip::{create_tooltip_bundle, WorkerTaskTooltip};
use self::tooltip::{update_tooltip, update_tooltip_text};

pub struct TaskPlugin;

impl Plugin for TaskPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(proceed_to_next_task)
                .with_system(update_tooltip_text)
                .with_system(update_tooltip),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub enum WorkerTask {
    CutTree { target_id: Entity },
    Plant { planting: Planting },
    DropItems,
    Harvest { target_id: Entity },
    MoveToTarget { target_id: Entity },
    MoveToPosition { position: Vec3 },
}

#[derive(Component)]
pub struct WorkerTasks(pub VecDeque<WorkerTask>);

#[derive(Component)]
pub struct IdlingWorker;

fn proceed_to_next_task(
    mut commands: Commands,
    mut idling_workers: Query<(Entity, &mut WorkerTasks), With<IdlingWorker>>,
) {
    for (worker_id, mut tasks) in &mut idling_workers {
        let next_task = tasks.0.pop_front().unwrap();
        commands
            .entity(worker_id)
            .remove::<IdlingWorker>()
            .insert(next_task);
        arrange_next_task(&mut commands, worker_id, next_task);
        if tasks.0.is_empty() {
            commands.entity(worker_id).remove::<WorkerTasks>();
        }
    }
}

fn arrange_next_task(commands: &mut Commands, worker_id: Entity, next_task: WorkerTask) {
    match next_task {
        WorkerTask::MoveToTarget { target_id } => {
            commands.entity(worker_id).insert(MovingToEntity {
                destination_entity: target_id,
                sufficient_range: 20.0,
            });
        }
        WorkerTask::MoveToPosition { position } => {
            commands.entity(worker_id).insert(MovingToPosition {
                position,
                sufficient_range: 20.0,
            });
        }
        WorkerTask::CutTree { target_id } => {
            start_cutting_tree(commands, worker_id, target_id, 1.0);
        }
        WorkerTask::Harvest { target_id } => start_harvesting(commands, worker_id, target_id, 1.0),
        WorkerTask::Plant { planting } => start_planting(commands, planting, worker_id, 1.0),
        WorkerTask::DropItems => schedule_dropping_items(commands, worker_id),
    }
}
