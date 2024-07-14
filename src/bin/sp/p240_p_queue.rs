// Path: src/bin/sp/p240_p_queue.rs
use std::{
    fmt::Debug,
    i32::{MAX, MIN},
};

struct PriorityQueue {
    size: usize,
    queue: Vec<i32>,
}

fn parent(u: usize) -> usize {
    u / 2
}

fn left(u: usize) -> usize {
    2 * u + 1
}

fn right(u: usize) -> usize {
    2 * u + 2
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue {
            size: 0,
            queue: Vec::new(),
        }
    }

    fn insert(&mut self, value: i32) {
        self.size += 1;
        self.queue.push(MIN);

        self.increase_key(self.size - 1, value);
    }

    fn increase_key(&mut self, i: usize, key: i32) {
        if key < self.queue[i] {
            panic!("new key is smaller than current key");
        }

        self.queue[i] = key;
        let mut j = i;

        // 親の値が子より大きくなるまで入れ替えを続ける、または根に達するまで
        while j > 0 && self.queue[parent(j)] < self.queue[j] {
            self.queue.swap(j, parent(j));
            j = parent(j);
        }
    }

    fn max_heapify(&mut self, i: usize) {
        let l = left(i);
        let r = right(i);
        let mut largest = i;

        if l < self.size && self.queue[l] > self.queue[largest] {
            largest = l;
        }

        if r < self.size && self.queue[r] > self.queue[largest] {
            largest = r;
        }

        if largest != i {
            self.queue.swap(i, largest);
            self.max_heapify(largest);
        }
    }

    fn extract_max(&mut self) -> i32 {
        if self.size < 1 {
            panic!("heap underflow");
        }

        let max = self.queue[0];
        self.queue[0] = self.queue[self.size - 1];
        self.size -= 1;
        self.max_heapify(0); // 0 からヒープを再構築

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p240_p_queue() {
        let mut queue = PriorityQueue::new();

        queue.insert(8);
        queue.insert(2);

        assert_eq!(8, queue.extract_max());

        queue.insert(10);
        queue.insert(11);
        queue.insert(3);

        assert_eq!(11, queue.extract_max());

        queue.insert(5);
        queue.insert(6);

        assert_eq!(10, queue.extract_max());
    }
}
