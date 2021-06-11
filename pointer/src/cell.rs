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

#[cfg(test)]
mod test {
    #[test]
    fn test_i32_cell() {
        let cell = super::Cell::new(100);

        assert_eq!(cell.get(), 100);
        cell.set(200);
        assert_eq!(cell.get(), 200);
    }
}
