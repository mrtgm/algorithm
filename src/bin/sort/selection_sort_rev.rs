use std::fmt::Debug;

fn selection_sort_2<T: Debug + Copy + Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_index = i;

        for j in i..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(min_index, i);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_selection_sort_2() {
        let mut a = [8, 7, 6, 5, 4, 3, 2, 1];
        selection_sort_2(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
