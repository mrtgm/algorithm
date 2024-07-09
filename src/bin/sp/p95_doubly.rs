// Path: src/bin/sp/p95_doubly.rs
use std::fmt::Debug;

fn p95_doubly<T: Copy + Debug + Ord>(arr: &mut [T]) {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p95_doubly() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        p95_doubly(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
