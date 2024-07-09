// Path: src/bin/sp/p55_insertion.rs
use std::{fmt::Debug, io};

fn p55_insertion<T: Copy + Debug + Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        println!("{:?}", arr);

        for j in (0..i).rev() {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    println!("{:?}", arr);
}

fn read_vec() -> Vec<i32> {
    let mut iter = io::stdin().lines();
    let mut vec = Vec::with_capacity(0);

    let length = iter.next().unwrap().unwrap().parse::<i32>().unwrap_or(0);

    while let Ok(second_line) = iter.next().unwrap() {
        let split_inputs = second_line.split_whitespace();
        for input in split_inputs {
            vec.push(input.parse::<i32>().unwrap_or(0));
        }
        if vec.len() == length as usize {
            break;
        } else {
            panic!("err");
        }
    }
    vec
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p55_insertion() {
        // let input = read_vec();
        // println!("{:?}", input);

        let mut arr = [5, 2, 4, 6, 1, 3];
        p55_insertion(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);

        let mut arr = [31, 41, 59, 26, 41, 58];
        p55_insertion(&mut arr);
        assert_eq!(arr, [26, 31, 41, 41, 58, 59]);

        println!("{:?}", arr);
    }
}
