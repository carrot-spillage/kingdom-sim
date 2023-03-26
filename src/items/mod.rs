use bevy::{
    math::Vec3,
    prelude::{Commands, Component, Entity, Handle, Image, Res, Resource, Transform},
    sprite::{Sprite, SpriteBundle},
    utils::hashbrown::HashMap,
};
use itertools::Itertools;

use crate::{
    create_world::WorldParams,
    movement::{isometrify_position, Position},
};

#[derive(Component, Debug)]
pub struct ConstructionSiteStorage {
    pub delivered_batches: Vec<ItemBatch>,
    pub unscheduled_batches: Vec<ItemBatch>,
    pub expected_batches: Vec<ScheduledItemBatch>,
}

#[derive(Debug)]
pub struct ScheduledItemBatch {
    pub owner_id: Entity,
    pub value: ItemBatch,
}

impl ConstructionSiteStorage {
    pub(crate) fn accept(&mut self, entity_id: Entity, item_batches: &mut Vec<ItemBatch>) {
        self.expected_batches.retain_mut(|expected| {
            if expected.owner_id != entity_id {
                return true;
            }

            let found = item_batches
                .iter_mut()
                .find_position(|x| x.prefab_id == expected.value.prefab_id);

            if let Some((index, item_batch)) = found {
                let result = deliver_quantity(expected.value.quantity, item_batch.quantity);
                self.delivered_batches.push(ItemBatch {
                    prefab_id: item_batch.prefab_id,
                    quantity: result.delivered_used,
                });

                if result.delivered_unused == 0 {
                    item_batches.remove(index);
                } else {
                    // TODO: if has some unused, try to transfer to unscheduled
                    item_batch.quantity = result.delivered_unused;
                }
                if result.expected_remains == 0 {
                    return false;
                } else {
                    expected.value.quantity = result.expected_remains;
                    return true;
                }
            }

            return false;
        });
    }
}

struct TransferResult {
    expected_remains: u32,
    delivered_unused: u32,
    delivered_used: u32,
}

fn deliver_quantity(expected: u32, delivered: u32) -> TransferResult {
    let delivered_used = delivered.min(expected);

    TransferResult {
        delivered_used,
        expected_remains: (expected - delivered).min(0),
        delivered_unused: expected - delivered_used,
    }
}

#[derive(Component, Debug)]
pub struct CarrierInventory {
    pub items: Vec<ItemBatch>,
    pub max_weight: u32,
    pub available_weight: u32,
}
impl CarrierInventory {
    pub(crate) fn accept(&mut self, item_prefab: &ItemPrefab, item_batch: &mut ItemBatch) {
        let ItemTakingResult { picked, left } = item_batch.take(item_prefab, self.available_weight);

        let maybe_existing_item_batch = self
            .items
            .iter_mut()
            .find(|x| x.prefab_id == item_prefab.id);

        if let Some((picked_item_batch, Weight(item_batch_weight))) = picked {
            if let Some(existing_item_batch) = maybe_existing_item_batch {
                existing_item_batch.quantity += picked_item_batch.quantity;
            } else {
                self.items.push(picked_item_batch);
            }

            self.available_weight -= item_batch_weight;
            item_batch.quantity = left.map(|x| x.quantity).unwrap_or(0);
        }
    }
}

#[derive(Component)]
pub enum CarrierHands {
    Separate { left: ItemBatch, right: ItemBatch },
    Combined(ItemBatch),
}

#[derive(Component, serde::Deserialize, bevy::reflect::TypeUuid, Debug, Clone, Copy)]
#[uuid = "7df1e471-50ac-4f76-a7d9-c8507f28fde4"]
pub enum ItemHandlingKind {
    TwoHanded,
    SingleHanded,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug, Clone)]
#[uuid = "ef93bff8-fd0c-472d-a9ac-410ed43d527b"]
pub struct ItemPrefab<T = Handle<Image>> {
    pub id: ItemPrefabId,
    pub packable: bool, // false - only handheld
    pub handling_kind: ItemHandlingKind,
    pub weight: u32,
    pub textures: ItemPrefabTextures<T>,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug, Clone)]
#[uuid = "ef93bff8-fd0c-472d-a9ac-410ed43d527a"]
pub struct ItemPrefabTextures<T> {
    pub dropped: T,
}

#[derive(
    Component, serde::Deserialize, bevy::reflect::TypeUuid, Clone, Copy, Debug, Hash, PartialEq, Eq,
)]
#[uuid = "3819241a-9f90-47dc-b5df-bc99f8fec014"]
pub struct ItemPrefabId(pub u32);

#[derive(Clone, Copy, Component, Debug)]
pub struct ItemBatch {
    pub prefab_id: ItemPrefabId,
    pub quantity: u32,
}

pub struct Weight(pub u32);

pub struct ItemTakingResult {
    picked: Option<(ItemBatch, Weight)>,
    left: Option<ItemBatch>,
}

#[derive(Resource, Debug)]
pub struct ItemPrefabMap(pub HashMap<ItemPrefabId, ItemPrefab>);

// pub struct ItemPlugin;

// impl Plugin for ItemPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(pick));
//     }

//     fn name(&self) -> &str {
//         std::any::type_name::<Self>()
//     }
// }

// fn pick(events: EventReader<PickItemBatchEvent>) {

// }

impl ItemBatch {
    pub fn take(&self, item_prefab: &ItemPrefab, max_weight: u32) -> ItemTakingResult {
        let picked_quantity = (max_weight as f32 / item_prefab.weight as f32).floor() as u32;

        if picked_quantity >= self.quantity {
            ItemTakingResult {
                picked: Some((
                    ItemBatch {
                        quantity: self.quantity,
                        prefab_id: self.prefab_id,
                    },
                    Weight(item_prefab.weight * picked_quantity),
                )),
                left: None,
            }
        } else if picked_quantity == 0 {
            ItemTakingResult {
                picked: None,
                left: Some(*self),
            }
        } else {
            ItemTakingResult {
                picked: Some((
                    ItemBatch {
                        quantity: picked_quantity,
                        prefab_id: self.prefab_id,
                    },
                    Weight(item_prefab.weight * picked_quantity),
                )),
                left: Some(ItemBatch {
                    quantity: self.quantity - picked_quantity,
                    prefab_id: self.prefab_id,
                }),
            }
        }
    }
}

pub fn spawn_item_batch(
    commands: &mut Commands,
    texture: Handle<Image>,
    item_batch: ItemBatch,
    position: Vec3,
    world_params: &Res<WorldParams>,
) -> Entity {
    commands
        .spawn_empty()
        .insert(Position(position))
        .insert(item_batch)
        .insert(SpriteBundle {
            texture,
            transform: Transform {
                translation: isometrify_position(position, &world_params),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Transform::default()
            },
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::BottomCenter,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}
