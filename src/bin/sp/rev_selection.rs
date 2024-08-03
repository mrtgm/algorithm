// Path: src/bin/sp/rev_selection.rs
use std::fmt::Debug;

fn rev_selection<T: Copy + Debug + Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_index = i;

        for j in i..arr.len() {
            if arr[min_index] > arr[j] {
                min_index = j;
            }
        }
        arr.swap(min_index, i);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rev_selection() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        rev_selection(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
