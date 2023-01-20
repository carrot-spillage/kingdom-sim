use bevy::{
    math::Vec3,
    prelude::{Commands, Component, Entity, Res, Transform},
    sprite::SpriteBundle,
};
use conditional_commands::ConditionalInsertBundleExt;

use crate::{
    loading::TextureAssets,
    movement::{hack_3d_position_to_2d, Position},
    stockpile::InStockpile,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ResourceKind {
    Wood,
}

#[derive(Clone, Copy, Component, Debug)]
pub struct ResourceChunk {
    pub kind: ResourceKind,
    pub quantity: usize,
}

/*
I have a game unit called Villager.
Villager can carry logs, wheat, gems, swords, wands.

Villager
    - has Iventory
    - has Hands (can hold)

*/

#[derive(Component)]
pub struct CarrierInventory {
    items: Vec<ItemGroup>,
    max_weight: usize,
}

#[derive(Component)]
pub enum CarrierHands {
    Separate { left: ItemGroup, right: ItemGroup },
    Combined(ItemGroup),
}

#[derive(Component, serde::Deserialize, bevy::reflect::TypeUuid, Debug)]
#[uuid = "7df1e471-50ac-4f76-a7d9-c8507f28fde4"]
pub enum ItemHandlingKind {
    TwoHanded,
    SingleHanded,
}

#[derive(Component, serde::Deserialize, bevy::reflect::TypeUuid, Debug)]
#[uuid = "ef93bff8-fd0c-472d-a9ac-410ed43d527b"]
pub struct ItemPrefab {
    pub id: ItemPrefabId,
    pub packable: bool, // false - only handheld
    pub handling_kind: ItemHandlingKind,
    pub weight: usize,
}

#[derive(
    Component, serde::Deserialize, bevy::reflect::TypeUuid, Clone, Copy, Debug, Hash, PartialEq, Eq,
)]
#[uuid = "3819241a-9f90-47dc-b5df-bc99f8fec014"]
pub struct ItemPrefabId(usize);

#[derive(Clone, Copy, Component, Debug)]
pub struct ItemGroup {
    pub prefab_id: ItemPrefabId,
    pub quantity: usize,
}

pub struct ItemTakingResult {
    picked: Option<ItemGroup>,
    left: Option<ItemGroup>,
}

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
    fn take(&self, item_prefab: &ItemPrefab, max_weight: usize) -> ItemTakingResult {
        let picked_quantity = (max_weight as f32 / item_prefab.weight as f32).floor() as usize;

        if picked_quantity >= self.quantity {
            ItemTakingResult {
                picked: Some(ItemGroup {
                    quantity: self.quantity,
                    prefab_id: item_prefab.id,
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
                    prefab_id: item_prefab.id,
                }),
                left: Some(ItemGroup {
                    quantity: self.quantity - picked_quantity,
                    prefab_id: item_prefab.id,
                }),
            }
        }
    }
}

pub fn spawn_item_group(
    commands: &mut Commands,
    textures: &Res<TextureAssets>,
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
            texture: textures.logs.clone(),
            transform: Transform {
                translation: hack_3d_position_to_2d(position),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..Transform::default()
            },
            ..Default::default()
        })
        .insert_if(is_in_stockpile, || InStockpile)
        .id()
}
