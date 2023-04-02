mod crafting_process;

use bevy::prelude::{Component, Entity};

pub use self::crafting_process::CraftingProcess;

#[derive(Clone, Copy, Debug)]
pub struct WorkParticipant {
    pub creature_id: Entity,
    pub proficiency: WorkProficiency,
}

#[derive(Clone, Copy, Debug)]
pub struct WorkProficiency {
    pub skill: f32,
    pub performance: f32,
} // each field has values from 0.0 to 1.0

#[derive(Clone, Debug)]
pub struct WorkQualityCounter {
    pub points: f32,
    pub instances: u32,
}

#[derive(Component, Clone, Debug)]
pub struct WorkProgress {
    pub units_of_work_left: f32,
    pub quality_counter: WorkQualityCounter,
    pub work_chunks: Vec<WorkProficiency>,
}

#[derive(Clone)]
pub enum WorkProgressUpdate {
    Incomplete { progress: WorkProgress, delta: f32 },
    Complete { quality: f32 },
}

impl WorkProgress {
    pub fn new(units_of_work: f32) -> Self {
        Self {
            quality_counter: WorkQualityCounter {
                instances: 0,
                points: 0.0,
            },
            units_of_work_left: units_of_work,
            work_chunks: vec![],
        }
    }
}

pub fn advance_work_progress(
    participants: &Vec<WorkParticipant>,
    state: &WorkProgress,
) -> WorkProgressUpdate {
    let WorkProgress {
        units_of_work_left,
        quality_counter,
        work_chunks,
    } = state;

    let mut new_work_chunks = participants.iter().map(|x| x.proficiency).collect();
    let progress = calc_work_chunks_progress(&new_work_chunks, 1.0);
    let units_of_work_left = f32::max(units_of_work_left - progress, 0.0);

    let quality_counter = WorkQualityCounter {
        instances: quality_counter.instances + new_work_chunks.len() as u32,
        points: quality_counter.points + calc_work_chunks_quality(&new_work_chunks, 1.0),
    };

    if units_of_work_left > 0.0 {
        let mut work_chunks_copy = work_chunks.clone(); // TODO: can we write something more elegant?
        work_chunks_copy.append(&mut new_work_chunks);
        return WorkProgressUpdate::Incomplete {
            progress: WorkProgress {
                units_of_work_left,
                quality_counter,
                work_chunks: work_chunks_copy,
            },
            delta: progress,
        };
    } else {
        return WorkProgressUpdate::Complete {
            quality: quality_counter.points / quality_counter.instances as f32,
        };
    }
}

pub fn calc_work_chunks_quality(worker_chunks: &Vec<WorkProficiency>, period: f32) -> f32 {
    worker_chunks
        .iter()
        .map(|x| x.skill * period)
        .reduce(|a, b| a + b)
        .unwrap_or_default()
}

pub fn calc_work_chunks_progress(work_chunks: &Vec<WorkProficiency>, period: f32) -> f32 {
    work_chunks
        .iter()
        .map(|x| x.performance)
        .reduce(|a, b| a + b)
        .unwrap_or_default()
        * period
}
