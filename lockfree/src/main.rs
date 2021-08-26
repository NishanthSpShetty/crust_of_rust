use std::{
    ptr::{self, null_mut},
    sync::atomic::AtomicPtr,
};

//#![feature(box_raw)]

pub struct Stack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            head: AtomicPtr::new(null_mut()),
        }
    }

    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(std::sync::atomic::Ordering::Acquire);

            if head == null_mut() {
                return None;
            } else {
                let next = unsafe { (*head).next };

                if self
                    .head
                    .compare_and_swap(head, next, std::sync::atomic::Ordering::Release)
                    == head
                {
                    return Some(unsafe { ptr::read(&(*head).data) });
                }
            }
        }
    }

    pub fn push(&self, t: T) {
        let n = Box::into_raw(Box::new(Node {
            data: t,
            next: null_mut(),
        }));

        loop {
            let head = self.head.load(std::sync::atomic::Ordering::Relaxed);

            unsafe {
                (*n).next = head;
            }

            //make sure previous head hasnt changed by the time we got here
            if self
                .head
                .compare_and_swap(head, n, std::sync::atomic::Ordering::Release)
                == head
            {
                break;
            }
        }
    }
}

fn main() {}
