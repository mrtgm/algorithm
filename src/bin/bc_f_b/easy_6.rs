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
        h: usize,
        w: usize
    }

    if h == 1 || w == 1 {
        println!("1");
        return;
    }

    println!("{}", ((h * w) as f64 / 2 as f64).ceil());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_6() {
        main();
    }
}
