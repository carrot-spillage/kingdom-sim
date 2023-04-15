use bevy::{
    math::Vec3,
    prelude::{Commands, Entity, Handle, Image, Res, Transform},
    sprite::SpriteBundle,
    utils::HashSet,
};

use crate::{
    building::{constructing::ConstructionSiteWorkers, ConstructionSite},
    create_world::WorldParams,
    items::{ConstructionSiteStorage, ItemBatch, ItemPrefab},
    movement::{isometrify_position, Position},
    work::CraftingProcess,
};

use super::{Building, BuildingPrefab, BuildingTextureSet};

pub fn spawn_construction_site(
    commands: &mut Commands,
    construction_site_id: Entity,
    position: Vec3,
    building_prefab: &BuildingPrefab,
    world_params: &Res<WorldParams>,
) {
    println!("Spawning construction site at {:?}", position);
    commands
        .entity(construction_site_id)
        .insert(ConstructionSite)
        .insert(Position(position))
        .insert(building_prefab.id)
        .insert(CraftingProcess::new(
            building_prefab.units_of_work,
            building_prefab
                .required_resources
                .iter()
                .map(|x| ItemBatch {
                    prefab_id: x.0,
                    quantity: x.1,
                })
                .collect(),
        ))
        .insert(ConstructionSiteStorage {
            available_batches: vec![],
            needed_batches: building_prefab
                .required_resources
                .iter()
                .map(|x| ItemBatch {
                    prefab_id: x.0,
                    quantity: x.1,
                })
                .collect(),
        })
        .insert(ConstructionSiteWorkers(HashSet::new()))
        .insert(SpriteBundle {
            texture: building_prefab
                .textures
                .in_progress
                .first()
                .unwrap()
                .clone(),
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
        .remove::<(CraftingProcess, ConstructionSite)>()
        .insert(Building)
        .insert(textures.completed.clone());
}
