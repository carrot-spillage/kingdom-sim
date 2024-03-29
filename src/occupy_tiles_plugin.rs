use bevy::prelude::{
    in_state, App, Color, Commands, EventReader, IntoSystemConfigs, Plugin, Query, Rect, Res,
    Update, With,
};
use bevy_ecs_tilemap::tiles::{TileColor, TilePos, TileStorage};

use crate::{
    create_world::{AreaOccupiedEvent, WorldParams},
    land_tilemap::LandTilemap,
    GameState,
};

pub struct OccupyTilesPlugin;

impl Plugin for OccupyTilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AreaOccupiedEvent>().add_systems(
            Update,
            mark_tiles_in_area_as_occupied.run_if(in_state(GameState::Playing)),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn mark_tiles_in_area_as_occupied(
    mut commands: Commands,
    grids: Query<&TileStorage, With<LandTilemap>>,
    mut events: EventReader<AreaOccupiedEvent>,
    world_params: Res<WorldParams>,
) {
    if events.is_empty() {
        return;
    }

    let tile_storage = grids.single();
    for AreaOccupiedEvent { area } in events.read() {
        let world_offset = world_params.size / 2.0;
        let offset_area = Rect {
            min: area.min + world_offset,
            max: area.max + world_offset,
        };

        let start_grid_x = (offset_area.min.x / world_params.tile_side).floor() as u32;
        let end_grid_x = (offset_area.max.x / world_params.tile_side).floor() as u32;

        let start_grid_y = (offset_area.min.y / world_params.tile_side).floor() as u32;
        let end_grid_y = (offset_area.max.y / world_params.tile_side).floor() as u32;

        // println!(
        //     "Occupying area {:?} {:?} start xy {:?} {:?} end xy {:?} {:?}",
        //     area, offset_area, start_grid_x, start_grid_y, end_grid_x, end_grid_y
        // );

        for x in start_grid_x..=end_grid_x {
            for y in start_grid_y..=end_grid_y {
                let tile = tile_storage.get(&TilePos { x, y }).unwrap();
                commands.entity(tile).insert(TileColor(Color::ORANGE_RED));
            }
        }
    }
}
