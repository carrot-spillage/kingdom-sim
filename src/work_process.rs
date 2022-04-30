#[derive(Clone)]
pub struct QualityCounter {
    pub points: f32,
    pub instances: u32,
}

#[derive(Clone)]
pub enum WorkProcessState {
    IncompleteWorkProcessState {
        units_of_work_left: f32,
        quality_counter: QualityCounter,
        work_chunks: Vec<WorkChunk>,
    },
    CompleteWorkProcessState {
        quality: f32,
    },
}

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
pub enum SkillType {
    PlantingCrops,
    Harvesting,
    Crafting,
}

#[derive(Component, Clone)]
pub struct Skilled {
    pub skills: HashMap<SkillType, f32>,
}

pub fn advance_work_process_state(
    workers: Vec<Skilled>,
    state: WorkProcessState,
    skill_struct: SkillType,
    interval: f32,
) -> WorkProcessState {
    match state {
        WorkProcessState::CompleteWorkProcessState { .. } => {
            panic!("CompleteWorkProcessState must not be passed here")
        }
        WorkProcessState::IncompleteWorkProcessState {
            units_of_work_left,
            quality_counter,
            mut work_chunks,
        } => {
            let mut new_work_chunks = calc_work_chunks(workers, skill_struct);
            let progress = calc_work_chunks_progress(&new_work_chunks, interval);
            let units_of_work_left = f32::max(units_of_work_left - progress, 0.0);

            let quality_counter = QualityCounter {
                instances: quality_counter.instances + new_work_chunks.len() as u32,
                points: quality_counter.points
                    + calc_work_chunks_quality(&new_work_chunks, interval),
            };

            if units_of_work_left > 0.0 {
                work_chunks.append(&mut new_work_chunks);

                return WorkProcessState::IncompleteWorkProcessState {
                    units_of_work_left,
                    quality_counter,
                    work_chunks,
                };
            } else {
                return WorkProcessState::CompleteWorkProcessState {
                    quality: quality_counter.points / quality_counter.instances as f32,
                };
            }
        }
    }
}

#[derive(Clone)]
pub struct WorkChunk {
    quality: f32,
    units_of_work: f32,
} // quality and progress go from 0.0 to 1.0

use std::collections::HashMap;

use bevy::prelude::{Component, Entity};

fn calc_work_chunks(workers: Vec<Skilled>, skill_type: SkillType) -> Vec<WorkChunk> {
    workers
        .iter()
        .map(|x| x.skills.get(&skill_type).unwrap())
        .map(|skill_value| WorkChunk {
            units_of_work: 0.5 + skill_value / 2.0,
            quality: *skill_value,
        })
        .collect()
}

fn calc_work_chunks_quality(worker_chunks: &Vec<WorkChunk>, interval: f32) -> f32 {
    worker_chunks
        .iter()
        .map(|x| x.quality * interval)
        .reduce(|a, b| a + b)
        .unwrap_or_default()
}

fn calc_work_chunks_progress(worker_chunks: &Vec<WorkChunk>, interval: f32) -> f32 {
    worker_chunks
        .iter()
        .map(|x| x.units_of_work)
        .reduce(|a, b| a + b)
        .unwrap_or_default()
        * interval
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
