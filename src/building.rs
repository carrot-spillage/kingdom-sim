use bevy::{
    math::{Vec2, Vec3},
    prelude::{AssetServer, Commands, Component, Entity, Handle, Image, Query, Res, Transform},
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    building_job::BuildingReference, common::CreationProgress, jobs::systems::WorkProgressedEvent,
    loading::TextureAssets, movement::{Position, hack_3d_position_to_2d},
};

#[derive(Component)]
pub struct ConstructionSite;

#[derive(Component)]
pub struct Building;

pub fn spawn_construction_site(
    commands: &mut Commands,
    position: Vec3,
    textures: &Res<TextureAssets>,
) -> Entity {
    println!("Spawning construction site at {:?}", position);
    commands
        .spawn()
        .insert(ConstructionSite)
        .insert(CreationProgress(0.0))
        .insert_bundle(SpriteBundle {
            texture: textures.construction_site_1.clone(),
            transform: Transform {
                translation: hack_3d_position_to_2d(position),
                ..Transform::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(12.0, 16.0)),
                ..Sprite::default()
            },
            ..Default::default()
        })
        .id()
}

pub fn update_construction_site(
    progress_event: &WorkProgressedEvent,
    building_references: &Query<&BuildingReference>,
    construction_progresses: &mut Query<(&mut CreationProgress, &mut Handle<Image>)>,
    textures: &Res<TextureAssets>,
) {
    let building_id = building_references
        .get(progress_event.work_process_id)
        .unwrap()
        .0;
    let (mut creation_progress, mut texture) =
        construction_progresses.get_mut(building_id).unwrap();
    let progress = progress_event.units_of_work_left / progress_event.units_of_work;

    // TODO: provide several frames of house building progress
    //*texture = asset_server.load(format!("textures/{name}_{variant}_{frame_index}.png"));
    (*creation_progress).0 = progress;
}

pub fn convert_construction_site_to_building(
    id: Entity,
    commands: &mut Commands,
    textures: &Res<TextureAssets>,
) {
    let texture: Handle<Image> = textures.house.clone();
    commands
        .entity(id)
        .remove::<ConstructionSite>()
        .remove::<CreationProgress>()
        .insert(Building)
        .insert(texture);
}
