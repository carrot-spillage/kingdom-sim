pub mod bundle;

use bevy::{
    prelude::{Commands, Entity, Query, Res, Transform, Vec3, SystemSet, Plugin, App, Handle, Image, With},
    sprite::{SpriteBundle, Sprite},
};

use crate::{
    movement::{hack_3d_position_to_2d, Position},
    GameState, planting::logic::PlantBundleMap,
};

use self::bundle::{Growing, PlantBundle, Germinator, GerminatorCountdown, PlantName, MaturePlant};

pub fn plant_germ(
    commands: &mut Commands,
    plant_bundle: PlantBundle,
    texture: Handle<Image>,
    position: Vec3,
) -> Entity {
    commands
        .spawn((
            plant_bundle,
            Position(position),
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: hack_3d_position_to_2d(position),
                    scale: Vec3::new(0.0, 0.0, 1.0),
                    ..Transform::default()
                },
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::BottomCenter,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id()
}

pub fn grow(mut commands: Commands, mut growing_query: Query<(Entity, &mut Growing, &mut Transform)>) {
    for (tree_id, mut growing, mut transform) in &mut growing_query {
        growing.maturity = (growing.maturity + growing.rate).min(1.0);
        transform.scale = Vec3::new(growing.maturity, growing.maturity, 1.0);
        if growing.maturity == 1.0 {
            commands.entity(tree_id).remove::<Growing>().insert(MaturePlant);
        }
    }
}

pub fn germinate(mut commands: Commands, plant_bundle_map: Res<PlantBundleMap>, mut germinating_query: Query<(&PlantName, &Position, &Germinator, &mut GerminatorCountdown), With<MaturePlant>>) {
    for (plant_name, position, germinating, mut germinating_countdown) in &mut germinating_query {
        germinating_countdown.0.tick();
        if germinating_countdown.0.is_done() {
            *germinating_countdown = germinating.gen_countdown();
            let germ_position = position.0 + germinating.gen_offset().extend(0.0);
            let (bundle, texture) = plant_bundle_map.0.get(&plant_name.0.clone()).unwrap();
            plant_germ(&mut commands, bundle.clone(), texture.clone(), germ_position);
        }
    }
}


pub struct PlantsPlugin;
impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(grow)
                .with_system(germinate),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}