use std::fmt::Debug;

fn linear<T: Debug + Ord>(arr: &[T], item: T) -> usize {
    for i in 0..arr.len() {
        if arr[i] == item {
            return i;
        }
    }
    return 0;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_linear() {
        let a = [1, 2, 3, 4, 5, 6];
        let i = linear(&a, 4);
        assert_eq!(3, i);
    }
}
