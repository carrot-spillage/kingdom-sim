use crate::common::{ClaimedBy, Countdown, SimpleDestructible};
use bevy::prelude::{Commands, Component, Entity, Query};

enum AdvanceResult {
    Continuing(Countdown, SimpleDestructible),
    Completed,
}

#[derive(Component)]
pub struct Harvester {
    target_id: Entity,
}

#[derive(Component)]
pub struct HarvestBatchCountdown(Countdown);

pub fn handle_task_progress(
    mut commands: Commands,
    mut tree_cutters_query: Query<(Entity, &Harvester, &mut HarvestBatchCountdown)>,
    mut destructibles: Query<&mut SimpleDestructible>,
) {
    for (worker_id, tree_cutter, mut tree_hit_countdown) in &mut tree_cutters_query {
        if let Ok(mut destructible) = destructibles.get_mut(tree_cutter.target_id) {
            let countdown = tree_hit_countdown.0;
            let result = advance(countdown, 20.0, destructible.clone());
            match result {
                AdvanceResult::Continuing(updated_countdown, updated_destructible) => {
                    *destructible = updated_destructible;
                    *tree_hit_countdown = HarvestBatchCountdown(updated_countdown)
                }
                AdvanceResult::Completed => {
                    cleanup(&mut commands, worker_id, Some(tree_cutter.target_id));
                }
            }
        } else {
            cleanup(&mut commands, worker_id, None);
        }
    }
}

pub fn start_harvesting(
    commands: &mut Commands,
    worker_id: Entity,
    hit_interval: usize,
    target_id: Entity,
) {
    commands.entity(target_id).insert(ClaimedBy(worker_id));
    commands.entity(worker_id).insert((
        Harvester { target_id },
        HarvestBatchCountdown(Countdown::new(hit_interval)),
    ));
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
    commands
        .entity(worker_id)
        .remove::<(Harvester, HarvestBatchCountdown)>();

    if let Some(target_id) = maybe_target_id {
        commands.entity(target_id).remove::<ClaimedBy>();
    }
}
