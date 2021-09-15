use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread::{self, spawn, yield_now};

pub const UNLOCKED: bool = false;
pub const LOCKED: bool = true;

pub struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(value),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {
            //failed to obtain the UNLOCKED state, so instead of asking cpu to get exclusive lock
            //using compare_exchange, we will read from shared cache until its UNLOCKED using busy
            //read loop
            while self.locked.load(Ordering::Relaxed) == LOCKED {}
            //only when it reads as UNLOCKED, we will try taking a lock
        }
        let ret = f(unsafe { &mut *self.v.get() });

        self.locked.store(UNLOCKED, Ordering::Relaxed);
        ret
    }
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

fn main() {
    let l: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    let handlers: Vec<_> = (0..1000)
        .map(|_| {
            thread::spawn(move || {
                for _ in 0..100 {
                    l.with_lock(|v| {
                        *v += 1;
                    })
                }
            })
        })
        .collect();

    for h in handlers {
        h.join().unwrap();
    }

    assert_eq!(l.with_lock(|v| *v), 1000 * 100);
}

#[test]
fn too_relaxed() {
    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let t1 = spawn(move || {
        yield_now();
        let r1 = y.load(Ordering::Acquire);
        x.store(r1, Ordering::Release);
        r1
    });
    let t2 = spawn(move || {
        let r2 = x.load(Ordering::Acquire);
        y.store(42, Ordering::Release);
        r2
    });

    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();
    println!(" r1 {}, r2 {} ", r1, r2);
}
