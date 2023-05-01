use bevy::prelude::{
    App, Changed, Commands, Component, Entity, IntoSystemConfig, OnUpdate, Plugin, Query,
};

use crate::{
    items::{add_batches_to, ItemBatch},
    GameState,
};

use super::{
    calc_work_chunks_progress, calc_work_chunks_quality, WorkParticipant, WorkQualityCounter,
};

#[derive(Component, Clone, Debug)]
pub struct CraftingProcess {
    pub units_of_work_left: f32,
    pub resources_per_unit_of_work: f32,
    pub quality_counter: WorkQualityCounter,
    pub item_batches: Vec<ItemBatch>,
}

#[derive(Component)]
pub struct CraftingProcessCanContinue;

#[derive(Clone)]
pub enum CraftingProcessUpdate {
    Incomplete { delta: f32 },
    InsufficientResources,
    Complete { quality: f32 },
}

pub struct CraftingProcessPlugin;

impl Plugin for CraftingProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(set_can_continue.in_set(OnUpdate(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn set_can_continue(
    mut commands: Commands,
    processes: Query<
        (
            Entity,
            &CraftingProcess,
            Option<&CraftingProcessCanContinue>,
        ),
        Changed<CraftingProcess>,
    >,
) {
    for (entity, process, maybe_can_continue) in &processes {
        if process.can_continue() && !maybe_can_continue.is_some() {
            commands.entity(entity).insert(CraftingProcessCanContinue);
        } else if !process.can_continue() && maybe_can_continue.is_some() {
            commands
                .entity(entity)
                .remove::<CraftingProcessCanContinue>();
        }
    }
}

impl CraftingProcess {
    pub fn new(units_of_work: f32, initially_required_resources: Vec<ItemBatch>) -> Self {
        Self {
            quality_counter: WorkQualityCounter {
                instances: 0,
                points: 0.0,
            },
            units_of_work_left: units_of_work,
            resources_per_unit_of_work: (initially_required_resources
                .iter()
                .map(|x| x.quantity)
                .sum::<u32>() as f32
                / units_of_work),
            item_batches: initially_required_resources
                .iter()
                .map(|initially_required| ItemBatch {
                    prefab_id: initially_required.prefab_id,
                    quantity: 0,
                })
                .collect(),
        }
    }

    pub fn can_continue(&self) -> bool {
        !self.item_batches.is_empty()
    }

    pub fn accept_batches(&mut self, item_batches: &mut Vec<ItemBatch>) {
        add_batches_to(&mut self.item_batches, item_batches);
    }

    pub fn advance(
        &mut self,
        participants: Vec<WorkParticipant>,
        period: f32, // 1.0
    ) -> CraftingProcessUpdate {
        if self.units_of_work_left == 0.0 {
            panic!("This process has already completed")
        }

        let CraftingProcess {
            units_of_work_left, ..
        } = self;

        if self.item_batches.iter().any(|x| x.quantity == 0) {
            return CraftingProcessUpdate::InsufficientResources;
        }
        // check that there is enough resources here to work on?

        let new_work_chunks = participants.iter().map(|x| x.proficiency).collect();
        let units_of_work_progress = calc_work_chunks_progress(&new_work_chunks, period);
        self.units_of_work_left = f32::max(*units_of_work_left - units_of_work_progress, 0.0);

        self.quality_counter.instances += new_work_chunks.len() as u32;
        self.quality_counter.points += calc_work_chunks_quality(&new_work_chunks, period);

        if self.units_of_work_left == 0.0 {
            return CraftingProcessUpdate::Complete {
                quality: self.quality_counter.points / self.quality_counter.instances as f32,
            };
        }

        for resource in &mut self.item_batches {
            try_consume_item_batch(
                resource,
                (units_of_work_progress * self.resources_per_unit_of_work).ceil() as u32,
            );
        }

        return CraftingProcessUpdate::Incomplete {
            delta: units_of_work_progress,
        };
    }
}

fn try_consume_item_batch(batch: &mut ItemBatch, amount: u32) -> Option<u32> {
    let to_be_consumed = batch.quantity - amount;
    if to_be_consumed > 0 {
        batch.quantity -= to_be_consumed as u32;
        Some(to_be_consumed as u32)
    } else {
        None
    }
}
