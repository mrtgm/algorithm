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
     n: usize,
    mut s: [String;n]
         }

    let max_length = s.iter().map(|v| v.len()).max().unwrap();
    let max_index = s
        .iter()
        .enumerate()
        .find(|v| v.1.len() == max_length)
        .unwrap()
        .0;

    s = s
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, mut v)| {
            for j in 0..n {
                if s[j].len() > v.len() {
                    v.push_str(&"*");
                }
            }

            v
        })
        .collect_vec();

    let mut ans = vec!["".to_string(); max_length];

    for i in 0..n {
        let len = s[n - i - 1].len();

        for j in 0..len {
            let original = s[n - i - 1].chars().nth(j).unwrap();

            ans[j].push_str(&original.to_string());
        }
    }

    for v in ans {
        println!("{}", v);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bbbbb() {
        main();
    }
}
