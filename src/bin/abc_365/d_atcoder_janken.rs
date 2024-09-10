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
        s: Chars
    }

    let mut check = vec![false; n];

    for (index, cs) in s.windows(2).enumerate() {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d_atcoder_janken() {
        main();
    }
}
