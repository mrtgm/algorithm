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
        n: i32
    }

    let ans = (1..=n)
        .map(|v| format!("{:b}", v & -v)[1..].len() as u32)
        .max()
        .unwrap();

    println!("{}", 2_i32.pow(ans));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_11() {
        main();
    }
}
