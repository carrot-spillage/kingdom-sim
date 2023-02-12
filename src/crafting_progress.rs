use bevy::prelude::Component;

use crate::{
    resources::ResourceKind,
    skills::{SkillType, Skilled},
    work_progress::{
        calc_work_chunks, calc_work_chunks_progress, calc_work_chunks_quality, QualityCounter,
        WorkChunk,
    },
};

#[derive(Component, Clone, Debug)]
pub struct CraftingProgress {
    pub units_of_work_left: f32,
    pub quality_counter: QualityCounter,
    pub work_chunks: Vec<WorkChunk>,
    pub resource_states: Vec<ConstructionResourceState>,
}

#[derive(Clone, Copy, Debug)]
pub struct ConstructionResourceState {
    kind: ResourceKind,
    initially_required: u32,
    consumed: u32,
    available: u32,
}

impl ConstructionResourceState {
    fn new(kind: ResourceKind, initially_required: u32) -> Self {
        Self {
            kind,
            initially_required,
            consumed: 0,
            available: 0,
        }
    }

    fn try_consume(mut self, amount: u32) -> Option<u32> {
        let to_be_consumed = self.available - amount;
        if to_be_consumed > 0 {
            self.consumed += to_be_consumed as u32;
            self.available -= to_be_consumed as u32;
            Some(to_be_consumed as u32)
        } else {
            None
        }
    }

    fn add(self, amount: u32) {
        if amount > self.get_missing() {
            panic!("Cannot add more resources that needed in total")
        }
    }

    fn get_missing(self) -> u32 {
        self.initially_required - self.consumed - self.available
    }
}

#[derive(Clone)]
pub enum CraftingProgressUpdate {
    Incomplete {
        progress: CraftingProgress,
        delta: f32,
    },
    NotEnoughResources,
    Complete {
        quality: f32,
    },
}

impl CraftingProgress {
    pub fn new(units_of_work: f32, initially_required_resources: Vec<(ResourceKind, u32)>) -> Self {
        Self {
            quality_counter: QualityCounter {
                instances: 0,
                points: 0.0,
            },
            units_of_work_left: units_of_work,
            work_chunks: vec![],
            resource_states: initially_required_resources
                .iter()
                .map(|(kind, initially_required)| {
                    ConstructionResourceState::new(*kind, *initially_required)
                })
                .collect(),
        }
    }
}

pub fn advance_crafting_process_state(
    workers: Vec<&Skilled>,
    state: &mut CraftingProgress,
    skill_type: SkillType,
    initially_required_units_of_work: f32,
    period: f32, // 1.0
) -> CraftingProgressUpdate {
    let CraftingProgress {
        units_of_work_left,
        quality_counter,
        work_chunks,
        ..
    } = state;

    if state.resource_states.iter().any(|x| x.available == 0) {
        return CraftingProgressUpdate::NotEnoughResources;
    }
    // check that there is enough resources here to work on?

    let mut new_work_chunks = calc_work_chunks(workers, skill_type);
    let progress = calc_work_chunks_progress(&new_work_chunks, period);
    let units_of_work_left = f32::max(*units_of_work_left - progress, 0.0);

    let quality_counter = QualityCounter {
        instances: quality_counter.instances + new_work_chunks.len() as u32,
        points: quality_counter.points + calc_work_chunks_quality(&new_work_chunks, period),
    };

    if units_of_work_left == 0.0 {
        return CraftingProgressUpdate::Complete {
            quality: quality_counter.points / quality_counter.instances as f32,
        };
    }

    for resource in state.resource_states.iter_mut() {
        resource.try_consume(
            ((progress / initially_required_units_of_work) * resource.initially_required as f32)
                .ceil() as u32,
        );
    }

    let mut work_chunks_copy = work_chunks.clone(); // TODO: can we write something more elegant?
    work_chunks_copy.append(&mut new_work_chunks);

    return CraftingProgressUpdate::Incomplete {
        progress: CraftingProgress {
            units_of_work_left,
            quality_counter,
            work_chunks: work_chunks_copy,
            resource_states: state.resource_states.clone(),
        },
        delta: progress,
    };
}
