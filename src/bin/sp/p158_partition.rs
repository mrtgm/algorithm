// Path: src/bin/sp/p158_partition.rs
use std::fmt::Debug;

fn p158_partition<T: Copy + Debug + Ord>(arr: &mut [T]) -> usize {
    // x[r]を取得する
    let key = arr[arr.len() - 1];
    // 左端から順番に比較する、iはx[r]以下の要素 i+1〜jはx[r]より大きい要素

    // iを-1に初期化する
    // jを一つずつ進めていく、x[j]がx[r]以下の場合はiを一つ進めてx[i]とx[j]を交換する
    let mut i = -1;
    for j in 0..arr.len() {
        if arr[j] <= key {
            i += 1;
            arr.swap(i as usize, j);
        }
    }

    // jが最後まで進んだらx[i+1]とx[r]を交換する
    arr.swap((i + 1) as usize, arr.len() - 1);
    println!("{:?}", arr);

    // i+1 を返却する
    i as usize + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p158_partition() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        let result = p158_partition(&mut arr);
        assert_eq!(result, 2);
    }
}
