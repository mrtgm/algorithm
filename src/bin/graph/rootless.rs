// Path: src/bin/graph/rootless.rs
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rootless() {
        let g = Graph::new(8);
    }
}

//     0
//    / \
//   1   2
//  / \
// 3   4
