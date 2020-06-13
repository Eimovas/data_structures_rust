pub struct Stack<T : Clone> {
    items : Vec<T>
}

impl<T : Clone> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { items : vec![] }
    }
    
    pub fn add(&mut self, item : T) {
        self.items.push(item);
    }
    
    pub fn remove(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    pub fn peek(&mut self) -> Option<T> {
        self.items
            .last()
            .map(|x| x.clone())
    }
    
    pub fn size(&mut self) -> usize {
        self.items.len()
    }
}