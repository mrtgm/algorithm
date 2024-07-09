// Path: src/bin/graph/topo.rs
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

fn topo(g: &Graph, size: usize) -> Vec<usize> {
    let mut order = Vec::with_capacity(0);

    fn dfs(g: &Graph, vertex: usize, parent: usize, order: &mut Vec<usize>) {
        for next in &g.edges[vertex] {
            if *next == parent {
                continue;
            }
            dfs(g, *next, vertex, order);
        }
        order.push(vertex);
    }

    dfs(g, 0, 0, &mut order);

    order.reserve(size);
    order
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_topo() {
        let mut g = Graph::new(6);
        let field: Vec<Vec<usize>> = vec![vec![1, 4], vec![2, 3], vec![], vec![], vec![5], vec![]];
        field.iter().enumerate().for_each(|(u, x)| {
            for i in 0..x.len() {
                g.add_edge(u, x[i]);
            }
        });

        let order = topo(&g, 6);
        println!("{:?}", order);
        assert_eq!(vec![2, 3, 1, 5, 4, 0], order);
    }
}
