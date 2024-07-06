use std::fmt::Debug;

fn selection_sort_3<T: Debug + Copy + Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i..arr.len() {
            if arr[min_index] > arr[j] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_selection_sort_3() {
        let mut arr = [5, 4, 3, 2, 1];
        selection_sort_3(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
