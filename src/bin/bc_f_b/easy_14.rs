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
        mut n: i64,
        k: i64,
    }

    // n -= (n / k) * k;
    let ans = min(n % k, k - n % k);
    println!("{}", ans);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_14() {
        main();
    }
}
