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
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input! {
        y: u32,
    }

    let mut ans = 365;

    // 4 の倍数でない
    if y % 4 > 0 {
        ans = 365;
    }
    // 4 の倍数で、100 の倍数ではない
    if y % 4 == 0 && y % 100 > 0 {
        ans = 366;
    }
    // 100 の倍数で、400 の倍数ではない
    if y % 100 == 0 && y % 400 > 0 {
        ans = 365;
    }
    // 400 の倍数である
    if y % 400 == 0 {
        ans = 366;
    }

    println!("{}", ans);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_leap_year() {
        main();
    }
}
