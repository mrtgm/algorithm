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

    // let a = d.into_iter().unique().collect::<Vec<u32>>().len();
    d.sort();
    d.dedup();

    println!("{:?}", d.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ABC085B() {
        main();
    }
}
