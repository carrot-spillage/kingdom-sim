use bevy::{
    math::Vec3,
    prelude::{Commands, Component, Entity, Handle, Image, Res, Resource, Transform},
    reflect::TypePath,
    sprite::{Sprite, SpriteBundle},
    utils::hashbrown::HashMap,
};

use crate::{
    create_world::WorldParams,
    movement::{isometrify_position, Position},
};

#[derive(Component, Debug)]
pub struct ConstructionSiteStorage {
    // The ones, that's been delivered and not part of a crafting process
    pub available_batches: Vec<ItemBatch>,
    pub needed_batches: Vec<ItemBatch>,
}

impl ConstructionSiteStorage {
    pub(crate) fn accept(&mut self, item_batches: &mut Vec<ItemBatch>) {
        self.needed_batches.retain_mut(|needed| {
            let found_index = item_batches
                .iter_mut()
                .position(|x| x.prefab_id == needed.prefab_id);

            if let Some(index) = found_index {
                let mut item_batch = item_batches[index];
                let result = deliver_quantity(needed.quantity, item_batch.quantity);
                self.available_batches.push(ItemBatch {
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
                    needed.quantity = result.expected_remains;
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
        expected_remains: expected.saturating_sub(delivered),
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

#[derive(Component, serde::Deserialize, TypePath, Debug, Clone, Copy)]
pub enum ItemHandlingKind {
    TwoHanded,
    SingleHanded,
}

#[derive(serde::Deserialize, TypePath, Debug, Clone)]
pub struct ItemPrefab<T = Handle<Image>> {
    pub id: ItemPrefabId,
    pub packable: bool, // false - only handheld
    pub handling_kind: ItemHandlingKind,
    pub weight: u32,
    pub textures: ItemPrefabTextures<T>,
}

#[derive(serde::Deserialize, TypePath, Debug, Clone)]
pub struct ItemPrefabTextures<T> {
    pub dropped: T,
}

#[derive(Component, serde::Deserialize, TypePath, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ItemPrefabId(pub u32);

#[derive(Component, serde::Deserialize, TypePath, Debug, Clone, Copy)]
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

pub fn add_batches_to(consumer: &mut Vec<ItemBatch>, provider: &mut Vec<ItemBatch>) {
    for incoming_item_batch in provider.iter_mut() {
        if let Some(receiving_item_batch) = consumer
            .iter_mut()
            .find(|x| x.prefab_id == incoming_item_batch.prefab_id)
        {
            receiving_item_batch.quantity += incoming_item_batch.quantity;
        } else {
            consumer.push(*incoming_item_batch);
        }
    }

    provider.clear();
}
