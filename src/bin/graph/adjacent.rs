use std::fmt::Debug;

struct Graph {
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            edges: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges[from].push(to);
    }
}

fn topo_sort(g: &Graph) -> Vec<usize> {
    let n = g.edges.len();
    let mut visited = vec![false; n];
    let mut order = vec![];

    fn dfs(g: &Graph, v: usize, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
        visited[v] = true;
        for &u in &g.edges[v] {
            if !visited[u] {
                dfs(g, u, visited, order);
            }
        }
        order.push(v);
    }

    for i in 0..n {
        if !visited[i] {
            dfs(g, i, &mut visited, &mut order);
        }
    }

    order.reverse();
    order
}

fn graph_visualize(g: &Graph) {
    for i in 0..g.edges.len() {
        for &j in &g.edges[i] {
            println!("{} -> {}", i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent_list() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        graph_visualize(&graph);

        let v = topo_sort(&graph);
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }
}
