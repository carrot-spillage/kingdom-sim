// import {
//   advance_work_process_state,
//   Entity,
//   IncompleteWorkProcessState,
//   SkillType,
//   Worker,
// } from "./advance_work_process";

// type ActivityUpdate = {
//   fatigue: number;
//   is_finished: boolean;
// };

use std::{
    collections::HashMap,
    iter::{FromFn, Map},
};

use bevy::prelude::Entity;

use crate::work_process::{get_most_skilled, QualityCounter, SkillType, Skilled, WorkProcessState};

#[derive(Clone, Copy)]
struct Job {
    id: Entity,
    name: &'static str,
    skill_type: SkillType,
}

fn create_job_generator(
    jobs: Vec<Job>,
    job_priorities: HashMap<Entity, f32>,
) -> impl Iterator<Item = Job> {
    let mut counter = 0;
    let mut accumulated_value_per_job: HashMap<_, _> = jobs.iter().map(|j| (j.id, 0.0)).collect();
    std::iter::from_fn(move || {
        let job = jobs[counter];

        let mut acc_value =
            accumulated_value_per_job.get(&job.id)? + job_priorities.get(&job.id)?;

        if acc_value >= 1.0 {
            acc_value -= 1.0;
            accumulated_value_per_job.insert(job.id, acc_value);
            return Some(job);
        }

        counter += 1;

        if counter >= jobs.len() {
            counter = 0;
        }

        return None;
    })
}

fn match_workers_with_jobs(
    workers_looking_for_jobs: &Vec<(Entity, Skilled)>,
    job_queue: impl Iterator<Item = Job>,
) -> Vec<(Entity, Job)> {
    let mut workers = workers_looking_for_jobs.clone();
    let workers_with_jobs: Vec<(Entity, Job)> = vec![];

    while workers.len() > 0 {
        let job = job_queue.next().unwrap();
        let top_worker = get_most_skilled(&workers, job.skill_type);
        workers.retain(|x| x.0 != top_worker);
        workers_with_jobs.push((top_worker, job));
    }
    return workers_with_jobs;
}

pub fn join_or_create_work_process(
    worker_id: Entity,
    job: &Job,
    available_work_processess: &Vec<WorkProcess>,
) -> JoinOrCreateWorkProcessResult {
    let available_process_index_opt = available_work_processess.iter().position(|x| {
        x.max_workers > (x.worker_ids.len() as u32) + (x.tentative_worker_ids.len() as u32)
            && job.id == x.job_id
    });

    if let Some(available_process_index) = available_process_index_opt {
        let work_process = join_work_process(
            &(available_work_processess)[available_process_index],
            worker_id,
        );
        return JoinOrCreateWorkProcessResult {
            _type: JoinOrCreateWorkProcessResultType::Joined,
            worker_id,
            work_process,
            updated_work_processes: available_work_processess
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    if i == available_process_index {
                        work_process
                    } else {
                        *x
                    }
                })
                .collect(),
        };
    } else {
        let work_process = create_work_process(worker_id, job);
        let mut updated_work_processes = *available_work_processess.clone();
        updated_work_processes.push(work_process);

        return JoinOrCreateWorkProcessResult {
            _type: JoinOrCreateWorkProcessResultType::Created,
            worker_id,
            work_process,
            updated_work_processes,
        };
    }
}

fn join_work_process(work_process: &WorkProcess, worker_id: Entity) -> WorkProcess {
    let mut tentative_worker_ids = work_process.tentative_worker_ids.clone();
    tentative_worker_ids.push(worker_id);
    return WorkProcess {
        tentative_worker_ids,
        ..*work_process
    };
}

fn create_work_process(worker_id: Entity, job: &Job) -> WorkProcess {
    let units_of_work = 10.0; // TODO: make this configurable
    return WorkProcess {
        job_id: job.id,
        max_workers: if job.name == "Harvesting" { 2 } else { 1 }, // TODO: make this configurable
        state: WorkProcessState::IncompleteWorkProcessState {
            quality_counter: QualityCounter {
                instances: 0,
                points: 0.0,
            },
            units_of_work_left: units_of_work,
            work_chunks: vec![],
        },
        units_of_work,
        tentative_worker_ids: vec![worker_id],
        worker_ids: vec![],
    };
}

struct WorkProcess {
    units_of_work: f32,
    job_id: Entity,
    max_workers: u32,

    state: WorkProcessState,
    worker_ids: Vec<Entity>,
    tentative_worker_ids: Vec<Entity>,
}

enum JoinOrCreateWorkProcessResultType {
    Joined,
    Created,
}

struct JoinOrCreateWorkProcessResult {
    _type: JoinOrCreateWorkProcessResultType,
    worker_id: Entity,
    work_process: WorkProcess,
    updated_work_processes: Vec<WorkProcess>,
}
