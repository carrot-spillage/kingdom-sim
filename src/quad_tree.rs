use bevy::prelude::{Resource, Rect};

#[derive(Resource)]
pub struct QuadTree;

impl QuadTree {
    pub fn fit_rect_in_radius(&mut self, rect: Rect, radius: f32) -> Option<Rect> {
        return Some(rect);
    }
}
