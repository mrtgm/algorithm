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
        a: u32,
        b: u32
    }

    let v = (a.to_string() + &b.to_string())
        .parse::<f32>()
        .unwrap()
        .sqrt();

    if v.fract() == 0.0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_8() {
        main();
    }
}
