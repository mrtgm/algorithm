// Path: src/bin/hash/chain2.rs
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

struct ChainedHash<T: Debug + Hash + Eq> {
    table: Vec<Vec<T>>,
    size: usize,
}

impl<T: Debug + Hash + Eq> ChainedHash<T> {
    // zは{1, 3, ..., 2^W - 1}の奇数から選択した定数
    const Z: u64 = 4102541685;
    // Wはハッシュ値のビット数を表す(32ビット)
    const W: u64 = 32;
    // Dは2^D個のバケットを持つハッシュテーブルを表す(2^D = 8ビット)
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

    fn hash(&self, element: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        let hash_code = hasher.finish();

        let hash = Self::Z.wrapping_mul(hash_code);
        (hash % (1 << Self::W)) >> (Self::W - Self::D)
    }

    fn add(&mut self, element: T) -> bool {
        if self.find(&element).is_some() {
            return false;
        }
        let hash = self.hash(&element);
        self.table[hash as usize].push(element);
        self.size += 1;
        true
    }

    fn find(&mut self, element: &T) -> Option<&T> {
        let hash = self.hash(&element);
        let list = &self.table[hash as usize];
        for i in 0..list.len() {
            if let Some(v) = list.get(i) {
                if v == element {
                    return Some(v);
                }
            }
        }
        None
    }

    fn remove(&mut self, element: &T) -> Option<T> {
        let hash = self.hash(&element);
        let list = &mut self.table[hash as usize];
        for i in 0..list.len() {
            if let Some(v) = list.get(i) {
                if v == element {
                    self.size -= 1;
                    return Some(list.remove(i));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chain2() {
        let a: i32 = 3;
        // let mut arr = [5, 2, 4, 6, 1, 3];
        // chain2(&mut arr);
        // assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
