// Path: src/bin/list/doubly2.rs
use std::{
    cell::RefCell,
    fmt::{self, Debug, Display},
    rc::Rc,
};

struct Node<T> {
    data: T,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

struct List<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            prev: None,
            next: None,
        }
    }

    fn append(node: &mut Rc<RefCell<Node<T>>>, data: T) -> Option<Rc<RefCell<Node<T>>>> {
        let is_last = node.borrow().next.is_none(); //Refcell から中身の不変参照を得る borrow, Rc は Deref 持ってるから透過的
        if is_last {
            let mut new_node = Node::new(data);
            new_node.prev = Some(node.clone()); //&mut Rc<T> から Rc<T> を得る clone
            let rc = Rc::new(RefCell::new(new_node));
            node.borrow_mut().next = Some(rc.clone());
            Some(rc)
        } else {
            if let Some(ref mut next) = node.borrow_mut().next {
                Self::append(next, data)
            } else {
                None
            }
        }
    }

    fn prepend(node: &mut Rc<RefCell<Node<T>>>, data: T) -> Option<Rc<RefCell<Node<T>>>> {
        // 今の先頭のノードを新しく作ったノードの next にセットする
        let mut new_node = Node::new(data);
        new_node.next = Some(node.clone());
        let rc = Rc::new(RefCell::new(new_node));
        Some(rc)
    }

    fn remove(node: &mut Rc<RefCell<Node<T>>>, index: usize) {
        if index == 0 {
            return;
        } else {
            if index == 1 {
                let mut n = node.borrow_mut();
                let next = n.next.clone();
                n.next = next.clone().unwrap().borrow_mut().next.clone(); //ここ整理したい・・・
                next.unwrap().borrow_mut().prev = Some(node.clone());
            } else if let Some(ref mut next) = node.borrow_mut().next {
                Self::remove(next, index - 1);
            }
        }
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }
    fn append(&mut self, data: T) {
        // 先頭を持ってくる
        if let Some(ref mut next) = self.head {
            self.tail = Node::append(next, data);
        } else {
            let f = Rc::new(RefCell::new(Node::new(data)));
            self.head = Some(f.clone());
            self.tail = Some(f);
        }
    }
    fn prepend(&mut self, data: T) {
        if let Some(ref mut next) = self.head {
            self.head = Node::prepend(next, data);
        } else {
            let f = Rc::new(RefCell::new(Node::new(data)));
            self.head = Some(f.clone());
            self.tail = Some(f);
        }
    }
    fn remove(&mut self, index: usize) {
        if index == 0 {
            if let Some(ref mut next) = self.head.clone() {
                self.head = next.borrow().next.clone();
                next.borrow_mut().prev = None;
            } else {
                self.head = None;
                self.tail = None;
            }
        }
        if let Some(ref mut next) = self.head.clone() {
            Node::remove(next, index);
        }
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut node = self.head.clone();
        while let Some(n) = node {
            write!(f, "{}", n.borrow().data)?;
            node = n.borrow().next.clone();
            if node.is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let mut list = List::new();
        assert_eq!(format!("{}", list), "[]");
        for i in 0..5 {
            list.append(i);
        }
        assert_eq!(format!("{}", list), "[0, 1, 2, 3, 4]");

        list.prepend(10);
        assert_eq!(format!("{}", list), "[10, 0, 1, 2, 3, 4]");

        list.remove(0);
        assert_eq!(format!("{}", list), "[0, 1, 2, 3, 4]");

        list.remove(3);
        assert_eq!(format!("{}", list), "[0, 1, 2, 4]");

        list.remove(1);
        assert_eq!(format!("{}", list), "[0, 2, 4]");
    }
}
