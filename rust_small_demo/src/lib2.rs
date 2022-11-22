use std::cell::UnsafeCell;

struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn bad() {
        let x = Arc::new(Cell::new(40));

        let x1 = x.clone();
        std::thread::spawn(|| {
            x1.set(41);
        });

        let x2 = x.clone();
        std::thread::spawn(|| {
            x2.set(42);
        });
    }

    fn bad2() {
        let x = Cell::new(vec![12]);
        let first = &x.get()[0];
        x.set(vec![]);
        eprintln!("{}", first);
    }
}
