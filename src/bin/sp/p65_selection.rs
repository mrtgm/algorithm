// Path: src/bin/sp/p65_selection.rs
use std::{fmt::Debug, mem::swap};

fn p65_selection<T: Copy + Debug + Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p65_selection() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        p65_selection(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
