// Path: src/bin/macros/intro.rs
use std::fmt::Display;

#[macro_export]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => { //identは変数名にマッチ、先頭から一個づつ再帰的に値を読み取り
                                                    //ttは単一の構文木にマッチ
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

macro_rules! vec_2 {
    ($($x:expr),*) => { //パターンマッチ（マクロパターン）
                        //$(...)でキャプチャ
                        //$x:exprは任意の式(計算すると値を返すもの)にマッチし、その式に$xという名前を与える
                        //マッチしたコードの後に","リテラルが0回以上（"*"）出現する場合にマッチ
        {
            let mut temp_vec = Vec::new();
            //$()* はマッチした回数ごとに生成される、$xが使える
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

macro_rules! map {
    ($($k:expr=>$v:expr),*) => {
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($k, $v);)*
        map
    };
}

pub fn intro() {
    input! {
        n: usize,
        m: usize,
    }
    println!("{:?}", n);
    println!("{:?}", m);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intro() {
        // let mut arr = [5, 2, 4, 6, 1, 3];
        intro();
        // assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
