use bevy::{
    prelude::{
        App, BuildChildren, Bundle, Commands, Component, Entity, Plugin, Query, Res, ResMut,
        SystemSet, Transform, Vec2, Vec3, With,
    },
    sprite::{Sprite, SpriteBundle},
};
use bevy_turborand::{GlobalRng, RngComponent};

use crate::{
    init::WorldParams,
    items::{spawn_item_batch, CarrierInventory, ItemPrefabMap},
    loading::{FontAssets, TextureAssets},
    movement::{isometrify_position, Position, Walker},
    tasks::{create_tooltip_bundle, CreatureTask, CreatureTaskTooltip, IdlingCreature},
    GameState,
};

#[derive(Bundle)]
struct WorkerBundle {
    creature: Creature,
    walker: Walker,
    position: Position,
    sprite: SpriteBundle,
    inventory: CarrierInventory,
}

#[derive(Component)]
pub struct Creature;

pub fn spawn_creature(
    commands: &mut Commands,
    global_rng: &mut ResMut<GlobalRng>,
    textures: &Res<TextureAssets>,
    fonts: &Res<FontAssets>,
    world_params: &Res<WorldParams>,
    position: Vec3,
) -> Entity {
    let bundle = WorkerBundle {
        creature: Creature,
        inventory: CarrierInventory {
            items: vec![],
            max_weight: 50,
        },
        walker: Walker {
            max_speed: 2.0,
            current_speed: 0.0,
            acceleration: 0.5,
        },
        position: Position(position),
        sprite: SpriteBundle {
            texture: textures.peasant.clone(),
            transform: Transform {
                translation: isometrify_position(position, &world_params),
                ..Transform::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(8.0, 12.25)),
                ..Sprite::default()
            },
            ..Default::default()
        },
    };

    let mut id = None::<Entity>;
    commands
        .spawn(bundle)
        .with_children(|parent| {
            id = Some(parent.spawn(create_tooltip_bundle(13.0, &fonts)).id());
        })
        .insert((
            Position(position),
            RngComponent::from(global_rng),
            IdlingCreature,
            CreatureTaskTooltip {
                title: "".to_string(),
                child: id.unwrap(),
            },
        ))
        .id()
}

pub struct CarrierPlugin;

impl Plugin for CarrierPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(drop_items));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

#[derive(Component)]
pub struct CarrierDroppingItems;

pub fn schedule_dropping_items(commands: &mut Commands, carrier_id: Entity) {
    commands.entity(carrier_id).insert(CarrierDroppingItems);
}

fn drop_items(
    mut commands: Commands,
    mut carriers: Query<(Entity, &Position, &mut CarrierInventory), With<CarrierDroppingItems>>,
    items: Res<ItemPrefabMap>,
    world_params: Res<WorldParams>,
) {
    for (carrier_id, position, mut item_container) in &mut carriers {
        for item_batch in &item_container.items {
            let (_, texture) = items.0.get(&item_batch.prefab_id).unwrap();
            spawn_item_batch(
                &mut commands,
                texture.clone(),
                item_batch.clone(),
                position.0,
                &world_params,
            );
        }

        item_container.items.clear();
        cleanup(&mut commands, carrier_id);
    }
}

fn cleanup(commands: &mut Commands, carrier_id: Entity) {
    commands
        .entity(carrier_id)
        .remove::<(CreatureTask, CarrierDroppingItems)>()
        .insert(IdlingCreature);
}
