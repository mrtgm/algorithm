// Path: src/bin/sort/selection4.rs
use std::fmt::Debug;

fn selection4<T: Copy + Debug + Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_idx = i;

        for j in i..arr.len() {
            if arr[min_idx] > arr[j] {
                min_idx = j;
            }
        }
        arr.swap(min_idx, i);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_selection4() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        selection4(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
