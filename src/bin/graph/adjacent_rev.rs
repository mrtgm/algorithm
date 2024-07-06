// 計算量はO(N+M)、頂点の数、辺の数

use std::fmt::Debug;

//隣接リスト表現
type Graph = Vec<Vec<usize>>; //頂点に対する隣接頂点

impl GraphExtension for Graph {
    fn new_graph(n: usize) -> Self {
        vec![Vec::new(); n]
    }
    fn push_edge(&mut self, from: usize, to: usize) {
        self[from].push(to);
    }
}

trait GraphExtension {
    fn new_graph(n: usize) -> Self;
    fn push_edge(&mut self, from: usize, to: usize);
}

fn dfs(g: &Graph, start: usize) -> Vec<usize> {
    let mut seen: Vec<usize> = vec![start];
    let mut todo: Vec<usize> = vec![];

    while seen.len() < g.len() {
        let last_seen = seen.last().unwrap().clone();
        let mut v = g[last_seen].to_vec();
        v.reverse();

        for i in 0..v.len() {
            todo.push(v[i]);
        }

        seen.push(todo.pop().unwrap());
    }

    seen
}

fn dfs2(
    g: &Graph,
    vertex: usize,
    seen: &mut Vec<bool>,
    last_order: &mut Vec<usize>,
    first_order: &mut Vec<usize>,
) {
    seen[vertex] = true; // すべての点について訪問済みにする O(N)
    first_order.push(vertex);

    // すべての辺を訪問する O(E)
    for &next_vertex in &g[vertex] {
        println!("★");
        if seen[next_vertex] {
            continue;
        }
        dfs2(g, next_vertex, seen, last_order, first_order);
    }

    last_order.push(vertex);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_adjacent_2() {
        let mut g: Graph = Graph::new_graph(9);
        g.push_edge(0, 1);
        g.push_edge(1, 2);
        g.push_edge(1, 3);
        g.push_edge(0, 4);
        g.push_edge(4, 5);
        g.push_edge(5, 6);
        g.push_edge(5, 7);
        g.push_edge(4, 8);

        for i in 0..g.len() {
            for j in g[i].iter() {
                println!("{} -> {}", i, j);
            }
        }

        let mut seen: Vec<bool> = vec![false; g.len()];
        let mut last_order: Vec<usize> = vec![];
        let mut first_order: Vec<usize> = vec![];
        dfs2(&mut g, 0, &mut seen, &mut last_order, &mut first_order);
        println!("{:?} {:?}", last_order, first_order);
    }
}
