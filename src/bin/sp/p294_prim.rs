// Path: src/bin/sp/p294.rs
use std::fmt::Debug;

// 最小全域木の重みの総和を求める
// 隣接行列

struct Graph {
    edges: Vec<Vec<i32>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Self {
            edges: vec![vec![0; size]; size],
        }
    }

    fn init(&mut self, input: Vec<Vec<i32>>) {
        self.edges = input;
    }

    fn push_edge(&mut self, v: usize, n: usize, w: i32) {
        self.edges[v][n] = w;
    }

    fn prim(&self) -> i32 {
        let mut color = vec![-1; self.edges.len()]; // -1: WHITE, 0: GRAY, 1: BLACK
        let mut d = vec![std::i32::MAX; self.edges.len()]; //重みを記録する
        let mut p: Vec<i32> = vec![-1; self.edges.len()];

        d[0] = 0; //

        loop {
            let mut u: i32 = -1;
            let mut mincost = std::i32::MAX;

            for i in 0..self.edges.len() {
                if color[i] != 1 && d[i] < mincost {
                    //
                    mincost = d[i];
                    u = i as i32;
                }
            }

            if u == -1 {
                break;
            }

            color[u as usize] = 1;

            for v in 0..self.edges.len() {
                if color[v] != 1    //　訪問済みの場合はスキップ
                    && self.edges[u as usize][v] != -1
                // 対象の頂点と辺がない場合はスキップ
                && self.edges[u as usize][v] < d[v]
                //過去に保持した重みよりも小さい場合は更新
                {
                    println!("{} {}", u, v);
                    d[v] = self.edges[u as usize][v]; //頂点 u から辿れる各頂点 v のコストをそれぞれ記録
                    p[v] = u; //頂点 u を頂点 v の親として記録
                    color[v] = 0; //GRAY でマーク
                }
            }
        }

        let mut res = 0;

        println!("{:?}", p);

        for i in 0..self.edges.len() {
            if p[i] != -1 {
                res += self.edges[i][p[i] as usize];
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p294() {
        let mut g = Graph::new(5);
        g.init(vec![
            vec![-1, 2, 3, 1, -1],
            vec![2, -1, -1, 4, -1],
            vec![3, -1, -1, 1, 1],
            vec![1, 4, 1, -1, 3],
            vec![-1, -1, 1, 3, -1],
        ]);

        println!("{}", g.prim());
    }
}
