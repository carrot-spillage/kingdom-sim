use bevy::{
    math::Vec3,
    prelude::{Commands, Component, Entity, Handle, Image, Query, Res, Transform},
    sprite::SpriteBundle,
};

use crate::{
    jobs::systems::WorkProgressedEvent, loading::TextureAssets, monkey_planner::BuildingReference,
    movement::hack_3d_position_to_2d,
};

#[derive(Component)]
pub struct ConstructionSite;

#[derive(Component)]
pub struct Building;

#[derive(Component)]
pub struct BuildingBlueprint {
    pub name: &'static str,
    pub texture_set: BuildingTextureSet,
    pub max_hp: f32, // max_hp and units_of_work can be probably calculated from the number of resources needed
    pub units_of_work: f32,
}

pub struct BuildingTextureSet {
    pub in_progress: Vec<Handle<Image>>,
    pub completed: Handle<Image>,
    pub scale: f32,
}

#[derive(Component)]
pub struct ConstructionProgress(pub f32);

pub fn spawn_construction_site(
    commands: &mut Commands,
    position: Vec3,
    texture_set: &BuildingTextureSet,
) -> Entity {
    println!("Spawning construction site at {:?}", position);
    commands
        .spawn()
        .insert(ConstructionSite)
        .insert(ConstructionProgress(0.0))
        .insert_bundle(SpriteBundle {
            texture: texture_set.in_progress[0].clone(),
            transform: Transform {
                translation: hack_3d_position_to_2d(position),
                scale: Vec3::new(texture_set.scale, texture_set.scale, 1.0),
                ..Transform::default()
            },
            ..Default::default()
        })
        .id()
}

pub fn update_construction_site(
    progress_event: &WorkProgressedEvent,
    building_references: &Query<&BuildingReference>,
    construction_progresses: &mut Query<(&mut ConstructionProgress, &mut Handle<Image>)>,
    texture_set: &BuildingTextureSet,
) {
    // TODO: provide several frames of house building progress

    let building_id = building_references
        .get(progress_event.work_process_id)
        .unwrap()
        .0;
    let (mut construction_progress, mut texture) =
        construction_progresses.get_mut(building_id).unwrap();
    let progress = progress_event.units_of_work_left / progress_event.units_of_work;
    let old_index = ((texture_set.in_progress.len() - 1) as f32 * construction_progress.0) as usize;
    let index = ((texture_set.in_progress.len() - 1) as f32 * progress) as usize;

    if index > 0 && index > old_index {
        *texture = texture_set.in_progress[index].clone();
    }

    (*construction_progress) = ConstructionProgress(progress);
}

pub fn convert_construction_site_to_building(
    id: Entity,
    commands: &mut Commands,
    texture_set: &BuildingTextureSet,
) {
    commands
        .entity(id)
        .remove::<ConstructionSite>()
        .remove::<ConstructionProgress>()
        .insert(Building)
        .insert(texture_set.completed.clone());
}
