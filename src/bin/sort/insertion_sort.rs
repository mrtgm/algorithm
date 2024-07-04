//先頭の要素を手前の要素と比較して適切な場所に挿入することを繰り返す、左脇から徐々に進んでく
// O(n^2)

use std::{cmp, fmt::Debug};

pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let val = arr[i];
        for j in (0..i).rev() {
            let comp = arr[j];
            if comp > val {
                arr[i - (i - (j + 1))] = comp;
                arr[j] = val;
            }
        }
    }
}

pub fn insertion_sort2<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn moveman(v: &[i8]) {
    println!("{:?}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        insertion_sort2(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
