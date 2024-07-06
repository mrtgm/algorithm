// Path: src/bin/sort/inse4.rs
use std::fmt::Debug;

fn inse4<T: Copy + Debug + Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let key = arr[i];
        for j in (0..i).rev() {
            if arr[j] > key {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_inse4() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        inse4(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
