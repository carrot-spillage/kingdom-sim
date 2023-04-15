use bevy::{
    prelude::{
        App, BuildChildren, Bundle, Commands, Component, Entity, IntoSystemConfigs, OnUpdate,
        Plugin, Query, Res, ResMut, Transform, Vec2, Vec3, With,
    },
    sprite::{Sprite, SpriteBundle},
};
use bevy_turborand::{GlobalRng, RngComponent};

use crate::{
    common::NeedsDestroying,
    create_world::WorldParams,
    items::{
        spawn_item_batch, CarrierInventory, ConstructionSiteStorage, ItemBatch, ItemPrefabMap,
    },
    loading::{FontAssets, TextureAssets},
    movement::{isometrify_position, Position, Walker},
    tasks::{create_tooltip_bundle, CreatureTask, CreatureTaskTooltip, IdlingCreature},
    work::CraftingProcess,
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
            available_weight: 50,
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
        app.add_systems(
            (drop_items, collect_items, transfer_items).in_set(OnUpdate(GameState::Playing)),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

#[derive(Component)]
pub struct CarrierDroppingItems;

#[derive(Component)]
pub struct CarrierCollectingItems {
    pub target_id: Entity,
}

#[derive(Component)]
pub struct CarrierTransferringItems {
    pub target_id: Entity,
}

pub fn schedule_dropping_items(commands: &mut Commands, carrier_id: Entity) {
    commands.entity(carrier_id).insert(CarrierDroppingItems);
}

pub fn schedule_collecting_items(commands: &mut Commands, carrier_id: Entity, target_id: Entity) {
    commands
        .entity(carrier_id)
        .insert(CarrierCollectingItems { target_id });
}

pub fn schedule_transferring_items(commands: &mut Commands, carrier_id: Entity, target_id: Entity) {
    commands
        .entity(carrier_id)
        .insert(CarrierTransferringItems { target_id });
}

fn drop_items(
    mut commands: Commands,
    mut carriers: Query<(Entity, &Position, &mut CarrierInventory), With<CarrierDroppingItems>>,
    items: Res<ItemPrefabMap>,
    world_params: Res<WorldParams>,
) {
    for (carrier_id, position, mut item_container) in &mut carriers {
        for item_batch in &item_container.items {
            let prefab = items.0.get(&item_batch.prefab_id).unwrap();

            spawn_item_batch(
                &mut commands,
                prefab.textures.dropped.clone(),
                item_batch.clone(),
                position.0,
                &world_params,
            );
        }

        item_container.items.clear();
        cleanup_drop(&mut commands, carrier_id);
    }
}

fn collect_items(
    mut commands: Commands,
    mut carriers: Query<(
        Entity,
        &Position,
        &mut CarrierInventory,
        &CarrierCollectingItems,
    )>,
    mut item_batches: Query<&mut ItemBatch>,

    items: Res<ItemPrefabMap>,
) {
    for (carrier_id, position, mut item_container, CarrierCollectingItems { target_id }) in
        &mut carriers
    {
        // TODO: check the position
        let mut item_batch = item_batches.get_mut(*target_id).unwrap();
        let prefab = items.0.get(&item_batch.prefab_id).unwrap();

        item_container.accept(prefab, &mut item_batch);
        println!("now item_container contains {:?}", item_container);

        cleanup_collect(
            &mut commands,
            carrier_id,
            if item_batch.quantity == 0 {
                Some(*target_id)
            } else {
                None
            },
        );
    }
}

fn transfer_items(
    mut commands: Commands,
    mut carriers: Query<(
        Entity,
        &Position,
        &mut CarrierInventory,
        &CarrierTransferringItems,
    )>,
    mut construction_site_storages: Query<(&mut ConstructionSiteStorage, &mut CraftingProcess)>,
) {
    for (carrier_id, position, mut item_container, CarrierTransferringItems { target_id }) in
        &mut carriers
    {
        println!("Item container has batches {:?}", item_container.items);

        let (mut storage, mut crafting_process) =
            construction_site_storages.get_mut(*target_id).unwrap(); // TODO: there might be other kinds of recepients of items
                                                                     // TODO: check the position
        storage.accept(carrier_id, &mut item_container.items);
        println!("Storage received batches {:?}", storage);

        crafting_process.accept_batches(&mut storage.available_batches);
        println!("Crafting process received batches {:?}", crafting_process);

        cleanup_transfer(&mut commands, carrier_id);
    }
}

fn cleanup_drop(commands: &mut Commands, carrier_id: Entity) {
    commands
        .entity(carrier_id)
        .remove::<(CreatureTask, CarrierDroppingItems)>()
        .insert(IdlingCreature);
}

fn cleanup_transfer(commands: &mut Commands, carrier_id: Entity) {
    commands
        .entity(carrier_id)
        .remove::<(CreatureTask, CarrierTransferringItems)>()
        .insert(IdlingCreature);
}

fn cleanup_collect(
    commands: &mut Commands,
    carrier_id: Entity,
    item_batch_id_to_remove: Option<Entity>,
) {
    commands
        .entity(carrier_id)
        .remove::<(CreatureTask, CarrierCollectingItems)>()
        .insert(IdlingCreature);

    if let Some(item_batch_id) = item_batch_id_to_remove {
        commands.entity(item_batch_id).insert(NeedsDestroying);
    }
}
