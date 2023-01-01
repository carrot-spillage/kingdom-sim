use bevy::{
    math::Vec3,
    prelude::{
        App, Commands, Component, Entity, EventReader, Plugin, Query, Res, SystemSet, Transform,
    },
    sprite::SpriteBundle,
};
use conditional_commands::ConditionalInsertBundleExt;

use crate::{
    loading::TextureAssets,
    movement::{hack_3d_position_to_2d, Position},
    stockpile::InStockpile,
    GameState,
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

pub enum ItemHandlingKind {
    TwoHanded,
    SingleHanded,
}

pub struct ItemBlueprint {
    id: usize,
    packable: bool, // false - only handheld
    handling_kind: ItemHandlingKind,
    weight: usize,
}

#[derive(Clone, Copy, Component, Debug)]
pub struct ItemGroup {
    blueprint_id: usize,
    quantity: usize,
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
    fn take(
        &self,
        item_blueprint: &ItemBlueprint,
        max_weight: usize,
    ) -> ItemTakingResult {
        let picked_quantity = (max_weight as f32 / item_blueprint.weight as f32).floor() as usize;
    
        if picked_quantity >= self.quantity {
            ItemTakingResult {
                picked: Some(ItemGroup {
                    quantity: self.quantity,
                    blueprint_id: item_blueprint.id,
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
                    blueprint_id: item_blueprint.id,
                }),
                left: Some(ItemGroup {
                    quantity: self.quantity - picked_quantity,
                    blueprint_id: item_blueprint.id,
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
