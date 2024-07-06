// Path: src/bin/graph/aja3.rs
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

fn dfs(
    g: &Graph,
    vertex: usize,
    seen: &mut Vec<bool>,
    last_order: &mut Vec<usize>,
    first_order: &mut Vec<usize>,
) {
    seen[vertex] = true; // すべての点について訪問済みにする O(N)
    first_order.push(vertex);

    // すべての辺を訪問する O(E)
    for &next_vertex in &g.edges[vertex] {
        println!("★");
        if seen[next_vertex] {
            continue;
        }
        dfs(g, next_vertex, seen, last_order, first_order);
    }

    last_order.push(vertex);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_aja3() {
        let mut graph = Graph::new(9);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(4, 5);
        graph.add_edge(4, 8);
        graph.add_edge(5, 6);
        graph.add_edge(5, 7);

        let mut seen: Vec<bool> = vec![false; graph.edges.len()];
        let mut last_order: Vec<usize> = vec![];
        let mut first_order: Vec<usize> = vec![];

        dfs(&mut graph, 4, &mut seen, &mut last_order, &mut first_order);

        println!("{:?}", seen[6]);

        println!("{:?} {:?}", last_order, first_order);
    }
}
