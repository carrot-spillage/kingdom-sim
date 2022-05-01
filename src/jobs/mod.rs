pub mod helpers;

use std::collections::HashMap;

use bevy::prelude::{App, Commands, Entity, Plugin, Query, Res, ResMut, SystemSet, Without, Component};

use crate::{
    work_process::{SkillType, Skilled},
    GameState,
};

use self::helpers::{match_workers_with_jobs};

pub struct JobsPlugin;

impl Plugin for JobsPlugin {
    fn build(&self, app: &mut App) {
        let jobs = vec![Job {
            id: 1,
            name: "PlantingCrops",
            skill_type: SkillType::PlantingCrops,
        }];

        let job_priorities = jobs.iter().map(|j| (j.id, 0.5)).collect();
        app.insert_resource(JobQueue::new(jobs.clone(), job_priorities));
        app.insert_resource(jobs);
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(assign_jobs_to_workers),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub struct JobQueue {
    jobs: Vec<Job>,
    counter: usize,
    accumulated_value_per_job: HashMap<u32, f32>,
    pub job_priorities: HashMap<u32, f32>,
}

impl JobQueue {
    pub fn new(jobs: Vec<Job>, job_priorities: HashMap<u32, f32>) -> Self {
        let accumulated_value_per_job = jobs.iter().map(|j| (j.id, 0.0)).collect();
        JobQueue {
            jobs,
            counter: 0,
            accumulated_value_per_job,
            job_priorities,
        }
    }

    pub fn next(&mut self) -> Option<Job> {
        let job = self.jobs[self.counter];

        let mut acc_value =
            self.accumulated_value_per_job.get(&job.id)? + self.job_priorities.get(&job.id)?;

        if acc_value >= 1.0 {
            acc_value -= 1.0;
            self.accumulated_value_per_job.insert(job.id, acc_value);
            return Some(job);
        }

        self.counter += 1;

        if self.counter >= self.jobs.len() {
            self.counter = 0;
        }

        return None;
    }
}

fn assign_jobs_to_workers(
    mut commands: Commands,
    jobs: Res<Vec<Job>>,
    mut job_queue: ResMut<JobQueue>,
    workers_looking_for_jobs: Query<(Entity, &Skilled), Without<Working>>,
) {
    let all_workers = workers_looking_for_jobs
        .iter()
        .map(|(entity, s)| (entity, s.clone()))
        .collect::<Vec<_>>();
    match_workers_with_jobs(&all_workers, &mut job_queue);

    // let planting_crops_job_id = jobs[0].id;

    // let work_process = WorkProcess {
    //     units_of_work: 2.0,
    //     job_id: planting_crops_job_id,
    //     max_workers: 1,
    //     state: WorkProcessState::IncompleteWorkProcessState {
    //         units_of_work_left: 0.0,
    //         quality_counter: crate::work_process::QualityCounter {
    //             points: 0.0,
    //             instances: 0,
    //         },
    //         work_chunks: vec![],
    //     },
    //     worker_ids: vec![],
    //     tentative_worker_ids: vec![],
    // };

    // let work_process_id = commands.spawn().insert(work_process).id();

    // let bundle = WorkerBundle {
    //     skilled: Skilled {
    //         skills: HashMap::from([(SkillType::PlantingCrops, 0.5)]),
    //     },
    //     walker: Walker {
    //         max_speed: 2.0,
    //         current_speed: 0.0,
    //         acceleration: 0.5,
    //     },
    //     position: position,
    //     sprite: SpriteBundle {
    //         texture: asset_server.load("assets/bevy.png"),
    //         transform: Transform {
    //             translation: position.0,
    //             ..Transform::default()
    //         },
    //         ..Default::default()
    //     },
    // };

    // commands
    //     .spawn_bundle(bundle)
    //     .insert(Working { work_process_id })
    //     .insert(position);
}

#[derive(Component, Clone, Copy)]
pub struct Job {
    pub id: u32,
    pub name: &'static str,
    pub skill_type: SkillType,
}

#[derive(Component)]
pub struct Working {
    pub work_process_id: Entity,
}
