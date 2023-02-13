use bevy::{
    math::Vec3,
    prelude::{Commands, Component, Entity, Handle, Image, Res, Resource, Transform, Vec2},
    sprite::{Sprite, SpriteBundle},
    utils::hashbrown::HashMap,
};
use conditional_commands::ConditionalInsertBundleExt;

use crate::{
    movement::{hack_3d_position_to_2d, Position},
    stockpile::InStockpile,
};

/*
I have a game unit called Villager.
Villager can carry logs, wheat, gems, swords, wands.

Villager
    - has Iventory
    - has Hands (can hold)

*/

#[derive(Component, Debug)]
pub struct CarrierInventory {
    pub items: Vec<ItemGroup>,
    pub max_weight: u32,
}
impl CarrierInventory {
    pub(crate) fn put_and_get_rest(
        &mut self,
        item_prefab: &ItemPrefab,
        picked_item_group: ItemGroup,
    ) -> Option<ItemGroup> {
        let ItemTakingResult { picked, left } =
            picked_item_group.take(item_prefab, self.max_weight);

        let maybe_item_group = self
            .items
            .iter_mut()
            .find(|x| x.prefab_id == item_prefab.id);

        if let Some(picked_item_group) = picked {
            if let Some(item_group) = maybe_item_group {
                item_group.quantity += picked_item_group.quantity;
            } else {
                self.items.push(picked_item_group);
            }
        }

        left
    }
}

#[derive(Component)]
pub enum CarrierHands {
    Separate { left: ItemGroup, right: ItemGroup },
    Combined(ItemGroup),
}

#[derive(Component, serde::Deserialize, bevy::reflect::TypeUuid, Debug, Clone)]
#[uuid = "7df1e471-50ac-4f76-a7d9-c8507f28fde4"]
pub enum ItemHandlingKind {
    TwoHanded,
    SingleHanded,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug, Clone)]
#[uuid = "ef93bff8-fd0c-472d-a9ac-410ed43d527a"]
pub struct ItemPrefabTextures {
    pub dropped: String,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Debug, Clone)]
#[uuid = "ef93bff8-fd0c-472d-a9ac-410ed43d527b"]
pub struct ItemPrefab {
    pub id: ItemPrefabId,
    pub packable: bool, // false - only handheld
    pub handling_kind: ItemHandlingKind,
    pub weight: u32,
    pub textures: ItemPrefabTextures,
}

#[derive(
    Component, serde::Deserialize, bevy::reflect::TypeUuid, Clone, Copy, Debug, Hash, PartialEq, Eq,
)]
#[uuid = "3819241a-9f90-47dc-b5df-bc99f8fec014"]
pub struct ItemPrefabId(pub u32);

#[derive(Clone, Copy, Component, Debug)]
pub struct ItemGroup {
    pub prefab_id: ItemPrefabId,
    pub quantity: u32,
}

pub struct ItemTakingResult {
    picked: Option<ItemGroup>,
    left: Option<ItemGroup>,
}

#[derive(Resource, Debug)]
pub struct ItemPrefabMap(pub HashMap<ItemPrefabId, (ItemPrefab, Handle<Image>)>);

// pub struct ItemPlugin;

// impl Plugin for ItemPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(pick));
//     }

//     fn name(&self) -> &str {
//         std::any::type_name::<Self>()
//     }
// }

// fn pick(events: EventReader<PickItemGroupEvent>) {

// }

impl ItemGroup {
    pub fn take(&self, item_prefab: &ItemPrefab, max_weight: u32) -> ItemTakingResult {
        let picked_quantity = (max_weight as f32 / item_prefab.weight as f32).floor() as u32;

        if picked_quantity >= self.quantity {
            ItemTakingResult {
                picked: Some(ItemGroup {
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
                picked: Some(ItemGroup {
                    quantity: picked_quantity,
                    prefab_id: self.prefab_id,
                }),
                left: Some(ItemGroup {
                    quantity: self.quantity - picked_quantity,
                    prefab_id: self.prefab_id,
                }),
            }
        }
    }
}

pub fn spawn_item_group(
    commands: &mut Commands,
    texture: Handle<Image>,
    item_group: ItemGroup,
    position: Vec3,
    is_in_stockpile: bool,
) -> Entity {
    println!("Spawning resource");
    commands
        .spawn_empty()
        .insert(Position(position))
        .insert(item_group)
        .insert(SpriteBundle {
            texture,
            transform: Transform {
                translation: hack_3d_position_to_2d(position),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Transform::default()
            },
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::BottomCenter,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_if(is_in_stockpile, || InStockpile)
        .id()
}
