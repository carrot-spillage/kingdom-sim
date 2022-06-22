use std::collections::HashMap;

use bevy::prelude::Component;

#[derive(PartialEq, Clone, Copy, Hash, Eq, Debug)]
pub enum SkillType {
    GrowingPlants,
    //Harvesting,
    //Crafting,
    Building,
    None, // TODO: whis is a temp workaround. Can we have jobs that don't require any skills?
}

#[derive(Component, Clone, Debug)]
pub struct Skilled {
    pub skills: HashMap<SkillType, f32>,
}
