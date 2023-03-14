use bevy::{
    math::Vec3,
    prelude::{Commands, Component, Entity, Handle, Image, Res, Resource, Transform},
    sprite::{Sprite, SpriteBundle},
    utils::hashbrown::HashMap,
};

use crate::{
    create_world::WorldParams,
    movement::{isometrify_position, Position},
};

#[derive(Component, Debug)]
pub struct CarrierInventory {
    pub items: Vec<ItemBatch>,
    pub max_weight: u32,
}
impl CarrierInventory {
    pub(crate) fn put_and_get_rest(
        &mut self,
        item_prefab: &ItemPrefab,
        picked_item_batch: ItemBatch,
    ) -> Option<ItemBatch> {
        let ItemTakingResult { picked, left } =
            picked_item_batch.take(item_prefab, self.max_weight);

        let maybe_item_batch = self
            .items
            .iter_mut()
            .find(|x| x.prefab_id == item_prefab.id);

        if let Some(picked_item_batch) = picked {
            if let Some(item_batch) = maybe_item_batch {
                item_batch.quantity += picked_item_batch.quantity;
            } else {
                self.items.push(picked_item_batch);
            }
        }

        left
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

pub struct ItemTakingResult {
    picked: Option<ItemBatch>,
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
                picked: Some(ItemBatch {
                    quantity: self.quantity,
                    prefab_id: self.prefab_id,
                }),
                left: None,
            }
        } else if picked_quantity == 0 {
            ItemTakingResult {
                picked: None,
                left: Some(*self),
            }
        } else {
            ItemTakingResult {
                picked: Some(ItemBatch {
                    quantity: picked_quantity,
                    prefab_id: self.prefab_id,
                }),
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
