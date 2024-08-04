use itertools::Itertools;
use proconio::input;
use std::fmt::Debug;

// if let Some(v) = s.match_indices(t).nth(0) {
//     if v.0 == 0 {
//         check(s.chars().skip(t.len()).collect(), ans, targets);
//     } else {
//         continue;
//     }
// }

fn check(s: &str, ans: &mut bool, targets: &[&str; 4]) {
    if s.len() == 0 {
        *ans = true;
        return;
    }

    for &t in targets {
        if s.ends_with(t) {
            check(&s[..(s.len() - t.len())], ans, targets);
        }
        continue;
    }
}

fn main() {
    input! {
        mut s: String
    }

    let targets = ["dream", "dreamer", "erase", "eraser"];
    let mut dp = vec![false; s.len() + 1];

    // check(&s, &mut ans, &targets);

    dp[0] = true;

    for index in 0..s.len() {
        if !dp[index] {
            continue;
        }
        for target in targets {
            let len = index + target.len();
            if s.len() >= len && &s[index..len] == target {
                dp[len] = true;
            }
        }
    }

    println!("{:?}", dp);

    if *dp.last().unwrap() {
        println!("YES")
    } else {
        println!("NO")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ABC049C() {
        main();
    }
}
