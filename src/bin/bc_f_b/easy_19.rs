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
        (n,m,x): (usize,usize,u32),
        a: [u32;m]
    }

    let index = a.iter().position(|v| *v > x).unwrap_or(0);
    let ans = min(index, a.len() - index);
    println!("{}", ans);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_19() {
        main();
    }
}
