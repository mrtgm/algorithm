// Path: src/bin/test/test.rs
use std::fmt::Debug;

fn test<T: Copy + Debug + Ord>(arr: &mut [T]) {}

#[cfg(test)]
mod test {
    use super::*;

    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_test() {
        let mut a = Point { x: 100, y: 200 };

        {
            let b = &mut a;

            // a.y;

            b;
        }
    }
}
