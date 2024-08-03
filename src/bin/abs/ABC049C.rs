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
    let mut ans = false;

    check(&s, &mut ans, &targets);

    if ans {
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
