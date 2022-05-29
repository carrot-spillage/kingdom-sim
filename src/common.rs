use bevy::{
    math::Vec3,
    prelude::{Component, Entity},
};

#[derive(Component)]
pub struct CreationProgress(pub f32);

#[derive(Clone, Copy)]
pub enum TargetOrPosition {
    Target(Entity),
    Position(Vec3),
}
