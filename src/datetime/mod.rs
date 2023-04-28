use std::{ops::AddAssign, str::FromStr};

use bevy::prelude::{App, IntoSystemConfig, OnUpdate, Plugin, ResMut, Resource};
use chrono::{Duration, NaiveDateTime};

use crate::GameState;

#[derive(Resource)]
pub struct GameTime(pub NaiveDateTime); // in seconds

impl GameTime {
    fn tick(&mut self) {
        self.0.add_assign(Duration::seconds(30));
    }
}

pub struct GameTimePlugin;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTime(
            NaiveDateTime::from_str("2100-01-01T00:00:00.000").unwrap(),
        ));
        app.add_system(tick.in_set(OnUpdate(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn tick(mut time_of_day: ResMut<GameTime>) {
    time_of_day.tick();
}
