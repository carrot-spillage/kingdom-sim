use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy)]
struct TimedQueItem<T: Clone>(T, u32);

impl<T: Clone> PartialEq for TimedQueItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl<T: Clone> PartialOrd for TimedQueItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Clone> Eq for TimedQueItem<T> {}

impl<T: Clone> Ord for TimedQueItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1).reverse()
    }
}

pub(crate) struct TimedQue<T: Clone> {
    tick: u32,
    heap: BinaryHeap<TimedQueItem<T>>,
}

impl<T: Clone + Eq + Ord + Default> Default for TimedQue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Eq + Ord> TimedQue<T> {
    pub fn new() -> Self {
        Self {
            tick: 0,
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, data: T, duration: u32) {
        self.heap.push(TimedQueItem(data, self.tick + duration));
    }

    // peeks and pops every item whose final_tick value equals to the current tick
    pub fn pop_elapsed(&mut self) -> Vec<T> {
        self.tick += 1;
        let mut elapsed: Vec<T> = Vec::new();
        while let Some(TimedQueItem(data, final_tick)) = self.heap.peek().cloned() {
            if final_tick == self.tick {
                self.heap.pop();
                elapsed.push(data);
            } else {
                break;
            }
        }
        elapsed
    }

    pub fn remove(&mut self, data: T) {
        println!("implement remove")
    }
}
