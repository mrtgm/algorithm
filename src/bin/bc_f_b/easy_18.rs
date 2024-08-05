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
        s: Chars
    }

    let m = ['A', 'C', 'G', 'T'];
    let mut v = vec![0; s.len()];
    let mut count = 0;

    for i in 0..s.len() {
        if m.contains(&s[i]) {
            count += 1;
        } else {
            count = 0;
        }
        v[i] = count;
    }

    println!("{:?}", v.iter().max().unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_18() {
        main();
    }
}
