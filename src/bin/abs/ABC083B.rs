use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32
    }

    // let mut res = 0;

    // for i in 1..=n {
    //     let digit_sum: u32 = i.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum();
    //     if digit_sum >= a && digit_sum <= b {
    //         res += i;
    //     }
    // }

    let res = (1..=n)
        .map(|n| {
            (
                n,
                n.to_string()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .sum::<u32>(),
            )
        })
        .filter(|(_, sum)| *sum >= a && *sum <= b)
        .map(|(num, _)| num)
        .sum::<u32>();

    println!("{}", res)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ABC083B() {
        main();
    }
}
