use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct Queue<T: Clone> {
    items: Vec<T>
}

pub struct SafeQueue<T: Clone> {
    reader: Receiver<T>,
    writer: Sender<T>,
}

impl<T: Clone> SafeQueue<T> {
    pub fn new() -> SafeQueue<T> {
        let (tx, rx) = channel::<T>();
        SafeQueue { reader: rx, writer: tx }
    }

    pub fn enqueue(&self, item: T) -> Result<(), String> {
        self.writer
            .send(item)
            .map_err(|err| { String::from("Unable to write item.") })
    }

    pub fn dequeue(&self) -> Option<T> {
        self.reader.try_recv().ok()
    }
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { items: Vec::new() }
    }

    pub fn from_vec(list: Vec<T>) -> Queue<T> {
        let mut clone = list.clone();
        clone.reverse();
        Queue { items: clone }
    }

    pub fn peek(&mut self) -> Option<T> {
        self.items
            .last()
            .map(|x| x.clone())
    }

    pub fn size(&mut self) -> usize {
        self.items.len()
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.insert(0, item)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop().clone()
    }
}