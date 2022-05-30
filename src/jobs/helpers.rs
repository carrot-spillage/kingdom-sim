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

use bevy::prelude::{App, Entity};

use crate::jobs::JobQueue;

use super::{
    work_process::{get_most_skilled, Skilled},
    Job, WorkProcess,
};

pub fn register_job(app: &mut App, job: Job) {
    app.world.get_resource_mut::<JobQueue>().unwrap().add(job);
}

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
