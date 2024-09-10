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
        a: [Usize1;n],
        b: [Usize1;n],
        c: [Usize1;n]
    }

    let mut count = vec![0; n];
    let mut ans: u64 = 0;

    for x in a {
        count[x] += 1;
    }

    for j in 0..n {
        ans += count[b[c[j]]];
    }

    println!("{}", ans);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn c_madeup() {
        main();
    }
}
