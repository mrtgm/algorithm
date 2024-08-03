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
        n: u32,
        a: [u32;n]
    }

    let ans =
        a.into_iter()
            .sorted_by_key(|v| *v)
            .rev()
            .enumerate()
            .fold((0, 0), |mut cur, (i, x)| {
                if i % 2 == 0 {
                    cur.0 += x;
                } else {
                    cur.1 += x;
                }
                cur
            });

    println!("{:?}", ans.0 - ans.1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ABC088B() {
        main();
    }
}
