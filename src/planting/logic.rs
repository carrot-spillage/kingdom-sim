use crate::{
    common::{ClaimedBy, Countdown},
    plants::{
        bundle::{PlantBundle, PlantPrefabId},
        plant_germ, IntrinsicPlantResourceGrower, PlantResourceProducer,
    },
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
pub struct PlantBundleMap(
    pub  HashMap<
        PlantPrefabId,
        (
            PlantBundle,
            Option<IntrinsicPlantResourceGrower>,
            Option<PlantResourceProducer>,
            Handle<Image>,
        ),
    >,
);

pub fn handle_task_progress(
    mut commands: Commands,
    plants: Res<PlantBundleMap>,
    mut planters_query: Query<(Entity, &Planting, &mut PlantingCountdown)>,
) {
    for (worker_id, planting, mut planting_countdown) in &mut planters_query {
        let mut countdown = planting_countdown.0;
        countdown.tick();
        if countdown.is_done() {
            let (bundle, maybe_grower, maybe_producer, texture) =
                plants.0.get(&planting.plant_prefab_id).unwrap();
            cleanup(&mut commands, worker_id);
            plant_germ(
                &mut commands,
                bundle.clone(),
                maybe_grower.clone(),
                maybe_producer.clone(),
                texture.clone(),
                planting.position,
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
    skill_value: f32,
    target_id: Entity,
) {
    commands.entity(target_id).insert(ClaimedBy(worker_id));
    let countdown = PlantingCountdown(Countdown::new((skill_value * 30.0).floor() as usize));
    commands.entity(worker_id).insert((planting, countdown));
}

fn cleanup(commands: &mut Commands, worker_id: Entity) {
    commands
        .entity(worker_id)
        .remove::<(Planting, PlantingCountdown)>();
}
