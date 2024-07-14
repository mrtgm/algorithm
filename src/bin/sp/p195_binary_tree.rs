// Path: src/bin/sp/p195_binary_tree.rs
use std::fmt::Debug;

// 左子・右子表現
#[derive(Debug, Clone)]
struct Node<T: Clone> {
    parent: T,        //親、根の場合は -1
    left: Option<T>,  // 左の子
    right: Option<T>, // 右の子
}

impl<T: Clone> Node<T> {
    fn new(value: T) -> Self {
        Node {
            parent: value,
            left: None,
            right: None,
        }
    }
}

struct Tree<T: Clone>(Vec<Node<T>>);

impl Tree<i32> {
    fn new(size: usize) -> Self {
        Tree(vec![Node::new(-1).clone(); size])
    }

    fn insert(&mut self, vertex: i32, parent: i32, left: Option<i32>, right: Option<i32>) {
        let index = vertex as usize;
        let node = &mut self.0[index];
        node.parent = parent;
        node.left = node.left.or(left);
        node.right = node.right.or(right);

        if let Some(left) = left {
            self.0[left as usize].parent = vertex;
        }
        if let Some(right) = right {
            self.0[right as usize].parent = vertex;
        }
    }

    fn get_depth(&self, vertex: i32) -> i32 {
        let mut depth_left = 0;
        let mut depth_right = 0;

        let node = &self.0[vertex as usize];

        if let Some(left) = node.left {
            depth_left = self.get_depth(left);
        }
        if let Some(right) = node.right {
            depth_right = self.get_depth(right);
        }

        1 + depth_left.max(depth_right)
    }

    // スタックを使う版
    fn print_children_preorder(&self, vertex: i32) {
        //dfs, preorder（行きがけ順）
        let mut stack = vec![vertex];

        while let Some(v) = stack.pop() {
            println!("{}", v);

            let node = &self.0[v as usize];
            if let Some(right) = node.right {
                stack.push(right);
            }
            if let Some(left) = node.left {
                stack.push(left);
            }
        }
    }

    //  再帰版
    fn print_children_preorder_recursive(&self, vertex: i32) {
        //dfs, preorder（行きがけ順）
        println!("{}", vertex);

        let node = &self.0[vertex as usize];
        if let Some(left) = node.left {
            self.print_children_preorder_recursive(left);
        }
        if let Some(right) = node.right {
            self.print_children_preorder_recursive(right);
        }
    }

    fn print_children_postorder(&self, vertex: i32) {
        //dfs, postorder（帰りがけ順）
        let mut stack = vec![(vertex, true)];

        while let Some((v, preorder)) = stack.pop() {
            if preorder {
                stack.push((v, false)); // 2回目の pop で出力するために false を積む

                let node = &self.0[v as usize];
                if let Some(right) = node.right {
                    stack.push((right, true));
                }
                if let Some(left) = node.left {
                    stack.push((left, true));
                }
            } else {
                println!("{}", v);
            }
        }
    }

    fn print_children_postorder_recursive(&self, vertex: i32) {
        //dfs, postorder（帰りがけ順）
        let node = &self.0[vertex as usize];
        if let Some(left) = node.left {
            self.print_children_postorder_recursive(left);
        }
        if let Some(right) = node.right {
            self.print_children_postorder_recursive(right);
        }
        println!("{}", vertex);
    }

    fn print_children_inorder(&self, vertex: i32) {
        //dfs, inorder（通りがけ順）
        let mut stack = vec![(vertex, true)];

        while let Some((v, preorder)) = stack.pop() {
            if preorder {
                let node = &self.0[v as usize];
                if let Some(right) = node.right {
                    stack.push((right, true));
                }

                stack.push((v, false)); // 2回目の pop で出力するために false を積む

                if let Some(left) = node.left {
                    stack.push((left, true));
                }
            } else {
                println!("{}", v);
            }
        }
    }

    fn print_children_inorder_recursive(&self, vertex: i32) {
        //dfs, inorder（通りがけ順）
        let node = &self.0[vertex as usize];
        if let Some(left) = node.left {
            self.print_children_inorder_recursive(left);
        }
        println!("{}", vertex);
        if let Some(right) = node.right {
            self.print_children_inorder_recursive(right);
        }
    }
}

fn p195_binary_tree<T: Copy + Debug + Ord>(arr: &mut [T]) {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p195_binary_tree() {
        let mut t = Tree::new(7);

        t.insert(0, -1, Some(1), Some(4));
        t.insert(1, 0, Some(2), Some(3));
        t.insert(2, 1, None, None);
        t.insert(3, 1, None, None);

        t.insert(4, 0, Some(5), Some(6));
        t.insert(5, 4, None, None);
        t.insert(6, 4, None, None);

        println!("{}", t.get_depth(0));
        println!("----");
        t.print_children_preorder(0);
        println!("----");
        t.print_children_preorder_recursive(0);
        println!("----");
        t.print_children_postorder(0);
        println!("----");
        t.print_children_inorder(0);
    }
}
