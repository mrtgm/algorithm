// Path: src/bin/sp/p48.rs
use std::{env, fmt::Debug};
use std::{io, io::prelude::*};

fn p48<T: Copy + Debug + Ord>(arr: &mut [T]) {}

// 6
//

fn read_vec() -> Vec<i32> {
    let mut itertor = std::io::stdin().lock().lines();

    let length = match itertor.next() {
        Some(Ok(line)) => line.trim().parse::<usize>().unwrap_or(0),
        _ => return Vec::new(),
    };
    let mut vec = Vec::with_capacity(length);
    for _ in 0..length {
        if let Some(Ok(line)) = itertor.next() {
            if let Ok(value) = line.trim().parse::<i32>() {
                vec.push(value);
            }
        }
    }

    vec
}

fn find_max_gap(arr: &Vec<i32>) -> i32 {
    let mut min_index = 0;
    // O(n)
    for i in 0..(arr.len() - 1) {
        if arr[i] < arr[min_index] {
            min_index = i;
        }
    }

    let min_value = arr[min_index];
    // O(n)
    let mut max_value = arr[min_index + 1];
    for j in (min_index + 1)..arr.len() {
        if arr[j] > max_value {
            max_value = arr[j];
        }
    }

    max_value - min_value
}

use std::cmp::max;
use std::cmp::min;

fn find_max_gap_2(arr: &Vec<i32>) -> i32 {
    let mut min_value = arr[0];
    let mut max_gap = -200000000;
    // O(n)
    for i in 1..arr.len() {
        max_gap = max(max_gap, arr[i] - min_value);
        min_value = min(min_value, arr[i]);
    }

    max_gap
}

fn main() {
    let input = read_vec();
    println!("{}", find_max_gap_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p48() {
        let input = vec![5, 3, 1, 3, 4, 3];
        assert_eq!(3, find_max_gap(&input));
        println!("{}", find_max_gap_2(&input));

        let input = vec![1, 2, 3, 4, 1, 6];
        assert_eq!(5, find_max_gap(&input));
        println!("{}", find_max_gap_2(&input));

        let input = vec![1, 1, 1, 1, 1, 1];
        assert_eq!(0, find_max_gap(&input));
        println!("{}", find_max_gap_2(&input));

        let input = vec![4, 3, 2];
        assert_eq!(-1, find_max_gap(&input));
        println!("{}", find_max_gap_2(&input));

        let input = vec![4, 3, 2, 1, 0, -1];
        assert_eq!(-1, find_max_gap_2(&input));
        println!("{}", find_max_gap_2(&input));
    }
}
