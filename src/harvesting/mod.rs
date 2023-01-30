pub mod logic;

use bevy::prelude::{App, Plugin, SystemSet};

use crate::GameState;

use self::logic::handle_task_progress;

pub use self::logic::start_harvesting;

pub struct HarvestingPlugin;

impl Plugin for HarvestingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(handle_task_progress),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
