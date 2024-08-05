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
        mut s: usize
    }

    let mut array = vec![s];

    loop {
        if (s & 1) == 0 {
            s = s / 2;
        } else {
            s = s * 3 + 1;
        }

        if array.iter().any(|v| *v == s) {
            array.push(s);
            break;
        }
        array.push(s);
    }

    println!("{:?}", array.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_20() {
        main();
    }
}
