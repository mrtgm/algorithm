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

fn main() {
    input! {
        a: [[u32;3];3],
        n: usize,
        b: [u32;n]
    }

    let hits = [
        [(0, 0), (1, 0), (2, 0)],
        [(0, 1), (1, 1), (2, 1)],
        [(0, 2), (1, 2), (2, 2)],
        [(0, 0), (0, 1), (0, 2)],
        [(1, 0), (1, 1), (1, 2)],
        [(2, 0), (2, 1), (2, 2)],
        [(0, 0), (1, 1), (2, 2)],
        [(0, 2), (1, 1), (2, 0)],
    ];

    for hit in hits {
        if hit.iter().all(|(i, j)| b.contains(&a[*i][*j])) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_7() {
        main();
    }
}
