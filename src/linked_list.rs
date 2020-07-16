use std::rc::Rc;

pub struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Rc<Node<T>>>
}

impl<T: Clone> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None }
    }

    pub fn add_front(&mut self, item: T) {
        match &self.head {
            None => {
                self.head = Some(Rc::new(Node { value: item.clone(), next: None }));
            }
            Some(node) => {
                let new_node = Node { value: item.clone(), next: Some(node.clone()) };
                self.head = Some(Rc::new(new_node));
            }
        }
    }

    pub fn head(&self) -> Option<T> {
        match &self.head {
            Some(x) => Some(x.value.clone()),
            None => None
        }
    }
}