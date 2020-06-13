pub struct Queue<T: Clone> {
    items: Vec<T>
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

