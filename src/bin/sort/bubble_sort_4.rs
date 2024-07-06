// Path: src/bin/sort/bubble_sort_4.rs
use std::fmt::Debug;

fn bubble_sort_4<T: Copy + Debug + Ord>(arr: &mut [T]) {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bubble_sort_4() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        bubble_sort_4(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
