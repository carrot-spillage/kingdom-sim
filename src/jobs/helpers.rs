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

use bevy::{math::Vec3, prelude::Entity};

use crate::jobs::JobQueue;

use super::{
    work_process::{get_most_skilled, QualityCounter, Skilled, WorkProcessState},
    Job, WorkProcess,
};

pub fn match_workers_with_jobs(
    workers_looking_for_jobs: &Vec<(Entity, Skilled)>,
    job_queue: &mut JobQueue,
) -> Vec<(Entity, Job)> {
    let mut workers = (*workers_looking_for_jobs).clone();
    let mut workers_with_jobs: Vec<(Entity, Job)> = vec![];

    while workers.len() > 0 {
        let job = job_queue.next();
        let top_worker = get_most_skilled(&workers, job.skill_type);
        workers.retain(|x| x.0 != top_worker);
        workers_with_jobs.push((top_worker, job));
    }
    return workers_with_jobs;
}

pub fn join_work_process(work_process: &WorkProcess, worker_id: Entity) -> WorkProcess {
    let mut tentative_worker_ids = work_process.tentative_worker_ids.clone();
    tentative_worker_ids.push(worker_id);
    WorkProcess {
        tentative_worker_ids,
        ..work_process.clone()
    }
}

pub fn create_work_process(worker_id: Entity, position: Vec3, job: &Job) -> WorkProcess {
    let units_of_work = 200.0; // TODO: make this configurable
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
        position,
    };
}
