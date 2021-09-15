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

    pub fn equeue(&mut self, item: T) {
        if (self.front == 0 && self.rear == self.size - 1)
            || (self.front - 1) % self.size == self.rear
        {
            println!("Queue is full");
            return;
        } else if self.front == -1 {
            self.front = 0;
            self.rear = -1;
        }

        self.rear = (self.rear + 1) % self.size;
        self.list[self.rear as usize] = Some(item);
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
    let mut q = CircularQueue::<i32>::new(4);
    q.equeue(1);
    q.equeue(2);
    q.equeue(3);
    println!(" {:?}", q.list);
    println!(" {:?} ", q.dequeue());
    q.equeue(4);
    println!(" {:?}", q.list);
    println!(" {:?} ", q.dequeue());
    q.equeue(5);
    q.equeue(6);
    println!(" {:?} ", q.dequeue());
    println!(" {:?} ", q.dequeue());
    q.equeue(1);
}
