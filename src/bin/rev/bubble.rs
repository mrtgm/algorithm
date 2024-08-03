// Path: src/bin/rev/bubble.rs
use std::fmt::Debug;

fn bubble<T: Copy + Debug + Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 1..(arr.len() - i) {
            if arr[j - 1] > arr[j] {
                arr.swap(j, j - 1);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bubble() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        bubble(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
