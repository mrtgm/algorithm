use std::fmt::Debug;

fn insertion_sort_3<T: Debug + Copy + Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        for j in (0..i).rev() {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_insertion_sort_3() {
        let mut arr = [5, 4, 3, 2, 1];
        insertion_sort_3(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
