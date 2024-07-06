// Path: src/bin/hash/chain2.rs
use std::{fmt::Debug, hash::Hash};

struct ChainedHash<T: Debug + Hash + Eq> {
    table: Vec<Vec<T>>,
    size: usize,
}

impl<T: Debug + Hash + Eq> ChainedHash<T> {
    const D: u64 = 8;

    fn new() -> Self {
        let mut hash_table = Self {
            table: Vec::new(),
            size: 0,
        };

        for _ in 0..(1 << (Self::D)) {
            //8桁ぶん左シフトする * 256
            hash_table.table.push(Vec::new());
        }
        hash_table
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chain2() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        chain2(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
