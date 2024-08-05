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
        (mut a,mut b,mut c): (usize,usize,usize)
    }

    let mut count = 0;

    if a == b && b == c && a % 2 == 0 {
        println!("-1");
        return;
    }

    while (a & 1) == 0 && (b & 1) == 0 && (c & 1) == 0 {
        let temp = (a >> 1, b >> 1, c >> 1);
        a = temp.1 + temp.2;
        b = temp.0 + temp.2;
        c = temp.0 + temp.1;
        count += 1;
    }

    println!("{:?}", count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_13() {
        main();
    }
}
