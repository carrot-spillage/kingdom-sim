pub mod logic;

use bevy::prelude::{SystemSet, Plugin, App};

use crate::GameState;

use self::logic::handle_task_progress;

pub struct PlantingPlugin;

impl Plugin for PlantingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(handle_task_progress));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
