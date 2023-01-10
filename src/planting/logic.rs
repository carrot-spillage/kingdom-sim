use crate::{
    common::{ClaimedBy, Countdown},
    loading::{TextureAssets},
    plants::{plant_germ, bundle::{PlantBundle, PlantPrefabId}},
};
use bevy::{prelude::{Commands, Component, Entity, Query, Res, Vec3, Resource, Handle, Image}, utils::HashMap};

#[derive(Component)]
pub struct Planting {
    plant_name: String,
    plant_position: Vec3,
}

#[derive(Component)]
pub struct PlantingCountdown(Countdown);

#[derive(Resource, Debug)]
pub struct PlantBundleMap(pub HashMap<PlantPrefabId, (PlantBundle, Handle<Image>)>);

pub fn handle_task_progress(
    mut commands: Commands,
    plants: Res<PlantBundleMap>,
    mut planters_query: Query<(Entity, &Planting, &mut PlantingCountdown)>,
) {
    for (worker_id, planting, mut planting_countdown) in &mut planters_query {
        let mut countdown = planting_countdown.0;
        countdown.tick();
        if countdown.is_done() {
            let (bundle, texture) = plants.0.values().next().unwrap();
            cleanup(&mut commands, worker_id);
            plant_germ(
                &mut commands,
                bundle.clone(),
                texture.clone(),
                planting.plant_position,
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
