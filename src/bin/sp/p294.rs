// Path: src/bin/sp/p294.rs
use std::fmt::Debug;

fn p294<T: Copy + Debug + Ord>(arr: &mut [T]) {
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p294() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        p294(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
