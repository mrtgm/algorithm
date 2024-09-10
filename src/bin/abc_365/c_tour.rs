#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut g: Vec<Vec<usize>> = vec![vec![]; n];

    for edge in edges {
        g[edge.0].push(edge.1);
    }

    fn dfs(v: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
        seen[v] = true;
        for &next_v in &g[v] {
            if seen[next_v] {
                continue;
            }
            dfs(next_v, g, seen);
        }
    }

    let mut ans: usize = 0;

    for i in 0..n {
        let mut seen = vec![false; n];
        dfs(i, &g, &mut seen);

        ans += seen.iter().filter(|v| **v).count();
    }

    println!("{:?}", ans);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn c_tour() {
        main();
    }
}
