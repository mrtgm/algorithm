use proconio::input;
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        plans: [[i32;3];n]
    }

    for (i, plan) in plans.iter().enumerate() {
        let time = plan[0];
        let sum = plan[1] + plan[2];

        if (sum % 2) != (time % 2) || sum > time {
            println!("No");
            return;
        }

        if i > 0 {
            let diff_time = plan[0] - plans[i - 1][0];
            let diff_pos = (plan[1] - plans[i - 1][1]).abs() + (plan[2] - plans[i - 1][2]).abs();

            if diff_pos > diff_time {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ABC086C() {
        main();
    }
}
