use crate::{
    common::{ClaimedBy, Countdown},
    tree::SimpleDestructible,
};
use bevy::prelude::{Commands, Component, Entity, Query};

enum AdvanceResult {
    Continuing(Countdown, SimpleDestructible),
    Completed,
}

#[derive(Component)]
struct NeedsDestroying;

#[derive(Component)]
pub struct TreeCutter {
    target_id: Entity,
}

#[derive(Component)]
pub struct TreeHitCountdown(Countdown);

// WHEN CREATING we need not to forget to add this task to a list of tasks to be cleaned up if the worker is destroyed
pub fn handle_task_progress(
    mut commands: Commands,
    tree_cutters_query: Query<(Entity, &TreeCutter, Option<&TreeHitCountdown>)>,
    mut destructibles: Query<&mut SimpleDestructible>,
) {
    for (worker_id, tree_cutter, maybe_tree_hit_countdown) in &tree_cutters_query {
        if let Ok(mut destructible) = destructibles.get_mut(tree_cutter.target_id) {
            let countdown = maybe_tree_hit_countdown
                .map(|x| x.0)
                .unwrap_or(Countdown::new(5));
            let result = advance(countdown, 20.0, destructible.clone());
            match result {
                AdvanceResult::Continuing(updated_countdown, updated_destructible) => {
                    *destructible = updated_destructible;
                    commands
                        .entity(worker_id)
                        .insert(TreeHitCountdown(updated_countdown));
                }
                AdvanceResult::Completed => {
                    commands
                        .entity(tree_cutter.target_id)
                        .insert(NeedsDestroying);
                    cleanup(&mut commands, worker_id, Some(tree_cutter.target_id));
                }
            }
        } else {
            cleanup(&mut commands, worker_id, None);
        }
    }
}

pub fn start_cutting_tree(commands: &mut Commands, worker_id: Entity, target_id: Entity) {
    commands.entity(target_id).insert(ClaimedBy(worker_id));
    commands.entity(worker_id).insert(TreeCutter { target_id });
}

fn advance(
    mut countdown: Countdown,
    task_effeciency: f32,
    mut simple_destructible: SimpleDestructible,
) -> AdvanceResult {
    countdown.tick();

    if countdown.is_done() {
        simple_destructible.current_health =
            (simple_destructible.current_health - task_effeciency).max(0.0);
        if simple_destructible.current_health == 0.0 {
            return AdvanceResult::Completed;
        }
    }

    AdvanceResult::Continuing(countdown, simple_destructible)
}

fn cleanup(commands: &mut Commands, worker_id: Entity, maybe_target_id: Option<Entity>) {
    commands.entity(worker_id).remove::<TreeCutter>();

    if let Some(target_id) = maybe_target_id {
        commands.entity(target_id).remove::<ClaimedBy>();
    }
}
