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
        mut d: [u32;n]
    }

    d.sort();

    let mid = n / 2;
    let ans = d[mid] - d[mid - 1];

    println!("{}", ans);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_15() {
        main();
    }
}
