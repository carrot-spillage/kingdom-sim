use bevy::{prelude::{Entity, Commands, Query, Component}};
use crate::{common::{ClaimedBy, Countdown}, tree::SimpleDestructible};

enum AdvanceResult { Continuing(Countdown, SimpleDestructible), Completed }

#[derive(Component)]
struct NeedsDestroying;

#[derive(Component)]
pub struct TreeCutter {
    countdown: Countdown,
    target_id: Entity
}

// WHEN CREATING we need not to forget to add this task to a list of tasks to be cleaned up if the worker is destroyed
pub fn handle_task_progress(mut commands: Commands, mut tree_cutters_query: Query<(Entity, &mut TreeCutter)>, mut destructibles: Query<&mut SimpleDestructible>) {
    for (worker_id, mut tree_cutter) in &mut tree_cutters_query {
        if let Ok(mut destructible) = destructibles.get_mut(tree_cutter.target_id) {
            let result = advance(tree_cutter.countdown, 20.0, destructible.clone());
            match result {
                AdvanceResult::Continuing(updated_countdown, updated_destructible) => {
                    *destructible = updated_destructible;
                    (*tree_cutter).countdown = updated_countdown;
                }
                AdvanceResult::Completed => {
                    commands.entity(tree_cutter.target_id).insert(NeedsDestroying);
                    cleanup(&mut commands, worker_id, Some(tree_cutter.target_id));
                }
            }
        }
        else {
            cleanup(&mut commands, worker_id, None);
        }
    }
    
}

fn advance(mut countdown: Countdown, task_effeciency: f32, mut simple_destructible: SimpleDestructible) -> AdvanceResult {
    countdown.tick();

    if countdown.is_done() {
        simple_destructible.current_health = (simple_destructible.current_health - task_effeciency).max(0.0);
        if simple_destructible.current_health == 0.0 {
            return AdvanceResult::Completed;
        }
    }

    AdvanceResult::Continuing(countdown, simple_destructible)
}

fn cleanup(commands: &mut Commands, worker_id: Entity, maybe_destructable_id: Option<Entity>) {
    commands.entity(worker_id).remove::<TreeCutter>();

    if let Some(destructable_id) = maybe_destructable_id {
        commands.entity(destructable_id).remove::<ClaimedBy>();
    }
}
