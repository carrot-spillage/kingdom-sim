use bevy::{
    math::{Vec2, Vec3},
    prelude::{AssetServer, Commands, Component, Entity, Handle, Image, Query, Res, Transform},
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    building_job::BuildingReference, common::CreationProgress, jobs::WorkProgressedEvent,
    movement::Position,
};

#[derive(Component)]
pub struct ConstructionSite;

#[derive(Component)]
pub struct Building;

pub fn spawn_construction_site(
    commands: &mut Commands,
    position: Vec3,
    asset_server: &Res<AssetServer>,
) -> Entity {
    commands
        .spawn()
        .insert(ConstructionSite)
        .insert(CreationProgress(0.0))
        .insert(Position(position))
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("textures/house.png"),
            transform: Transform {
                translation: position,
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
    asset_server: &Res<AssetServer>,
) {
    let building_id = building_references
        .get(progress_event.work_process_id)
        .unwrap()
        .0;
    let (mut creation_progress, mut texture) =
        construction_progresses.get_mut(building_id).unwrap();
    let progress = progress_event.units_of_work_left / progress_event.units_of_work;

    *texture = asset_server.load("textures/construction_site_1.png");
    (*creation_progress).0 = progress;
}

pub fn convert_construction_site_to_building(
    id: Entity,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let texture: Handle<Image> = asset_server.load("textures/house.png");
    commands
        .entity(id)
        .remove::<ConstructionSite>()
        .remove::<CreationProgress>()
        .remove::<Handle<Image>>()
        .insert(Building)
        .insert(texture);
}
