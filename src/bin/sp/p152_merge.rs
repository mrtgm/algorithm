// Path: src/bin/sp/p152_merge.rs
use std::fmt::Debug;

fn p152_merge<T: Copy + Debug + Ord>(arr: &mut [T]) {
    if arr.len() == 1 {
        return;
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);

    p152_merge(left);
    p152_merge(right);

    let left = left.to_vec();
    let right = right.to_vec();

    let mut arr_index = 0;
    let mut left_index = 0;
    let mut right_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if right[right_index] < left[left_index] {
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
    fn test_p152_merge() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        p152_merge(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);

        let mut arr2 = [90, 80, 70, 60, 50, 40, 30, 20, 10];
        p152_merge(&mut arr2);
        assert_eq!(arr2, [10, 20, 30, 40, 50, 60, 70, 80, 90]);
    }
}
