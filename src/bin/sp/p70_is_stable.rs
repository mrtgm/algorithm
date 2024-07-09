// Path: src/bin/sp/p70_is_stable.rs
use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
struct Card {
    suit: char,
    value: i32,
}

fn p58_bubble(arr: &mut [Card]) {
    for i in 0..arr.len() {
        for j in 1..(arr.len() - i) {
            if arr[j].value < arr[j - 1].value {
                arr.swap(j, j - 1);
            }
        }
    }
}

fn p65_selection(arr: &mut [Card]) {
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i..arr.len() {
            if arr[j].value < arr[min_index].value {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

fn p70_is_stable(input: &[Card], output: &[Card]) -> bool {
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            for a in 0..output.len() {
                for b in (a + 1)..output.len() {
                    if input[i].value == input[j].value
                        && input[i].suit == output[b].suit
                        && input[j].suit == output[a].suit
                    {
                        return false;
                    }
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p70_is_stable() {
        let mut arr = [
            Card {
                suit: 'H',
                value: 4,
            },
            Card {
                suit: 'C',
                value: 9,
            },
            Card {
                suit: 'S',
                value: 4,
            },
            Card {
                suit: 'D',
                value: 2,
            },
            Card {
                suit: 'C',
                value: 3,
            },
        ];
        let mut arr2 = arr.clone();
        p58_bubble(&mut arr2);
        println!("{:?}", arr2.iter().map(|x| x.suit).collect::<Vec<char>>());
        println!("{:?}", p70_is_stable(&arr, &arr2));
        // stable
        assert_eq!(true, p70_is_stable(&arr, &arr2));

        let mut arr3 = arr.clone();
        p65_selection(&mut arr3);
        println!("{:?}", arr3.iter().map(|x| x.suit).collect::<Vec<char>>());
        println!("{:?}", p70_is_stable(&arr, &arr3));
        assert_eq!(false, p70_is_stable(&arr, &arr3));

        // not stable
    }
}
