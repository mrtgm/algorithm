// Path: src/bin/test/test.rs
use std::fmt::Debug;

fn test<T: Copy + Debug + Ord>(arr: &mut [T]) {}

#[cfg(test)]
mod test {
    use std::{ops::Deref, rc::Rc};

    use super::*;

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    struct A<'a> {
        b: &'a mut Option<&'a mut B<'a>>,
    }

    struct B<'a> {
        a: &'a mut Option<&'a mut A<'a>>,
    }

    struct PP<'a> {
        p: &'a mut Point,
    }

    #[test]
    fn test_test() {
        let mut p = Point { x: 1, y: 2 };
        let pp = &mut p;
        let p_moved = p;
        println!("{:?}", pp);

        let mut rc = Rc::new(Point { x: 1, y: 2 });

        let a = Some("aaaa");
        let b = a.as_ref();

        let a = rc.deref();

        let rc2 = rc.clone();
        let rc3 = Rc::clone(&rc);

        let shared_ref = &rc.clone();
        let mutable_ref = &mut rc.clone();

        println!("{:?}", Rc::strong_count(&rc));

        *mutable_ref = Rc::new(Point { x: 3, y: 4 });

        // (*rc).x = 5; cannot assign to data in an `Rc` trait `DerefMut` is required to modify through a dereference
        // 共有されているものには書き込めない原則

        println!("{:?}", mutable_ref);
    }
}
