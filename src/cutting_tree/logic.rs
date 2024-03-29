use crate::{
    common::{ClaimedBy, Countdown, NeedsDestroying, SimpleDestructible},
    tasks::{CreatureTask, IdlingCreature},
};
use bevy::prelude::{Commands, Component, Entity, Query};

#[derive(Debug)]
enum AdvanceResult {
    Continuing(Countdown, SimpleDestructible),
    Completed,
}

#[derive(Component)]
pub struct TreeCutter {
    target_id: Entity,
    performance: f32,
}

#[derive(Component)]
pub struct TreeHitCountdown(Countdown);

pub fn handle_task_progress(
    mut commands: Commands,
    mut tree_cutters_query: Query<(Entity, &TreeCutter, &mut TreeHitCountdown)>,
    mut destructibles: Query<&mut SimpleDestructible>,
) {
    for (worker_id, tree_cutter, mut tree_hit_countdown) in &mut tree_cutters_query {
        if let Ok(mut destructible) = destructibles.get_mut(tree_cutter.target_id) {
            let countdown = tree_hit_countdown.0;
            let result = advance(countdown, tree_cutter.performance, destructible.clone());

            match result {
                AdvanceResult::Continuing(updated_countdown, updated_destructible) => {
                    *destructible = updated_destructible;
                    *tree_hit_countdown = TreeHitCountdown(updated_countdown)
                }
                AdvanceResult::Completed => {
                    println!("Finished a tree");

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

pub fn start_cutting_tree(
    commands: &mut Commands,
    worker_id: Entity,
    target_id: Entity,
    performance: f32,
) {
    commands.entity(target_id).insert(ClaimedBy(worker_id));
    commands.entity(worker_id).insert((
        TreeCutter {
            target_id,
            performance,
        },
        TreeHitCountdown(Countdown::new((8.0 / performance).ceil() as u32)),
    ));
}

fn advance(
    mut countdown: Countdown,
    performance: f32,
    mut simple_destructible: SimpleDestructible,
) -> AdvanceResult {
    if countdown.tick_yield() {
        simple_destructible.current_health =
            (simple_destructible.current_health - (20.0 / performance)).max(0.0);
        if simple_destructible.current_health == 0.0 {
            return AdvanceResult::Completed;
        }
    }

    AdvanceResult::Continuing(countdown, simple_destructible)
}

fn cleanup(commands: &mut Commands, worker_id: Entity, maybe_target_id: Option<Entity>) {
    commands
        .entity(worker_id)
        .remove::<(CreatureTask, TreeCutter, TreeHitCountdown)>()
        .insert(IdlingCreature);

    if let Some(target_id) = maybe_target_id {
        commands.entity(target_id).remove::<ClaimedBy>();
    }
}
