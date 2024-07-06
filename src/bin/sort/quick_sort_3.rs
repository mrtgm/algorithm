use std::fmt::Debug;

fn quick_sort_3<T: Debug + Ord + Copy>(arr: &mut [T]) {
    let pivot_index = arr.len() / 2;
    let pivot_value = arr[pivot_index];

    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        while arr[left] < pivot_value {
            left += 1;
        }

        while arr[right] > pivot_value {
            right -= 1;
        }

        if left <= right {
            arr.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    if right > 0 {
        quick_sort_3(&mut arr[..=right]);
    }

    if left < arr.len() - 1 {
        quick_sort_3(&mut arr[left..]);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_quick_sort_3() {
        let mut arr = [5, 6, 7, 8, 9, 4, 3, 2, 1];
        quick_sort_3(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
