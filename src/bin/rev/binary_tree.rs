// Path: src/bin/rev/binary_tree.rs
use std::fmt::Debug;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl Node<i32> {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        //小さかったら再帰的に左方向に入れる
        if value < self.value {
            match self.left {
                Some(ref mut node) => node.insert(value),
                None => self.left = Some(Box::new(Node::new(value))),
            }
        } else {
            match self.right {
                Some(ref mut node) => node.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            }
        }
    }

    fn search(&self, value: i32) -> bool {
        if value == self.value {
            return true;
        }

        if value < self.value {
            match self.left {
                Some(ref node) => node.search(value),
                None => false,
            }
        } else {
            match self.right {
                Some(ref node) => node.search(value),
                None => false,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_tree() {
        let mut arr = [5, 2, 4, 6, 1, 3];

        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
