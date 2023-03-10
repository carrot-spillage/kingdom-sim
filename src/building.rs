use bevy::{
    math::Vec3,
    prelude::{Commands, Component, Entity, Handle, Image, Res, Resource, Transform, Vec2},
    sprite::SpriteBundle,
    utils::hashbrown::HashMap,
};

use crate::{
    init::WorldParams, items::ItemPrefabId, movement::isometrify_position, plants::bundle::Size,
};

#[derive(Component)]
pub struct ConstructionSite;

#[derive(Component)]
pub struct Building; // TODO: do we need it?

#[derive(
    Component, serde::Deserialize, bevy::reflect::TypeUuid, Clone, Copy, Debug, Hash, PartialEq, Eq,
)]
#[uuid = "38192aaa-9f90-47dc-b5df-bc99f8fec014"]
pub struct BuildingPrefabId(pub u32);

#[derive(Resource, Debug)]
pub struct BuildingPrefab {
    pub id: BuildingPrefabId,
    pub name: String,
    pub textures: BuildingTextureSet,
    pub max_hp: f32, // max_hp and units_of_work can be probably calculated from the number of resources needed
    pub units_of_work: f32,
    pub max_workers: u32,
    pub collision_box: Vec2,
    pub required_resources: Vec<(ItemPrefabId, u32)>,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug)]
#[uuid = "78612f76-3507-4c70-a926-65baf3e042ae"]
pub struct BuildingPrefabRaw {
    pub id: BuildingPrefabId,
    pub name: String,
    pub textures: BuildingTextureSetRaw,
    pub max_hp: f32, // max_hp and units_of_work can be probably calculated from the number of resources needed
    pub units_of_work: f32,
    pub max_workers: u32,
    pub collision_box: Size,
    pub required_resources: Vec<(ItemPrefabId, u32)>,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug)]
#[uuid = "7aa12f76-3507-4c70-a926-65baf3e042ae"]
pub struct BuildingTextureSetRaw {
    pub in_progress: Vec<String>,
    pub completed: String,
}

#[derive(Resource, Debug)]
pub struct BuildingPrefabMap(pub HashMap<BuildingPrefabId, BuildingPrefab>);

#[derive(Debug)]
pub struct BuildingTextureSet {
    pub in_progress: Vec<Handle<Image>>,
    pub completed: Handle<Image>,
}

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
