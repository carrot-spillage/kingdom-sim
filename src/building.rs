use bevy::{
    math::Vec3,
    prelude::{Commands, Component, Entity, Handle, Image, Transform},
    sprite::SpriteBundle,
};

use crate::{movement::hack_3d_position_to_2d, resources::ResourceKind};

#[derive(Component)]
pub struct ConstructionSite;

#[derive(Component)]
pub struct Building;

#[derive(Component)]
pub struct BuildingPrefab {
    pub name: &'static str,
    pub texture_set: BuildingTextureSet,
    pub max_hp: f32, // max_hp and units_of_work can be probably calculated from the number of resources needed
    pub units_of_work: f32,
    pub max_workers: usize,
    pub required_resources: Vec<(ResourceKind, usize)>,
}

pub struct BuildingTextureSet {
    pub in_progress: Vec<Handle<Image>>,
    pub completed: Handle<Image>,
    pub scale: f32,
}

pub fn spawn_construction_site(
    commands: &mut Commands,
    construction_site_id: Entity,
    position: Vec3,
    texture_set: &BuildingTextureSet,
) {
    println!("Spawning construction site at {:?}", position);
    commands
        .entity(construction_site_id)
        .insert(ConstructionSite)
        .insert(SpriteBundle {
            transform: Transform {
                translation: hack_3d_position_to_2d(position),
                scale: Vec3::new(texture_set.scale, texture_set.scale, 1.0),
                ..Transform::default()
            },
            ..Default::default()
        });
}

pub fn get_construction_site_texture(
    previous_progress: f32,
    progress: f32,
    building_prefab: &BuildingPrefab,
) -> Option<Handle<Image>> {
    let max_index = (building_prefab.texture_set.in_progress.len() - 1) as f32;
    let old_index = (max_index * previous_progress).round() as usize;
    let index = (max_index * progress).round() as usize;

    if previous_progress == 0.0 || index != old_index {
        return Some(building_prefab.texture_set.in_progress[index].clone());
    } else {
        return None;
    }
}

pub fn convert_construction_site_to_building(
    id: Entity,
    commands: &mut Commands,
    texture_set: &BuildingTextureSet,
) {
    commands
        .entity(id)
        .remove::<ConstructionSite>()
        .insert(Building)
        .insert(texture_set.completed.clone());
}
