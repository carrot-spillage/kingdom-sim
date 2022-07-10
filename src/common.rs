use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct CreationProgress(pub f32);

/**
 * Means no one of the player's workers should be able to claim this entityfor anything else.
 */
#[derive(Component)]
pub struct ClaimedBy(pub Entity);
