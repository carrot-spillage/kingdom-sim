use crate::{
    common::Countdown,
    plants::{
        bundle::{PlantPrefab, PlantPrefabId},
        spawn_plant, PlantMaturityStage,
    },
    tasks::IdlingWorker,
};
use bevy::{
    prelude::{Commands, Component, Entity, Handle, Image, Query, Res, Resource, Vec3},
    utils::HashMap,
};

#[derive(Component)]
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
    plants: Res<PlantPrefabMap>,
    mut planters_query: Query<(Entity, &Planting, &mut PlantingCountdown)>,
) {
    for (worker_id, planting, mut planting_countdown) in &mut planters_query {
        let mut countdown = planting_countdown.0;
        countdown.tick();
        if countdown.is_done() {
            let (prefab, texture) = plants.0.get(&planting.plant_prefab_id).unwrap();
            cleanup(&mut commands, worker_id);
            spawn_plant(
                &mut commands,
                &prefab,
                texture.clone(),
                planting.position,
                &PlantMaturityStage::Germ,
            );
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
    let countdown = PlantingCountdown(Countdown::new((performance * 30.0).floor() as usize));
    commands.entity(worker_id).insert((planting, countdown));
}

fn cleanup(commands: &mut Commands, worker_id: Entity) {
    commands
        .entity(worker_id)
        .remove::<(Planting, PlantingCountdown)>()
        .insert(IdlingWorker);
}
