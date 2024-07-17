// Path: src/bin/sp/p269_graph.rs
use std::fmt::Debug;

struct Graph {
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Self {
            edges: vec![vec![0; size]; size],
        }
    }

    fn add_edge(&mut self, v: usize, n: usize) {
        self.edges[v][n] = 1;
    }

    fn print(&self) {
        for edge in &self.edges {
            println!("{:?}", edge);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p269_graph() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 3);
        g.add_edge(1, 3);
        g.add_edge(3, 2);

        g.print();
    }
}
