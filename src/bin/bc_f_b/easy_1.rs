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
        a: u32,
        b: u32,
    }

    println!("{}", (a + b - 3) / (a - 1));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_1() {
        loop {
            main();
        }
    }
}
