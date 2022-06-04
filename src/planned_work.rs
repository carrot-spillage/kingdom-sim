use bevy::{
    math::Vec3,
    prelude::{
        App, Commands, Component, Entity, EventReader, EventWriter, Plugin, Query, Res, SystemSet,
    },
};

use crate::{
    building::{BuildingBlueprint, BuildingTextureSet},
    loading::TextureAssets,
    movement::MovingToPosition,
};
use crate::{
    movement::{ArrivalEvent, Position},
    GameState,
};

pub static BUILDING_JOB_NAME: &'static str = "Building";
#[derive(Component)]
pub struct BuildingReference(pub Entity);

#[derive(Component)]
pub struct PlannedWork;

#[derive(Component, Clone, Copy)]
pub struct PlannedWorkReference {
    pub planned_work_id: Entity,
    pub job_id: &'static str,
}

pub fn plan_building(
    commands: &mut Commands,
    worker_id: Entity,
    building_blueprint: BuildingBlueprint,
    position: Vec3,
) -> Entity {
    let planned_work_id = commands
        .spawn()
        .insert(PlannedWork)
        .insert(Position(position))
        .id();

    commands
        .entity(worker_id)
        .insert(PlannedWorkReference {
            planned_work_id,
            job_id: BUILDING_JOB_NAME,
        })
        .insert(MovingToPosition {
            position,
            sufficient_range: 15.0,
        });

    planned_work_id
}

pub struct ArrivedToWorkEvent(PlannedWorkReference);

pub struct WorkOnArrivalPlugin;

impl Plugin for WorkOnArrivalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ArrivedToWorkEvent>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(make_arrivals_work),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn make_arrivals_work(
    mut arrival_events: EventReader<ArrivalEvent>,
    query: Query<&PlannedWorkReference>,
    mut work_ref_events: EventWriter<ArrivedToWorkEvent>,
) {
    for event in arrival_events.iter() {
        if let Ok(work_ref) = query.get(event.0) {
            work_ref_events.send(ArrivedToWorkEvent(*work_ref));
        }
    }
}

pub struct BuildingJobPlugin;

impl Plugin for BuildingJobPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(handle_planed_work_events),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn handle_planed_work_events(
    mut commands: Commands,
    mut work_ref_events: EventReader<ArrivedToWorkEvent>,
) {
    for ArrivedToWorkEvent(PlannedWorkReference {
        planned_work_id,
        job_id,
    }) in work_ref_events.iter()
    {
        if *job_id != BUILDING_JOB_NAME {
            continue;
        }
    }
}
