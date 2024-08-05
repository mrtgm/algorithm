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
        (n, mut x) : (usize, u32),
        mut a: [u32;n]
    }

    a.sort();

    let mut count = 0;

    for v in a {
        if x >= v {
            x -= v;
            count += 1;
        } else {
            println!("{}", count);
            return;
        }
    }

    println!("{}", n);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_22() {
        main();
    }
}
