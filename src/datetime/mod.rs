use std::{ops::AddAssign, str::FromStr};

use bevy::prelude::{in_state, App, IntoSystemConfigs, Plugin, ResMut, Resource, Update};
use chrono::{DateTime, Duration, Utc};

use crate::GameState;

#[derive(Resource)]
pub struct GameTime(pub DateTime<Utc>); // in seconds

impl GameTime {
    fn tick(&mut self) {
        self.0.add_assign(Duration::seconds(30));
    }
}

pub struct GameTimePlugin;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTime(
            DateTime::from_str("2023-06-01T03:00:00.000Z").unwrap(),
        ));
        app.add_systems(Update, tick.run_if(in_state(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn tick(mut time_of_day: ResMut<GameTime>) {
    time_of_day.tick();
}
