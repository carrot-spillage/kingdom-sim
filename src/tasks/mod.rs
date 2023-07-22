mod tooltip;

use crate::{
    building::CreatureConstructingTask,
    creature::{schedule_collecting_items, schedule_dropping_items, schedule_transferring_items},
    cutting_tree::start_cutting_tree,
    harvesting::start_harvesting,
    movement::{MovingToEntity, MovingToPosition},
    planting::logic::{start_planting, Planting},
    GameState,
};
use bevy::prelude::{
    in_state, App, Commands, Component, Entity, IntoSystemConfigs, Plugin, Query, Update, Vec3,
    With,
};
use std::collections::VecDeque;

pub use self::tooltip::{create_tooltip_bundle, CreatureTaskTooltip};
use self::tooltip::{update_tooltip, update_tooltip_text};

pub struct TaskPlugin;

impl Plugin for TaskPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (proceed_to_next_task, update_tooltip_text, update_tooltip)
                .run_if(in_state(GameState::Playing)),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub enum CreatureTask {
    CutTree { target_id: Entity },
    Plant { planting: Planting },
    DropItems,
    CollectItems { target_id: Entity },
    TransferItems { target_id: Entity },
    Harvest { target_id: Entity },
    MoveToTarget { target_id: Entity },
    MoveToPosition { position: Vec3 },
    Build { target_id: Entity },
}

#[derive(Component)]
pub struct CreatureTaskStopping;

#[derive(Component)]
pub struct CreatureTasks(pub VecDeque<CreatureTask>);

#[derive(Component)]
pub struct IdlingCreature;

fn proceed_to_next_task(
    mut commands: Commands,
    mut idling_creatures: Query<(Entity, &mut CreatureTasks), With<IdlingCreature>>,
) {
    for (creature_id, mut tasks) in &mut idling_creatures {
        let next_task = tasks.0.pop_front().unwrap();
        commands
            .entity(creature_id)
            .remove::<IdlingCreature>()
            .insert(next_task);
        arrange_next_task(&mut commands, creature_id, next_task);
        if tasks.0.is_empty() {
            commands.entity(creature_id).remove::<CreatureTasks>();
        }
    }
}

fn arrange_next_task(commands: &mut Commands, creature_id: Entity, next_task_type: CreatureTask) {
    println!("next_task_type {:?}", next_task_type);
    match next_task_type {
        CreatureTask::MoveToTarget { target_id } => {
            commands.entity(creature_id).insert(MovingToEntity {
                destination_entity: target_id,
                sufficient_range: 20.0,
            });
        }
        CreatureTask::MoveToPosition { position } => {
            commands.entity(creature_id).insert(MovingToPosition {
                position,
                sufficient_range: 20.0,
            });
        }
        CreatureTask::CutTree { target_id } => {
            start_cutting_tree(commands, creature_id, target_id, 1.0);
        }
        CreatureTask::Harvest { target_id } => {
            start_harvesting(commands, creature_id, target_id, 1.0)
        }
        CreatureTask::Plant { planting } => start_planting(commands, creature_id, planting, 1.0),
        CreatureTask::DropItems => schedule_dropping_items(commands, creature_id),
        CreatureTask::CollectItems { target_id } => {
            schedule_collecting_items(commands, creature_id, target_id);
        }
        CreatureTask::TransferItems { target_id } => {
            schedule_transferring_items(commands, creature_id, target_id);
        }
        CreatureTask::Build { target_id } => {
            commands
                .entity(creature_id)
                .insert(CreatureConstructingTask {
                    construction_site_id: target_id,
                });
        }
    }
}
