#[derive(Clone)]
pub struct QualityCounter {
    pub points: f32,
    pub instances: u32,
}

#[derive(Clone)]
pub struct WorkProgress {
    pub units_of_work_left: f32,
    pub quality_counter: QualityCounter,
    pub work_chunks: Vec<WorkChunk>,
}

#[derive(Clone)]
pub enum WorkProcessState {
    IncompleteWorkProcessState(WorkProgress),
    CompleteWorkProcessState { quality: f32 },
}

#[derive(PartialEq, Clone, Copy, Hash, Eq, Debug)]
pub enum SkillType {
    PlantingCrops,
    Harvesting,
    Crafting,
    Building,
}

#[derive(Component, Clone, Debug)]
pub struct Skilled {
    pub skills: HashMap<SkillType, f32>,
}

pub fn advance_work_process_state(
    workers: Vec<&Skilled>,
    state: &WorkProgress,
    skill_type: SkillType,
) -> WorkProcessState {
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
        return WorkProcessState::IncompleteWorkProcessState(WorkProgress {
            units_of_work_left,
            quality_counter,
            work_chunks: work_chunks_copy,
        });
    } else {
        return WorkProcessState::CompleteWorkProcessState {
            quality: quality_counter.points / quality_counter.instances as f32,
        };
    }
}

#[derive(Clone)]
pub struct WorkChunk {
    quality: f32,
    units_of_work: f32,
} // quality and progress go from 0.0 to 1.0

use std::collections::HashMap;

use bevy::prelude::{Component, Entity};

fn calc_work_chunks(workers: Vec<&Skilled>, skill_type: SkillType) -> Vec<WorkChunk> {
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
