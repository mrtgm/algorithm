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
        (n, a, b): (usize,usize,usize),
        cs: Chars,
    }

    let mut count = a + b;
    let mut foregin_count = b;

    for c in cs {
        match c {
            'a' => {
                if count == 0 {
                    println!("No");
                } else {
                    println!("Yes");
                    count -= 1;
                }
            }
            'b' => {
                if count == 0 || foregin_count == 0 {
                    println!("No")
                } else {
                    println!("Yes");
                    foregin_count -= 1;
                    count -= 1;
                }
            }
            _ => {
                println!("No")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_3() {
        main();
    }
}
