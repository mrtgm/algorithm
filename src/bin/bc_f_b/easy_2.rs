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
        mut x: [i32;n]
    }

    let a = (0..=100)
        .map(|i| x.iter().map(|pos| (*pos - i).pow(2)).sum::<i32>())
        .min();

    println!("{}", a.unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_2() {
        main();
    }
}
