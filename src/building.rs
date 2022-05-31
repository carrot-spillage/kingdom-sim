use bevy::{
    math::Vec3,
    prelude::{Commands, Component, Entity, Handle, Image, Query, Res, Transform},
    sprite::SpriteBundle,
};

use crate::{
    building_job::BuildingReference, common::CreationProgress, jobs::systems::WorkProgressedEvent,
    loading::TextureAssets, movement::hack_3d_position_to_2d,
};

#[derive(Component)]
pub struct ConstructionSite;

#[derive(Component)]
pub struct Building;

pub fn spawn_construction_site(
    commands: &mut Commands,
    position: Vec3,
    texture: Handle<Image>,
) -> Entity {
    println!("Spawning construction site at {:?}", position);
    commands
        .spawn()
        .insert(ConstructionSite)
        .insert(CreationProgress(0.0))
        .insert_bundle(SpriteBundle {
            texture,
            transform: Transform {
                translation: hack_3d_position_to_2d(position),
                scale: Vec3::new(0.03, 0.03, 1.0),
                ..Transform::default()
            },
            ..Default::default()
        })
        .id()
}

pub fn update_construction_site(
    progress_event: &WorkProgressedEvent,
    building_references: &Query<&BuildingReference>,
    construction_progresses: &mut Query<(&mut CreationProgress, &mut Handle<Image>)>,
    _textures: &Res<TextureAssets>,
) {
    // TODO: provide several frames of house building progress

    let building_id = building_references
        .get(progress_event.work_process_id)
        .unwrap()
        .0;
    let (mut creation_progress, _) = construction_progresses.get_mut(building_id).unwrap();
    let progress = progress_event.units_of_work_left / progress_event.units_of_work;

    (*creation_progress).0 = progress;
}

pub fn convert_construction_site_to_building(
    id: Entity,
    commands: &mut Commands,
    texture: Handle<Image>,
) {
    commands
        .entity(id)
        .remove::<ConstructionSite>()
        .remove::<CreationProgress>()
        .insert(Building)
        .insert(texture);
}
