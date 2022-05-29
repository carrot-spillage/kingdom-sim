pub mod helpers;
pub mod systems;
pub mod work_process;

use itertools::Itertools;
use std::{any::Any, collections::HashMap};

use bevy::{
    math::{Vec2, Vec3},
    prelude::{
        App, Commands, Component, Entity, EventReader, EventWriter,
        ParallelSystemDescriptorCoercion, Plugin, Query, Res, ResMut, SystemSet, With, Without,
    },
};

use crate::{
    activity_info::ActivityInfo,
    init::{get_random_pos, WorldParams},
    movement::{ArrivalEvent, MovingToPosition},
    GameState, common::TargetOrPosition,
};

use self::{
    helpers::{join_work_process, match_workers_with_jobs},
    systems::{add_work_systems, Job},
    work_process::{QualityCounter, SkillType, Skilled, WorkProcessState, WorkProgress},
};

pub struct JobsPlugin;

impl Plugin for JobsPlugin {
    fn build(&self, app: &mut App) {
        let jobs: Vec<Job> = vec![];

        let job_priorities = jobs.iter().map(|j| (j.id, 0.5)).collect();
        app.insert_resource(JobQueue::new(jobs.clone(), job_priorities))
            .insert_resource(jobs);

        add_work_systems(app);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub struct JobQueue {
    pub jobs: Vec<Job>,
    counter: usize,
    accumulated_value_per_job: HashMap<&'static str, f32>,
    pub job_priorities: HashMap<&'static str, f32>,
}

impl JobQueue {
    pub fn new(jobs: Vec<Job>, job_priorities: HashMap<&'static str, f32>) -> Self {
        let accumulated_value_per_job = jobs.iter().map(|j| (j.id, 0.0)).collect();
        JobQueue {
            jobs,
            counter: 0,
            accumulated_value_per_job,
            job_priorities,
        }
    }

    pub fn add(&mut self, job: Job) {
        self.jobs.push(job);
        self.accumulated_value_per_job.insert(job.id, 0.0);
        self.job_priorities.insert(job.id, 0.5);
    }

    pub fn next(&mut self) -> Job {
        loop {
            let job = self.jobs[self.counter];

            let mut acc_value = self.accumulated_value_per_job.get(&job.id).unwrap()
                + self.job_priorities.get(&job.id).unwrap();
            if acc_value >= 1.0 {
                acc_value -= 1.0;
                self.accumulated_value_per_job.insert(job.id, acc_value);
                return job;
            }

            self.accumulated_value_per_job.insert(job.id, acc_value);

            self.counter += 1;

            if self.counter >= self.jobs.len() {
                self.counter = 0;
            }
        }
    }
}

#[derive(Component)]
pub struct AssignedToWorkProcess {
    pub work_process_id: Entity,
}

#[derive(Component, Clone)]
pub struct WorkProcess {
    pub units_of_work: f32,
    pub job_id: &'static str,
    pub max_workers: usize,

    pub progress: WorkProgress,
    pub worker_ids: Vec<Entity>,
    pub tentative_worker_ids: Vec<Entity>,
    pub target: TargetOrPosition,
}

impl WorkProcess {
    pub fn new(
        target: TargetOrPosition,
        job_id: &'static str,
        units_of_work: f32,
        max_workers: usize,
    ) -> Self {
        if max_workers == 0 {
            panic!("max_workers must be greater than 0");
        }

        return Self {
            job_id,
            max_workers,
            progress: WorkProgress {
                quality_counter: QualityCounter {
                    instances: 0,
                    points: 0.0,
                },
                units_of_work_left: units_of_work,
                work_chunks: vec![],
            },
            units_of_work,
            tentative_worker_ids: vec![],
            worker_ids: vec![],
            target,
        };
    }
}
