pub mod logic;

use bevy::prelude::{App, IntoSystemConfig, OnUpdate, Plugin};

use crate::GameState;

use self::logic::handle_task_progress;

pub use self::logic::start_harvesting;

pub struct HarvestingPlugin;

impl Plugin for HarvestingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_task_progress.in_set(OnUpdate(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
