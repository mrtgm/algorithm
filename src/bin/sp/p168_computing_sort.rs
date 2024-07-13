// Path: src/bin/sp/p168_computing_sort.rs
use std::fmt::Debug;

fn p168_computing_sort(arr: &mut [usize]) {
    let mut count = vec![0; 10];
    let mut output = vec![arr[0]; arr.len()];

    for i in 0..arr.len() {
        count[arr[i]] += 1;
    }

    for i in 1..count.len() {
        count[i] += count[i - 1];
    }

    for i in (0..arr.len()).rev() {
        output[count[arr[i]] - 1] = arr[i];
        count[arr[i]] -= 1;
    }

    for i in 0..arr.len() {
        arr[i] = output[i];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p168_computing_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        p168_computing_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
