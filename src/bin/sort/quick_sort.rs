// https://www.seplus.jp/dokushuzemi/ec/fe/fenavi/similar_programming/quick_sort/
// 乱択アルゴリズム
use std::fmt::Debug;

fn quick_sort<T: Copy + Debug + Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = arr.len() / 2;
    let pivot_value = arr[pivot];
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        // pivot より大きい値を左から探す
        while arr[left] < pivot_value {
            left += 1;
        }
        // pivot より小さい値を右から探す
        while arr[right] > pivot_value {
            right -= 1;
        }
        // 左右が逆転していない場合は交換、それを繰り返す
        if left <= right {
            arr.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    println!("a:{:?} l:{:?} r:{:?} p:{:?}", arr, left, right, pivot);

    if 0 < right {
        quick_sort(&mut arr[0..=right]); // 0..=right は 0 から right までの範囲
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
        let mut arr = [7, 8, 1, 3, 2, 4, 6, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
