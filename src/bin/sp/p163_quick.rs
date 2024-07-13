// Path: src/bin/sp/p163_quick.rs
use std::fmt::Debug;

fn p158_partition<T: Copy + Debug + Ord>(arr: &mut [T]) -> usize {
    let key = arr[arr.len() - 1];
    let mut i = -1 as i64;
    for j in 0..(arr.len() - 1) {
        if arr[j] <= key {
            i += 1;
            arr.swap(i as usize, j);
        }
    }
    arr.swap((i + 1) as usize, arr.len() - 1);
    (i + 1) as usize
}

fn p163_quick<T: Copy + Debug + Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let q = p158_partition(arr);
    p163_quick(&mut arr[0..q]);
    p163_quick(&mut arr[q..]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p163_quick() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        p163_quick(&mut arr);

        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
