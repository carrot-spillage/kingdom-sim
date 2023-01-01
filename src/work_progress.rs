use bevy::prelude::{Component, Entity};

use crate::skills::{SkillType, Skilled};

#[derive(Clone, Debug)]
pub struct QualityCounter {
    pub points: f32,
    pub instances: u32,
}

#[derive(Component, Clone, Debug)]
pub struct WorkProgress {
    pub units_of_work_left: f32,
    pub quality_counter: QualityCounter,
    pub work_chunks: Vec<WorkChunk>,
}

#[derive(Clone)]
pub enum WorkProgressUpdate {
    Incomplete { progress: WorkProgress, delta: f32 },
    Complete { quality: f32 },
}

impl WorkProgress {
    pub fn new(units_of_work: f32) -> Self {
        Self {
            quality_counter: QualityCounter {
                instances: 0,
                points: 0.0,
            },
            units_of_work_left: units_of_work,
            work_chunks: vec![],
        }
    }
}

pub fn advance_work_progress(
    workers: Vec<&Skilled>,
    state: &WorkProgress,
    skill_type: SkillType,
) -> WorkProgressUpdate {
    let WorkProgress {
        units_of_work_left,
        quality_counter,
        work_chunks,
    } = state;

    let mut new_work_chunks = calc_work_chunks(workers, skill_type);
    let progress = calc_work_chunks_progress(&new_work_chunks, 1.0);
    let units_of_work_left = f32::max(units_of_work_left - progress, 0.0);

    let quality_counter = QualityCounter {
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

#[derive(Clone, Debug)]
pub struct WorkChunk {
    quality: f32,
    units_of_work: f32,
} // quality and progress go from 0.0 to 1.0

pub fn calc_work_chunks(workers: Vec<&Skilled>, skill_type: SkillType) -> Vec<WorkChunk> {
    workers
        .iter()
        .map(|x| x.skills.get(&skill_type).unwrap())
        .map(|skill_value| WorkChunk {
            units_of_work: 0.5 + skill_value / 2.0,
            quality: *skill_value,
        })
        .collect()
}

pub fn calc_work_chunks_quality(worker_chunks: &Vec<WorkChunk>, period: f32) -> f32 {
    worker_chunks
        .iter()
        .map(|x| x.quality * period)
        .reduce(|a, b| a + b)
        .unwrap_or_default()
}

pub fn calc_work_chunks_progress(worker_chunks: &Vec<WorkChunk>, period: f32) -> f32 {
    worker_chunks
        .iter()
        .map(|x| x.units_of_work)
        .reduce(|a, b| a + b)
        .unwrap_or_default()
        * period
}

pub fn get_most_skilled(workers: &Vec<(Entity, Skilled)>, skill_type: SkillType) -> Entity {
    workers
        .iter()
        .max_by(|a, b| {
            a.1.skills
                .get(&skill_type)
                .unwrap()
                .partial_cmp(b.1.skills.get(&skill_type).unwrap())
                .unwrap()
        })
        .unwrap()
        .0
}
