// Path: src/bin/sp/p119_linear_search.rs
use std::fmt::Debug;

fn p119_linear_search<T: Copy + Debug + Ord>(input: &mut [T], target: &mut [T]) -> u64 {
    let mut count = 0;
    for i in 0..input.len() {
        let key = input[i];
        for j in 0..target.len() {
            if target[j] == key {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p119_linear_search() {
        let mut input = [1, 3];
        let mut target = [5, 2, 4, 6, 1, 3];
        assert_eq!(2, p119_linear_search(&mut input, &mut target));
    }
}
