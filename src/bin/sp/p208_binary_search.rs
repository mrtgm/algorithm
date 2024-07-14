// Path: src/bin/sp/p208_binary_search.rs
use std::fmt::Debug;

#[derive(Clone)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Clone + Copy> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T)
    where
        T: Ord,
    {
        // 値が現在の節点より小さい場合、すでに左に値がある場合は左方向に再帰、なければ左に値を入れる
        if value < self.value {
            match self.left {
                Some(ref mut node) => node.insert(value),
                None => self.left = Some(Box::new(Node::new(value))),
            }
        } else {
            // 大きい場合は右方向に処理
            match self.right {
                Some(ref mut node) => node.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            }
        }
    }

    fn search(&self, value: T) -> bool
    where
        T: Ord,
    {
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

    fn find_min(&self) -> &Node<T>
    where
        T: Ord,
    {
        match self.left {
            Some(ref node) => node.find_min(),
            None => self,
        }
    }

    fn delete(&mut self, value: T) -> Option<Box<Node<T>>>
    where
        T: Ord + Debug,
    {
        // 節点を削除する
        // もし削除対象が葉であれば、その節点を削除する
        // もし削除対象が左の子のみ持っている場合、左の子を削除する
        // もし削除対象が左・右の子を持っている場合、右の子の最小値を削除対象に置き換え、右の子の最小値を削除する

        // 値が小さい場合、左の子を再帰的に探索
        if value < self.value {
            if let Some(ref mut node) = self.left {
                self.left = node.delete(value);
            }
        } else if value > self.value {
            // 値が大きい場合、右の子を再帰的に探索
            if let Some(ref mut node) = self.right {
                self.right = node.delete(value);
            }
        } else {
            // 削除対象が見つかった場合
            // 左の子がない場合、葉に行き着いたとみなして None で置き換える
            if self.left.is_none() {
                return None;
            }

            // 右の子がない場合、左の子を返して None で置き換える
            if self.right.is_none() {
                return self.left.take();
            }

            // 左右の子がある場合、右の子の最小値を取得し、削除対象に置き換える
            let right = self.right.as_mut().unwrap();
            let min = right.find_min().value;
            self.value = min;

            self.right = right.delete(min);
        }

        Some(Box::new(self.clone()))
    }

    fn print(&self)
    where
        T: Debug,
    {
        // 通りがけ順に再帰するとソートした値が得られる
        // 2分木は、左の子<自分自身<右の子となるため、左->自分->右の順で再帰的に出力する
        if let Some(ref node) = self.left {
            node.print();
        }
        println!("{:?}", self.value);
        if let Some(ref node) = self.right {
            node.print();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p208_binary_search() {
        let mut root = Node::new(5);
        root.insert(3);
        root.insert(7);
        root.insert(1);
        root.insert(4);
        root.insert(6);
        root.insert(9);

        assert_eq!(true, root.search(3));
        assert_eq!(false, root.search(8));

        root.print();

        println!("-----------delete 5,4,9-------");
        root.delete(5);
        // root.delete(4);
        // root.delete(9);

        root.print();
    }
}
