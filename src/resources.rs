use bevy::prelude::Component;

pub enum ResourceKind {
    Wood,
}

pub struct ResourceChunk {
    pub kind: ResourceKind,
    pub quantity: f32,
}

#[derive(Component)]
pub struct BreaksIntoResources(pub Vec<ResourceChunk>);
