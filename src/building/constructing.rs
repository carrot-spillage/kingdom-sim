use bevy::{
    prelude::{
        in_state, Added, App, Commands, Component, Entity, IntoSystemConfigs, Plugin, Query,
        ResMut, Update, With,
    },
    utils::HashSet,
};

use crate::{
    tasks::{CreatureTask, CreatureTaskStopping, IdlingCreature},
    work::{
        CraftingProcess, CraftingProcessCanContinue, CraftingProcessUpdate, WorkParticipant,
        WorkProficiency,
    },
    GameState,
};

use super::{convert_construction_site_to_building, BuildingPrefabId, BuildingPrefabMap};

#[derive(Component)]
pub struct ConstructionSiteWorkers(pub HashSet<ConstructedBy>);

#[derive(Component)]
pub struct CreatureConstructingTask {
    pub construction_site_id: Entity,
}

#[derive(Component, Debug, Hash, Eq, PartialEq, PartialOrd)]
pub struct ConstructedBy(Entity);

pub struct CreatureConstructingTaskPlugin;

impl Plugin for CreatureConstructingTaskPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (start, stop).run_if(in_state(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn start(
    creatures_with_tasks: Query<
        (Entity, &CreatureConstructingTask),
        Added<CreatureConstructingTask>,
    >,
    mut construction_sites_with_workers: Query<&mut ConstructionSiteWorkers>,
) {
    for (creature_id, task) in &creatures_with_tasks {
        println!("Construction joined by {:?}", creature_id);
        if let Ok(mut construction_site_workers) =
            construction_sites_with_workers.get_mut(task.construction_site_id)
        {
            construction_site_workers
                .0
                .insert(ConstructedBy(creature_id)); // TODO: deduplication needed
        }
    }
}

fn stop(
    mut commands: Commands,
    creatures_with_tasks: Query<(Entity, &CreatureConstructingTask), Added<CreatureTaskStopping>>,
    mut construction_sites_with_workers: Query<&mut ConstructionSiteWorkers>,
) {
    for (creature_id, task) in &creatures_with_tasks {
        println!("Construction stopped by {:?}", creature_id);

        commands
            .entity(creature_id)
            .remove::<(CreatureTask, CreatureConstructingTask)>()
            .insert(IdlingCreature);

        if let Ok(mut construction_site_workers) =
            construction_sites_with_workers.get_mut(task.construction_site_id)
        {
            construction_site_workers
                .0
                .remove(&ConstructedBy(creature_id)); // there is no duplication prevention
        }
    }
}

pub struct ConstructionPlugin;

impl Plugin for ConstructionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_task_process.run_if(in_state(GameState::Playing)),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn handle_task_process(
    mut commands: Commands,
    mut construction_sites: Query<
        (
            Entity,
            &mut CraftingProcess,
            &BuildingPrefabId,
            &ConstructionSiteWorkers,
        ),
        With<CraftingProcessCanContinue>,
    >,
    buildings: ResMut<BuildingPrefabMap>,
) {
    for (
        construction_site_id,
        mut crafting_process,
        building_prefab_id,
        ConstructionSiteWorkers(workers),
    ) in &mut construction_sites
    {
        if workers.is_empty() {
            continue;
        }

        let dummy_proficiency = WorkProficiency {
            performance: 0.5,
            skill: 0.5,
        };
        let work_participants: Vec<WorkParticipant> = workers
            .iter()
            .map(|w| WorkParticipant {
                creature_id: w.0,
                proficiency: dummy_proficiency,
            })
            .collect();
        match crafting_process.advance(work_participants, 1.0) {
            CraftingProcessUpdate::Complete { .. } => {
                println!("Constructing: Complete");

                for worker_id in workers.iter().map(|x| x.0) {
                    commands.entity(worker_id).insert(CreatureTaskStopping); // TODO: more ergonomic way to stop a task
                }

                commands
                    .entity(construction_site_id)
                    .remove::<ConstructionSiteWorkers>();
                let building_prefab = buildings.0.get(building_prefab_id).unwrap();
                convert_construction_site_to_building(
                    construction_site_id,
                    &mut commands,
                    &building_prefab.textures,
                )
            }
            CraftingProcessUpdate::Incomplete { delta: _ } => {
                // println!("Constructing: Incomplete");
                // TODO: update textures
                // if let Some(new_texture) = get_construction_site_texture(
                //     1.0 - (process.units_of_work_left + delta) / work.units_of_work,
                //     1.0 - process.units_of_work_left / work.units_of_work,
                //     building_prefab,
                // ) {
                //     commands.entity(building_id).insert(new_texture);
                // }
            }
            CraftingProcessUpdate::InsufficientResources => {
                println!("Constructing: InsufficientResources");
                // release all workers
                for worker_id in workers.iter().map(|x| x.0) {
                    commands.entity(worker_id).insert(CreatureTaskStopping); // TODO: more ergonomic way to stop a task
                }

                commands
                    .entity(construction_site_id)
                    .insert(ConstructionSiteWorkers(HashSet::new()));
            }
        }
    }
}
