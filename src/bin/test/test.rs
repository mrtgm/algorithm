// Path: src/bin/test/test.rs
use std::fmt::Debug;

fn test<T: Copy + Debug + Ord>(arr: &mut [T]) {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_test() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        test(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
