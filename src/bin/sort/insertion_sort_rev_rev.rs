use std::fmt::Debug;

fn insertion_sort_rev_rev<T: Debug + Copy + Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_insertion_sort_rev_rev() {
        let mut a = [3, 4, 5, 6, 1, 2];
        insertion_sort_rev_rev(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5, 6]);
    }
}
