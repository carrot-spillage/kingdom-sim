use bevy::{
    prelude::{Rect, Resource, Vec2},
    render::render_resource::encase::rts_array::Length,
};
use std::fmt::Debug;
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

#[derive(Resource)]
pub struct QuadTree<T: Copy + Eq + Hash + Debug> {
    nodes: Vec<QuadTreeNode<T>>,
    tenant_keys_and_nodes: HashMap<T, Vec<usize>>,
}

#[derive(Debug)]
struct QuadTreeNode<T: Copy + Eq + Hash + Debug> {
    quad: Rect,
    level: u32,
    parent_index: Option<usize>,
    index: usize,
    tenant_key: Option<T>,
    child_indexes: Option<Vec<usize>>,
}

impl<T: Copy + Eq + Hash + Debug> QuadTree<T> {
    pub fn new(quad: Rect, max_level: u32) -> Self {
        Self {
            tenant_keys_and_nodes: HashMap::new(),
            nodes: traverse_nodes(quad, max_level),
        }
    }

    pub fn try_occupy_rect(&mut self, rect: Rect, tenant_key: T) -> bool {
        let mut found_indexes: Vec<usize> = vec![];
        let root_quad = self.nodes[0].quad;
        if rect.min.x < root_quad.min.x
            || rect.min.y < root_quad.min.y
            || rect.max.x > root_quad.max.x
            || rect.max.y > root_quad.max.y
        {
            println!(
                "Rect {:?} is outside of the world {:?} by xy {:?} {:?}",
                rect,
                root_quad,
                rect.min.distance(root_quad.min),
                rect.max.distance(root_quad.max)
            );
            return false;
        }

        let success = self.try_find_leaf_indexes(0, rect, &mut found_indexes);
        if success {
            for index in &found_indexes {
                self.nodes
                    .get_mut(*index)
                    .map(|node| node.tenant_key = Some(tenant_key));
            }

            self.tenant_keys_and_nodes.insert(tenant_key, found_indexes);
            println!("Tenant occupied rect {:?}", rect);

            return true;
        }

        return false;
    }

    fn try_find_leaf_indexes(
        &self,
        node_index: usize,
        rect: Rect,
        found_indexes: &mut Vec<usize>,
    ) -> bool {
        let node = self.nodes.get(node_index).unwrap();

        if let Some(child_indexes) = &node.child_indexes {
            // branch node
            for index in child_indexes {
                let child = self.nodes.get(*index).unwrap();
                let intersects = child.quad.contains(rect.min) || child.quad.contains(rect.max);
                let descendants_have_tenant =
                    intersects && !self.try_find_leaf_indexes(child.index, rect, found_indexes);
                if descendants_have_tenant {
                    return false;
                }
            }
            return true;
        } else {
            // leaf node
            if node.tenant_key.is_some() {
                println!("Node {:?} already has tenant", node);
                return false;
            }

            found_indexes.push(node.index);
            return true;
        }
    }

    pub fn fit_rect_in_radius(&mut self, rect: Rect, radius: f32) -> Option<Rect> {
        return Some(rect);
    }
}

fn traverse_nodes<T: Copy + Eq + Hash + Debug>(quad: Rect, max_level: u32) -> Vec<QuadTreeNode<T>> {
    let root_node = QuadTreeNode::<T> {
        index: 0,
        level: 0,
        tenant_key: None,
        child_indexes: None,
        parent_index: None,
        quad,
    };
    let mut nodes: Vec<QuadTreeNode<T>> = vec![root_node];
    let mut untraversed_node_indexes: VecDeque<usize> = VecDeque::new();
    untraversed_node_indexes.push_front(0);

    while !untraversed_node_indexes.is_empty() {
        let node_index = untraversed_node_indexes.pop_back().unwrap();
        let node = nodes.get(node_index).unwrap();
        let quad = node.quad;
        let level = node.level;
        if level < max_level {
            let child_indexes = build_children(&mut nodes, node_index, level + 1, &quad);
            child_indexes
                .iter()
                .for_each(|index| untraversed_node_indexes.push_front(*index));
            nodes.get_mut(node_index).unwrap().child_indexes = Some(child_indexes);
        }
    }

    return nodes;
}

fn build_children<T: Copy + Eq + Hash + Debug>(
    all_nodes: &mut Vec<QuadTreeNode<T>>,
    parent_index: usize,
    level: u32,
    quad: &Rect,
) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];

    let center = quad.center();

    let quads = [
        Rect::from_corners(center, quad.max), // ◳
        Rect::from_corners(
            Vec2::new(center.x, quad.min.y),
            Vec2::new(quad.max.x, center.y),
        ), // ◲
        Rect::from_corners(quad.min, center), // ◱
        Rect::from_corners(
            Vec2::new(quad.min.x, center.y),
            Vec2::new(center.x, quad.max.y),
        ), // ◰
    ];

    for quad in quads {
        let child_node = QuadTreeNode::<T> {
            index: all_nodes.length(),
            level,
            tenant_key: None,
            child_indexes: None,
            parent_index: Some(parent_index),
            quad,
        };
        result.push(child_node.index);
        all_nodes.push(child_node);
    }

    return result;
}
