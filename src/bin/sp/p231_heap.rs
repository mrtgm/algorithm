// Path: src/bin/sp/p231_heap.rs
use std::fmt::Debug;

fn parent(u: usize) -> usize {
    u / 2
}
fn left(u: usize) -> usize {
    2 * u
}
fn right(u: usize) -> usize {
    2 * u + 1
}

fn p231_heap<T: Copy + Debug + Ord>(arr: &[T]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let parent_key = if i == 0 { None } else { arr.get(parent(i)) };
        let left_key = arr.get(left(i));
        let right_key = arr.get(right(i));

        println!(
            "node:{}, key:{:?} parent:{:?} left:{:?} right:{:?}",
            i, key, parent_key, left_key, right_key
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p231_heap() {
        let mut arr = [0, 7, 8, 1, 2, 3];
        p231_heap(&arr);
    }
}
