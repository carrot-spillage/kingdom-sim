use crate::{plants::bundle::{PlantPrefab, Growing, PlantBundle}, GameState, tree::SimpleDestructible, planting::logic::PlantBundleMap};
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
            .continue_to_state(GameState::Playing) // TODO: change to GameState::Menu
            .build(app);

        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(setup_prefabs),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection, Resource)]
pub struct PlantPrefabAssets {
    #[asset(path = "prefabs", collection(typed))]
    pub items: Vec<Handle<PlantPrefab>>,
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

fn setup_prefabs(app: &mut App, plants: Res<Assets<PlantPrefab>>) {
    let map: HashMap<_, _> = plants
        .iter()
        .map(|x| (x.1.name, create_plant_bundle_from_prefab(x.1)))
        .collect();
    app.insert_resource(PlantBundleMap(map));
}

fn create_plant_bundle_from_prefab(prefab: &PlantPrefab) -> PlantBundle {
    PlantBundle {
        germinating: prefab.germinating,
        growing: Growing {
            growth_speed: prefab.growth_speed,
        },
        simple_destructible: SimpleDestructible {
            max_health: prefab.health as f32,
            current_health: prefab.health as f32,
        },
    }
}
