use bevy::{
    math::Vec3,
    prelude::{App, Commands, Entity, EventWriter, Plugin, Query, SystemSet},
};

use crate::{
    building::{
        convert_construction_site_to_building, get_construction_site_texture,
        spawn_construction_site, BuildingBlueprint,
    },
    crafting_progress::{advance_crafting_process_state, CraftingProgress, CraftingProgressUpdate},
    movement::{MovingToEntity, Position},
    planned_work::{PlannedWork, WorkerCompletedWorkEvent, WorksOn, BUILDING_JOB_NAME},
    skills::{SkillType, Skilled},
    GameState,
};

pub struct BuildingJobPlugin;

impl Plugin for BuildingJobPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(handle_building_process),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn handle_building_process(
    mut commands: Commands,
    mut construction_sites: Query<(
        Entity,
        &PlannedWork,
        &mut CraftingProgress,
        &BuildingBlueprint,
    )>,
    workers: Query<&Skilled>,
    mut worker_completion_events: EventWriter<WorkerCompletedWorkEvent>,
) {
    for (planned_work_id, work, mut crafting_progress, building_blueprint) in
        construction_sites.iter_mut()
    {
        let building_id = planned_work_id; // building is the planned work

        let workers: Vec<&Skilled> = work
            .worker_ids
            .iter()
            .map(|worker_id| workers.get(*worker_id).unwrap())
            .collect();

        if workers.is_empty() {
            continue;
        }

        match advance_crafting_process_state(
            workers,
            &mut crafting_progress,
            SkillType::Building,
            work.units_of_work,
            1.0,
        ) {
            CraftingProgressUpdate::Complete { .. } => {
                for worker_id in work
                    .worker_ids
                    .iter()
                    .chain(work.tentative_worker_ids.iter())
                {
                    remove_planned_work(&mut commands, planned_work_id);

                    worker_completion_events.send(WorkerCompletedWorkEvent {
                        worker_id: *worker_id,
                    })
                }

                convert_construction_site_to_building(
                    building_id,
                    &mut commands,
                    &building_blueprint.texture_set,
                );
            }
            CraftingProgressUpdate::Incomplete { progress, delta } => {
                if let Some(new_texture) = get_construction_site_texture(
                    1.0 - (progress.units_of_work_left + delta) / work.units_of_work,
                    1.0 - progress.units_of_work_left / work.units_of_work,
                    building_blueprint,
                ) {
                    commands.entity(building_id).insert(new_texture);
                }

                *crafting_progress = progress;
            }
            CraftingProgressUpdate::NotEnoughResources => {
                free_workers(&mut commands, &work.worker_ids)
            }
        }
    }
}

pub fn plan_building(
    commands: &mut Commands,
    building_blueprint: BuildingBlueprint,
    position: Vec3,
) -> Entity {
    let id = commands
        .spawn_empty()
        .insert(PlannedWork::new(
            BUILDING_JOB_NAME,
            building_blueprint.units_of_work,
            building_blueprint.max_workers,
        ))
        .insert(CraftingProgress::new(
            building_blueprint.units_of_work,
            building_blueprint.required_resources.clone(),
        ))
        .insert(Position(position))
        .id();

    spawn_construction_site(commands, id, position, &building_blueprint.texture_set);

    commands.entity(id).insert(building_blueprint);

    return id;
}

fn remove_planned_work(commands: &mut Commands, planned_work_id: Entity) {
    commands
        .entity(planned_work_id)
        .remove::<CraftingProgress>()
        .remove::<PlannedWork>();
}

fn free_workers(commands: &mut Commands, workers: &Vec<Entity>) {
    for worker_id in workers {
        commands
            .entity(*worker_id)
            .remove::<WorksOn>()
            .remove::<MovingToEntity>();
    }
}
