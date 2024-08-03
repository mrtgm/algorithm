use std::collections::HashSet;

fn find_articulation_points(n: usize, adj: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut visited = vec![false; n];
    let mut disc = vec![0; n];
    let mut low = vec![0; n];
    let mut parent = vec![None; n];
    let mut ap = vec![false; n];
    let mut time = 0;

    for i in 0..n {
        if !visited[i] {
            dfs(
                i,
                &adj,
                &mut visited,
                &mut disc,
                &mut low,
                &mut parent,
                &mut ap,
                &mut time,
            );
        }
    }

    ap.into_iter()
        .enumerate()
        .filter(|&(_, is_ap)| is_ap)
        .map(|(i, _)| i)
        .collect()
}

fn dfs(
    u: usize,
    adj: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    disc: &mut Vec<usize>,
    low: &mut Vec<usize>,
    parent: &mut Vec<Option<usize>>,
    ap: &mut Vec<bool>,
    time: &mut usize,
) {
    let mut children = 0;
    visited[u] = true;
    *time += 1;
    disc[u] = *time;
    low[u] = *time;

    for &v in &adj[u] {
        if !visited[v] {
            children += 1;
            parent[v] = Some(u);
            dfs(v, adj, visited, disc, low, parent, ap, time);

            low[u] = low[u].min(low[v]);

            if parent[u].is_none() && children > 1 {
                ap[u] = true;
            }

            if parent[u].is_some() && low[v] >= disc[u] {
                ap[u] = true;
            }
        } else if Some(v) != parent[u] {
            low[u] = low[u].min(disc[v]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_articulation() {
        let n = 5; // ノード数
        let adj = vec![
            vec![1, 2],    // ノード0の隣接リスト
            vec![0, 2],    // ノード1の隣接リスト
            vec![0, 1, 3], // ノード2の隣接リスト
            vec![2, 4],    // ノード3の隣接リスト
            vec![3],       // ノード4の隣接リスト
        ];

        let articulation_points = find_articulation_points(n, &adj);
        println!("Articulation points: {:?}", articulation_points);
    }
}
