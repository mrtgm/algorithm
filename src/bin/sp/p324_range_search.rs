// Path: src/bin/sp/p324_range_search.rs
use std::fmt::Debug;

// 2次元平面上の点の集合に対して、与えられた範囲に含まれる点を列挙する問題
// 与えられる点の集合は、平面上の点の座標を表す (x, y) の組で表される
// 与えられる点の集合を平面上に配置し、その点の集合に対してクエリを処理する
// クエリは、平面上の点の集合に対して、与えられる範囲に含まれる点を列挙するもの
// 与えられる範囲は、平面上の点の座標を表す (x1, x2, y1, y2) の組で表される
// 与えられる範囲に含まれる点を列挙する問題を解くためには、平面上の点の集合を効率的に管理するデータ構造が必要
#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Clone> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        if value < self.value {
            if let Some(ref mut left) = self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(TreeNode::new(value)));
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    fn contains(&self, value: &T) -> bool {
        if *value == self.value {
            true
        } else if *value < self.value {
            match self.left {
                Some(ref left) => left.contains(value),
                None => false,
            }
        } else {
            match self.right {
                Some(ref right) => right.contains(value),
                None => false,
            }
        }
    }

    fn range_search<'a>(&'a self, low: T, high: T, result: &mut Vec<&'a T>) {
        if low <= self.value && high >= self.value {
            result.push(&self.value);
        }

        if let Some(ref left) = self.left {
            if low < self.value {
                left.range_search(low.clone(), high.clone(), result);
            }
        }

        if let Some(ref right) = self.right {
            if high > self.value {
                right.range_search(low.clone(), high.clone(), result);
            }
        }
    }
}

#[derive(Debug)]
struct BinarySearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Clone> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => node.insert(value),
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
    }

    fn contains(&self, value: &T) -> bool {
        match self.root {
            Some(ref node) => node.contains(value),
            None => false,
        }
    }

    fn range_search<'a>(&'a self, low: T, high: T) -> Vec<&'a T> {
        let mut result = Vec::new();
        if let Some(ref node) = self.root {
            node.range_search(low, high, &mut result);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p324_disjoint_set() {
        let mut bst = BinarySearchTree::new();
        let values = vec![10, 5, 20, 3, 7, 15, 30];

        for value in values {
            bst.insert(value);
        }

        let low = 5;
        let high = 15;
        let result = bst.range_search(low, high);

        println!("Values in range [{}, {}]: {:?}", low, high, result);
    }
}
