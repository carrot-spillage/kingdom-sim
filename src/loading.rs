use crate::{
    building::{BuildingPrefab, BuildingPrefabMap, BuildingTextureSet},
    items::{ItemPrefab, ItemPrefabMap, ItemPrefabTextures},
    planting::logic::PlantPrefabMap,
    plants::bundle::{PlantPrefab, Size},
    GameState,
};
use bevy::{prelude::*, utils::hashbrown::HashMap};

use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::CreatingWorld),
        )
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        // .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading) // NOTE: disabled audio, as if this failes to load, the game never starts
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, PlantPrefabAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, ItemPrefabAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, BuildingPrefabAssets>(GameState::Loading);

        app.add_system(setup_prefabs.in_schedule(OnExit(GameState::Loading)));
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c48"]
pub struct PlantPrefabVec {
    pub plants: Vec<PlantPrefab<String, Size>>,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug)]
#[uuid = "160a57b6-2417-47c7-bd3b-52ace245cc49"]
pub struct ItemPrefabVec {
    pub items: Vec<ItemPrefab<String>>,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug)]
#[uuid = "2d6e164e-73cc-4b74-b7d3-cdbfc59ef727"]
pub struct BuildingPrefabVec {
    pub buildings: Vec<BuildingPrefab<String, Size>>,
}

#[derive(AssetCollection, Resource)]
pub struct PlantPrefabAssets {
    #[asset(path = "prefabs/_.plants.yaml", typed)]
    pub plants: Handle<PlantPrefabVec>,
}

#[derive(AssetCollection, Resource)]
pub struct ItemPrefabAssets {
    #[asset(path = "prefabs/_.items.yaml", typed)]
    pub items: Handle<ItemPrefabVec>,
}

#[derive(AssetCollection, Resource)]
pub struct BuildingPrefabAssets {
    #[asset(path = "prefabs/_.buildings.yaml", typed)]
    pub buildings: Handle<BuildingPrefabVec>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,

    #[asset(path = "fonts/Hack-Regular.ttf")]
    pub hack: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/tile.png")]
    pub tile: Handle<Image>,

    #[asset(path = "textures/campfire.png")]
    pub campfire: Handle<Image>,

    #[asset(path = "textures/house.png")]
    pub house: Handle<Image>,

    #[asset(path = "textures/house_in_progress.png")]
    pub house_in_progress: Handle<Image>,

    #[asset(path = "textures/farm_field_in_progress_1.png")]
    pub farm_field_sowing_1: Handle<Image>,

    #[asset(path = "textures/farm_field_in_progress_2.png")]
    pub farm_field_sowing_2: Handle<Image>,

    #[asset(path = "textures/farm_field.png")]
    pub farm_field_sowing_3: Handle<Image>,

    #[asset(path = "textures/farm_field.png")]
    pub farm_field: Handle<Image>,

    #[asset(path = "textures/farm_field_in_progress_1.png")]
    pub farm_field_in_progress_1: Handle<Image>,

    #[asset(path = "textures/farm_field_in_progress_2.png")]
    pub farm_field_in_progress_2: Handle<Image>,

    #[asset(path = "textures/peasant.png")]
    pub peasant: Handle<Image>,

    #[asset(path = "textures/tree1.png")]
    pub tree1: Handle<Image>,

    #[asset(path = "textures/tree2.png")]
    pub tree2: Handle<Image>,

    #[asset(path = "textures/logs.png")]
    pub logs: Handle<Image>,
}

fn setup_prefabs(
    mut commands: Commands,
    plants: Res<Assets<PlantPrefabVec>>,
    items: Res<Assets<ItemPrefabVec>>,
    buildings: Res<Assets<BuildingPrefabVec>>,
    p: Res<PlantPrefabAssets>,
    ip: Res<ItemPrefabAssets>,
    bp: Res<BuildingPrefabAssets>,
    asset_server: Res<AssetServer>,
) {
    let plant_vec = plants.get(&p.plants).unwrap();
    let map: HashMap<_, _> = plant_vec
        .plants
        .iter()
        .map(|x| {
            let default: Handle<Image> = asset_server.load(x.textures.default.clone());
            (
                x.id,
                PlantPrefab::<Handle<Image>> {
                    collision_box: x.collision_box.to_vec(),
                    germinator: x.germinator,
                    growth_rate: x.growth_rate,
                    health: x.health,
                    id: x.id,
                    intrinsic_resource: x.intrinsic_resource,
                    name: x.name.clone(),
                    resource_producer: x.resource_producer,
                    textures: crate::plants::bundle::PlantPrefabTextureSet::<Handle<Image>> {
                        default,
                    },
                },
            )
        })
        .collect();
    commands.insert_resource(PlantPrefabMap(map));

    let item_vec = items.get(&ip.items).unwrap();
    let item_prefab_map: HashMap<_, _> = item_vec
        .items
        .iter()
        .map(|x| {
            let dropped: Handle<Image> = asset_server.load(x.textures.dropped.clone());
            (
                x.id,
                ItemPrefab {
                    id: x.id,
                    packable: x.packable,
                    weight: x.weight,
                    handling_kind: x.handling_kind,
                    textures: ItemPrefabTextures { dropped },
                },
            )
        })
        .collect();
    commands.insert_resource(ItemPrefabMap(item_prefab_map));
    let building_vec = buildings.get(&bp.buildings).unwrap();
    let building_prefab_map: HashMap<_, _> = building_vec
        .buildings
        .iter()
        .map(|x| {
            let in_progress: Vec<Handle<Image>> = x
                .textures
                .in_progress
                .iter()
                .map(|path| asset_server.load(path.clone()))
                .collect();
            let completed: Handle<Image> = asset_server.load(x.textures.completed.clone());

            (
                x.id,
                BuildingPrefab {
                    id: x.id,
                    collision_box: x.collision_box.to_vec(),
                    max_hp: x.max_hp,
                    max_workers: x.max_workers,
                    name: x.name.clone(),
                    required_resources: x.required_resources.clone(),
                    units_of_work: x.units_of_work,
                    textures: BuildingTextureSet {
                        in_progress,
                        completed,
                    },
                },
            )
        })
        .collect();
    commands.insert_resource(BuildingPrefabMap(building_prefab_map));
}
