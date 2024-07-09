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

// ある頂点の深さと部分木のサイズを求める
// 深さは再帰で求まる
// 部分木の大きさの取得は子供を巡回した後に分かることなので、帰りがけ（再帰の後ろ）で行う
fn dfs_tree(
    g: &Graph,
    vertex: usize,
    parent: isize,
    depth_arr: &mut Vec<usize>,
    depth: usize,
    subtree_arr: &mut Vec<usize>,
) {
    depth_arr[vertex] = depth;

    for next_vertex in &g.edges[vertex] {
        // すでに巡回済みとなるのが親の場合だけだけなので、このようにする
        if *next_vertex == parent as usize {
            continue;
        }
        dfs_tree(
            g,
            *next_vertex,
            vertex as isize,
            depth_arr,
            depth + 1,
            subtree_arr,
        );
    }

    subtree_arr[vertex] = 1;
    // subtree[next_vertex] にはすでに巡回済みのサブツリー情報が入っている
    for next_vertex in &g.edges[vertex] {
        if *next_vertex == parent as usize {
            continue;
        }
        subtree_arr[vertex] += subtree_arr[*next_vertex];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rootless() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(1, 4);

        let mut depth_arr = vec![0; 5];
        let mut subtree_arr = vec![0; 5];

        dfs_tree(&g, 0, -1, &mut depth_arr, 0, &mut subtree_arr);

        println!("{:?} {:?}", depth_arr, subtree_arr);
    }
}

//     0
//    / \
//   1   2
//  / \
// 3   4
