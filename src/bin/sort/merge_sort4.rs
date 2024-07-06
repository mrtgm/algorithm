// Path: src/bin/sort/merge_sort4.rs
use std::fmt::Debug;

fn merge_sort4<T: Copy + Debug + Ord>(arr: &mut [T]) {
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_sort4() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        merge_sort4(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
