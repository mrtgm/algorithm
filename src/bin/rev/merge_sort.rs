// Path: src/bin/rev/merge_sort.rs
use std::fmt::Debug;

fn merge_sort<T: Copy + Debug + Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);

    merge_sort(left);
    merge_sort(right);

    let left = left.to_vec();
    let right = right.to_vec();

    let mut left_i = 0;
    let mut right_i = 0;
    let mut arr_i = 0;

    while left.len() > left_i && right.len() > right_i {
        if left[left_i] > right[right_i] {
            arr[arr_i] = right[right_i];
            right_i += 1;
        } else {
            arr[arr_i] = left[left_i];
            left_i += 1;
        }
        arr_i += 1;
    }

    while left.len() > left_i {
        arr[arr_i] = left[left_i];
        left_i += 1;
        arr_i += 1;
    }

    while right.len() > right_i {
        arr[arr_i] = right[right_i];
        right_i += 1;
        arr_i += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
