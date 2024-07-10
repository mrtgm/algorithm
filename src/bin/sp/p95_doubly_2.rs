use core::borrow;
// Path: src/bin/sp/p95_doubly_2.rs
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    fmt::Debug,
    ops::Deref,
    rc::{Rc, Weak},
};

struct Node<T: Debug> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
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
        List { head: None }
    }
    fn prepend(&mut self, data: T) {
        let new = Rc::new(RefCell::new(Node::new(data)));
        let weaken = Rc::downgrade(&new);
        let old = self.head.replace(new.clone()).map(|v| {
            v.as_ref().borrow_mut().prev = Some(weaken);
            v
        });
        new.as_ref().borrow_mut().next = old;
    }
    fn delete(&mut self, data: T) {
        let mut top: Option<Rc<RefCell<Node<T>>>> = self.head.clone();
        while let Some(node) = top {
            let borrowed = node.as_ref().borrow_mut();
            if borrowed.data == data {
                let prev = borrowed.prev.as_ref().map(|v| {
                    v.upgrade().unwrap().as_ref().borrow_mut().next = borrowed.next.clone();
                    v.clone() //あかんやろ
                });
                borrowed.next.as_deref().map(|v| {
                    v.borrow_mut().prev = prev;
                });
                break;
            }
            top = borrowed.next.clone();
        }
    }
    fn delete_first(&mut self) {
        if let Some(next) = self
            .head
            .as_deref()
            .map(|v| v.borrow().next.clone())
            .flatten()
        {
            self.head.replace(next);
        } else {
            panic!("no item")
        }
    }
    fn delete_last(&mut self) {
        let mut last: Option<Rc<RefCell<Node<T>>>> = self.head.clone();

        while let Some(node) = last.clone() {
            let borrowed = node.as_ref().borrow_mut();
            if borrowed.next.is_none() {
                break;
            } else {
                last = borrowed.next.clone();
            }
        }

        last.map(|v| {
            let prev = v.as_ref().borrow_mut().prev.clone();
            prev.map(|v| {
                v.upgrade().unwrap().as_ref().borrow_mut().next = None;
            })
        });
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut top: Option<Rc<RefCell<Node<T>>>> = self.head.clone();
        while let Some(node) = top {
            write!(f, "{:?}", node.as_ref().borrow().data)?;
            let borrowed_node = node.as_ref().borrow();
            top = borrowed_node.next.clone();
            if borrowed_node.next.is_some() {
                write!(f, ",")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p95_doubly_2() {
        let mut l = List::new();
        l.prepend(30);
        l.prepend(60);
        l.prepend(120);
        println!("{:?}", l);
        l.delete(30);
        println!("{:?}", l);
        l.delete(60);
        println!("{:?}", l);
        l.prepend(200);
        println!("{:?}", l);
        l.delete_first();
        println!("{:?}", l);
        l.prepend(230);
        l.prepend(30);
        l.prepend(60);
        println!("{:?}", l);
        l.delete_last();
        println!("{:?}", l);
    }
}
