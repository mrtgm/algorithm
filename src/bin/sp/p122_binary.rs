// Path: src/bin/sp/p122_binary.rs
use std::fmt::Debug;

fn p122_binary<T: Copy + Debug + Ord>(input: &mut [T], target: &mut [T]) -> u64 {
    let mut count = 0;

    for i in 0..input.len() {
        let value = input[i];
        let mut left = 0;
        let mut right = target.len();
        while left <= right {
            let mid = (left + right) / 2;
            let target_value = target[mid];
            if target_value > value {
                right = mid - 1;
            } else if target_value < value {
                left = mid + 1;
            } else if target_value == value {
                count += 1;
                break;
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p122_binary() {
        let mut input = [1, 3];
        let mut target = [1, 2, 3, 4, 5, 6];

        assert_eq!(2, p122_binary(&mut input, &mut target));
    }
}
