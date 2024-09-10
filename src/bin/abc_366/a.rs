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
    (n,t,a):(u32,u32,u32),
        }

    if t == a {
        println!("No");
        return;
    }

    let diff = n - t - a;

    if t > a {
        if (a + diff) >= t {
            println!("No");
            return;
        } else {
            println!("Yes");
            return;
        }
    } else {
        if (t + diff) >= a {
            println!("No");
            return;
        } else {
            println!("Yes");
            return;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d() {
        main();
    }
}
