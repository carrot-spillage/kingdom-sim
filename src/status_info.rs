use bevy::{
    hierarchy::{Children, Parent},
    math::Vec3,
    prelude::{
        Added, App, AssetServer, Commands, Component, Entity, Plugin, Query, ResMut, SystemSet,
        Transform,
    },
};

use crate::{movement::Position, GameState};

#[derive(Component, Clone)]
pub struct StatusInfo(String);

pub struct StatusInfoPlugin;

impl Plugin for StatusInfoPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system_set(
        //     SystemSet::on_update(GameState::Playing).with_system(display_status_info),
        // )
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(create_status_info),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn create_status_info(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    positioned: Query<(&Position, &Children)>,
    added_statuses: Query<&Parent, Added<StatusInfo>>,
    statuses: Query<&StatusInfo>,
    mut transforms: Query<&mut Transform>,
) {
    let parents: Vec<(&Position, &Children)> = added_statuses
        .iter()
        .map(|parent| positioned.get(parent.0).unwrap())
        .collect();

    for (position, children) in parents.iter() {
        let mut siblings: Vec<(Entity, StatusInfo)> = vec![];
        for child in children.iter() {
            let status = statuses.get(*child).unwrap();
            siblings.push((*child, status.clone()));
        }

        siblings.sort_by(|(_, si_a), (_, si_b)| si_a.0.cmp(&si_b.0));

        let gap = 2.0;
        let icon_size = 12.0;
        let width = icon_size + gap;
        for (index, (child, _)) in siblings.iter().enumerate() {
            let status_position = position.0 + Vec3::new(index as f32 * width, 0.0, 0.0);
            let maybe_transform = transforms.get_mut(*child);
            match maybe_transform {
                Result::Err(_) => {
                    commands.spawn().insert(Transform {
                        translation: status_position,
                        ..Transform::default()
                    });
                }
                Result::Ok(mut transform) => {
                    (*transform).translation = status_position;
                }
            }
        }
    }
}

// fn display_status_info(
//     mut commands: Commands,
//     mut asset_server: ResMut<AssetServer>,
//     entity_with_statuses: Query<(Entity, &StatusInfoList, &Children), Changed<StatusInfoList>>,
// ) {
//     for (entity, StatusList(statuses), children) in entity_with_statuses.iter() {
//         for child in children.iter() {
//             commands.entity(*child).despawn()
//         }

//         commands.entity(entity).with_children(|parent| {
//             for status in statuses {

//                 let path = format!("textures/{status}.png");
//                 parent
//                     .spawn_bundle(SpriteBundle {
//                         texture: asset_server.load(&path),
//                         sprite: Sprite {
//                             custom_size: Some(Vec2::new(12.0, 12.0)),
//                             ..Sprite::default()
//                         },
//                         ..Default::default()
//                     });
//             }
//         });
//     }
// }
