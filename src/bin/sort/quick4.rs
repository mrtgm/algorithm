// Path: src/bin/sort/quick4.rs
use std::fmt::Debug;

fn quick4<T: Copy + Debug + Ord>(arr: &mut [T]) {
    let pivot_index = arr.len() / 2;
    let pivot_val = arr[pivot_index];

    let mut left_index = 0;
    let mut right_index = arr.len() - 1;

    while left_index <= right_index {
        while arr[left_index] < pivot_val {
            left_index += 1;
        }

        while arr[right_index] > pivot_val {
            right_index -= 1;
        }

        if left_index <= right_index {
            arr.swap(left_index, right_index);
            left_index += 1;
            right_index -= 1;
        }
    }

    if right_index > 0 {
        quick4(&mut arr[..=right_index]);
    }

    if left_index > arr.len() - 1 {
        quick4(&mut arr[..left_index]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_quick4() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        quick4(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
