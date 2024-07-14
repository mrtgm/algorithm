// Path: src/bin/sp/p176_revese.rs
use std::fmt::Debug;

// 反転数

fn p176_revese<T: Copy + Debug + Ord>(arr: &mut [T]) -> i32 {
    let mut count: i32 = 0;

    if arr.len() == 1 {
        return 0;
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);

    let inversion_first = p176_revese(left);
    let inversion_second = p176_revese(right);

    let left = left.to_vec();
    let right = right.to_vec();

    let mut arr_index = 0;
    let mut right_index = 0;
    let mut left_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            arr[arr_index] = left[left_index];
            left_index += 1;
        } else {
            arr[arr_index] = right[right_index];
            right_index += 1;
            count += (left.len() - left_index) as i32;
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

    inversion_first + inversion_second + count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p176_revese() {
        let mut arr = [5, 3, 6, 2, 1, 4];
        let inversion_count = p176_revese(&mut arr);
        println!("{:?}", inversion_count);
    }
}
