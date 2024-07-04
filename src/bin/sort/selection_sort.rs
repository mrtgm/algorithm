//配列の中から「残っているもののうち一番小さいものを前に持って来る」を繰り返す、配列を左脇から狭める
// O(n^2)

use std::fmt::Debug;

fn selection_sort<T: Copy + Debug + Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i..arr.len() {
            if (arr[min_index] > arr[j]) {
                min_index = j;
            }
        }
        arr.swap(i, min_index)
    }
}
//range は半開領域だから、末尾は含まんのや

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
