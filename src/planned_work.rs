use bevy::prelude::{App, Component, Entity, EventReader, Plugin, Query, SystemSet};

use crate::{activity_info::ActivityInfo, movement::ArrivedToEntityEvent, GameState};

pub static BUILDING_JOB_NAME: &'static str = "Building";

#[derive(Component)]
pub struct PlannedWork {
    pub units_of_work: f32,
    pub job_id: &'static str,
    pub max_workers: usize,
    pub worker_ids: Vec<Entity>,
    pub tentative_worker_ids: Vec<Entity>,
}

impl PlannedWork {
    pub fn new(job_id: &'static str, units_of_work: f32, max_workers: usize) -> Self {
        if max_workers == 0 {
            panic!("max_workers must be greater than 0");
        }

        return Self {
            job_id,
            max_workers,
            units_of_work,
            tentative_worker_ids: vec![],
            worker_ids: vec![],
        };
    }
}

#[derive(Component, Clone, Copy)]
pub struct WorksOn {
    pub work_id: Entity,
    pub job_id: &'static str,
}

pub struct WorkerCompletedWorkEvent {
    pub worker_id: Entity,
}

pub struct WorkOnArrivalPlugin;

impl Plugin for WorkOnArrivalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorkerCompletedWorkEvent>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(make_arrivals_work),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn make_arrivals_work(
    mut arrival_events: EventReader<ArrivedToEntityEvent>,
    mut activities: Query<&mut ActivityInfo>,
    workers: Query<&WorksOn>,
    mut work_query: Query<&mut PlannedWork>,
) {
    for event in arrival_events.iter() {
        let worker_id = event.moving_entity;
        if let Ok(works_on) = workers.get(worker_id) {
            let mut work = work_query.get_mut(works_on.work_id).unwrap();
            (*work).worker_ids.push(worker_id);
            (*work).tentative_worker_ids.retain(|x| *x != worker_id);

            let job_id = work.job_id;
            let mut activity = activities.get_mut(worker_id).unwrap();
            (*activity).title = format!("Working: {job_id}");
        }
    }
}
