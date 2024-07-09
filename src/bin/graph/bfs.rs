// Path: src/bin/graph/bfs.rs
use std::fmt::Debug;

struct Graph {
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new(u: usize) -> Self {
        Graph {
            edges: vec![vec![]; u],
        }
    }
    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges[from].push(to);
    }
}

fn bfs(g: &Graph, start: usize, dist: &mut Vec<i32>) {
    let mut queue = std::collections::VecDeque::new();

    // queue に最初の頂点を追加
    queue.push_back(start);
    // 訪問済みにする
    dist[start] = 0;

    // queue が空になるまで繰り返す
    while !queue.is_empty() {
        // queue から頂点を取り出す
        let vertex = queue.pop_front().unwrap();

        // 隣接する頂点を queue に追加
        for &next_vertex in &g.edges[vertex] {
            // すでに訪問済みの場合はスキップ
            if dist[next_vertex] != -1 {
                continue;
            }

            queue.push_back(next_vertex);
            // 訪問済みにする
            dist[next_vertex] = dist[vertex] + 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bfs() {
        let mut g = Graph::new(6);
        let field: Vec<Vec<usize>> = vec![vec![1, 4], vec![2, 3], vec![], vec![], vec![5], vec![]];
        field.iter().enumerate().for_each(|(u, x)| {
            for i in 0..x.len() {
                g.add_edge(u, x[i]);
            }
        });

        let mut dist: Vec<i32> = vec![-1; g.edges.len()];

        bfs(&g, 0, &mut dist);

        println!("{:?}", dist);
    }
}
