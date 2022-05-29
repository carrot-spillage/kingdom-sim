use bevy::prelude::{App, Component, Entity, Plugin};

pub enum ResourceKind {
    Wood,
}

pub struct ResourceChunk {
    pub kind: ResourceKind,
    pub quantity: f32,
}

#[derive(Component)]
pub struct BreaksIntoResources(pub Vec<ResourceChunk>);

pub struct BreaksIntoResourcesEvent(pub Entity);

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BreaksIntoResourcesEvent>();
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
