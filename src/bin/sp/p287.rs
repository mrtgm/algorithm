// Path: src/bin/sp/p287.rs
// 色分けで連結成分を求める

use std::fmt::Debug;

use std::collections::VecDeque;

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

    fn bfs(&self, v: usize, color: i32, color_list: &mut Vec<i32>) {
        let mut queue = VecDeque::new();
        queue.push_back(v);
        color_list[v] = color;

        while !queue.is_empty() {
            let vertex = queue.pop_front().unwrap();
            for &next_vertex in &self.edges[vertex] {
                if color_list[next_vertex] == -1 {
                    color_list[next_vertex] = color;
                    queue.push_back(next_vertex);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p287() {
        let mut g = Graph::new(10);

        g.push_edge(0, 1);
        g.push_edge(0, 2);
        g.push_edge(3, 4);
        g.push_edge(5, 7);
        g.push_edge(5, 6);
        g.push_edge(6, 7);
        g.push_edge(6, 8);
        g.push_edge(7, 8);
        g.push_edge(8, 9);

        let mut color = -1;
        let mut color_list = vec![-1; 10];

        for i in 0..10 {
            if color_list[i] == -1 {
                println!("{:?}", color_list);
                color += 1;
                g.bfs(i, color, &mut color_list);
            }
        }

        println!("{:?}", color_list);
        println!("{:?}", color_list[0] == color_list[1]);
        println!("{:?}", color_list[5] == color_list[9]);
        println!("{:?}", color_list[1] == color_list[3]);
    }
}
