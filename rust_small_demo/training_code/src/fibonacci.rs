struct Fibonacci {
    a: u64,
    b: u64,
    cur: u8,
    total: u8,
}

impl Fibonacci {
    fn new(total: u8) -> Fibonacci {
        Fibonacci {
            a: 0,
            b: 0,
            cur: 0,
            total,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.total {
            return None;
        }
        if self.a == 0 {
            self.a = 1;
            self.b = 1;
        } else {
            let c = self.a + self.b;
            self.a = self.b;
            self.b = c;
        }
        self.cur += 1;
        Some(self.a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fibonacci_test() {
        let f = Fibonacci::new(8);
        for item in f {
            println!("item is {}", item);
        }
    }
}
