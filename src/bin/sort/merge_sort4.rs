// Path: src/bin/sort/merge_sort4.rs
use std::fmt::Debug;

fn merge_sort4<T: Copy + Debug + Ord>(arr: &mut [T]) {
    if arr.len() == 1 {
        return;
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);

    merge_sort4(left);
    merge_sort4(right);

    let left = left.to_vec();
    let right = right.to_vec();

    let mut arr_index = 0;
    let mut right_index = 0;
    let mut left_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] > right[right_index] {
            arr[arr_index] = right[right_index];
            right_index += 1;
        } else {
            arr[arr_index] = left[left_index];
            left_index += 1;
        }
        arr_index += 1;
    }

    while left_index < left.len() {
        arr[arr_index] = left[left_index];
        left_index += 1;
        arr_index += 1;
    }

    while right_index < right.len() {
        arr[arr_index] = right[right_index];
        right_index += 1;
        arr_index += 1;
    }
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
