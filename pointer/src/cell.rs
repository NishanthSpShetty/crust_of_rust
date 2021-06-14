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

    pub fn get(&self) -> &mut T {
        unsafe { &mut *self.value.get() }
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

    #[test]
    fn test_bad_pointer() {
        let x = Cell::new(String::from("hello worldmjdjdnbd"));
        //get the pointer to the data
        let first = x.get();
        //while we hold pointer to data, we replace what cell holds it.
        x.set(String::from(
            " lorem epssum ujbjhsdbfjhasdb jghkzxcbcvjkahsdb jkhsdb:World",
        ));
        first.push_str("pushh");
        eprintln!("{}", first);
    }
}
