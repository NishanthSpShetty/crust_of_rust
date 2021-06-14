use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

///UnsafeCell is nopt thread safe and cannot be shared across thread boudaries
///it impl !Sync marker which enforces the struct `Cell` also not sync
unsafe impl<T> Sync for Cell<T> {}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread::spawn;

    #[test]
    fn test_i32_cell() {
        let cell = super::Cell::new(100);

        assert_eq!(cell.get(), 100);
        cell.set(200);
        assert_eq!(cell.get(), 200);

        let x = std::sync::Arc::new(Cell::new(100));

        let x1 = x.clone();
        spawn(move || {
            x1.set(200);
        });

        let x2 = x.clone();
        spawn(move || {
            x2.set(200);
        });
    }
}
