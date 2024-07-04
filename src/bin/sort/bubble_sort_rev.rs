use std::{fmt::Debug, mem::swap};

fn bubble_sort<T: Debug + Ord + Copy>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 1..(arr.len() - i) {
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bubble_sort_rev2() {
        let mut a = [8, 7, 6, 5, 4, 3, 2, 1];
        bubble_sort(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
