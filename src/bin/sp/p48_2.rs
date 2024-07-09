// Path: src/bin/sp/p48_2.rs
use std::{cmp, fmt::Debug};

fn find_max_gap(arr: &Vec<i32>) -> i32 {
    let mut min_value = arr[0];
    let mut max_gap = -200000;

    for i in 1..arr.len() {
        max_gap = cmp::max(max_gap, arr[i] - min_value);
        min_value = cmp::min(min_value, arr[i]);
    }

    println!("{}", max_gap);
    max_gap
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p48_2() {
        let input = vec![5, 3, 1, 3, 4, 3];
        assert_eq!(3, find_max_gap(&input));

        let input = vec![1, 2, 3, 4, 1, 6];
        assert_eq!(5, find_max_gap(&input));

        let input = vec![1, 1, 1, 1, 1, 1];
        assert_eq!(0, find_max_gap(&input));

        let input = vec![4, 3, 2];
        assert_eq!(-1, find_max_gap(&input));

        let input = vec![4, 3, 2, 1, 0, -1];
        assert_eq!(-1, find_max_gap(&input));
    }
}
