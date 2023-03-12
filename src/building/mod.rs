mod logic;

use bevy::{
    prelude::{Component, Handle, Image, Resource, Vec2},
    utils::hashbrown::HashMap,
};

use crate::{items::ItemPrefabId, plants::bundle::Size};

pub use self::logic::{
    convert_construction_site_to_building, get_construction_site_texture, spawn_construction_site,
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
