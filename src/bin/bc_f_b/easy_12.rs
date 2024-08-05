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
        (k,n): (u32, usize),
        mut a: [u32; n]
    }

    a.push(k + a[0]);
    let b = a.windows(2).map(|v| v[1] - v[0]).max().unwrap();
    println!("{}", k - b);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_12() {
        main();
    }
}
