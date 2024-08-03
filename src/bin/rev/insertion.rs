// Path: src/bin/rev/insertion.rs
use std::fmt::Debug;

fn insertion<T: Copy + Debug + Ord>(arr: &mut [T]) {
    for i in 1..(arr.len() + 1) {
        for j in (1..i).rev() {
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insertion() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        insertion(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
