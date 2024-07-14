// Path: src/bin/sp/p236_max_heap.rs
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

fn max_heapify<T: Copy + Debug + Ord>(arr: &mut [T], i: usize) {
    // 左の子と右の子、大きい方を選び、現在のノードと比較して、大きい方が親になる
    let left = left(i);
    let right = right(i);
    let mut largest = i; //一旦親を最大値とする

    // 左の子と親を比較
    // 左が配列の範囲を超えてる場合、左の子は存在しないので比較しない
    if left < arr.len() && arr[left] > arr[largest] {
        largest = left;
    }
    // 右の子と親を比較
    // 右が配列の範囲を超えてる場合、右の子は存在しないので比較しない
    if right < arr.len() && arr[right] > arr[largest] {
        largest = right;
    }

    // 親が最大値でない場合、親と最大値を交換し、再帰的に最大ヒープを構築する
    if largest != i {
        arr.swap(i, largest);
        max_heapify(arr, largest);
    }
}

fn build_max_heap<T: Copy + Debug + Ord>(arr: &mut [T]) {
    let mut i = arr.len() / 2;
    while i >= 1 {
        max_heapify(arr, i);
        i -= 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p236_max_heap() {
        let mut arr = [0, 4, 1, 3, 2, 16, 9, 10, 14, 8, 7];

        build_max_heap(&mut arr);

        assert_eq!(arr, [0, 16, 14, 10, 8, 7, 9, 3, 2, 4, 1]);

        let mut arr2 = [0, 2, 6, 3, 1, 7];
        build_max_heap(&mut arr2);
        assert_eq!(arr2, [0, 7, 6, 3, 1, 2]);
    }
}
