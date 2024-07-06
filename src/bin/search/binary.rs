use std::fmt::Debug;

fn binary<T: Debug + Copy + Ord>(arr: &[T], item: T) -> usize {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while high >= low {
        let mid = (high + low) / 2; // floor

        let key = arr[mid];

        if key == item {
            return mid;
        } else if key > item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_binary() {
        let a = [1, 2, 3, 4, 5, 6, 7];
        let i = binary(&a, 4);
        assert_eq!(3, i);
    }
}
