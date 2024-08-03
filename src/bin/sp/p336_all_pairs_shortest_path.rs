const INF: i32 = std::i32::MAX / 2; // 無限大の定義（オーバーフローを防ぐために半分の値を使用）

// Path: src/bin/sp/p336_all_pairs_shortest_path.rs
use std::fmt::Debug;

fn initialize_matrix(n: usize) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![INF; n]; n];
    for i in 0..n {
        matrix[i][i] = 0; // 自分自身への距離は0
    }
    matrix
}

fn floyd_warshall(graph: &mut Vec<Vec<i32>>) -> Vec<Vec<Vec<usize>>> {
    let n = graph.len();
    let mut next = vec![vec![None; n]; n];

    // 初期化
    for i in 0..n {
        for j in 0..n {
            if graph[i][j] != INF && i != j {
                next[i][j] = Some(j);
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if graph[i][j] > graph[i][k] + graph[k][j] {
                    graph[i][j] = graph[i][k] + graph[k][j];
                    next[i][j] = next[i][k];
                }
            }
        }
    }

    next.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|col| match col {
                    Some(index) => vec![index],
                    None => vec![],
                })
                .collect()
        })
        .collect()
}

fn construct_path(next: &Vec<Vec<Vec<usize>>>, start: usize, end: usize) -> Vec<usize> {
    let mut path = vec![start];
    let mut current = start;

    while current != end {
        if next[current][end].is_empty() {
            return vec![]; // 経路が存在しない場合は空のベクタを返す
        }
        current = next[current][end][0];
        path.push(current);
    }

    path
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p336_all_pairs_shortest_path() {
        let mut graph = initialize_matrix(4);

        graph[0][1] = 3;
        graph[0][2] = 8;
        graph[1][2] = 2;
        graph[2][3] = 1;
        graph[1][3] = 7;

        let next = floyd_warshall(&mut graph);

        for i in 0..graph.len() {
            for j in 0..graph.len() {
                if i != j {
                    let path = construct_path(&next, i, j);
                    if !path.is_empty() {
                        println!("Shortest path from {} to {}: {:?}", i, j, path);
                    } else {
                        println!("No path from {} to {}", i, j);
                    }
                }
            }
        }
    }
}
