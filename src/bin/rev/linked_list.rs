// Path: src/bin/rev/linked_list.rs
use std::{fmt::Debug, ops::Deref};

struct Node<T: Debug + Clone> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct List<T: Debug + Clone> {
    head: Option<Box<Node<T>>>,
}

impl<T: Debug + Clone> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

impl<T: Debug + Clone> List<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn find_last(&mut self) -> &mut Node<T> {
        let mut last_node = self.head.as_deref_mut().unwrap();
        while let Some(ref mut node) = last_node.next {
            last_node = node;
        }
        last_node
    }

    fn append(&mut self, data: T) {
        let last_node = List::find_last(self);
        let new_node = Node::new(data);
        last_node.next.replace(Box::new(new_node));
    }

    fn prepend(&mut self, data: T) {
        let old = self.head.take();
        let mut new = Node::new(data);
        new.next = old;
        self.head = Some(Box::new(new));
    }
}

impl<T: Debug + Clone> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut node = &self.head;
        while let Some(v) = node {
            write!(f, "{:?}", v.as_ref().value)?;
            node = &v.as_ref().next;
            if node.is_some() {
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
    fn test_linked_list() {
        let mut l = List::new();
        l.prepend(4);
        l.prepend(8);
        l.prepend(12);
        l.append(11);

        println!("{:?}", l);
    }
}
