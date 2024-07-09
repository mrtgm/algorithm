// Path: src/bin/graph/cicle_detec.rs
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

fn dfs(g: &Graph, vertex: usize, parent: usize, seen: &mut Vec<bool>, finished: &mut Vec<bool>) {
    seen[vertex] = true;

    for &next_vertex in &g.edges[vertex] {
        if next_vertex == parent {
            continue;
        }

        if seen[next_vertex] {
            println!("already seen: {}", next_vertex);

            if !finished[next_vertex] {
                println!("cicled: {}", next_vertex);
            }

            continue;
        }

        dfs(g, next_vertex, vertex, seen, finished);
    }

    finished[vertex] = true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cicle_detec() {
        let field: Vec<Vec<usize>> =
            vec![vec![1], vec![0, 2], vec![1], vec![4], vec![3, 5], vec![4]];
        let mut g = Graph::new(6);

        field.iter().enumerate().for_each(|(i, f)| {
            for j in 0..f.len() {
                g.add_edge(i, f[j]);

                println!("{}->{}", i, f[j]);
            }
        });

        let mut seen = vec![false; 6];
        let mut finished = vec![false; 6];

        for i in 0..6 {
            dfs(&g, i, i, &mut seen, &mut finished);
            println!("{:?}", seen);
        }

        let cicled_field: Vec<Vec<usize>> =
            vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![0]];

        let mut cicled_g = Graph::new(6);

        cicled_field.iter().enumerate().for_each(|(i, f)| {
            for j in 0..f.len() {
                cicled_g.add_edge(i, f[j]);
                println!("{}->{}", i, f[j]);
            }
        });

        let mut seen = vec![false; 6];
        let mut finished = vec![false; 6];

        for i in 0..6 {
            dfs(&cicled_g, i, i, &mut seen, &mut finished);
        }
    }
}
