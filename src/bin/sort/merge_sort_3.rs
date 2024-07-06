use std::fmt::Debug;

fn merge_sort_3<T: Debug + Copy + Ord>(arr: &mut [T]) {
    if arr.len() == 1 {
        return;
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);

    merge_sort_3(left);
    merge_sort_3(right);

    let left = left.to_vec();
    let right = right.to_vec();

    let mut arr_index = 0;
    let mut left_index = 0;
    let mut right_index = 0;

    while left.len() > left_index && right.len() > right_index {
        if left[left_index] > right[right_index] {
            // ここ、おっきい方入れるようにしたら逆順になるんやな
            arr[arr_index] = right[right_index];
            right_index += 1;
        } else {
            arr[arr_index] = left[left_index];
            left_index += 1;
        }
        arr_index += 1;
    }

    while left.len() > left_index {
        arr[arr_index] = left[left_index];
        left_index += 1;
        arr_index += 1;
    }

    while right.len() > right_index {
        arr[arr_index] = right[right_index];
        right_index += 1;
        arr_index += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_merge_sort_3() {
        let mut arr = [4, 5, 3, 2, 1];
        merge_sort_3(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
