use std::{
    borrow::Borrow,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

struct Node<T: Debug> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct List<T: Debug> {
    head: Option<Box<Node<T>>>,
}

impl<T: Debug + Default> List<T> {
    fn prepend(&mut self, data: T) {
        let new_node = Node { data, next: None };
        let old_head = std::mem::replace(&mut self.head, Some(Box::new(new_node)));
        let head = self.head.as_deref_mut().unwrap();
        head.next = old_head;
    }
    fn append(&mut self, data: T) {
        let new_node = Node { data, next: None };
        let mut last_node = self.head.as_deref_mut().unwrap();
        while let Some(ref mut node) = last_node.next {
            last_node = node;
        }
        last_node.next = Some(Box::new(new_node));
    }
    fn insert(&mut self, index: usize, data: T) {
        let new_node = Node { data, next: None };
        let mut target_node = self.head.as_deref_mut().unwrap();

        for _ in 0..index {
            if let Some(ref mut node) = target_node.next {
                target_node = node.deref_mut()
            } else {
                target_node.next = Some(Box::new(new_node));
                return;
            }
        }

        let old_node = std::mem::replace(target_node, new_node);
        target_node.next = Some(Box::new(old_node));
    }

    fn remove_last(&mut self) {
        fn inner<T: Debug>(node: &mut Option<Box<Node<T>>>) {
            if let Some(ref mut n) = node {
                if n.next.is_none() {
                    *node = None;
                } else {
                    inner(&mut n.next);
                }
            }
        }
        inner(&mut self.head);
    }

    fn remove(&mut self, index: usize) {
        let mut target_node = &mut self.head;

        {
            for _ in 0..index {
                let n = target_node.as_mut().unwrap();

                if n.next.is_none() {
                    return self.remove_last();
                }
                target_node = &mut n.next;

                // スコープ的に target_node に対する mut の借用が効いており、
                // *target_node によって値を変更することができない？
                // なんで再帰にするといけるのかわからん、ループで書くとだめなのか？

                // if let Some(n) = target_node {
                //     if n.next.is_none() {
                //         *target_node = None;
                //         return;
                //     }
                //     target_node = &mut n.next;
                // }
            }
        }

        let mut old_node = target_node.take();

        if let Some(old_node) = old_node.as_deref_mut() {
            let old_node_next = old_node.next.take();
            *target_node = old_node_next; // Box は * 経由で値書き換えられる
        } else {
            return self.remove_last();
        }
    }

    fn print(&self) {
        let mut node = &self.head;
        print!("{{");
        while let Some(n) = node {
            print!("{:?}", n.data);
            node = &n.next;
            if node.is_some() {
                print!(",");
            }
        }
        print!("}}");
        println!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_linked_list() {
        let mut l = List { head: None };
        l.prepend(30);
        l.prepend(50);
        l.prepend(20);

        l.insert(0, 1);

        l.remove(5);

        l.print();
    }
}
