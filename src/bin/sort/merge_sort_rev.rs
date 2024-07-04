//O(n log n)
// 分割統治法

use std::fmt::Debug;

fn merge_sort<T: Debug + Copy + Ord>(arr: &mut [T]) {
    if arr.len() == 1 {
        return;
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);

    merge_sort(left);
    merge_sort(right);

    let left = left.to_vec();
    let right = right.to_vec();

    let mut arr_idx = 0;
    let mut right_idx = 0;
    let mut left_idx = 0;

    // サブリストが両方ある間は比較して先頭に追加していく
    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] > right[right_idx] {
            arr[arr_idx] = right[right_idx];
            right_idx += 1;
        } else {
            arr[arr_idx] = left[left_idx];
            left_idx += 1;
        }
        arr_idx += 1;
    }

    // 片側のみ余ったら残りを追加
    while left_idx < left.len() {
        arr[arr_idx] = left[left_idx];
        arr_idx += 1;
        left_idx += 1;
    }

    while right_idx < right.len() {
        arr[arr_idx] = right[right_idx];
        arr_idx += 1;
        right_idx += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = [9, 3, 7, 5, 6, 4, 8, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
