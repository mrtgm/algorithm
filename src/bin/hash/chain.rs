struct ChainedHashTable<T: std::fmt::Debug + Hash + Eq> {
    table: Vec<Vec<T>>,
    size: usize,
}

trait USet<T> {
    fn new() -> Self;
    fn add(&mut self, element: T) -> bool;
    fn find(&self, element: &T) -> Option<&T>;
    fn remove(&mut self, element: &T) -> Option<T>;
}

impl<T: std::fmt::Debug + Hash + Eq> USet<T> for ChainedHashTable<T> {
    fn new() -> Self {
        let mut hash_table = Self {
            table: Vec::new(),
            size: 0,
        };
        // テーブルの初期化(ハッシュテーブルは2^D個のバケットを持つ)
        for _ in 0..(1 << (Self::D)) {
            hash_table.table.push(Vec::new());
        }
        hash_table
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
    fn find(&self, element: &T) -> Option<&T> {
        let hash = self.hash(element);
        let list = &self.table[hash as usize];
        for i in 0..list.len() {
            let node = list.get(i).unwrap();
            if *node == *element {
                return Some(node);
            }
        }
        None
    }
    fn remove(&mut self, element: &T) -> Option<T> {
        let hash = self.hash(element);
        let list = &mut self.table[hash as usize];
        for i in 0..list.len() {
            let node = list.get(i).unwrap();
            if *node == *element {
                self.size -= 1;
                return Some(list.remove(i));
            }
        }
        None
    }
}

impl<T: std::fmt::Debug + Hash + Eq> ChainedHashTable<T> {
    // zは{1, 3, ..., 2^W - 1}の奇数から選択した定数
    const Z: u64 = 4102541685;
    // Wはハッシュ値のビット数を表す(32ビット)
    const W: u64 = 32;
    // Dは2^D個のバケットを持つハッシュテーブルを表す(2^D = 8ビット)
    const D: u64 = 8;

    // 乗算ハッシュ法によるハッシュ値の計算
    // 計算式: (Z * hash_code) % 2^W >> (W - D)
    fn hash(&self, element: &T) -> u64 {
        // データのハッシュ値を計算
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        let hash_code = hasher.finish();

        let hash = Self::Z.wrapping_mul(hash_code);
        (hash % (1 << Self::W)) >> (Self::W - Self::D)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_linked_list() {}
}
