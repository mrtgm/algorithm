// 先頭から辿り、次の要素より大きかったらその要素を次の要素と交換するのを繰り返し、一番後ろに最大の要素を寄せてく　探索範囲は末尾から狭める
// O(n^2)

use std::fmt::Debug;

fn bubble_sort<T: Debug + Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        for j in 0..(arr.len() - i) {
            print!("{}", j);
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
        println!("")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
