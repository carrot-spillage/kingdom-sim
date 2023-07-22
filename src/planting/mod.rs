pub mod logic;

use bevy::prelude::{in_state, App, IntoSystemConfigs, Plugin, Update};

use crate::GameState;

use self::logic::handle_task_progress;

pub struct PlantingPlugin;

impl Plugin for PlantingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_task_progress.run_if(in_state(GameState::Playing)),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
