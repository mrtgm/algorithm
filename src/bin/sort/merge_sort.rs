//O(n log n)
// 分割統治法

// 整列されていないリストを2つのサブリストに分割する
// サブリストを整列する
// サブリストをマージしてひとつの整列済みリストにする

use std::fmt::Debug;

fn merge_sort<T: Ord + Copy + Debug>(arr: &mut [T]) {
    // 1. [1,3,4,2]
    // 3. [1,3]
    // 5. [1]
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);
    // 2. [1,3], [4,2]
    // 4. [1], [3]
    merge_sort(left);
    merge_sort(right);

    let mut left_index = 0;
    let mut right_index = 0;
    let mut arr_index = 0;

    // 6. [1], [3]
    // 14. [4], [2]
    let mut left = left.to_vec();
    let mut right = right.to_vec();

    // マージの手順
    // 1. 2つのサブリストの先頭要素を比較する
    // 2. 小さい方を元のリストに戻す
    // 3. 1, 2を繰り返す

    // 7. left_index = 0 < 1, right_index = 0 < 1
    // 15. left_index = 0 < 1, right_index = 0 < 1

    println!("{:?}", arr);
    while left_index < left.len() && right_index < right.len() {
        // 8. 1 < 3
        if left[left_index] < right[right_index] {
            // 9. arr[0] = 1
            arr[arr_index] = left[left_index];
            // 10. left_index = 1
            left_index += 1;
        } else {
            // 16. 4 < 2
            // 17. arr[1] = 2
            arr[arr_index] = right[right_index];
            right_index += 1;
        }
        arr_index += 1;
    }

    // 11. left_index = 1 < 1
    while left_index < left.len() {
        arr[arr_index] = left[left_index];
        left_index += 1;
        arr_index += 1;
    }

    // 12. right_index = 0 < 1
    while right_index < right.len() {
        // 13. arr[1] = 3
        arr[arr_index] = right[right_index];
        right_index += 1;
        arr_index += 1;
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
