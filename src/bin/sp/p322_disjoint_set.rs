// Path: src/bin/sp/p304_disjoint_set.rs
use std::fmt::Debug;

struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(), // 初期値は自分自身
            rank: vec![0; size],         // 木の高さ、初期値は0
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            // 根の場合
            x
        } else {
            self.parent[x] = self.find(self.parent[x]); // 経路圧縮
            self.parent[x]
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);

        if x == y {
            return;
        }

        if self.rank[x] < self.rank[y] {
            // ランクの低い木をランクの高い木に結合
            // 親を高い木に変更
            self.parent[x] = y;
        } else {
            self.parent[y] = x;

            // ランクが同じ場合は結合後の木のランクを1つ上げる
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p322_disjoint_set() {
        let mut ds = DisjointSet::new(5);

        assert_eq!(ds.same(0, 1), false);
        ds.unite(0, 1);
        assert_eq!(ds.same(0, 1), true);
        ds.unite(1, 2);
        assert_eq!(ds.same(0, 2), true);
        ds.unite(3, 4);
        assert_eq!(ds.same(0, 4), false);
    }
}
