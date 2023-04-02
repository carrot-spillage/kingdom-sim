use bevy::prelude::{Commands, Component, Entity};

use crate::tasks::IdlingCreature;

#[derive(Component)]
pub struct CreatureConstructingTask {
    pub creature_id: Entity,
    pub construction_site_id: Entity,
}

#[derive(Component)]
pub struct ConstructedBy(Entity);

impl CreatureConstructingTask {
    pub fn insert(commands: &mut Commands, creature_id: Entity, construction_site_id: Entity) {
        commands
            .entity(creature_id)
            .insert(CreatureConstructingTask {
                creature_id,
                construction_site_id,
            });

        commands
            .entity(construction_site_id)
            .insert(ConstructedBy(creature_id));
    }
}

impl CreatureTask for CreatureConstructingTask {
    fn stop(&self, commands: &mut Commands) {
        commands
            .entity(self.creature_id)
            .remove::<CreatureConstructingTask>()
            .insert(IdlingCreature);

        commands
            .entity(self.construction_site_id)
            .
            .remove::<ConstructedBy>();
    }
}

pub trait CreatureTask {
    fn stop(&self, commands: &mut Commands);
}
