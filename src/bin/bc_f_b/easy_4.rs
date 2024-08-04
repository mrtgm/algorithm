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
        n: f32,
    }

    let a = n / 1.08;
    if (a.ceil() * 1.08).floor() == n {
        println!("{}", a.ceil());
    } else {
        println!(":(");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_4() {
        main();
    }
}
