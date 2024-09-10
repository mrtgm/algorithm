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
        (h,w,n): (usize, usize, usize),
        ab: [(usize, usize);n]
    }

    let (mut al, mut bl): (Vec<_>, Vec<_>) = ab.iter().cloned().unzip();

    al.sort();
    al.dedup();

    bl.sort();
    bl.dedup();

    let mut a = al
        .iter()
        .enumerate()
        .map(|(i, v)| (v, i)) // (2, 0), (3, 1)
        .collect::<HashMap<_, _>>();

    let mut b = bl
        .iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<HashMap<_, _>>();

    ab.iter()
        .map(|&v| (a[&v.0] + 1, b[&v.1] + 1))
        .for_each(|v| println!("{} {}", v.0, v.1))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn c_reorder() {
        main();
    }
}
