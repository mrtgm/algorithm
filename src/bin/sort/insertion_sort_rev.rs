use std::fmt::Debug;

pub fn insertion_sort<T: Copy + Debug + Ord>(arr: &mut [T]) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut a = [3, 4, 5, 6, 1, 2];
        insertion_sort(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5, 6]);
    }
}
