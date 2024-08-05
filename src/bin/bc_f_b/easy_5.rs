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
        (n,m,c) : (usize,usize,i32),
        b: [i32;m],
        a: [[i32;m];n]
    }

    let ans = a
        .iter()
        .filter(|v| v.iter().enumerate().map(|(i, v)| v * b[i]).sum::<i32>() + c > 0)
        .count();

    println!("{}", ans);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_5() {
        main();
    }
}
