mod logic;

use bevy::prelude::{in_state, App, IntoSystemConfigs, Plugin, Update};

use crate::GameState;

use self::logic::handle_task_progress;

pub use self::logic::start_cutting_tree;

pub struct TreeCuttingPlugin;

impl Plugin for TreeCuttingPlugin {
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
