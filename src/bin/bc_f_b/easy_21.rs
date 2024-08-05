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
        x: usize
    }

    // fn is_prime(x: usize) -> bool {
    //     if x == 2 {
    //         return true;
    //     }
    //     if x < 2 || x % 2 == 0 {
    //         return false;
    //     }

    //     let mut i = 2;

    //     while i * i <= x {
    //         if x % i == 0 {
    //             return false;
    //         }
    //         i += 1;
    //     }
    //     true
    // }

    fn eratosthenes(n: usize) -> Vec<bool> {
        let mut is_prime = vec![true; n + 1];
        is_prime[0] = false;
        is_prime[1] = false;

        for p in 2..=n {
            if !is_prime[p] {
                continue;
            }

            let mut q = p * 2;
            while q <= n {
                is_prime[q] = false;
                q += p;
            }
        }
        return is_prime;
    }

    let primes = eratosthenes(100003);
    let ans = primes.iter().skip(x).position(|&x| x).map_or(x, |i| x + i);

    println!("{}", ans);

    // for i in x..=100003 {
    //     if is_prime(i) {
    //         println!("{:?}", i);
    //         break;
    //     }

    // let sq: usize = (i as f32).sqrt() as usize;
    // for j in 2..=sq {
    //     if i % j == 0 {
    //         break;
    //     }
    //     if j == sq {
    //         println!("{}", i);
    //         return;
    //     }
    // }
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn easy_21() {
        main();
    }
}
