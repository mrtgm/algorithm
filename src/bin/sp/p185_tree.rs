// Path: src/bin/sp/p185_tree.rs
use std::fmt::Debug;

// 左子右兄弟表現
#[derive(Debug, Clone)]
struct Node<T: Clone> {
    parent: T,
    left: Option<T>,  // もっとも左の子
    right: Option<T>, // すぐ右の兄弟
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

        node.left = node.left.or(left); // ２つの option のマージには or が使える
                                        // node.left に値があればそれを返し、なければ left を返す
        node.right = node.right.or(right);
    }

    fn insert_by_array(&mut self, parent: i32, children: Vec<i32>) {
        let grand_father = self.0[parent as usize].parent;
        let first_child = children.get(0).cloned(); // get 使えば良いのか・・・
        self.insert(parent, grand_father, first_child, None);

        for i in 0..children.len() {
            self.insert(children[i], parent, None, children.get(i + 1).cloned());
        }
    }

    fn get_depth(&self, vertex: i32) -> i32 {
        let mut depth = 0;
        let mut node: usize = vertex as usize;
        while self.0[node].parent != -1 {
            node = self.0[node].parent as usize;
            depth += 1;
        }
        depth
    }

    fn print_children(&self, vertex: i32) {
        // もっとも左の子から、その子の右の兄弟をたどる
        let mut node = &self.0[vertex as usize].left;

        while let Some(left) = node {
            println!("{}", left);
            node = &self.0[*left as usize].right;
        }
    }
}

fn p185_tree<T: Copy + Debug + Ord>(arr: &mut [T]) {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p185_tree() {
        let mut tree = Tree::new(11);

        // 0 は 1 4 8 を子に持つ
        tree.insert_by_array(0, vec![1, 4, 10]);
        // 1 は 2 3 を子に持つ
        tree.insert_by_array(1, vec![2, 3]);
        // 2 は子を持たない
        tree.insert_by_array(2, vec![]);
        // 3 は子を持たない
        tree.insert_by_array(3, vec![]);
        // 4 は 5 8 を子に持つ
        tree.insert_by_array(4, vec![5, 8]);
        // 5 は 6 7 を子に持つ
        tree.insert_by_array(5, vec![6, 7]);
        // 6 は子を持たない
        tree.insert_by_array(6, vec![]);
        // 7 は子を持たない
        tree.insert_by_array(7, vec![]);
        // 8 は 9 を子に持つ
        tree.insert_by_array(8, vec![9]);
        // 9 は 子を持たない
        tree.insert_by_array(9, vec![]);
        // 10 は 子を持たない
        tree.insert_by_array(10, vec![]);

        tree.print_children(0); // 1 4 8
        tree.print_children(1); // 2 3
    }
}
