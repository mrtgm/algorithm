// Path: src/bin/rev/binary_search.rs
use std::fmt::Debug;

fn binary_search<T: Copy + Debug + Ord>(arr: &[T], target: T) {
    let mut left = 0;
    let mut right = arr.len();

    loop {
        let mid = (left + right) / 2;

        if arr[mid] == target {
            println!("found {:?}", target);
            return;
        } else if arr[mid] < target {
            if left == arr.len() - 1 {
                break;
            }
            left = mid + 1;
        } else if arr[mid] > target {
            if right == 0 {
                break;
            }
            right = mid - 1;
        }
    }

    println!("not found");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8];
        binary_search(&arr, 4);
        binary_search(&arr, -3);
        binary_search(&arr, 12);
        binary_search(&arr, 8);
        binary_search(&arr, 1);
    }
}
