use std::fmt::Debug;

fn linear_sentinel<T: Debug + Ord + Copy>(arr: &mut [T], item: T) -> usize {
    let last = arr[arr.len() - 1];
    arr[arr.len() - 1] = item;
    let mut i = 0;

    while arr[i] != item {
        i += 1;
    }

    arr[arr.len() - 1] = last;

    if i < (arr.len() - 1) || last == item {
        return i;
    } else {
        return 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_linear_sentinel() {
        let mut a = [1, 2, 3, 4, 5, 6, 7, 8];
        let i = linear_sentinel(&mut a, 9);

        assert_eq!(0, i);
    }
}
