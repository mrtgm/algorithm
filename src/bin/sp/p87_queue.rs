// Path: src/bin/sp/p87_queue.rs
use anyhow::{Error, Result};
use std::{default, fmt::Debug};

struct Queue<T: Debug> {
    head: usize,
    tail: usize,
    data: Vec<T>,
}

#[derive(Debug, Clone, Default)]
struct Process {
    name: String,
    num: i32,
}

impl<T: Debug + Default + Clone> Queue<T> {
    const SIZE: usize = 10;

    fn new() -> Self {
        Self {
            head: 0,
            tail: 0,
            data: Vec::with_capacity(0),
        }
    }

    fn enqueue(&mut self, data: T) {
        if let Some(_) = self.data.get(self.tail) {
            self.data[self.tail] = data;
        } else {
            self.data.push(data);
        }
        self.tail = (self.tail + 1) % Self::SIZE;
    }

    fn dequeue(&mut self) -> Result<T> {
        if let Some(value) = self.data.get(self.head) {
            self.head = (self.head + 1) % Self::SIZE;
            Ok(value.clone())
        } else {
            Err(anyhow::Error::msg(""))
        }
    }
}

impl<T: Debug + Default + Copy> Debug for Queue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut head = self.head;
        while head != self.tail {
            write!(f, "{:?},", self.data[head])?;
            head = (head + 1) % Self::SIZE;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cmp::min;

    #[test]
    fn test_p87_queue() {
        let mut q: Queue<Process> = Queue::new();

        let mut elapse = 0;

        q.enqueue(Process {
            name: "p1".to_string(),
            num: 150,
        });
        q.enqueue(Process {
            name: "p2".to_string(),
            num: 80,
        });
        q.enqueue(Process {
            name: "p3".to_string(),
            num: 200,
        });
        q.enqueue(Process {
            name: "p4".to_string(),
            num: 350,
        });
        q.enqueue(Process {
            name: "p5".to_string(),
            num: 20,
        });

        while q.head != q.tail {
            let mut top: Process = q.dequeue().unwrap();
            let v = min(top.num, 100);
            top.num -= v;
            elapse += v;
            if top.num > 0 {
                q.enqueue(top);
            } else {
                println!("elapse {}, name: {}", elapse, top.name)
            }
        }
    }
}
