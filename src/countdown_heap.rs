use std::collections::BinaryHeap;

use bevy::prelude::{App, Entity, Plugin};

struct Produced;

#[derive(Debug, Clone, Copy)]
struct OrderedEntity(Entity, u32);

impl PartialEq for OrderedEntity {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl PartialOrd for OrderedEntity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for OrderedEntity {}

impl Ord for OrderedEntity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1).reverse()
    }
}

struct ProducerHeap {
    tick: u32,
    heap: BinaryHeap<OrderedEntity>,
}

impl ProducerHeap {
    pub fn new() -> Self {
        Self {
            tick: 0,
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, entity: Entity, duration: u32) {
        self.heap.push(OrderedEntity(entity, self.tick + duration));
    }

    // peeks and pops every item whose index equals to the given one
    pub fn try_produce(&mut self) -> Vec<Entity> {
        self.tick += 1;
        let mut produced: Vec<Entity> = Vec::new();
        while let Some(OrderedEntity(item, item_index)) = self.heap.peek().cloned() {
            if item_index == self.tick {
                self.heap.pop();
                produced.push(item);
            } else {
                break;
            }
        }
        produced
    }
}

// pub struct ProducerPlugin<T> {
//     heap: ProducerHeap<T>,
// }

// impl<T: std::marker::Sync> Plugin for ProducerPlugin<T> {
//     fn build(&self, app: &mut App) {
//         app.add_systems(
//             Update,
//             (proceed_to_next_task, update_tooltip_text, update_tooltip)
//                 .run_if(in_state(GameState::Playing)),
//         );
//     }

//     fn name(&self) -> &str {
//         std::any::type_name::<Self>()
//     }
// }
