// Path: src/bin/graph/grid.rs
use std::fmt::Debug;

// s.........
// #########.
// #.......#.
// #..####.#.
// ##....#.#.
// #####.#.#.
// g.#.#.#.#.
// #.#.#.#.#.
// #.#.#.#.#.
// #.....#...

struct Graph {
    edges: Vec<Vec<char>>,
}

impl Graph {
    fn new(usize: usize) -> Self {
        Graph {
            edges: vec![vec![]; usize],
        }
    }

    fn add_edge(&mut self, column: usize, data: char) {
        self.edges[column].push(data);
    }
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const FIELDS: [&str; 10] = [
    "s.........",
    "#########.",
    "#.......#.",
    "#..####.#.",
    "##....#.#.",
    "#####.#.#.",
    "g.#.#.#.#.",
    "#.#.#.#.#.",
    "#.#.#.#.#.",
    "#.....#...",
];

fn dfs(g: &Graph, h: usize, w: usize, seen: &mut Vec<Vec<bool>>) {
    seen[h][w] = true;
    for (dx, dy) in DIRECTIONS.iter() {
        let nh = h as i32 + dx;
        let nw = w as i32 + dy;

        if nh < 0 || nh >= 10 || nw < 0 || nw >= 10 {
            continue;
        }
        if FIELDS[nh as usize].chars().nth(nw as usize).unwrap() == '#' {
            continue;
        }
        if seen[nh as usize][nw as usize] {
            continue;
        }
        dfs(g, nh as usize, nw as usize, seen);
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_grid() {
        let mut g = Graph::new(10);
        FIELDS.iter().enumerate().for_each(|(i, &x)| {
            for c in x.chars() {
                g.add_edge(i, c);
            }
        });

        let h = 10;
        let w = 10;
        let mut seen = vec![vec![false; w]; h];

        dfs(&g, 0, 0, &mut seen);

        println!("{:?}", seen);

        for i in 0..h {
            for j in 0..w {
                if FIELDS[i].chars().nth(j).unwrap() == 'g' {
                    println!("{:?}: g", seen[i][j]);
                    assert_eq!(seen[i][j], true);
                }
            }
        }
    }
}
