pub mod logic;

use bevy::prelude::{App, IntoSystemConfig, OnUpdate, Plugin};

use crate::GameState;

use self::logic::handle_task_progress;

pub struct PlantingPlugin;

impl Plugin for PlantingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_task_progress.in_set(OnUpdate(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
