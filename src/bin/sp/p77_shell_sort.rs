// Path: src/bin/sp/p77_shell_sort.rs
use std::fmt::Debug;

fn p77_shell_sort<T: Copy + Debug + Ord>(arr: &mut [T]) {
    let mut gap = arr.len() / 2;

    while gap >= 1 {
        for i in gap..arr.len() {
            let mut j = i;
            while j >= gap {
                if arr[j] < arr[j - gap] {
                    arr.swap(j, j - gap);
                }
                j -= gap;
            }
        }
        gap = gap / 2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p77_shell_sort() {
        println!("{:?}", 333);

        let mut arr = [5, 2, 4, 6, 1, 3];
        p77_shell_sort(&mut arr);
        println!("{:?}", arr);

        // assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
