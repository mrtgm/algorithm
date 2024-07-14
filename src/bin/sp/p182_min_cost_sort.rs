// Path: src/bin/sp/p182_min_cost_sort.rs
use std::cmp::min;
use std::fmt::Debug;

fn p182_min_cost_sort(arr: &mut [i32], sorted: &mut [i32]) -> i32 {
    let mut ans = 0;
    let mut min_num = sorted[0];

    for i in 0..arr.len() {
        // find the index of the element in the sorted array
        let mut x = sorted.binary_search(&arr[i]).unwrap_or_else(|x| x);

        // if x == i, then the element is already in the correct position
        if x == i {
            continue;
        }

        let mut sum_cost = 0;
        let mut min_loop_n = 10_000 + 1;
        let mut loop_n = 0;

        // 要素ひとつひとつについて、正しい位置に移動するまでのコストを計算
        // x（正しい位置）と i（現在位置）の添字が一致するまで、arr を入れ替える
        loop {
            loop_n += 1;
            sum_cost += arr[i];

            min_loop_n = min(min_loop_n, arr[i]);
            if x == i {
                break;
            }
            arr.swap(i, x);
            x = sorted.binary_search(&arr[i]).unwrap_or_else(|x| x);
        }

        ans += (sum_cost + (loop_n - 2) * min_loop_n)
            .min(sum_cost + min_loop_n + (loop_n + 1) * min_num);
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p182_min_cost_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        let mut sorted = [1, 2, 3, 4, 5, 6];
        let cost = p182_min_cost_sort(&mut arr, &mut sorted);
        println!("{:?}", cost);
        assert_eq!(cost, 22);
    }
}
