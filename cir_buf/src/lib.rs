#[warn(unused)]

pub struct CircularQueue<T> {
    list: Vec<Option<T>>,
    front: isize,
    rear: isize,
    size: isize,
}

impl<T: Copy> CircularQueue<T> {
    pub fn new(_size: isize) -> Self {
        Self {
            front: -1,
            rear: -1,
            size: _size,
            list: vec![None; _size as usize],
        }
    }

    pub fn equeue(&mut self, item: T) -> Result<T, String> {
        if (self.front == 0 && self.rear == self.size - 1)
            || (self.front - 1) % self.size == self.rear
        {
            return Err(format!("queue is full"));
        } else if self.front == -1 {
            self.front = 0;
            self.rear = -1;
        }

        self.rear = (self.rear + 1) % self.size;
        self.list[self.rear as usize] = Some(item);
        Ok(item)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.front == -1 {
            return None;
        }

        let i: usize = self.front as usize;
        let val = self.list[i];
        self.list[i] = None;
        self.front = (self.front + 1) % self.size;

        if self.front - 1 == self.rear {
            //no element, so we can reset
            self.front = -1;
            self.rear = -1;
        }
        val
    }
}

#[test]
fn test1() {
    let mut q = CircularQueue::<i32>::new(3);
    assert_eq!(q.dequeue(), None, "dequeue on empty queue returns None");
    assert!(
        q.equeue(2).is_ok(),
        "enqueue insert item and returns Ok(item)"
    );

    assert!(
        q.equeue(3).is_ok(),
        "enqueue insert item and returns Ok(item)"
    );

    assert!(
        q.equeue(4).is_ok(),
        "enqueue insert item and returns Ok(item)"
    );
    assert_eq!(vec![Some(2), Some(3), Some(4)], q.list);
    assert_eq!(
        q.dequeue(),
        Some(2),
        "return the element from  the front oft the queue"
    );
    assert!(
        q.equeue(5).is_ok(),
        "enqueue insert item and returns Ok(item)"
    );
    assert_eq!(vec![Some(5), Some(3), Some(4)], q.list);
    assert_eq!(
        q.dequeue(),
        Some(3),
        "return the element from  the second front oft the queue"
    );
    assert_eq!(
        q.dequeue(),
        Some(4),
        "return the element from  the end oft the queue"
    );
    assert_eq!(
        q.dequeue(),
        Some(5),
        "return the element from  the front oft the queue"
    );
    assert_eq!(q.dequeue(), None, "return None as queue is empty");
    q.equeue(1).unwrap();
}
