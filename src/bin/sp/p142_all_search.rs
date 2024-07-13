// Path: src/bin/sp/p142_all_search.rs
use std::fmt::Debug;

fn rec(i: usize, result: &mut [usize]) {
    if i == result.len() {
        println!("{:?}", result);
        return;
    }
    rec(i + 1, result);
    result[i] = 1;
    rec(i + 1, result);
    result[i] = 0;
}

fn solve(i: usize, m: isize, arr: &[isize]) -> bool {
    if m == 0 {
        return true;
    }
    if i >= arr.len() || m < 0 {
        return false;
    }
    solve(i + 1, m, arr) || solve(i + 1, m - arr[i], arr)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p142_all_search() {
        let mut arr = [5, 2, 4];

        let target = [11, 5, 3, 9];
        let mut res = [0; 4];
        rec(0, &mut res);

        // p142_all_search(&mut arr);

        for i in 0..target.len() {
            let res = solve(0, target[i], &mut arr);
            println!("{:?}", res);
        }

        // assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
