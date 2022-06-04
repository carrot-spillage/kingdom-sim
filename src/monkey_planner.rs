use bevy::{
    math::Vec3,
    prelude::{
        App, Commands, Component, Entity, EventReader, EventWriter, Plugin, Query, Res, SystemSet,
    },
};

use crate::{
    building::{BuildingBlueprint, BuildingTextureSet},
    loading::TextureAssets,
    movement::MovingToPosition,
    planned_work::plan_building,
};
use crate::{
    movement::{ArrivalEvent, Position},
    GameState,
};

pub struct MonkeyPlanner;

impl MonkeyPlanner {
    pub fn plan_house(
        commands: &mut Commands,
        textures: &Res<TextureAssets>,
        worker_id: Entity,
        position: Vec3,
    ) {
        let building_blueprint = BuildingBlueprint {
            name: "House",
            max_hp: 2000.0,
            units_of_work: 100.0,
            texture_set: BuildingTextureSet {
                in_progress: vec![textures.house_in_progress.clone()],
                completed: textures.house.clone(),
                scale: 0.03,
            },
        };
        plan_building(commands, worker_id, building_blueprint, position);
    }

    pub fn plan_farm_field(
        commands: &mut Commands,
        textures: &Res<TextureAssets>,
        worker_id: Entity,
        position: Vec3,
    ) {
        let building_blueprint = BuildingBlueprint {
            name: "Farm field",
            max_hp: 200.0,
            units_of_work: 100.0,
            texture_set: BuildingTextureSet {
                in_progress: vec![
                    textures.farm_field_in_progress_1.clone(),
                    textures.farm_field_in_progress_2.clone(),
                ],
                completed: textures.farm_field.clone(),
                scale: 0.2,
            },
        };

        plan_building(commands, worker_id, building_blueprint, position);
    }

    pub fn plan_training_ground() {
        println!("Planning to build a mine");
    }

    pub fn plan_buildingshop() {
        println!("Planning to build a factory");
    }

    pub fn plan_shrine() {
        println!("Planning to build a factory");
    }
}
