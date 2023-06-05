use bevy::prelude::{
    App, Component, IntoSystemConfig, OnUpdate, Plugin, Query, Resource, Vec2, With,
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
        app.add_system(update_air_blocks.in_set(OnUpdate(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn update_air_blocks(mut blocks: Query<(&mut Temperature, &mut Humidity), With<AirBlock>>) {}
