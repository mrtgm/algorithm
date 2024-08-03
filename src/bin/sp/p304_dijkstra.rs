// Path: src/bin/sp/p304.rs
use std::collections::BinaryHeap;
use std::fmt::Debug;

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

    fn dijkstra_pq(&self, start: usize) -> Vec<i32> {
        let mut dist = vec![i32::MAX; self.edges.len()]; // 始点からの距離
        let mut pq = BinaryHeap::new();

        dist[start] = 0;
        pq.push((0, start)); // (距離, 頂点)

        while let Some((d, u)) = pq.pop() {
            if dist[u] < d {
                // 最短距離が確定済みの場合はスキップ
                continue;
            }

            for v in 0..self.edges.len() {
                // 隣接行列の列を走査
                if self.edges[u][v] != 0 {
                    // 辺が存在する場合
                    if dist[v] > dist[u] + self.edges[u][v] {
                        // 確定済みの最短距離よりも短い場合は更新
                        dist[v] = dist[u] + self.edges[u][v];
                        pq.push((dist[v], v)); // (距離, 頂点)
                    }
                }
            }
        }

        dist
    }

    fn dijkstra_2(&self, start: usize) -> Vec<i32> {
        let mut dist = vec![i32::MAX; self.edges.len()]; // 始点からの距離
        let mut color = vec![-1; self.edges.len()];

        dist[start] = 0;

        loop {
            let mut min = i32::MAX;
            let mut min_index = 0;

            for v in 0..self.edges.len() {
                if color[v] == -1 && dist[v] <= min {
                    //未訪問で、最小の距離を持つノードを選択
                    min = dist[v];
                    min_index = v;
                }
            }

            if min == i32::MAX {
                break;
            }

            let u = min_index;
            color[u] = 1;

            for v in 0..self.edges.len() {
                // 未訪問の、当該頂点から辿れるノードに対して、最短距離を更新
                if color[v] == -1 && self.edges[u][v] != 0 {
                    dist[v] = dist[v].min(dist[u] + self.edges[u][v]); // 確定済みの当該頂点の距離と辺の重みを足して短い場合は、最短距離を更新
                }
            }
        }

        dist
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p304() {
        let mut g = Graph::new(5);

        g.init(vec![
            vec![0, 2, 0, 1, 0],
            vec![2, 0, 0, 4, 0],
            vec![3, 0, 0, 1, 1],
            vec![1, 4, 1, 0, 3],
            vec![0, 0, 1, 3, 0],
        ]);

        println!("{:?}", g.dijkstra_pq(0));

        assert_eq!(g.dijkstra_pq(0), vec![0, 2, 2, 1, 3]);
    }
}
