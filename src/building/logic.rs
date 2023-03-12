use bevy::{
    math::Vec3,
    prelude::{Commands, Entity, Handle, Image, Res, Transform},
    sprite::SpriteBundle,
};

use crate::{create_world::WorldParams, movement::isometrify_position, building::ConstructionSite};

use super::{BuildingTextureSet, BuildingPrefab, Building};

pub fn spawn_construction_site(
    commands: &mut Commands,
    construction_site_id: Entity,
    position: Vec3,
    textures: &BuildingTextureSet,
    world_params: &Res<WorldParams>,
) {
    println!("Spawning construction site at {:?}", position);
    commands
        .entity(construction_site_id)
        .insert(ConstructionSite)
        .insert(SpriteBundle {
            transform: Transform {
                translation: isometrify_position(position, &world_params),
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
    let max_index = building_prefab.textures.in_progress.len() - 1;
    let old_index = (max_index as f32 * previous_progress).round() as usize;
    let index = (max_index as f32 * progress).round() as usize;

    if previous_progress == 0.0 || index != old_index {
        return Some(building_prefab.textures.in_progress[index].clone());
    } else {
        return None;
    }
}

pub fn convert_construction_site_to_building(
    id: Entity,
    commands: &mut Commands,
    textures: &BuildingTextureSet,
) {
    commands
        .entity(id)
        .remove::<ConstructionSite>()
        .insert(Building)
        .insert(textures.completed.clone());
}
