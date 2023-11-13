use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy)]
struct TimerHeapItem<T: Clone>(T, u32);

impl<T: Clone> PartialEq for TimerHeapItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl<T: Clone> PartialOrd for TimerHeapItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Clone> Eq for TimerHeapItem<T> {}

impl<T: Clone> Ord for TimerHeapItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1).reverse()
    }
}

pub(crate) struct TimerHeap<T: Clone> {
    tick: u32,
    heap: BinaryHeap<TimerHeapItem<T>>,
}

impl<T: Clone + Default> Default for TimerHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> TimerHeap<T> {
    pub fn new() -> Self {
        Self {
            tick: 0,
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, data: T, duration: u32) {
        self.heap.push(TimerHeapItem(data, self.tick + duration));
    }

    // peeks and pops every item whose index equals to the given one
    pub fn try_produce(&mut self) -> Vec<T> {
        self.tick += 1;
        let mut elapsed: Vec<T> = Vec::new();
        while let Some(TimerHeapItem(data, final_tick)) = self.heap.peek().cloned() {
            if final_tick == self.tick {
                self.heap.pop();
                elapsed.push(data);
            } else {
                break;
            }
        }
        elapsed
    }
}
