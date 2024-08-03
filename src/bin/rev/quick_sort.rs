// Path: src/bin/rev/quick_sort.rs
use std::fmt::Debug;

fn quick_sort<T: Copy + Debug + Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let mut left = 0;
    let mut right = arr.len() - 1;
    let mid = (left + right) / 2;
    let key = arr[mid];

    while left <= right {
        while arr[left] < key {
            left += 1;
        }

        while arr[right] > key {
            right -= 1;
        }

        if left <= right {
            arr.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    if right > 0 {
        quick_sort(&mut arr[..=right]);
    }

    if left < arr.len() - 1 {
        quick_sort(&mut arr[left..]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
