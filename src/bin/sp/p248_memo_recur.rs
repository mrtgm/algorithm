// Path: src/bin/sp/p248_memo_recur.rs
use std::fmt::Debug;

fn p248_memo_recur<T: Copy + Debug + Ord>(arr: &mut [T]) {}

fn fibo(n: usize, memo: &mut Vec<usize>) -> usize {
    if n == 0 || n == 1 {
        memo[n] = 1;
        return 1;
    }

    if memo[n] > 0 {
        return memo[n];
    }

    memo[n] = fibo(n - 2, memo) + fibo(n - 1, memo);
    memo[n]
}

fn fibo_v2(n: usize, memo: &mut Vec<usize>) {
    memo[0] = 1;
    memo[1] = 1;

    for i in 2..=n {
        memo[i] = memo[i - 2] + memo[i - 1];
    }
}

fn fibo_no_memo(n: usize) -> usize {
    if n == 0 || n == 1 {
        return 1;
    }

    fibo_no_memo(n - 2) + fibo_no_memo(n - 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p248_memo_recur() {
        let mut memo = vec![0; 10];
        fibo_v2(8, &mut memo);

        println!("{:?}", memo);
    }
}
