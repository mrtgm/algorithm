// Path: src/bin/graph/bipartite.rs
use std::fmt::Debug;

struct Graph {
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new(s: usize) -> Self {
        Graph {
            edges: vec![vec![]; s],
        }
    }
    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges[from].push(to);
    }
}

// -1: 未訪問, 0: 色1, 1: 色2
fn is_bipartite_dfs(g: &Graph, vertex: usize, color: &mut Vec<i32>, current_color: i32) -> bool {
    color[vertex] = if current_color != 0 { current_color } else { 0 };

    for &next_vertex in &g.edges[vertex] {
        // 隣接頂点がすでに色確定していた場合
        if color[next_vertex] != -1 {
            // 同じ色が隣接したらダメ
            if color[next_vertex] == current_color {
                return false;
            }
            // してなければOK
            continue;
        }
        // 色味確定の場合、
        // 頂点の色を変えて再帰的に探索、一回でも false 返したら false
        if !is_bipartite_dfs(g, next_vertex, color, 1 - current_color) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bipartite() {
        let mut g = Graph::new(8);
        let field: Vec<Vec<usize>> = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        field.iter().enumerate().for_each(|(u, x)| {
            for i in 0..x.len() {
                g.add_edge(u, x[i]);
            }
        });

        let mut color = vec![-1; 8];
        let mut is_bipartite = true;

        for start in 0..7 {
            if color[start] == -1 {
                if !is_bipartite_dfs(&mut g, start, &mut color, 0) {
                    is_bipartite = false;
                }
            }
        }

        println!("{}", is_bipartite);
    }
}
