use crate::{
    items::{ItemPrefab, ItemPrefabId, ItemPrefabMap},
    planting::logic::PlantBundleMap,
    plants::bundle::{PlantPrefab, PlantPrefabId},
    GameState,
};
use bevy::{prelude::*, utils::hashbrown::HashMap};

use bevy_asset_loader::prelude::{AssetCollection, LoadingState};
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        LoadingState::new(GameState::Loading)
            .with_collection::<FontAssets>()
            // .with_collection::<AudioAssets>() // NOTE: disabled audio, as if this failes to load, the game never starts
            .with_collection::<TextureAssets>()
            .with_collection::<PlantPrefabAssets>()
            .with_collection::<ItemPrefabAssets>()
            .continue_to_state(GameState::Playing) // TODO: change to GameState::Menu
            .build(app);

        app.add_system_set(SystemSet::on_exit(GameState::Loading).with_system(setup_prefabs));
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c48"]
pub struct PlantPrefabVec {
    pub plants: Vec<PlantPrefab>,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug)]
#[uuid = "160a57b6-2417-47c7-bd3b-52ace245cc49"]
pub struct ItemPrefabVec {
    pub items: Vec<ItemPrefab>,
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
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,

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
    p: Res<PlantPrefabAssets>,
    ip: Res<ItemPrefabAssets>,
    asset_server: Res<AssetServer>,
) {
    let plant_vec = plants.get(&p.plants).unwrap();
    let map: HashMap<_, _> = plant_vec
        .plants
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let (bundle, maybe_grower, maybe_producer) = x.to_plant_bundle(PlantPrefabId(i));
            (
                PlantPrefabId(i),
                (
                    bundle,
                    maybe_grower,
                    maybe_producer,
                    asset_server.load(x.texture.clone()),
                ),
            )
        })
        .collect();
    commands.insert_resource(PlantBundleMap(map));

    let item_vec = items.get(&ip.items).unwrap();
    let item_prefab_map: HashMap<_, _> = item_vec
        .items
        .iter()
        .enumerate()
        .map(|(i, x)| (ItemPrefabId(i), x.clone()))
        .collect();

    commands.insert_resource(ItemPrefabMap(item_prefab_map));
}
