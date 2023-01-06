use crate::{
    common::{ClaimedBy, Countdown},
    loading::{TextureAssets, PlantPrefabAssets},
    plants::{init_plant, bundle::{PlantPrefab, PlantBundle}},
};
use bevy::{prelude::{Commands, Component, Entity, Query, Res, Vec3, Assets, Resource}, utils::HashMap};

#[derive(Component)]
pub struct Planting {
    plant_name: String,
    plant_position: Vec3,
}

#[derive(Component)]
pub struct PlantingCountdown(Countdown);

#[derive(Resource)]
pub struct PlantBundleMap(pub HashMap<String, PlantBundle>);

pub fn handle_task_progress(
    mut commands: Commands,
    plants: Res<PlantBundleMap>,
    textures: Res<TextureAssets>,
    mut planters_query: Query<(Entity, &Planting, &mut PlantingCountdown)>,
) {
    for (worker_id, planting, mut planting_countdown) in &mut planters_query {
        let mut countdown = planting_countdown.0;
        countdown.tick();
        if countdown.is_done() {
            cleanup(&mut commands, worker_id);
            init_plant(
                &mut commands,
                *plants.0.get(&planting.plant_name).unwrap(),
                &textures,
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
