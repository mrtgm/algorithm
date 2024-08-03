use proconio::input;

fn check(a: usize, b: usize, c: usize, x: i32, viewed: &mut Vec<Vec<Vec<bool>>>, count: &mut u32) {
    viewed[a][b][c] = true;

    if x < 0 {
        return;
    }
    if x == 0 {
        *count += 1;
    }

    if a > 0 && !viewed[a - 1][b][c] {
        check(a - 1, b, c, x - 500, viewed, count);
    }
    if b > 0 && !viewed[a][b - 1][c] {
        check(a, b - 1, c, x - 100, viewed, count);
    }
    if c > 0 && !viewed[a][b][c - 1] {
        check(a, b, c - 1, x - 50, viewed, count);
    }
}

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: i32,
    }

    let mut count = 0;
    let mut viewed = vec![vec![vec![false; c + 1]; b + 1]; a + 1];
    check(a, b, c, x, &mut viewed, &mut count);

    println!("{}", count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ABC087B() {
        main();
    }
}
