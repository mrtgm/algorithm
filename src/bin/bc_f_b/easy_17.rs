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
        n: usize,
        mut v: [f64; n]
    }

    v.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for i in 0..(n - 1) {
        v[i + 1] = (v[i] + v[i + 1]) / 2_f64;
    }

    println!("{:?}", v[n - 1]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_17() {
        main();
    }
}
