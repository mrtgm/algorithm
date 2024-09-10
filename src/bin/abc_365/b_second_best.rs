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
        mut a: [u32;n]
    }

    let a_sorted = a
        .iter()
        .enumerate()
        .sorted_by(|a, b| a.1.cmp(b.1))
        .collect_vec();

    println!("{:?}", a_sorted[n - 2].0 + 1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn b_second_best() {
        main();
    }
}
