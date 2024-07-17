// Path: src/bin/sp/p253_longest_common_subsequence.rs
use std::{cmp::max, fmt::Debug};

fn p253_longest_common_subsequence<T: Copy + Debug + Ord>(left: &mut [T], right: &mut [T]) -> i32 {
    let m = left.len();
    let n = right.len();

    let mut counter: Vec<Vec<i32>> = vec![vec![0; n]; m];
    let mut max_l = 0;

    for i in 1..m {
        for j in 1..n {
            if left[i] == right[j] {
                // 同じ場合はひとつ手前に一個足す
                counter[i][j] = counter[i - 1][j - 1] + 1;
            } else {
                // X_m-1 + Y_n か X_m + Y_n-1 の LCS 長い方を択ぶ
                counter[i][j] = max(counter[i - 1][j], counter[i][j - 1]);
            }

            max_l = max(counter[i][j], max_l);
        }
    }

    println!("{:?}", counter);

    max_l
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p253_longest_common_subsequence() {
        let mut left = [' ', 'a', 'b', 'c', 'd', 'a', 'b'];
        let mut right = [' ', 'b', 'd', 'c', 'a', 'b', 'a'];

        let res = p253_longest_common_subsequence(&mut left, &mut right);
        println!("{:?}", res);
    }
}
