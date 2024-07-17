// Path: src/bin/sp/p273_dfs.rs
use std::{
    collections::VecDeque,
    fmt::Debug,
    time::{SystemTime, UNIX_EPOCH},
};

struct Graph {
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Self {
            edges: vec![vec![]; size],
        }
    }

    fn push_edge(&mut self, v: usize, n: usize) {
        self.edges[v].push(n);
    }

    fn dfs(&self, v: usize, seen: &mut Vec<bool>, res: &mut Vec<(i32, i32)>, lapsed: &mut i32) {
        seen[v] = true;
        *lapsed += 1;
        res[v].0 = *lapsed;

        for next_v in &self.edges[v] {
            if seen[*next_v] {
                continue;
            }

            Self::dfs(&self, *next_v, seen, res, lapsed);
        }

        *lapsed += 1;
        res[v].1 = *lapsed;
    }

    fn bfs(&self, v: usize, d: &mut Vec<i32>) {
        let mut queue = VecDeque::new();
        queue.push_back(v);
        d[v] = 0;

        while !queue.is_empty() {
            let vertex = queue.pop_front().unwrap();

            for &next_vertex in &self.edges[vertex] {
                if d[next_vertex] != -1 {
                    continue;
                }

                queue.push_back(next_vertex);
                d[next_vertex] = d[vertex] + 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p273_dfs() {
        let mut g = Graph::new(6);

        g.push_edge(0, 1);
        g.push_edge(0, 2);
        g.push_edge(1, 2);
        g.push_edge(1, 3);
        g.push_edge(2, 4);
        g.push_edge(3, 5);
        g.push_edge(4, 5);

        let mut res = vec![(0, 0); 6];
        let mut seen = vec![false; 6];

        let mut lapsed = 0;

        g.dfs(0, &mut seen, &mut res, &mut lapsed);

        println!("{:?}", res);
        println!("{:?}", seen);

        res.iter().enumerate().for_each(|(i, (visited, finished))| {
            println!("{} {} {}", i + 1, visited, finished);
        });

        let mut g = Graph::new(4);

        g.push_edge(0, 1);
        g.push_edge(0, 3);
        g.push_edge(1, 3);
        g.push_edge(3, 2);

        let mut d = vec![-1; 4];
        g.bfs(0, &mut d);
        println!("{:?}", d);
    }
}
