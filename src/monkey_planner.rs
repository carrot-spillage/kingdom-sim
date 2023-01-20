use bevy::{
    math::Vec3,
    prelude::{Commands, Entity, Query, Res, With, Without},
    utils::HashMap,
};

use crate::{
    building::{BuildingPrefab, BuildingTextureSet},
    building_job::plan_building,
    loading::TextureAssets,
    movement::MovingToEntity,
    planned_work::{NotWorking, PlannedWork, WorkingOn, BUILDING_JOB_NAME},
    resources::ResourceKind,
    skills::Skilled,
};

pub struct MonkeyPlanner;

impl MonkeyPlanner {
    pub fn recruit_workers_to_build(
        self,
        commands: &mut Commands,
        work_id: Entity,
        work_query: &mut Query<&mut PlannedWork>,
        workers: Query<Entity, (With<Skilled>, Without<WorkingOn>)>,
    ) {
        let work = work_query.get(work_id).unwrap();
        MonkeyPlanner::temp_recruit_workers(
            commands,
            work_id,
            workers.iter().take(work.max_workers).collect(),
            BUILDING_JOB_NAME,
        )
    }

    pub fn temp_recruit_workers(
        commands: &mut Commands,
        work_id: Entity,
        worker_ids: Vec<Entity>,
        job_id: &'static str,
    ) {
        for worker_id in worker_ids {
            move_to_work(commands, worker_id, work_id);
        }
    }

    pub fn plan_house(
        commands: &mut Commands,
        textures: &Res<TextureAssets>,
        position: Vec3,
    ) -> Entity {
        let building_prefab = BuildingPrefab {
            name: "House",
            max_hp: 2000.0,
            units_of_work: 100.0,
            texture_set: BuildingTextureSet {
                in_progress: vec![
                    textures.house_in_progress.clone(),
                    textures.house_in_progress.clone(),
                ],
                completed: textures.house.clone(),
                scale: 0.03,
            },
            max_workers: 2,
            required_resources: vec![(ResourceKind::Wood, 4)],
        };
        plan_building(commands, building_prefab, position)
    }

    pub fn plan_training_ground() {
        println!("Planning to build a mine");
    }

    pub fn plan_workshop() {
        println!("Planning to build a workshop");
    }

    pub fn plan_shrine() {
        println!("Planning to build a shrine");
    }
}

pub fn move_to_work(commands: &mut Commands, worker_id: Entity, work_id: Entity) {
    commands
        .entity(worker_id)
        .remove::<NotWorking>()
        .insert(WorkingOn {
            work_id,
            job_id: BUILDING_JOB_NAME,
        })
        .insert(MovingToEntity {
            destination_entity: work_id,
            sufficient_range: 15.0,
        });
}
