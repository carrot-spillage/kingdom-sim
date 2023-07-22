use bevy::prelude::{
    in_state, App, Component, IntoSystemConfigs, Plugin, Query, Resource, Update, Vec2, With,
};

use crate::GameState;

#[derive(Component)]
struct AirBlock;

#[derive(Component)]
pub struct Humidity(pub f32); // 0..1

#[derive(Component)]
pub struct Temperature(pub f32); // -50..+50

#[derive(Resource)]
pub struct Wind {
    pub speed: f32,
    pub direction: Vec2,
}

pub struct AirBlockPlugin;

impl Plugin for AirBlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_air_blocks.run_if(in_state(GameState::Playing)),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn update_air_blocks(mut blocks: Query<(&mut Temperature, &mut Humidity), With<AirBlock>>) {}
