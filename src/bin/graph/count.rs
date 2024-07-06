// Path: src/bin/graph/count.rs
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

//連結成分の個数を求める DFS
fn dfs(g: &Graph, vertex: usize, seen: &mut Vec<bool>) {
    seen[vertex] = true;

    for &next_vertex in &g.edges[vertex] {
        if seen[next_vertex] == true {
            continue;
        }
        dfs(g, next_vertex, seen);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count() {
        let field = vec![vec![1], vec![0, 2], vec![1], vec![4], vec![3, 5], vec![4]];
        let mut g = Graph::new(6);

        field.iter().enumerate().for_each(|(i, f)| {
            for j in 0..f.len() {
                g.add_edge(i, f[j]);

                println!("{}->{}", i, f[j]);
            }
        });

        let mut seen = vec![false; 6];
        let mut count = 0;

        for i in 0..6 {
            if seen[i] == true {
                continue;
            }
            dfs(&g, i, &mut seen);
            count += 1;
            println!("{:?}", seen);
        }
        assert_eq!(2, count);
    }
}
