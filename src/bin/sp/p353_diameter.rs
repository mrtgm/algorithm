use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    weight: i32,
}

fn bfs(start: usize, adj: &Vec<Vec<Edge>>) -> (usize, i32) {
    let n = adj.len();
    let mut dist = vec![-1; n];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    dist[start] = 0;

    let mut farthest_node = start;
    let mut max_distance = 0;

    while let Some(node) = queue.pop_front() {
        for edge in &adj[node] {
            if dist[edge.to] == -1 {
                dist[edge.to] = dist[node] + edge.weight;
                queue.push_back(edge.to);

                if dist[edge.to] > max_distance {
                    max_distance = dist[edge.to];
                    farthest_node = edge.to;
                }
            }
        }
    }

    (farthest_node, max_distance)
}

fn tree_diameter(adj: &Vec<Vec<Edge>>) -> i32 {
    let (farthest_from_start, _) = bfs(0, adj);
    let (_, diameter) = bfs(farthest_from_start, adj);
    diameter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_articulation() {
        let n = 5;
        let mut adj = vec![vec![]; n];

        adj[0].push(Edge { to: 1, weight: 3 });
        adj[1].push(Edge { to: 0, weight: 3 });

        adj[1].push(Edge { to: 2, weight: 2 });
        adj[2].push(Edge { to: 1, weight: 2 });

        adj[2].push(Edge { to: 3, weight: 4 });
        adj[3].push(Edge { to: 2, weight: 4 });

        adj[3].push(Edge { to: 4, weight: 6 });
        adj[4].push(Edge { to: 3, weight: 6 });

        let diameter = tree_diameter(&adj);
        println!("Tree diameter: {}", diameter);
    }
}
