use crate::quad_tree::QuadTree;
use crate::{
    common::Countdown,
    init::{AreaOccupiedEvent, WorldParams},
    plants::{
        bundle::{PlantPrefab, PlantPrefabId},
        spawn_plant, PlantMaturityStage,
    },
    tasks::{CreatureTask, IdlingCreature},
};

use bevy::prelude::Rect;
use bevy::{
    prelude::{
        Commands, Component, Entity, EventWriter, Handle, Image, Query, Res, ResMut, Resource, Vec3,
    },
    utils::HashMap,
};
use bevy_turborand::GlobalRng;

#[derive(Component, Debug, Clone, Copy)]
pub struct Planting {
    pub plant_prefab_id: PlantPrefabId,
    pub position: Vec3,
}

#[derive(Component)]
pub struct PlantingCountdown(Countdown);

#[derive(Resource, Debug)]
pub struct PlantPrefabMap(pub HashMap<PlantPrefabId, (PlantPrefab, Handle<Image>)>);

pub fn handle_task_progress(
    mut commands: Commands,
    mut global_rng: ResMut<GlobalRng>,
    plants: Res<PlantPrefabMap>,
    world_params: Res<WorldParams>,
    mut planters_query: Query<(Entity, &Planting, &mut PlantingCountdown)>,
    mut quad_tree: ResMut<QuadTree<Entity>>,
    mut area_occupied_events: EventWriter<AreaOccupiedEvent>,
) {
    for (worker_id, planting, mut planting_countdown) in &mut planters_query {
        let mut countdown = planting_countdown.0;
        if countdown.tick_yield() {
            if let Some((prefab, texture)) = plants.0.get(&planting.plant_prefab_id) {
                cleanup(&mut commands, worker_id);

                let germ_rect = Rect::from_center_size(
                    planting.position.truncate(),
                    prefab.collision_box.to_vec(),
                );
                let tenant_id = commands.spawn_empty().id();
                if quad_tree.try_occupy_rect(germ_rect, tenant_id) {
                    println!("Sends AreaOccupiedEvent");
                    area_occupied_events.send(AreaOccupiedEvent { area: germ_rect });
                    spawn_plant(
                        &mut commands,
                        &mut global_rng,
                        &world_params,
                        &prefab,
                        texture.clone(),
                        planting.position,
                        &PlantMaturityStage::Germ,
                    );
                }
            }
        } else {
            *planting_countdown = PlantingCountdown(countdown);
        }
    }
}

pub fn start_planting(
    commands: &mut Commands,
    planting: Planting,
    worker_id: Entity,
    performance: f32,
) {
    let countdown = PlantingCountdown(Countdown::new((performance * 30.0).floor() as u32));
    commands.entity(worker_id).insert((planting, countdown));
}

fn cleanup(commands: &mut Commands, worker_id: Entity) {
    commands
        .entity(worker_id)
        .remove::<(CreatureTask, Planting, PlantingCountdown)>()
        .insert(IdlingCreature);
}
