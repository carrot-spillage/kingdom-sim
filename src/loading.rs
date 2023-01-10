use crate::{
    planting::logic::PlantBundleMap,
    plants::bundle::{Growing, PlantBundle, PlantPrefab},
    tree::SimpleDestructible,
    GameState,
};
use bevy::{prelude::*, utils::hashbrown::HashMap, reflect::TypeUuid};

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
    pub plants: Vec<PlantPrefab>
}

#[derive(AssetCollection, Resource)]
pub struct PlantPrefabAssets {
    #[asset(path = "prefabs/plants.yaml", typed)]
    pub items: Handle<PlantPrefabVec>,
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
    p: Res<PlantPrefabAssets>,
    asset_server: Res<AssetServer>
) {
    let plantvec = plants.get(&p.items).unwrap();
    let map: HashMap<_, _> = plantvec
        .plants
        .iter()
        .map(|x| (x.name.clone(), (x.to_plant_bundle(), asset_server.load(x.texture_path.clone()))))
        .collect();

    commands.insert_resource(PlantBundleMap(map));
}

