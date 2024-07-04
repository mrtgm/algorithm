use std::{fmt::Debug, mem::swap};

pub fn quick_sort<T: Debug + Copy + Ord>(arr: &mut [T]) {
    let pivot_index = arr.len() / 2;
    let pivot = arr[pivot_index];

    let mut left_index = 0;
    let mut right_index = arr.len() - 1;

    while left_index <= right_index {
        // pivot と等しい、もしくは大きいものを探す
        while arr[left_index] < pivot {
            left_index += 1;
        }
        // pivot と等しい、もしくは小さいものを探す
        while arr[right_index] > pivot {
            right_index -= 1;
        }

        if left_index <= right_index {
            arr.swap(left_index, right_index);
            left_index += 1;
            right_index -= 1;
        }
    }

    if right_index > 0 {
        quick_sort(&mut arr[..=right_index]);
    }
    if left_index < arr.len() - 1 {
        quick_sort(&mut arr[left_index..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
