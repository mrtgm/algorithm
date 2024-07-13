// Path: src/bin/sp/p147_koch.rs
use std::f64::consts::PI;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn p147_koch(n: i32, a: Point, b: Point) {
    if n == 0 {
        return;
    }

    let th = PI * 60.0 / 180.0; // 60 degrees in radians

    let s = Point {
        x: (2.0 * a.x + 1.0 * b.x) / 3.0,
        y: (2.0 * a.y + 1.0 * b.y) / 3.0,
    };

    let t = Point {
        x: (1.0 * a.x + 2.0 * b.x) / 3.0,
        y: (1.0 * a.y + 2.0 * b.y) / 3.0,
    };

    let u = Point {
        x: (t.x - s.x) * (th).cos() - (t.y - s.y) * (th).sin() + s.x,
        y: (t.x - s.x) * (th).sin() + (t.y - s.y) * (th).cos() + s.y,
    };

    p147_koch(n - 1, a, s);
    println!("{:.8} {:.8}", s.x, s.y);

    p147_koch(n - 1, s, u);
    println!("{:.8} {:.8}", u.x, u.y);

    p147_koch(n - 1, u, t);
    println!("{:.8} {:.8}", t.x, t.y);

    p147_koch(n - 1, t, b);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p147_koch() {
        let a = Point { x: 0.0, y: 0.0 };
        let b = Point { x: 100.0, y: 0.0 };

        println!("{:.8} {:.8}", a.x, a.y);
        p147_koch(1, a, b);
        println!("{:.8} {:.8}", b.x, b.y);
    }
}
