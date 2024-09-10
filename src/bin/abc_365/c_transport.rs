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
        (n,m): (u64,u64),
        mut a:[u64;n]
    }

    let sum = a.iter().sum::<u64>();
    let mut ans = 0;

    if m >= sum {
        println!("infinite");
        return;
    }

    let mn = m / n;

    for i in mn..=m {
        let sum = a.iter().map(|v| min(*v, i)).sum::<u64>();

        if sum > m {
            break;
        }

        ans = i;
    }

    println!("{}", ans);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn c_transport() {
        main();
    }
}
