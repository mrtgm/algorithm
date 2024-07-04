use std::fmt::Debug;

fn merge_sort<T: Debug + Ord + Copy>(arr: &mut [T]) {
    if arr.len() == 1 {
        return;
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);

    merge_sort(left);
    merge_sort(right);

    let left = left.to_owned();
    let right = right.to_owned();

    let mut arr_index = 0;
    let mut right_index = 0;
    let mut left_index = 0;

    // left と right が両方とも残ってるとき
    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            arr[arr_index] = left[left_index];
            left_index += 1;
        } else {
            arr[arr_index] = right[right_index];
            right_index += 1;
        }
        arr_index += 1;
    }

    // 片方しか残ってないとき
    while left_index < left.len() {
        arr[arr_index] = left[left_index];
        arr_index += 1;
        left_index += 1;
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
    fn test_merge_sort_rev2() {
        let mut arr = [8, 7, 6, 5, 4, 2, 3, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
