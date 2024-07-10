// Path: src/bin/sp/p95_doubly.rs
use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Ref, RefCell},
    fmt::Debug,
    ops::Deref,
    rc::Rc,
};

struct Node<T: Debug> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

struct List<T: Debug> {
    head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Debug> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            next: None,
            prev: None,
        }
    }
}

impl<T: Debug + Ord> List<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn insert(&mut self, data: T) {
        let mut new_node = Node::new(data);
        let mut old_node = self.head.clone();
        if let Some(ref mut node) = old_node {
            node.as_ref().borrow_mut().prev = Some(Rc::clone(node));
            new_node.next = Some(Rc::clone(node));
        }
        self.head = Some(Rc::new(RefCell::new(new_node)));
    }

    fn find_node<'a>(node: Rc<RefCell<Node<T>>>, data: T) -> Rc<RefCell<Node<T>>> {
        if node.as_ref().borrow().data == data {
            return node;
        }
        if let Some(ref next_node) = node.as_ref().borrow().next {
            Self::find_node(next_node.clone(), data)
        } else {
            node.clone()
        }
    }

    fn delete(&mut self, data: T) {
        let node = self.head.clone();
        let target_node = Self::find_node(node.unwrap(), data);
        let prev_node = target_node.as_ref().borrow().prev.clone();
        let next_node = target_node.as_ref().borrow().next.clone();
        if let Some(ref prev) = prev_node {
            prev.as_ref().borrow_mut().next = next_node.clone();
        }
        if let Some(ref next) = next_node {
            next.as_ref().borrow_mut().prev = prev_node.clone();
        }
    }
    fn deleteFirst(&mut self, data: T) {}
    fn deleteLast(&mut self) {}
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut node = self.head.clone();
        while let Some(n) = node {
            write!(f, "{:?}", n.as_ref().borrow().data)?;
            node = n.as_ref().borrow().next.clone();
            if node.is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p95_doubly() {
        let mut list = List::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);
        list.insert(4);
        list.insert(5);
        list.insert(6);
        list.delete(3);
        println!("{:?}", list);

        // let mut arr = [5, 2, 4, 6, 1, 3];
    }
}
