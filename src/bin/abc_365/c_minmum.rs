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
        (n,mut x,mut y): (usize,i64,i64),
         a: [i64;n],
         b: [i64;n]
    }

    let ab = a.into_iter().zip(b);

    let mut ans = n;

    for (i, (_a, _b)) in ab.enumerate() {
        x -= _a;
        y -= _b;

        if x < 0 || y < 0 {
            ans = i + 1;
            break;
        }
    }

    println!("{}", ans);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn c_minmum() {
        main();
    }
}
