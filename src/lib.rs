mod actions;
mod activity;
mod audio;
mod init;
mod loading;
mod menu;
pub mod movement;
mod player;
mod work_process;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use activity::{create_job_generator, Job};
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use init::{InitPlugin, WorldParams};
use movement::MovementPlugin;
use work_process::SkillType;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let world_params = WorldParams {
            size: Vec2::new(800.0, 800.0),
        };
        app.insert_resource(world_params);

        let jobs = vec![Job {
            id: 1,
            name: "PlantingCrops",
            skill_type: SkillType::PlantingCrops,
        }];

        app.insert_resource(create_job_generator_with_default_priorities(&jobs));
        app.insert_resource(jobs);

        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(InitPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

fn create_job_generator_with_default_priorities(jobs: &Vec<Job>) -> impl Iterator<Item = Job> {
    let job_priorities = jobs.iter().map(|j| (j.id, 0.5)).collect();
    create_job_generator(jobs.clone(), job_priorities)
}
