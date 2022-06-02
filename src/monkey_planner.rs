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

pub struct MonkeyPlanner;

impl MonkeyPlanner {
    pub fn plan_house(
        commands: &mut Commands,
        textures: &Res<TextureAssets>,
        worker_id: Entity,
        position: Vec3,
    ) {
        let building_blueprint = BuildingBlueprint {
            name: "House",
            max_hp: 2000.0,
            units_of_work: 100.0,
            texture_set: BuildingTextureSet {
                in_progress: vec![textures.house_in_progress.clone()],
                completed: textures.house.clone(),
                scale: 0.03,
            },
        };
        plan_work(commands, worker_id, building_blueprint, position);
    }

    pub fn plan_farm_field(
        commands: &mut Commands,
        textures: &Res<TextureAssets>,
        worker_id: Entity,
        position: Vec3,
    ) {
        let building_blueprint = BuildingBlueprint {
            name: "Farm field",
            max_hp: 200.0,
            units_of_work: 100.0,
            texture_set: BuildingTextureSet {
                in_progress: vec![
                    textures.farm_field_in_progress_1.clone(),
                    textures.farm_field_in_progress_2.clone(),
                ],
                completed: textures.farm_field.clone(),
                scale: 0.2,
            },
        };

        plan_work(commands, worker_id, building_blueprint, position);
    }

    pub fn plan_training_ground() {
        println!("Planning to build a mine");
    }

    pub fn plan_workshop() {
        println!("Planning to build a factory");
    }

    pub fn plan_shrine() {
        println!("Planning to build a factory");
    }
}

#[derive(Component)]
pub struct PlannedWork;

#[derive(Component, Clone, Copy)]
pub struct PlannedWorkReference {
    pub planned_work_id: Entity,
    pub job_id: &'static str,
}

fn plan_work(
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
