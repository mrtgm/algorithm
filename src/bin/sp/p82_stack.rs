// Path: src/bin/sp/p82_stack.rs
use std::fmt::Debug;

struct Stack<T: Debug> {
    items: Vec<T>,
    capacity: usize,
}

impl<T: Debug> Stack<T> {
    fn new() -> Self {
        Stack {
            items: Vec::new(),
            capacity: 10,
        }
    }
    fn push(&mut self, data: T) {
        if self.is_full() {
            return;
        }
        self.items.push(data);
    }
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    fn is_full(&self) -> bool {
        self.items.len() == self.capacity
    }

    fn apply<F>(&mut self, fun: F)
    where
        F: Fn(T, T) -> T,
    {
        if let (Some(y), Some(x)) = (self.pop(), self.pop()) {
            let z = fun(x, y);
            self.push(z);
        } else {
            panic!("Stack underflow.");
        }
    }

    fn print(&self) {
        for v in &self.items {
            print!("{:?},", v);
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p82_stack() {
        let input = ['1', '2', '+', '3', '4', '-', '*']; // (1+2)*(3-4) = -3
        let mut stack: Stack<i32> = Stack::new();

        for v in input {
            if let Ok(num) = v.to_string().parse::<i32>() {
                stack.push(num);
            } else {
                match v {
                    '+' => {
                        stack.apply(|a, b| a + b);
                    }
                    '-' => {
                        stack.apply(|a, b| a - b);
                    }
                    '*' => {
                        stack.apply(|a, b| a * b);
                    }
                    _ => {
                        panic!("wtf")
                    }
                }
            }
        }

        let res = stack.pop().unwrap();

        assert_eq!(-3, res);
    }
}
