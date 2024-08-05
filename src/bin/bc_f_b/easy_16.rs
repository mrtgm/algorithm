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
        n: usize,
        a: [u32; n]
    }

    a.iter()
        .enumerate()
        .sorted_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .map(|(i, _)| i + 1)
        .for_each(|v| {
            print!("{} ", v);
        });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_16() {
        main();
    }
}
